use crate::{
    devices::{
        Device, StampPackage, capacitor::Capacitor, inductor::Inductor, resistor::Resistor,
        voltage::Voltage,
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

#[derive(Debug)]
pub struct Circuit {
    pub devices: Vec<Box<dyn Device>>,
}

impl Circuit {
    pub fn new() -> Circuit {
        Circuit { devices: vec![] }
    }

    pub fn simulate(&self, max_time: f64, init_value: Option<Solution>) {
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
        let unknowns: IndexSet<_> = devices.iter().map(|x| x.unknown()).flatten().collect();
        let unknowns = unknowns
            .iter()
            .filter(|x| !matches!(x, Unknown::Voltage(REF_NODE)))
            .collect::<IndexSet<_>>();

        let mut unknown_map = IndexMap::new();
        for (i, v) in unknowns.iter().enumerate() {
            unknown_map.insert(**v, i);
        }

        let mut coef = Matrix::zeros(unknowns.len(), unknowns.len());
        let mut rhs = Matrix::zeros(unknowns.len(), 1);

        for device in &devices {
            device.stamp(StampPackage {
                coef: &mut coef,
                rhs: &mut rhs,
                unknown_map: &unknown_map,
                dt,
                init_value: &init_value,
            });
        }

        let x = solve(&coef, &rhs);
        let mut solution = Solution::new();
        for (unknown, index) in unknown_map {
            solution.insert(unknown, x[(index, 0)]);
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
        let mut device_id = 0;

        for i in a {
            let device_type = i["type"].as_str().unwrap();

            match device_type {
                "resistor" => {
                    devices.push(Box::new(Resistor {
                        device_id,
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                    }));
                }
                "voltage" => {
                    devices.push(Box::new(Voltage {
                        device_id,
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                        current: i["current"].as_u64().unwrap() as CurrentId,
                    }));
                }
                "capacitor" => {
                    devices.push(Box::new(Capacitor {
                        device_id,
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                    }));
                }
                "inductor" => {
                    devices.push(Box::new(Inductor {
                        device_id,
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                        current: i["current"].as_u64().unwrap() as CurrentId,
                    }));
                }
                _ => unreachable!(),
            }

            device_id += 1;
        }

        Circuit { devices }
    }
}
