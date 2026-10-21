use crate::{
    anyhow_trace,
    circuit::{NodeId, Unknown},
    devices::{Device, DeviceId, StampPackage},
    get_unknown_index,
};
use anyhow::{Result, anyhow};
use indexmap::IndexSet;

#[derive(Debug)]
pub struct Resistor {
    pub device_id: DeviceId,
    pub value: f64, //电导
    pub pos: NodeId,
    pub neg: NodeId,
}

impl Device for Resistor {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn nodes(&self) -> Vec<NodeId> {
        vec![self.pos, self.neg]
    }

    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::from([Unknown::Voltage(self.pos), Unknown::Voltage(self.neg)])
    }

    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) -> Result<()> {
        let StampPackage {
            coef, unknown_map, ..
        } = pack;

        let p = get_unknown_index!(unknown_map, Unknown::Voltage(self.pos))?;
        let n = get_unknown_index!(unknown_map, Unknown::Voltage(self.neg))?;

        coef[(p, p)] += self.value;
        coef[(p, n)] += -self.value;
        coef[(n, p)] += -self.value;
        coef[(n, n)] += self.value;

        Ok(())
    }
}
