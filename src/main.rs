pub mod builder;
pub mod circuit;
pub mod devices;
pub mod solver;

use crate::{
    circuit::{Circuit, REF_NODE, Solution, Unknown},
    devices::{
        DeviceId, capacitor::Capacitor, diode::Diode, inductor::Inductor, resistor::Resistor,
        voltage::Voltage,
    },
};
use anyhow::Result;
use evalexpr::*;
use indexmap::IndexMap;
use serde_json::{Map, Value, json};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc::channel},
    thread::{Builder, spawn},
};

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Step,
    Continuous,
    Reset(Option<Value>),
    Update(Value),
}

#[macro_export]
macro_rules! anyhow_trace {
    ($msg:literal $(,)?) => {
        anyhow!($msg).context(format!("文件'{}'的第'{}'行",file!(),line!()))
    };
    ($err:expr $(,)?) => {
        anyhow!($err).context(format!("文件'{}'的第'{}'行",file!(),line!()))
    };
    ($fmt:expr, $($arg:tt)*) => {
        anyhow!($fmt, $($arg)*).context(format!("文件'{}'的第'{}'行",file!(),line!()))
    };
}

pub fn send_to_client(writer: &mut BufWriter<TcpStream>, value: Value) {
    writer
        .write_all((value.to_string() + "\0").as_bytes())
        .unwrap();
    writer.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1025").unwrap();

    loop {
        let stream = if let Ok(t) = listener.accept() {
            t.0
        } else {
            continue;
        };

        Builder::new()
            .spawn(move || {
                println!("Connected {}", stream.peer_addr().unwrap());

                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut writer = BufWriter::new(stream.try_clone().unwrap());

                let state = Arc::new(Mutex::new(State::Idle));
                let state2 = state.clone();

                let pre_state = Arc::new(Mutex::new(State::Idle));
                let pre_state2 = pre_state.clone();

                let (tx, rx) = channel::<Value>();
                let tx2 = tx.clone();

                spawn(move || {
                    for data in rx {
                        send_to_client(&mut writer, data);
                    }
                });

                Builder::new()
                    .spawn(move || {
                        let mut circuit = Circuit::new();
                        let mut t = 0.0;
                        let dt = 0.001;
                        let mut init_value = Solution::new();

                        loop {
                            let Ok(mut state) = state2.try_lock() else {
                                continue;
                            };
                            let Ok(mut pre_state) = pre_state2.try_lock() else {
                                continue;
                            };

                            match (|| -> Result<()> {
                                match &*state {
                                    State::Idle => {}
                                    State::Step => {
                                        init_value = circuit.step(t, dt, &init_value)?;

                                        let mut solution = Map::new();
                                        for (unknown, value) in &init_value {
                                            solution.insert(unknown.to_string(), (*value).into());
                                        }

                                        tx2.send(json!({
                                            "type":"simulate_solution",
                                            "time":t,
                                            "solution":solution,
                                        }))?;

                                        t += dt;

                                        *state = (*pre_state).clone();
                                    }
                                    State::Continuous => {
                                        *state = State::Step;
                                        *pre_state = State::Step;
                                    }
                                    State::Reset(c) => {
                                        if let Some(c) = c {
                                            circuit = Circuit::try_from(c)?;
                                        }
                                        t = 0.0;
                                        init_value = Solution::new();

                                        *state = State::Idle;
                                    }
                                    State::Update(value) => {
                                        let device_id =
                                            value["device_id"].as_u64().unwrap() as DeviceId;
                                        for device in &mut circuit.devices {
                                            if device.device_id() != device_id {
                                                continue;
                                            }
                                            device
                                                .update(value["attributes"].as_object().unwrap())?;
                                            break;
                                        }

                                        *state = (*pre_state).clone();
                                    }
                                }
                                Ok(())
                            })() {
                                Ok(_) => {}
                                Err(e) => {
                                    *state = State::Idle;

                                    println!("Error: {e:?}");
                                    tx2.send(json!({
                                        "type":"error",
                                        "message":format!("{e:?}"),
                                    }))
                                    .unwrap();
                                }
                            }
                        }
                    })
                    .unwrap();

                loop {
                    let mut buf: Vec<u8> = vec![];
                    if match reader.read_until(b'\0', &mut buf) {
                        Ok(t) => t,
                        Err(e) => panic!("{e}"),
                    } == 0
                    {
                        break;
                    }

                    if let Some(&b'\0') = buf.last() {
                        buf.pop();
                    }
                    let buf = String::from_utf8_lossy(&buf);

                    let value = if let Ok(t) = serde_json::from_str::<Value>(&buf) {
                        t
                    } else {
                        continue;
                    };

                    let Some(command) = value["command"].as_str() else {
                        continue;
                    };

                    match (|| -> Result<()> {
                        match command {
                            "reset_simulation" => {
                                *pre_state.lock().unwrap() = State::Idle;
                                *state.lock().unwrap() =
                                    State::Reset(Some(value["circuit"].clone()));
                            }
                            "step_simulation" => {
                                *pre_state.lock().unwrap() = State::Idle;
                                *state.lock().unwrap() = State::Step;
                            }
                            "update_device" => {
                                *pre_state.lock().unwrap() = (*state.lock().unwrap()).clone();
                                *state.lock().unwrap() = State::Update(value.clone());
                            }
                            "start_simulation" => {
                                *pre_state.lock().unwrap() = State::Idle;
                                *state.lock().unwrap() = State::Continuous;
                            }
                            "stop_simulation" => {
                                *pre_state.lock().unwrap() = State::Idle;
                                *state.lock().unwrap() = State::Idle;
                            }
                            _ => unreachable!(),
                        }
                        Ok(())
                    })() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error: {e:?}");
                            tx.send(json!({
                                "type":"error",
                                "message":format!("{e:?}"),
                            }))
                            .unwrap();
                        }
                    }
                }

                println!("Disconnected {}", stream.peer_addr().unwrap());
            })
            .unwrap();
    }
}

pub fn test_rl_oscillator() {
    let inductor = Inductor {
        device_id: 0,
        value: 1.0,
        pos: 0,
        neg: REF_NODE,
        current: 0,
    };
    let capacitor = Capacitor {
        device_id: 1,
        value: 1.0,
        pos: 0,
        neg: REF_NODE,
    };

    let circuit = Circuit {
        devices: vec![Box::new(inductor), Box::new(capacitor)],
    };
    circuit
        .simulate(10.0, Some(Solution::from([(Unknown::Voltage(0), 1.0)])))
        .unwrap();
}

pub fn test_diode() {
    let mut i = -5.0;
    while i <= 5.0 {
        let solution = Circuit {
            devices: vec![
                Box::new(Voltage {
                    device_id: 0,
                    pos: 0,
                    neg: REF_NODE,
                    value_expr: build_operator_tree(&format!("{i}")).unwrap(),
                    current: 0,
                }),
                Box::new(Resistor {
                    device_id: 1,
                    pos: 0,
                    neg: 1,
                    value: 1e3,
                }),
                Box::new(Diode {
                    device_id: 2,
                    pos: 1,
                    neg: REF_NODE,
                    saturation_current: 10e-9,
                    thermal_voltage: 28.85e-3,
                }),
            ],
        }
        .step(0.0, 0.001, &IndexMap::new())
        .unwrap();

        println!("{i}");
        for (unknown, value) in &solution {
            println!("{unknown} = {value}");
        }
        println!();

        i += 0.1;
    }
}
