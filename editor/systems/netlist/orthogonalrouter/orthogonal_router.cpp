#include "orthogonal_router.h"
#include "api.h"

#include <cstddef>
#include <cstdint>

constexpr size_t VARIANT_SIZE = 24;
constexpr GDExtensionInt ARRAY_APPEND_HASH = 3316032543;

void set_call_error(GDExtensionCallError *error, GDExtensionCallErrorType type,
                    int32_t argument = 0, int32_t expected = 0)
{
    if (error == NULL) return;
    error->error = type;
    error->argument = argument;
    error->expected = expected;
}

void set_nil_return(GDExtensionVariantPtr return_value)
{
    if (return_value != NULL)
        variant_construct(GDEXTENSION_VARIANT_TYPE_NIL, return_value, NULL, 0,
                          NULL);
}

void append_path_to_array(void *array, const vector<Vector2i> &path)
{
    StringName append_name;
    string_name_new_with_latin1_chars(&append_name, "append", false);
    GDExtensionPtrBuiltInMethod append = variant_get_ptr_builtin_method(
        GDEXTENSION_VARIANT_TYPE_ARRAY, &append_name, ARRAY_APPEND_HASH);
    string_name_destructor(&append_name);

    GDExtensionVariantFromTypeConstructorFunc make_vector2i_variant =
        get_variant_from_type_constructor(GDEXTENSION_VARIANT_TYPE_VECTOR2I);
    for (const Vector2i &point : path)
    {
        alignas(std::max_align_t) uint8_t point_variant[VARIANT_SIZE];
        make_vector2i_variant(point_variant, (GDExtensionTypePtr)&point);
        const GDExtensionConstTypePtr args[] = {point_variant};
        append(array, args, NULL, 1);
        variant_destroy(point_variant);
    }
}

void orthogonal_router_clear_call(void *, GDExtensionClassInstancePtr instance,
                                  const GDExtensionConstVariantPtr *args,
                                  GDExtensionInt argument_count,
                                  GDExtensionVariantPtr return_value,
                                  GDExtensionCallError *error)
{
    if (argument_count != 0)
    {
        set_call_error(error, argument_count < 0 ?
                                  GDEXTENSION_CALL_ERROR_TOO_FEW_ARGUMENTS :
                                  GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS);
        set_nil_return(return_value);
        return;
    }
    orthogonal_router_class_clear((OrthogonalRouter *)instance);
    set_nil_return(return_value);
    set_call_error(error, GDEXTENSION_CALL_OK);
}

void orthogonal_router_clear_ptrcall(void *,
                                     GDExtensionClassInstancePtr instance,
                                     const GDExtensionConstTypePtr *,
                                     GDExtensionTypePtr)
{
    orthogonal_router_class_clear((OrthogonalRouter *)instance);
}

void orthogonal_router_set_obstacle_call(void *,
                                         GDExtensionClassInstancePtr instance,
                                         const GDExtensionConstVariantPtr *args,
                                         GDExtensionInt argument_count,
                                         GDExtensionVariantPtr return_value,
                                         GDExtensionCallError *error)
{
    if (argument_count != 2)
    {
        set_call_error(error, argument_count < 2 ?
                                  GDEXTENSION_CALL_ERROR_TOO_FEW_ARGUMENTS :
                                  GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS);
        set_nil_return(return_value);
        return;
    }
    for (int i = 0; i < 2; i++)
    {
        const GDExtensionVariantType expected[] = {
            GDEXTENSION_VARIANT_TYPE_VECTOR2I, GDEXTENSION_VARIANT_TYPE_BOOL};
        if (variant_get_type(args[i]) != expected[i])
        {
            set_call_error(error, GDEXTENSION_CALL_ERROR_INVALID_ARGUMENT, i,
                           expected[i]);
            set_nil_return(return_value);
            return;
        }
    }

    Vector2i position;
    GDExtensionBool is_remove;
    get_variant_to_type_constructor(GDEXTENSION_VARIANT_TYPE_VECTOR2I)(
        &position, const_cast<GDExtensionVariantPtr>(args[0]));
    get_variant_to_type_constructor(GDEXTENSION_VARIANT_TYPE_BOOL)(
        &is_remove, const_cast<GDExtensionVariantPtr>(args[1]));
    orthogonal_router_class_set_obstacle((OrthogonalRouter *)instance, position,
                                         is_remove);
    set_nil_return(return_value);
    set_call_error(error, GDEXTENSION_CALL_OK);
}

void orthogonal_router_set_obstacle_ptrcall(
    void *, GDExtensionClassInstancePtr instance,
    const GDExtensionConstTypePtr *args, GDExtensionTypePtr)
{
    orthogonal_router_class_set_obstacle((OrthogonalRouter *)instance,
                                         *(const Vector2i *)args[0],
                                         *(const GDExtensionBool *)args[1]);
}

