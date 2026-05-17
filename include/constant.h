#pragma once

#include "equation.h"

struct Constant : Equation {
    sunrealtype value;

    virtual sunrealtype evaluate(sunrealtype t, N_Vector y0, N_Vector y1)
    {
        return value;
    }
};