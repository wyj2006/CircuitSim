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
use evalexpr::*;
use indexmap::IndexMap;
use serde_json::{Map, Value, json};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpListener,
    thread::Builder,
};

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
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);

                let mut circuit = Circuit::new();
                let mut t = 0.0;
                let dt = 0.001;
                let mut init_value = Solution::new();

                println!("Connected {}", stream.peer_addr().unwrap());

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

                    let command = value["command"].as_str().unwrap();
                    match command {
                        "reset_simulation" => {
                            circuit = Circuit::from(&value["circuit"]);
                            t = 0.0;
                            init_value = Solution::new();
                        }
                        "step_simulation" => {
                            init_value = circuit.step(t, dt, &init_value);

                            let mut solution = Map::new();
                            for (unknown, value) in &init_value {
                                solution.insert(unknown.to_string(), (*value).into());
                            }
                            println!("{solution:?}");

                            writer
                                .write_all(
                                    json!({
                                        "type":"simulate_solution",
                                        "time":t,
                                        "solution":solution,
                                    })
                                    .to_string()
                                    .as_bytes(),
                                )
                                .unwrap();
                            writer.flush().unwrap();

                            t += dt;
                        }
                        "update_device" => {
                            let device_id = value["device_id"].as_u64().unwrap() as DeviceId;
                            for device in &mut circuit.devices {
                                if device.device_id() != device_id {
                                    continue;
                                }
                                device.update(value["attributes"].as_object().unwrap());
                                break;
                            }
                        }
                        _ => unreachable!(),
                    }

                    println!("{circuit:#?}");
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
    circuit.simulate(10.0, Some(Solution::from([(Unknown::Voltage(0), 1.0)])));
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
        .step(0.0, 0.001, &IndexMap::new());

        println!("{i}");
        for (unknown, value) in &solution {
            println!("{unknown} = {value}");
        }
        println!();

        i += 0.1;
    }
}
