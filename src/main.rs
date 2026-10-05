pub mod circuit;
pub mod components;
pub mod solver;

use crate::{
    circuit::{Circuit, REF_NODE},
    components::{capacitor::Capacitor, resistor::Resistor, voltage::Voltage},
};

fn main() {
    let resistor = Resistor {
        value: 1.0,
        pos: 0,
        neg: REF_NODE,
    };
    let voltage = Voltage {
        value: 1.0,
        pos: 0,
        neg: REF_NODE,
        branch: 0,
    };
    let capacitor = Capacitor {
        value: 1.0,
        pos: 0,
        neg: REF_NODE,
    };
    let circuit = Circuit {
        components: vec![Box::new(resistor), Box::new(voltage), Box::new(capacitor)],
    };
    circuit.simulate(10.0);
}
