#pragma once

#include <memory>

#include "equation.h"
#include "variable.h"

using namespace std;

// 对变量t的一阶导
struct Derivative : Equation {
    shared_ptr<Variable> var;

    virtual sunrealtype evaluate(sunrealtype t, N_Vector y0, N_Vector y1)
    {
        return NV_Ith_S(y1, var->index);
    }
};