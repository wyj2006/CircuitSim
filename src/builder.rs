use crate::{
    anyhow_trace,
    circuit::{Circuit, CurrentId, NodeId},
    devices::{
        Device, DeviceId, capacitor::Capacitor, diode::Diode, inductor::Inductor,
        resistor::Resistor, spdt_switch::SPDTSwitch, voltage::Voltage,
    },
};
use anyhow::{Error, anyhow};
use evalexpr::*;
use serde_json::Value;

impl TryFrom<&Value> for Circuit {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let Some(a) = value.as_array() else {
            return Err(anyhow_trace!("格式不正确"));
        };

        let mut devices: Vec<Box<dyn Device>> = vec![];

        for i in a {
            let Some(device_type) = i["type"].as_str() else {
                return Err(anyhow_trace!("缺少'type'字段"));
            };

            let device_id: Result<DeviceId, Self::Error> = match i.get("device_id") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as DeviceId),
                    None => Err(anyhow_trace!("'device_id'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'device_id'字段")),
            };

            let value: Result<f64, Self::Error> = match i.get("value") {
                Some(t) => t.as_f64().ok_or(anyhow_trace!("'value'字段必须为浮点数")),
                None => Err(anyhow_trace!("缺少'value'字段")),
            };
            let saturation_current: Result<f64, Self::Error> = match i.get("saturation_current") {
                Some(t) => t
                    .as_f64()
                    .ok_or(anyhow_trace!("'saturation_current'字段必须为浮点数")),
                None => Err(anyhow_trace!("缺少'saturation_current'字段")),
            };
            let thermal_voltage: Result<f64, Self::Error> = match i.get("thermal_voltage") {
                Some(t) => t
                    .as_f64()
                    .ok_or(anyhow_trace!("'thermal_voltage'字段必须为浮点数")),
                None => Err(anyhow_trace!("缺少'thermal_voltage'字段")),
            };

            let is_touch1: Result<bool, Self::Error> = match i.get("is_touch1") {
                Some(t) => t
                    .as_bool()
                    .ok_or(anyhow_trace!("'is_touch1'字段必须为浮点数")),
                None => Err(anyhow_trace!("缺少'is_touch1'字段")),
            };

            let pos: Result<NodeId, Self::Error> = match i.get("pos") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as NodeId),
                    None => Err(anyhow_trace!("'pos'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'pos'字段")),
            };
            let neg: Result<NodeId, Self::Error> = match i.get("neg") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as NodeId),
                    None => Err(anyhow_trace!("'neg'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'neg'字段")),
            };
            let common: Result<NodeId, Self::Error> = match i.get("common") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as NodeId),
                    None => Err(anyhow_trace!("'common'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'common'字段")),
            };
            let throw1: Result<NodeId, Self::Error> = match i.get("throw1") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as NodeId),
                    None => Err(anyhow_trace!("'throw1'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'throw1'字段")),
            };
            let throw2: Result<NodeId, Self::Error> = match i.get("throw2") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as NodeId),
                    None => Err(anyhow_trace!("'throw2'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'throw2'字段")),
            };

            let current: Result<CurrentId, Self::Error> = match i.get("current") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as CurrentId),
                    None => Err(anyhow_trace!("'current'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'current'字段")),
            };
            let current1: Result<CurrentId, Self::Error> = match i.get("current1") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as CurrentId),
                    None => Err(anyhow_trace!("'current1'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'current1'字段")),
            };
            let current2: Result<CurrentId, Self::Error> = match i.get("current2") {
                Some(t) => match t.as_u64() {
                    Some(t) => Ok(t as CurrentId),
                    None => Err(anyhow_trace!("'current2'字段必须为整数")),
                },
                None => Err(anyhow_trace!("缺少'current2'字段")),
            };

            let value_expr: Result<Node, Self::Error> = match i.get("value_expr") {
                Some(t) => match t.as_str() {
                    Some(t) => {
                        build_operator_tree::<DefaultNumericTypes>(t).map_err(|e| anyhow_trace!(e))
                    }
                    None => Err(anyhow_trace!("'value_expr'字段必须为字符串")),
                },
                None => Err(anyhow_trace!("缺少'value_expr'字段")),
            };

            match device_type {
                "resistor" => {
                    devices.push(Box::new(Resistor {
                        device_id: device_id?,
                        value: value?,
                        pos: pos?,
                        neg: neg?,
                    }));
                }
                "voltage" => {
                    devices.push(Box::new(Voltage {
                        device_id: device_id?,
                        value_expr: value_expr?,
                        pos: pos?,
                        neg: neg?,
                        current: current?,
                    }));
                }
                "capacitor" => {
                    devices.push(Box::new(Capacitor {
                        device_id: device_id?,
                        value: value?,
                        pos: pos?,
                        neg: neg?,
                    }));
                }
                "inductor" => {
                    devices.push(Box::new(Inductor {
                        device_id: device_id?,
                        value: value?,
                        pos: pos?,
                        neg: neg?,
                        current: current?,
                    }));
                }
                "spdt_switch" => {
                    devices.push(Box::new(SPDTSwitch {
                        device_id: device_id?,
                        common: common?,
                        throw1: throw1?,
                        throw2: throw2?,
                        is_touch1: is_touch1?,
                        current1: current1?,
                        current2: current2?,
                    }));
                }
                "diode" => {
                    devices.push(Box::new(Diode {
                        device_id: device_id?,
                        pos: pos?,
                        neg: neg?,
                        saturation_current: saturation_current?,
                        thermal_voltage: thermal_voltage?,
                    }));
                }
                _ => unreachable!(),
            }
        }

        Ok(Circuit { devices })
    }
}
