#pragma once

#include <memory>
#include <vector>

#include "component.h"
#include "variable.h"

using namespace std;

struct Component;
struct Node {
    // 该节点连接的元件
    vector<shared_ptr<Component>> components;
    // 该节点的点位
    shared_ptr<Variable> voltage = nullptr;
};