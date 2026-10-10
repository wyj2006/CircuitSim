pub mod capacitor;
pub mod resistor;
pub mod voltage;

use crate::{
    circuit::{Solution, Unknown},
    solver::matrix::Matrix,
};
use indexmap::{IndexMap, IndexSet};
use std::fmt::Debug;

pub trait Device: Debug {
    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::new()
    }

    #[allow(unused_variables)]
    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) {}
}

pub struct StampPackage<'a> {
    pub coef: &'a mut Matrix,
    pub rhs: &'a mut Matrix,
    pub unknown_map: &'a IndexMap<Unknown, usize>,
    pub dt: f64,
    pub init_value: &'a Solution,
}
