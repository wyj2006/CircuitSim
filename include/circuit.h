#pragma once

#include <memory>
#include <vector>

#include "component.h"
#include "node.h"

using namespace std;

extern SUNContext sunctx;

struct Circuit {
    vector<shared_ptr<Component>> components;
    vector<shared_ptr<Node>> nodes;

    void simulate();
};