#pragma once

#include <nvector/nvector_serial.h>

struct Equation {
    virtual sunrealtype evaluate(sunrealtype t, N_Vector y0, N_Vector y1) = 0;
};