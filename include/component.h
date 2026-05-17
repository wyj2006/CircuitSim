#pragma once

#include <memory>
#include <vector>

#include <ida/ida.hpp>

#include "node.h"
#include "variable.h"

using namespace std;

struct Node;
struct Component {
    // 元件连接的节点
    vector<shared_ptr<Node>> nodes;
    shared_ptr<Variable> current;
};