void orthogonal_router_find_path_call(void *,
                                      GDExtensionClassInstancePtr instance,
                                      const GDExtensionConstVariantPtr *args,
                                      GDExtensionInt argument_count,
                                      GDExtensionVariantPtr return_value,
                                      GDExtensionCallError *error)
{
    if (argument_count != 2)
    {
        set_call_error(error, argument_count < 2 ?
                                  GDEXTENSION_CALL_ERROR_TOO_FEW_ARGUMENTS :
                                  GDEXTENSION_CALL_ERROR_TOO_MANY_ARGUMENTS);
        set_nil_return(return_value);
        return;
    }
    for (int i = 0; i < 2; i++)
    {
        if (variant_get_type(args[i]) != GDEXTENSION_VARIANT_TYPE_VECTOR2I)
        {
            set_call_error(error, GDEXTENSION_CALL_ERROR_INVALID_ARGUMENT, i,
                           GDEXTENSION_VARIANT_TYPE_VECTOR2I);
            set_nil_return(return_value);
            return;
        }
    }

    Vector2i start;
    Vector2i end;
    GDExtensionTypeFromVariantConstructorFunc to_vector2i =
        get_variant_to_type_constructor(GDEXTENSION_VARIANT_TYPE_VECTOR2I);
    to_vector2i(&start, const_cast<GDExtensionVariantPtr>(args[0]));
    to_vector2i(&end, const_cast<GDExtensionVariantPtr>(args[1]));

    variant_construct(GDEXTENSION_VARIANT_TYPE_ARRAY, return_value, NULL, 0,
                      NULL);
    void *array = get_variant_get_internal_ptr_func(
        GDEXTENSION_VARIANT_TYPE_ARRAY)(return_value);
    append_path_to_array(array, orthogonal_router_class_find_path(
                                    (OrthogonalRouter *)instance, start, end));
    set_call_error(error, GDEXTENSION_CALL_OK);
}

void orthogonal_router_find_path_ptrcall(void *,
                                         GDExtensionClassInstancePtr instance,
                                         const GDExtensionConstTypePtr *args,
                                         GDExtensionTypePtr return_value)
{
    variant_get_ptr_constructor(GDEXTENSION_VARIANT_TYPE_ARRAY, 0)(return_value,
                                                                   NULL);
    append_path_to_array(return_value, orthogonal_router_class_find_path(
                                           (OrthogonalRouter *)instance,
                                           *(const Vector2i *)args[0],
                                           *(const Vector2i *)args[1]));
}

void register_method(const char *name, GDExtensionClassMethodCall call,
                     GDExtensionClassMethodPtrCall ptrcall,
                     GDExtensionPropertyInfo *return_value_info,
                     uint32_t argument_count,
                     GDExtensionPropertyInfo *arguments_info,
                     GDExtensionClassMethodArgumentMetadata *arguments_metadata)
{
    StringName class_name;
    StringName method_name;
    string_name_new_with_latin1_chars(&class_name, "OrthogonalRouter", false);
    string_name_new_with_latin1_chars(&method_name, name, false);
    GDExtensionClassMethodInfo method = {
        .name = &method_name,
        .method_userdata = NULL,
        .call_func = call,
        .ptrcall_func = ptrcall,
        .method_flags = GDEXTENSION_METHOD_FLAGS_DEFAULT,
        .has_return_value = return_value_info != NULL,
        .return_value_info = return_value_info,
        .return_value_metadata = GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE,
        .argument_count = argument_count,
        .arguments_info = arguments_info,
        .arguments_metadata = arguments_metadata,
        .default_argument_count = 0,
        .default_arguments = NULL,
    };
    classdb_register_extension_class_method(class_library, &class_name,
                                            &method);
    string_name_destructor(&method_name);
    string_name_destructor(&class_name);
}

const GDExtensionInstanceBindingCallbacks
    orthogonal_router_class_binding_callbacks = {
        .create_callback = NULL,
        .free_callback = NULL,
        .reference_callback = NULL,
};

GDExtensionObjectPtr
orthogonal_router_class_create_instance(void *p_class_userdata)
{
    // Create native Godot object;
    StringName class_name;
    string_name_new_with_latin1_chars(&class_name, "Object", false);
    GDExtensionObjectPtr object = classdb_construct_object(&class_name);
    string_name_destructor(&class_name);

    // Create extension object.
    OrthogonalRouter *self =
        (OrthogonalRouter *)mem_alloc(sizeof(OrthogonalRouter));
    orthogonal_router_class_constructor(self);
    self->object = object;

    // Set the extension instance in the native Godot object.
    string_name_new_with_latin1_chars(&class_name, "OrthogonalRouter", false);
    object_set_instance(object, &class_name, self);
    object_set_instance_binding(object, class_library, self,
                                &orthogonal_router_class_binding_callbacks);
    string_name_destructor(&class_name);

    return object;
}

void orthogonal_router_class_free_instance(
    void *p_class_userdata, GDExtensionClassInstancePtr p_instance)
{
    if (p_instance == NULL)
    {
        return;
    }
    OrthogonalRouter *self = (OrthogonalRouter *)p_instance;
    orthogonal_router_class_destructor(self);
    mem_free(self);
}

void orthogonal_router_class_constructor(OrthogonalRouter *self)
{
    new (self) OrthogonalRouter();
}

