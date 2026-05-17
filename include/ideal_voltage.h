#pragma once

#include "component.h"

struct IdealVoltage : Component {
    double value;

    IdealVoltage(double value) : value(value) {}
};