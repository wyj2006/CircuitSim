use crate::{
    circuit::{NodeId, Unknown},
    components::{Component, StampPackage},
};
use indexmap::IndexSet;

#[derive(Debug)]
pub struct Capacitor {
    pub value: f64,
    pub pos: NodeId,
    pub neg: NodeId,
}

impl Component for Capacitor {
    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::from([Unknown::Voltage(self.pos), Unknown::Voltage(self.neg)])
    }

    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) {
        let StampPackage {
            coef,
            unknown_map,
            rhs,
            dt,
            init_value,
            ..
        } = pack;

        let g = self.value / dt;
        let p = unknown_map.get(&Unknown::Voltage(self.pos));
        let n = unknown_map.get(&Unknown::Voltage(self.neg));
        let old = init_value.get(&Unknown::Voltage(self.pos)).unwrap_or(&0.0)
            - init_value.get(&Unknown::Voltage(self.neg)).unwrap_or(&0.0);

        if let Some(p) = p
            && let Some(n) = n
        {
            coef[(*p, *p)] += g;
            coef[(*p, *n)] += -g;
            coef[(*n, *p)] += -g;
            coef[(*n, *n)] += g;

            rhs[(*p, 0)] += g * old;
            rhs[(*n, 0)] -= g * old;
        } else if let Some(p) = p {
            coef[(*p, *p)] += g;

            rhs[(*p, 0)] += g * old;
        } else if let Some(n) = n {
            coef[(*n, *n)] += g;

            rhs[(*n, 0)] -= g * old;
        }
    }
}
