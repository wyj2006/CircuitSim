use crate::{
    devices::{
        Device, DeviceId, StampPackage, capacitor::Capacitor, diode::Diode, inductor::Inductor,
        resistor::Resistor, spdt_switch::SPDTSwitch, voltage::Voltage,
    },
    solver::{matrix::Matrix, solve},
};
use indexmap::{IndexMap, IndexSet};
use serde_json::Value;
use std::{collections::VecDeque, fmt::Display};

pub type NodeId = u32;
pub type CurrentId = u32;
pub type Solution = IndexMap<Unknown, f64>;

pub const REF_NODE: NodeId = NodeId::MAX;
pub const REL_LIMIT: f64 = 1e-3;
pub const V_ABS_LIMIT: f64 = 1e-6;
pub const I_ABS_LIMIT: f64 = 10e-12;

#[derive(Debug)]
pub struct Circuit {
    pub devices: Vec<Box<dyn Device>>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit { devices: vec![] }
    }

    pub fn simulate(&self, max_time: f64, init_value: Option<Solution>) -> Solution {
        let mut t = 0.0;
        let dt = 0.001;
        let mut init_value = init_value.unwrap_or(Solution::new());

        while t < max_time {
            init_value = self.step(dt, &init_value);

            println!("{t}");
            for (unknown, value) in &init_value {
                println!("{unknown} = {value}");
            }
            println!();

            t += dt;
        }

        init_value
    }

    pub fn split_devices(&self) -> Vec<Vec<&Box<dyn Device>>> {
        let mut device_map = IndexMap::new();
        let mut node_connection = IndexMap::new();

        for device in &self.devices {
            device_map.insert(device.device_id(), device);

            for node in device.nodes() {
                if !node_connection.contains_key(&node) {
                    node_connection.insert(node, vec![]);
                }
                node_connection[&node].push(device);
            }
        }

        let mut group_map = IndexMap::new();
        let mut scanned = vec![];
        let mut group_id = 0;

        for device in &self.devices {
            if scanned.contains(&device.device_id()) {
                continue;
            }

            let mut queue = VecDeque::from([device]);

            while let Some(device) = queue.pop_front() {
                let device_id = device.device_id();
                if scanned.contains(&device_id) {
                    continue;
                }
                scanned.push(device_id);
                group_map.insert(device_id, group_id);
                for node in device.nodes() {
                    queue.extend(&node_connection[&node]);
                }
            }

            group_id += 1;
        }

        let mut groups = vec![vec![]; group_id];

        for (device_id, group_id) in &group_map {
            groups[*group_id].push(device_map[device_id]);
        }

        groups.retain(|x| !x.is_empty());

        groups
    }

    pub fn step(&self, dt: f64, init_value: &Solution) -> Solution {
        let mut solution = Solution::new();

        for devices in self.split_devices() {
            solution.extend(self.solve(dt, init_value, devices));
        }

        solution
    }

    pub fn solve(
        &self,
        dt: f64,
        init_value: &Solution,
        devices: Vec<&Box<dyn Device>>,
    ) -> Solution {
        let mut iter_value = init_value.clone();
        let mut solution;

        let unknowns: IndexSet<_> = devices.iter().map(|x| x.unknown()).flatten().collect();
        let unknowns = unknowns
            .iter()
            .filter(|x| !matches!(x, Unknown::Voltage(REF_NODE)))
            .collect::<IndexSet<_>>();

        let mut unknown_map = IndexMap::new();
        for (i, v) in unknowns.iter().enumerate() {
            unknown_map.insert(**v, i);
        }
        //参考节点的未知量永远在所有未知量的最后
        unknown_map.insert(Unknown::Voltage(REF_NODE), unknowns.len());

        loop {
            let mut coef_with_ref = Matrix::zeros(unknown_map.len(), unknown_map.len());
            let mut rhs_with_ref = Matrix::zeros(unknown_map.len(), 1);

            for device in &devices {
                device.stamp(StampPackage {
                    coef: &mut coef_with_ref,
                    rhs: &mut rhs_with_ref,
                    unknown_map: &unknown_map,
                    dt,
                    init_value: init_value,
                    iter_value: &iter_value,
                });
            }

            //从方程中删除参考节点相关的部分
            let mut coef = Matrix::zeros(unknown_map.len() - 1, unknown_map.len() - 1);
            let mut rhs = Matrix::zeros(unknown_map.len() - 1, 1);
            let ref_index = *unknown_map.get(&Unknown::Voltage(REF_NODE)).unwrap();

            for ((i, j), value) in coef_with_ref.data {
                if i == ref_index || j == ref_index {
                    continue;
                }
                let x = if i > ref_index { i - 1 } else { i };
                let y = if j > ref_index { j - 1 } else { j };
                coef[(x, y)] = value;
            }

            for ((i, j), value) in rhs_with_ref.data {
                assert!(j == 0);
                if i == ref_index {
                    continue;
                }
                rhs[(if i > ref_index { i - 1 } else { i }, 0)] = value;
            }

            let x = solve(&coef, &rhs);
            solution = Solution::new();
            for (unknown, index) in &unknown_map {
                if *index == ref_index {
                    continue;
                }
                solution.insert(
                    *unknown,
                    x[(
                        if *index > ref_index {
                            *index - 1
                        } else {
                            *index
                        },
                        0,
                    )],
                );
            }

            if solution.iter().all(|(unknown, value)| {
                let a = *value;
                let Some(&b) = iter_value.get(unknown) else {
                    return false;
                };
                if !a.is_finite() || !b.is_finite() {
                    //求解失败
                    return true;
                }

                (a - b).abs()
                    <= (REL_LIMIT * a.abs().max(b.abs())
                        + match unknown {
                            Unknown::Voltage(..) => V_ABS_LIMIT,
                            Unknown::Current(..) => I_ABS_LIMIT,
                        })
            }) {
                break;
            }

            iter_value = solution;
        }
        solution
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Unknown {
    Voltage(NodeId),
    Current(CurrentId),
}

impl Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unknown::Voltage(a) => write!(f, "u{a}"),
            Unknown::Current(a) => write!(f, "i{a}"),
        }
    }
}

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
                        value: i["value"].as_f64().unwrap(),
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
