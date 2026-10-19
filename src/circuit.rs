use crate::{
    devices::{Device, StampPackage},
    solver::{matrix::Matrix, solve},
};
use evalexpr::*;
use indexmap::{IndexMap, IndexSet};
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
            init_value = self.step(t, dt, &init_value);

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

    pub fn step(&self, time: f64, dt: f64, init_value: &Solution) -> Solution {
        let mut solution = Solution::new();

        for devices in self.split_devices() {
            solution.extend(self.solve(time, dt, init_value, devices));
        }

        solution
    }

    pub fn solve(
        &self,
        time: f64,
        dt: f64,
        init_value: &Solution,
        devices: Vec<&Box<dyn Device>>,
    ) -> Solution {
        let mut iter_value = init_value.clone();
        let mut solution;

        let context = context_map! {
            "t" => float time,
            "ln" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.ln()))
            }),
            "log" => Function::new(|arg: &Value| {
                let args = arg.as_tuple()?;

                if args.len() != 2 {
                    return Err(EvalexprError::WrongFunctionArgumentAmount {
                        expected: 2..=2,
                        actual: args.len(),
                    });
                }

                let a = args[0].as_number()?;
                let b = args[1].as_number()?;

                Ok(Value::from_float(a.log(b)))
            }),
            "log2" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.log2()))
            }),
            "log10" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.log10()))
            }),
            "exp" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.exp()))
            }),
            "exp2" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.exp2()))
            }),
            "pow" => Function::new(|arg: &Value| {
                let args = arg.as_tuple()?;

                if args.len() != 2 {
                    return Err(EvalexprError::WrongFunctionArgumentAmount {
                        expected: 2..=2,
                        actual: args.len(),
                    });
                }

                let a = args[0].as_number()?;
                let b = args[1].as_number()?;

                Ok(Value::from_float(a.powf(b)))
            }),
            "cos" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.cos()))
            }),
            "acos" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.acos()))
            }),
            "cosh" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.cosh()))
            }),
            "acosh" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.acosh()))
            }),
            "sin" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.sin()))
            }),
            "asin" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.asin()))
            }),
            "sinh" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.sinh()))
            }),
            "asinh" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.asinh()))
            }),
            "tan" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.tan()))
            }),
            "atan" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.atan()))
            }),
            "atan2" => Function::new(|arg: &Value| {
                let args = arg.as_tuple()?;

                if args.len() != 2 {
                    return Err(EvalexprError::WrongFunctionArgumentAmount {
                        expected: 2..=2,
                        actual: args.len(),
                    });
                }

                let a = args[0].as_number()?;
                let b = args[1].as_number()?;

                Ok(Value::from_float(a.atan2(b)))
            }),
            "tanh" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.tanh()))
            }),
            "atanh" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.atanh()))
            }),
            "sqrt" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.sqrt()))
            }),
            "cbrt" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.cbrt()))
            }),
            "abs" => Function::new(|arg: &Value| {
                Ok(Value::from_float(arg.as_number()?.abs()))
            }),
        }
        .unwrap();

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
                    context: &context,
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
