#pragma once

#include "defs.h"
#include "gdextension_interface.h"

extern GDExtensionClassLibraryPtr class_library;
extern GDExtensionInterfaceStringNameNewWithLatin1Chars
    string_name_new_with_latin1_chars;
extern GDExtensionPtrDestructor string_name_destructor;
extern GDExtensionInterfaceClassdbRegisterExtensionClass2
    classdb_register_extension_class2;
extern GDExtensionInterfaceClassdbConstructObject classdb_construct_object;
extern GDExtensionInterfaceObjectSetInstance object_set_instance;
extern GDExtensionInterfaceObjectSetInstanceBinding object_set_instance_binding;
extern GDExtensionInterfaceMemAlloc mem_alloc;
extern GDExtensionInterfaceMemFree mem_free;
extern GDExtensionInterfaceGetVariantFromTypeConstructor
    get_variant_from_type_constructor;
extern GDExtensionInterfaceGetVariantToTypeConstructor
    get_variant_to_type_constructor;
extern GDExtensionInterfaceVariantGetType variant_get_type;
extern GDExtensionInterfaceVariantConstruct variant_construct;
extern GDExtensionInterfaceGetVariantGetInternalPtrFunc
    get_variant_get_internal_ptr_func;
extern GDExtensionInterfaceVariantGetPtrBuiltinMethod
    variant_get_ptr_builtin_method;
extern GDExtensionInterfaceVariantGetPtrConstructor variant_get_ptr_constructor;
extern GDExtensionInterfaceVariantDestroy variant_destroy;
extern GDExtensionInterfaceStringNewWithUtf8Chars string_new_with_utf8_chars;
extern GDExtensionPtrDestructor string_destructor;
extern GDExtensionInterfaceClassdbRegisterExtensionClassMethod
    classdb_register_extension_class_method;

void load_api(GDExtensionInterfaceGetProcAddress p_get_proc_address);

GDExtensionPropertyInfo make_property(GDExtensionVariantType type,
                                      const char *name);

GDExtensionPropertyInfo make_property_full(GDExtensionVariantType type,
                                           const char *name, uint32_t hint,
                                           const char *hint_string,
                                           const char *class_name,
                                           uint32_t usage_flags);

void destruct_property(GDExtensionPropertyInfo *info);
