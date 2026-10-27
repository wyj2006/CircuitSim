use crate::{
    anyhow_trace,
    circuit::{NodeId, Unknown},
    devices::{Device, DeviceId, StampPackage},
    get_unknown_index,
};
use anyhow::{Result, anyhow};
use evalexpr::{Node, Value};
use indexmap::IndexSet;

#[derive(Debug)]
pub struct CurrentSource {
    pub device_id: DeviceId,
    pub value_expr: Node,
    pub pos: NodeId,
    pub neg: NodeId,
}

impl Device for CurrentSource {
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
            rhs,
            unknown_map,
            context,
            ..
        } = pack;

        let p = get_unknown_index!(unknown_map, Unknown::Voltage(self.pos))?;
        let n = get_unknown_index!(unknown_map, Unknown::Voltage(self.neg))?;

        let current = match self.value_expr.eval_with_context(context).unwrap() {
            Value::Boolean(a) => a as i64 as f64,
            Value::Float(a) => a,
            Value::Int(a) => a as f64,
            _ => return Err(anyhow_trace!("表达式的计算结果无法表示成数字")),
        };

        rhs[(p, 0)] -= current;
        rhs[(n, 0)] += current;

        Ok(())
    }
}
