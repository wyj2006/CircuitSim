use crate::{
    devices::{
        Device, StampPackage, capacitor::Capacitor, inductor::Inductor, resistor::Resistor,
        voltage::Voltage,
    },
    solver::{matrix::Matrix, solve},
};
use indexmap::{IndexMap, IndexSet};
use serde_json::Value;
use std::fmt::Display;

pub type NodeId = u32;
pub type BranchId = i32;
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

    pub fn step(&self, dt: f64, init_value: &Solution) -> Solution {
        let unknowns: IndexSet<_> = self.devices.iter().map(|x| x.unknown()).flatten().collect();
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

        for device in &self.devices {
            device.stamp(StampPackage {
                coef: &mut coef,
                rhs: &mut rhs,
                unknown_map: &unknown_map,
                dt,
                init_value: &init_value,
            });
        }

        let x = solve(&coef, &rhs);
        let mut solution = IndexMap::new();
        for (unknown, index) in unknown_map {
            solution.insert(unknown, x[(index, 0)]);
        }
        solution
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Unknown {
    Voltage(NodeId),
    Current(BranchId),
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
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                    }));
                }
                "voltage" => {
                    devices.push(Box::new(Voltage {
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                        branch: i["branch"].as_u64().unwrap() as BranchId,
                    }));
                }
                "capacitor" => {
                    devices.push(Box::new(Capacitor {
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                    }));
                }
                "inductor" => {
                    devices.push(Box::new(Inductor {
                        value: i["value"].as_f64().unwrap(),
                        pos: i["pos"].as_u64().unwrap() as NodeId,
                        neg: i["neg"].as_u64().unwrap() as NodeId,
                        branch: i["branch"].as_u64().unwrap() as BranchId,
                    }));
                }
                _ => unreachable!(),
            }
        }

        Circuit { devices }
    }
}
