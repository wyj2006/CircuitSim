#pragma once

#include <memory>
#include <vector>

#include "equation.h"

using namespace std;

struct Add : Equation {
    vector<shared_ptr<Equation>> terms;

    virtual sunrealtype evaluate(sunrealtype t, N_Vector y0, N_Vector y1)
    {
        sunrealtype a = SUN_RCONST(0);
        for (auto term : terms) a += term->evaluate(t, y0, y1);
        return a;
    }
};