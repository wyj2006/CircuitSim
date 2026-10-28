#include "init.h"

#include "api.h"
#include "orthogonal_router.h"

void initialize_orthogonal_router_module(void *p_userdata,
                                         GDExtensionInitializationLevel p_level)
{
    if (p_level != GDEXTENSION_INITIALIZATION_SCENE)
    {
        return;
    }

    // Register class.
    StringName class_name;
    string_name_new_with_latin1_chars(&class_name, "OrthogonalRouter", false);
    StringName parent_class_name;
    string_name_new_with_latin1_chars(&parent_class_name, "Object", false);

    GDExtensionClassCreationInfo2 class_info = {
        .is_virtual = false,
        .is_abstract = false,
        .is_exposed = true,
        .set_func = NULL,
        .get_func = NULL,
        .get_property_list_func = NULL,
        .free_property_list_func = NULL,
        .property_can_revert_func = NULL,
        .property_get_revert_func = NULL,
        .validate_property_func = NULL,
        .notification_func = NULL,
        .to_string_func = NULL,
        .reference_func = NULL,
        .unreference_func = NULL,
        .create_instance_func = orthogonal_router_class_create_instance,
        .free_instance_func = orthogonal_router_class_free_instance,
        .recreate_instance_func = NULL,
        .get_virtual_func = NULL,
        .get_virtual_call_data_func = NULL,
        .call_virtual_with_data_func = NULL,
        .get_rid_func = NULL,
        .class_userdata = NULL,
    };

    classdb_register_extension_class2(class_library, &class_name,
                                      &parent_class_name, &class_info);

    orthogonal_router_class_bind_methods();

    string_name_destructor(&class_name);
    string_name_destructor(&parent_class_name);
}

void deinitialize_orthogonal_router_module(
    void *p_userdata, GDExtensionInitializationLevel p_level)
{}

GDExtensionBool GDE_EXPORT orthogonal_router_library_init(
    GDExtensionInterfaceGetProcAddress p_get_proc_address,
    GDExtensionClassLibraryPtr p_library,
    GDExtensionInitialization *r_initialization)
{
    class_library = p_library;
    load_api(p_get_proc_address);

    r_initialization->initialize = initialize_orthogonal_router_module;
    r_initialization->deinitialize = deinitialize_orthogonal_router_module;
    r_initialization->userdata = NULL;
    r_initialization->minimum_initialization_level =
        GDEXTENSION_INITIALIZATION_SCENE;

    return true;
}