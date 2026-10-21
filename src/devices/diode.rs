use anyhow::{Result, anyhow};
use indexmap::IndexSet;

use crate::{
    anyhow_trace,
    circuit::{NodeId, Unknown},
    devices::{Device, DeviceId, StampPackage},
    get_unknown_index,
};

#[derive(Debug)]
pub struct Diode {
    pub device_id: DeviceId,
    pub pos: NodeId,
    pub neg: NodeId,
    pub saturation_current: f64,
    pub thermal_voltage: f64,
}

impl Device for Diode {
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
            rhs,
            unknown_map,
            iter_value,
            ..
        } = pack;

        let a = get_unknown_index!(unknown_map, Unknown::Voltage(self.pos))?;
        let b = get_unknown_index!(unknown_map, Unknown::Voltage(self.neg))?;

        let vd = *iter_value.get(&Unknown::Voltage(self.pos)).unwrap_or(&0.0)
            - *iter_value.get(&Unknown::Voltage(self.neg)).unwrap_or(&0.0);
        let g = self.saturation_current * (vd / self.thermal_voltage).exp() / self.thermal_voltage;
        let id = self.saturation_current * ((vd / self.thermal_voltage).exp() - 1.0);
        let ieq = id - g * vd;

        coef[(a, a)] += g;
        coef[(a, b)] -= g;
        coef[(b, a)] -= g;
        coef[(b, b)] += g;

        rhs[(a, 0)] -= ieq;
        rhs[(b, 0)] += ieq;

        Ok(())
    }
}
