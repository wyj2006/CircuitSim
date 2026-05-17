#include <ida/ida.hpp>

#include "circuit.h"
#include "component.h"
#include "ideal_voltage.h"
#include "node.h"
#include "resistor.h"

int main()
{
    SUNContext_Create(SUN_COMM_NULL, &sunctx);

    shared_ptr<IdealVoltage> voltage = make_shared<IdealVoltage>(3.0);
    shared_ptr<Resistor> resistor = make_shared<Resistor>(1.0);

    shared_ptr<Node> nodes[2] = {make_shared<Node>(), make_shared<Node>()};
    nodes[0]->components.push_back(voltage);
    nodes[0]->components.push_back(resistor);
    nodes[1]->components.push_back(voltage);
    nodes[1]->components.push_back(resistor);

    voltage->nodes.push_back(nodes[0]);
    resistor->nodes.push_back(nodes[0]);
    voltage->nodes.push_back(nodes[1]);
    resistor->nodes.push_back(nodes[1]);

    Circuit circuit;
    circuit.components.push_back(voltage);
    circuit.components.push_back(resistor);
    circuit.nodes.push_back(nodes[0]);
    circuit.nodes.push_back(nodes[1]);

    circuit.simulate();
}