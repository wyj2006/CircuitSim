#pragma once

#include "component.h"

// 电阻
struct Resistor : Component {
    double value;

    Resistor(double value) : value(value) {}
};