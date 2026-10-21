use crate::{
    anyhow_trace,
    circuit::{NodeId, Unknown},
    devices::{Device, DeviceId, StampPackage},
    get_unknown_index,
};
use anyhow::{Result, anyhow};
use indexmap::IndexSet;

#[derive(Debug)]
pub struct Capacitor {
    pub device_id: DeviceId,
    pub value: f64,
    pub pos: NodeId,
    pub neg: NodeId,
}

impl Device for Capacitor {
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
            coef,
            unknown_map,
            rhs,
            dt,
            init_value,
            ..
        } = pack;

        let g = self.value / dt;
        let p = get_unknown_index!(unknown_map, Unknown::Voltage(self.pos))?;
        let n = get_unknown_index!(unknown_map, Unknown::Voltage(self.neg))?;
        let old = init_value.get(&Unknown::Voltage(self.pos)).unwrap_or(&0.0)
            - init_value.get(&Unknown::Voltage(self.neg)).unwrap_or(&0.0);

        coef[(p, p)] += g;
        coef[(p, n)] += -g;
        coef[(n, p)] += -g;
        coef[(n, n)] += g;

        rhs[(p, 0)] += g * old;
        rhs[(n, 0)] -= g * old;

        Ok(())
    }
}
