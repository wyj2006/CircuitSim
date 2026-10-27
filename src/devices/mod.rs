pub mod capacitor;
pub mod current_source;
pub mod diode;
pub mod inductor;
pub mod resistor;
pub mod spdt_switch;
pub mod voltage_source;

use crate::{
    circuit::{NodeId, Solution, Unknown},
    solver::matrix::Matrix,
};
use anyhow::Result;
use evalexpr::*;
use indexmap::{IndexMap, IndexSet};
use serde_json::{Map, Value};
use std::fmt::Debug;

pub type DeviceId = u32;

#[macro_export]
macro_rules! get_unknown_index {
    ($unknown_map:expr, $unknown:expr) => {
        match $unknown_map.get(&$unknown) {
            Some(t) => Ok(*t),
            None => Err(anyhow_trace!("未定义的未知量: {}", $unknown)),
        }
    };
}

pub trait Device: Debug {
    fn device_id(&self) -> DeviceId;
    fn nodes(&self) -> Vec<NodeId>;

    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::new()
    }

    #[allow(unused_variables)]
    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn update(&mut self, attributes: &Map<String, Value>) -> Result<()> {
        Ok(())
    }
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
