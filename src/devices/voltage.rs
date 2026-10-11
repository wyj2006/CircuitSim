use crate::{
    circuit::{CurrentId, NodeId, Unknown},
    devices::{Device, StampPackage},
};
use indexmap::IndexSet;

#[derive(Debug)]
pub struct Voltage {
    pub value: f64,
    pub pos: NodeId,
    pub neg: NodeId,
    pub current: CurrentId,
}

impl Device for Voltage {
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
        if let Some(p) = unknown_map.get(&Unknown::Voltage(self.pos)) {
            coef[(*p, k)] += 1.0;
            coef[(k, *p)] += 1.0;
        }
        if let Some(n) = unknown_map.get(&Unknown::Voltage(self.neg)) {
            coef[(*n, k)] -= 1.0;
            coef[(k, *n)] -= 1.0;
        }
        rhs[(k, 0)] += self.value;
    }
}
