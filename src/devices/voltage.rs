use crate::{
    circuit::{CurrentId, NodeId, Unknown},
    devices::{Device, DeviceId, StampPackage},
};
use indexmap::IndexSet;

#[derive(Debug)]
pub struct Voltage {
    pub device_id: DeviceId,
    pub value: f64,
    pub pos: NodeId,
    pub neg: NodeId,
    pub current: CurrentId,
}

impl Device for Voltage {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn nodes(&self) -> Vec<NodeId> {
        vec![self.pos, self.neg]
    }

    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::from([
            Unknown::Voltage(self.pos),
            Unknown::Voltage(self.neg),
            Unknown::Current(self.current),
        ])
    }

    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) {
        let StampPackage {
            coef,
            rhs,
            unknown_map,
            ..
        } = pack;

        let k = unknown_map[&Unknown::Current(self.current)];
        let p = unknown_map[&Unknown::Voltage(self.pos)];
        let n = unknown_map[&Unknown::Voltage(self.neg)];

        coef[(p, k)] += 1.0;
        coef[(k, p)] += 1.0;

        coef[(n, k)] -= 1.0;
        coef[(k, n)] -= 1.0;

        rhs[(k, 0)] += self.value;
    }
}
