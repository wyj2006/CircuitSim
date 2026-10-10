use crate::{
    circuit::{NodeId, Unknown},
    devices::{Device, StampPackage},
};
use indexmap::IndexSet;

#[derive(Debug)]
pub struct Resistor {
    pub value: f64,
    pub pos: NodeId,
    pub neg: NodeId,
}

impl Device for Resistor {
    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::from([Unknown::Voltage(self.pos), Unknown::Voltage(self.neg)])
    }

    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) {
        let StampPackage {
            coef, unknown_map, ..
        } = pack;

        let p = unknown_map.get(&Unknown::Voltage(self.pos));
        let n = unknown_map.get(&Unknown::Voltage(self.neg));

        if let Some(p) = p
            && let Some(n) = n
        {
            coef[(*p, *p)] += self.value;
            coef[(*p, *n)] += -self.value;
            coef[(*n, *p)] += -self.value;
            coef[(*n, *n)] += self.value;
        } else if let Some(p) = p {
            coef[(*p, *p)] += self.value;
        } else if let Some(n) = n {
            coef[(*n, *n)] += self.value;
        }
    }
}
