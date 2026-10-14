use crate::{
    circuit::{CurrentId, NodeId, Unknown},
    devices::{Device, DeviceId, StampPackage},
};
use indexmap::IndexSet;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct SPDTSwitch {
    pub device_id: DeviceId,
    pub common: NodeId,
    pub throw1: NodeId,
    pub throw2: NodeId,
    pub is_touch1: bool, //是否与触点1连接
    pub current1: CurrentId,
    pub current2: CurrentId,
}

impl Device for SPDTSwitch {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn nodes(&self) -> Vec<NodeId> {
        vec![self.common, self.throw1, self.throw2]
    }

    fn unknown(&self) -> IndexSet<Unknown> {
        IndexSet::from([
            Unknown::Voltage(self.common),
            Unknown::Voltage(self.throw1),
            Unknown::Voltage(self.throw2),
            Unknown::Current(self.current1),
            Unknown::Current(self.current2),
        ])
    }

    fn stamp<'a, 'b>(&'a self, pack: StampPackage<'b>) {
        let StampPackage {
            coef, unknown_map, ..
        } = pack;

        let p = unknown_map[&Unknown::Voltage(self.common)];
        let n1 = unknown_map[&Unknown::Voltage(self.throw1)];
        let n2 = unknown_map[&Unknown::Voltage(self.throw2)];
        let k1 = unknown_map[&Unknown::Current(self.current1)];
        let k2 = unknown_map[&Unknown::Current(self.current2)];

        coef[(p, k1)] += 1.0;
        coef[(p, k2)] += 1.0;

        coef[(n1, k1)] -= 1.0;
        coef[(n2, k2)] -= 1.0;

        if self.is_touch1 {
            coef[(k1, p)] += 1.0;
            coef[(k1, n1)] -= 1.0;

            coef[(k2, k2)] += 1.0;
        } else {
            coef[(k2, p)] += 1.0;
            coef[(k2, n2)] -= 1.0;

            coef[(k1, k1)] += 1.0;
        }
    }

    fn update(&mut self, attributes: &Map<String, Value>) {
        let Some(value) = attributes.get("is_touch1") else {
            return;
        };
        self.is_touch1 = value.as_bool().unwrap();
    }
}
