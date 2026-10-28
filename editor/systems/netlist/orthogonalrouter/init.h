#pragma once

#include "defs.h"
#include "gdextension_interface.h"

void initialize_orthogonal_router_module(
    void *p_userdata, GDExtensionInitializationLevel p_level);
void deinitialize_orthogonal_router_module(
    void *p_userdata, GDExtensionInitializationLevel p_level);

#ifdef __cplusplus
extern "C" {
#endif

GDExtensionBool GDE_EXPORT orthogonal_router_library_init(
    GDExtensionInterfaceGetProcAddress p_get_proc_address,
    GDExtensionClassLibraryPtr p_library,
    GDExtensionInitialization *r_initialization);

#ifdef __cplusplus
}
#endif