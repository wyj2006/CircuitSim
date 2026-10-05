use crate::{
    components::{Component, StampPackage},
    solver::{matrix::Matrix, solve},
};
use indexmap::{IndexMap, IndexSet};
use std::fmt::Display;

pub type NodeId = usize;
pub type BranchId = usize;
pub type Solution = IndexMap<Unknown, f64>;

pub const REF_NODE: NodeId = NodeId::MAX;

#[derive(Debug)]
pub struct Circuit {
    pub components: Vec<Box<dyn Component>>,
}

impl Circuit {
    pub fn simulate(&self, max_time: f64) {
        let mut t = 0.0;
        let dt = 0.001;
        let mut init_value = Solution::new();

        while t < max_time {
            init_value = self.step(dt, &init_value);

            println!("{t}");
            for (unknown, value) in &init_value {
                println!("{unknown} = {value}")
            }
            println!();

            t += dt;
        }
    }

    pub fn step(&self, dt: f64, init_value: &Solution) -> Solution {
        let unknowns: IndexSet<_> = self
            .components
            .iter()
            .map(|x| x.unknown())
            .flatten()
            .collect();
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

        for component in &self.components {
            component.stamp(StampPackage {
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
