#include "api.h"
#include "defs.h"

GDExtensionClassLibraryPtr class_library = NULL;
GDExtensionInterfaceStringNameNewWithLatin1Chars
    string_name_new_with_latin1_chars;
GDExtensionPtrDestructor string_name_destructor;
GDExtensionInterfaceClassdbRegisterExtensionClass2
    classdb_register_extension_class2;
GDExtensionInterfaceClassdbConstructObject classdb_construct_object;
GDExtensionInterfaceObjectSetInstance object_set_instance;
GDExtensionInterfaceObjectSetInstanceBinding object_set_instance_binding;
GDExtensionInterfaceMemAlloc mem_alloc;
GDExtensionInterfaceMemFree mem_free;
GDExtensionInterfaceGetVariantFromTypeConstructor
    get_variant_from_type_constructor;
GDExtensionInterfaceGetVariantToTypeConstructor get_variant_to_type_constructor;
GDExtensionInterfaceVariantGetType variant_get_type;
GDExtensionInterfaceVariantConstruct variant_construct;
GDExtensionInterfaceGetVariantGetInternalPtrFunc
    get_variant_get_internal_ptr_func;
GDExtensionInterfaceVariantGetPtrBuiltinMethod variant_get_ptr_builtin_method;
GDExtensionInterfaceVariantGetPtrConstructor variant_get_ptr_constructor;
GDExtensionInterfaceVariantDestroy variant_destroy;
GDExtensionInterfaceStringNewWithUtf8Chars string_new_with_utf8_chars;
GDExtensionPtrDestructor string_destructor;
GDExtensionInterfaceClassdbRegisterExtensionClassMethod
    classdb_register_extension_class_method;

void load_api(GDExtensionInterfaceGetProcAddress p_get_proc_address)
{
    GDExtensionInterfaceVariantGetPtrDestructor variant_get_ptr_destructor =
        (GDExtensionInterfaceVariantGetPtrDestructor)p_get_proc_address(
            "variant_get_ptr_destructor");
    classdb_register_extension_class2 =
        (GDExtensionInterfaceClassdbRegisterExtensionClass2)p_get_proc_address(
            "classdb_register_extension_class2");
    classdb_construct_object =
        (GDExtensionInterfaceClassdbConstructObject)p_get_proc_address(
            "classdb_construct_object");
    object_set_instance =
        (GDExtensionInterfaceObjectSetInstance)p_get_proc_address(
            "object_set_instance");
    object_set_instance_binding =
        (GDExtensionInterfaceObjectSetInstanceBinding)p_get_proc_address(
            "object_set_instance_binding");
    mem_alloc = (GDExtensionInterfaceMemAlloc)p_get_proc_address("mem_alloc");
    mem_free = (GDExtensionInterfaceMemFree)p_get_proc_address("mem_free");
    string_name_new_with_latin1_chars =
        (GDExtensionInterfaceStringNameNewWithLatin1Chars)p_get_proc_address(
            "string_name_new_with_latin1_chars");
    string_name_destructor =
        variant_get_ptr_destructor(GDEXTENSION_VARIANT_TYPE_STRING_NAME);
    get_variant_from_type_constructor =
        (GDExtensionInterfaceGetVariantFromTypeConstructor)p_get_proc_address(
            "get_variant_from_type_constructor");
    get_variant_to_type_constructor =
        (GDExtensionInterfaceGetVariantToTypeConstructor)p_get_proc_address(
            "get_variant_to_type_constructor");
    variant_get_type = (GDExtensionInterfaceVariantGetType)p_get_proc_address(
        "variant_get_type");
    variant_construct =
        (GDExtensionInterfaceVariantConstruct)p_get_proc_address(
            "variant_construct");
    get_variant_get_internal_ptr_func =
        (GDExtensionInterfaceGetVariantGetInternalPtrFunc)p_get_proc_address(
            "variant_get_ptr_internal_getter");
    variant_get_ptr_builtin_method =
        (GDExtensionInterfaceVariantGetPtrBuiltinMethod)p_get_proc_address(
            "variant_get_ptr_builtin_method");
    variant_get_ptr_constructor =
        (GDExtensionInterfaceVariantGetPtrConstructor)p_get_proc_address(
            "variant_get_ptr_constructor");
    variant_destroy = (GDExtensionInterfaceVariantDestroy)p_get_proc_address(
        "variant_destroy");
    classdb_register_extension_class_method =
        (GDExtensionInterfaceClassdbRegisterExtensionClassMethod)
            p_get_proc_address("classdb_register_extension_class_method");
    string_new_with_utf8_chars =
        (GDExtensionInterfaceStringNewWithUtf8Chars)p_get_proc_address(
            "string_new_with_utf8_chars");
    string_destructor =
        variant_get_ptr_destructor(GDEXTENSION_VARIANT_TYPE_STRING);
}

GDExtensionPropertyInfo make_property(GDExtensionVariantType type,
                                      const char *name)
{
    return make_property_full(type, name, PROPERTY_HINT_NONE, "", "",
                              PROPERTY_USAGE_DEFAULT);
}

GDExtensionPropertyInfo make_property_full(GDExtensionVariantType type,
                                           const char *name, uint32_t hint,
                                           const char *hint_string,
                                           const char *class_name,
                                           uint32_t usage_flags)
{
    auto prop_name = (StringName *)mem_alloc(sizeof(StringName));
    string_name_new_with_latin1_chars(prop_name, name, false);
    auto prop_hint_string = (String *)mem_alloc(sizeof(String));
    string_new_with_utf8_chars(prop_hint_string, hint_string);
    auto prop_class_name = (StringName *)mem_alloc(sizeof(StringName));
    string_name_new_with_latin1_chars(prop_class_name, class_name, false);

    GDExtensionPropertyInfo info = {
        .type = type,
        .name = prop_name,
        .class_name = prop_class_name,
        .hint = hint,
        .hint_string = prop_hint_string,
        .usage = usage_flags,
    };

    return info;
}

void destruct_property(GDExtensionPropertyInfo *info)
{
    string_name_destructor(info->name);
    string_destructor(info->hint_string);
    string_name_destructor(info->class_name);
    mem_free(info->name);
    mem_free(info->hint_string);
    mem_free(info->class_name);
}