void orthogonal_router_class_destructor(OrthogonalRouter *self)
{
    self->~OrthogonalRouter();
}

void orthogonal_router_class_bind_methods()
{
    register_method("clear", orthogonal_router_clear_call,
                    orthogonal_router_clear_ptrcall, NULL, 0, NULL, NULL);

    GDExtensionPropertyInfo set_obstacle_arguments[] = {
        make_property(GDEXTENSION_VARIANT_TYPE_VECTOR2I, "position"),
        make_property(GDEXTENSION_VARIANT_TYPE_BOOL, "is_remove"),
    };
    GDExtensionClassMethodArgumentMetadata set_obstacle_metadata[] = {
        GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE,
        GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE,
    };
    register_method("set_obstacle", orthogonal_router_set_obstacle_call,
                    orthogonal_router_set_obstacle_ptrcall, NULL, 2,
                    set_obstacle_arguments, set_obstacle_metadata);
    for (GDExtensionPropertyInfo &argument : set_obstacle_arguments)
        destruct_property(&argument);

    GDExtensionPropertyInfo find_path_arguments[] = {
        make_property(GDEXTENSION_VARIANT_TYPE_VECTOR2I, "start"),
        make_property(GDEXTENSION_VARIANT_TYPE_VECTOR2I, "end"),
    };
    GDExtensionPropertyInfo find_path_return =
        make_property(GDEXTENSION_VARIANT_TYPE_ARRAY, "path");
    GDExtensionClassMethodArgumentMetadata find_path_metadata[] = {
        GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE,
        GDEXTENSION_METHOD_ARGUMENT_METADATA_NONE,
    };
    register_method("find_path", orthogonal_router_find_path_call,
                    orthogonal_router_find_path_ptrcall, &find_path_return, 2,
                    find_path_arguments, find_path_metadata);
    destruct_property(&find_path_return);
    for (GDExtensionPropertyInfo &argument : find_path_arguments)
        destruct_property(&argument);
}

void orthogonal_router_class_clear(OrthogonalRouter *self)
{
    self->obstacles.clear();
}

void orthogonal_router_class_set_obstacle(OrthogonalRouter *self,
                                          Vector2i position, bool is_remove)
{
    if (is_remove)
        self->obstacles.erase(position);
    else
        self->obstacles.insert(position);
}

vector<Vector2i> orthogonal_router_class_find_path(OrthogonalRouter *self,
                                                   Vector2i start, Vector2i end)
{
    vector<Vector2i> dirs = {
        Vector2i(-1, 0),
        Vector2i(1, 0),
        Vector2i(0, 1),
        Vector2i(0, -1),
    };

    for (auto dir : dirs)
    {
        if (!self->obstacles.contains(end + dir)
            && !self->obstacles.contains(start + dir))
            goto reachable;
    }

    return {start, end};

reachable:

    struct QueueElement {
        Vector2i pos;
        int cost;
        int priority;

        QueueElement(Vector2i pos, int cost, int priority) :
            pos(pos), cost(cost), priority(priority)
        {}

        bool operator<(const QueueElement &other) const
        {
            return priority > other.priority;
        }
    };

    int min_turned_point = INT_MAX;
    vector<Vector2i> final_path;

    for (auto start_dir : dirs)
    {
        priority_queue<QueueElement> queue;
        queue.push(QueueElement(start, 0, 0));

        unordered_map<Vector2i, Vector2i> came_from = {
            {start, start - start_dir}};
        unordered_map<Vector2i, int> cost = {{start, 0}};

        while (!queue.empty())
        {
            auto element = queue.top();
            queue.pop();

            auto cur = element.pos;
            if (cur == end) break;
            if (element.cost != cost[cur]) continue;

            for (auto dir : dirs)
            {
                if (cur == start && dir != start_dir) continue;

                auto next = cur + dir;

                if (self->obstacles.contains(next)
                    && self->obstacles.contains(cur))
                    continue;

                auto last_dir = cur - came_from[cur];
                bool turned = last_dir != dir;

                int new_cost = cost[cur] + 1 + turned;

                if (!(!cost.contains(next) || new_cost < cost[next])) continue;

                cost[next] = new_cost;
                came_from[next] = cur;
                queue.push(QueueElement(next, new_cost,
                                        new_cost + abs(end.x - next.x)
                                            + abs(end.y - next.y)));
            }
        }

        int turned_point = 0;
        vector<Vector2i> path = {end};
        while (path.back() != start && came_from.contains(path.back()))
        {
            path.push_back(came_from[path.back()]);

            auto size = path.size();
            if (size >= 3
                && path[size - 1] - path[size - 2]
                       != path[size - 2] - path[size - 3])
                turned_point++;
        }

        if (turned_point < min_turned_point)
        {
            min_turned_point = turned_point;
            final_path = path;
        }
    }

    int i = 1;
    while (i + 1 < final_path.size())
    {
        if (final_path[i - 1] - final_path[i]
            == final_path[i] - final_path[i + 1])
            final_path.erase(final_path.begin() + i);
        else
            i++;
    }

    return final_path;
}
