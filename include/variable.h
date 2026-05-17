#pragma once

#include "equation.h"

struct Variable : Equation {
    size_t index;

    Variable(size_t index) : index(index) {}

    virtual sunrealtype evaluate(sunrealtype t, N_Vector y0, N_Vector y1)
    {
        return NV_Ith_S(y0, index);
    }
};