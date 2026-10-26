#pragma once

#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "defs.h"
#include "gdextension_interface.h"

using namespace std;

struct Vector2i {
    int x, y;

    Vector2i() : x(0), y(0) {}
    Vector2i(int x, int y) : x(x), y(y) {}

    bool operator==(const Vector2i &other) const
    {
        return x == other.x && y == other.y;
    }

    bool operator<(const Vector2i &other) const
    {
        return x < other.x || (x == other.x && y < other.y);
    }

    Vector2i operator+(const Vector2i &other) const
    {
        return Vector2i(x + other.x, y + other.y);
    }

    Vector2i operator-(const Vector2i &other) const
    {
        return Vector2i(x - other.x, y - other.y);
    }
};

template <>
struct hash<Vector2i> {
    size_t operator()(const Vector2i &v) const noexcept
    {
        size_t h1 = hash<int>{}(v.x);
        size_t h2 = hash<int>{}(v.y);

        return h1 ^ (h2 + 0x9e3779b9 + (h1 << 6) + (h1 >> 2));
    }
};

typedef struct {
    GDExtensionObjectPtr object;
    unordered_set<Vector2i> obstacles;
} OrthogonalRouter;

void orthogonal_router_class_constructor(OrthogonalRouter *self);
void orthogonal_router_class_destructor(OrthogonalRouter *self);
void orthogonal_router_class_bind_methods();

void orthogonal_router_class_clear(OrthogonalRouter *self);
void orthogonal_router_class_set_obstacle(OrthogonalRouter *self,
                                          Vector2i position, bool is_remove);
vector<Vector2i> orthogonal_router_class_find_path(OrthogonalRouter *self,
                                                   Vector2i start,
                                                   Vector2i end);

GDExtensionObjectPtr
orthogonal_router_class_create_instance(void *p_class_userdata);
void orthogonal_router_class_free_instance(
    void *p_class_userdata, GDExtensionClassInstancePtr p_instance);