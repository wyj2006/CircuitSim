pub mod capacitor;
pub mod diode;
pub mod inductor;
pub mod resistor;
pub mod spdt_switch;
pub mod voltage;

use crate::{
    circuit::{NodeId, Solution, Unknown},
    solver::matrix::Matrix,
};
use evalexpr::*;
use indexmap::{IndexMap, IndexSet};
use serde_json::{Map, Value};
use std::fmt::Debug;

pub type DeviceId = u32;

pub trait Device: Debug {
    fn device_id(&self) -> DeviceId;
    fn nodes(&self) -> Vec<NodeId>;

    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::new()
    }

    #[allow(unused_variables)]
    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) {}

    #[allow(unused_variables)]
    fn update(&mut self, attributes: &Map<String, Value>) {}
}

pub struct StampPackage<'a> {
    pub coef: &'a mut Matrix,
    pub rhs: &'a mut Matrix,
    pub unknown_map: &'a IndexMap<Unknown, usize>,
    pub dt: f64,
    pub init_value: &'a Solution,
    pub iter_value: &'a Solution,
    pub context: &'a HashMapContext<DefaultNumericTypes>,
}
