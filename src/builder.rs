use crate::{
    circuit::{Circuit, CurrentId, NodeId},
    devices::{
        Device, DeviceId, capacitor::Capacitor, diode::Diode, inductor::Inductor,
        resistor::Resistor, spdt_switch::SPDTSwitch, voltage::Voltage,
    },
};
use evalexpr::*;
use serde_json::Value;

impl From<&Value> for Circuit {
    fn from(value: &Value) -> Self {
        let a = value.as_array().unwrap();

        let mut devices: Vec<Box<dyn Device>> = vec![];

        for i in a {
            let device_type = i["type"].as_str().unwrap();

            match device_type {
                "resistor" => {
                    devices.push(Box::new(Resistor {
                        device_id: i["device_id"].as_u64().unwrap() as DeviceId,
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                    }));
                }
                "voltage" => {
                    devices.push(Box::new(Voltage {
                        device_id: i["device_id"].as_u64().unwrap() as DeviceId,
                        value_expr: build_operator_tree(i["value_expr"].as_str().unwrap()).unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                        current: i["current"].as_u64().unwrap() as CurrentId,
                    }));
                }
                "capacitor" => {
                    devices.push(Box::new(Capacitor {
                        device_id: i["device_id"].as_u64().unwrap() as DeviceId,
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                    }));
                }
                "inductor" => {
                    devices.push(Box::new(Inductor {
                        device_id: i["device_id"].as_u64().unwrap() as DeviceId,
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                        current: i["current"].as_u64().unwrap() as CurrentId,
                    }));
                }
                "spdt_switch" => {
                    devices.push(Box::new(SPDTSwitch {
                        device_id: i["device_id"].as_u64().unwrap() as DeviceId,
                        common: i["common"].as_f64().unwrap() as NodeId,
                        throw1: i["throw1"].as_u64().unwrap() as NodeId,
                        throw2: i["throw2"].as_u64().unwrap() as NodeId,
                        is_touch1: i["is_touch1"].as_bool().unwrap(),
                        current1: i["current1"].as_u64().unwrap() as CurrentId,
                        current2: i["current2"].as_u64().unwrap() as CurrentId,
                    }));
                }
                "diode" => {
                    devices.push(Box::new(Diode {
                        device_id: i["device_id"].as_u64().unwrap() as DeviceId,
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                        saturation_current: i["saturation_current"].as_f64().unwrap(),
                        thermal_voltage: i["thermal_voltage"].as_f64().unwrap(),
                    }));
                }
                _ => unreachable!(),
            }
        }

        Circuit { devices }
    }
}
