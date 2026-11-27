use crate::stub::*;

/// The helper that returns the record one `@ stub` export raises.
pub type StubHelper = fn() -> StubException;

/// The `resutils.spec` export table, in spec order. Every entry is `@ stub`, so each name is paired
/// with the helper that returns the record its stub raises.
pub const EXPORTS: [(&str, StubHelper); 71] = [
    ("ClusWorkerCheckTerminate", clus_worker_check_terminate),
    ("ClusWorkerCreate", clus_worker_create),
    ("ClusWorkerStart", clus_worker_start),
    ("ClusWorkerTerminate", clus_worker_terminate),
    (
        "ResUtilAddUnknownProperties",
        res_util_add_unknown_properties,
    ),
    ("ResUtilCreateDirectoryTree", res_util_create_directory_tree),
    ("ResUtilDupParameterBlock", res_util_dup_parameter_block),
    ("ResUtilDupString", res_util_dup_string),
    (
        "ResUtilEnumPrivateProperties",
        res_util_enum_private_properties,
    ),
    ("ResUtilEnumProperties", res_util_enum_properties),
    ("ResUtilEnumResources", res_util_enum_resources),
    ("ResUtilEnumResourcesEx", res_util_enum_resources_ex),
    (
        "ResUtilExpandEnvironmentStrings",
        res_util_expand_environment_strings,
    ),
    ("ResUtilFindBinaryProperty", res_util_find_binary_property),
    (
        "ResUtilFindDependentDiskResourceDriveLetter",
        res_util_find_dependent_disk_resource_drive_letter,
    ),
    ("ResUtilFindDwordProperty", res_util_find_dword_property),
    (
        "ResUtilFindExpandSzProperty",
        res_util_find_expand_sz_property,
    ),
    (
        "ResUtilFindExpandedSzProperty",
        res_util_find_expanded_sz_property,
    ),
    ("ResUtilFindLongProperty", res_util_find_long_property),
    (
        "ResUtilFindMultiSzProperty",
        res_util_find_multi_sz_property,
    ),
    ("ResUtilFindSzProperty", res_util_find_sz_property),
    ("ResUtilFreeEnvironment", res_util_free_environment),
    ("ResUtilFreeParameterBlock", res_util_free_parameter_block),
    ("ResUtilGetAllProperties", res_util_get_all_properties),
    ("ResUtilGetBinaryProperty", res_util_get_binary_property),
    ("ResUtilGetBinaryValue", res_util_get_binary_value),
    (
        "ResUtilGetCoreClusterResources",
        res_util_get_core_cluster_resources,
    ),
    ("ResUtilGetDwordProperty", res_util_get_dword_property),
    ("ResUtilGetDwordValue", res_util_get_dword_value),
    (
        "ResUtilGetEnvironmentWithNetName",
        res_util_get_environment_with_net_name,
    ),
    ("ResUtilGetMultiSzProperty", res_util_get_multi_sz_property),
    (
        "ResUtilGetPrivateProperties",
        res_util_get_private_properties,
    ),
    ("ResUtilGetProperties", res_util_get_properties),
    (
        "ResUtilGetPropertiesToParameterBlock",
        res_util_get_properties_to_parameter_block,
    ),
    ("ResUtilGetProperty", res_util_get_property),
    ("ResUtilGetPropertyFormats", res_util_get_property_formats),
    ("ResUtilGetPropertySize", res_util_get_property_size),
    (
        "ResUtilGetResourceDependency",
        res_util_get_resource_dependency,
    ),
    (
        "ResUtilGetResourceDependencyByClass",
        res_util_get_resource_dependency_by_class,
    ),
    (
        "ResUtilGetResourceDependencyByName",
        res_util_get_resource_dependency_by_name,
    ),
    (
        "ResUtilGetResourceDependentIPAddressProps",
        res_util_get_resource_dependent_ip_address_props,
    ),
    ("ResUtilGetResourceName", res_util_get_resource_name),
    (
        "ResUtilGetResourceNameDependency",
        res_util_get_resource_name_dependency,
    ),
    ("ResUtilGetSzProperty", res_util_get_sz_property),
    ("ResUtilGetSzValue", res_util_get_sz_value),
    ("ResUtilIsPathValid", res_util_is_path_valid),
    (
        "ResUtilIsResourceClassEqual",
        res_util_is_resource_class_equal,
    ),
    (
        "ResUtilPropertyListFromParameterBlock",
        res_util_property_list_from_parameter_block,
    ),
    ("ResUtilResourceTypesEqual", res_util_resource_types_equal),
    ("ResUtilResourcesEqual", res_util_resources_equal),
    ("ResUtilSetBinaryValue", res_util_set_binary_value),
    ("ResUtilSetDwordValue", res_util_set_dword_value),
    ("ResUtilSetExpandSzValue", res_util_set_expand_sz_value),
    ("ResUtilSetMultiSzValue", res_util_set_multi_sz_value),
    (
        "ResUtilSetPrivatePropertyList",
        res_util_set_private_property_list,
    ),
    (
        "ResUtilSetPropertyParameterBlock",
        res_util_set_property_parameter_block,
    ),
    (
        "ResUtilSetPropertyParameterBlockEx",
        res_util_set_property_parameter_block_ex,
    ),
    ("ResUtilSetPropertyTable", res_util_set_property_table),
    ("ResUtilSetPropertyTableEx", res_util_set_property_table_ex),
    (
        "ResUtilSetResourceServiceEnvironment",
        res_util_set_resource_service_environment,
    ),
    (
        "ResUtilSetResourceServiceStartParameters",
        res_util_set_resource_service_start_parameters,
    ),
    ("ResUtilSetSzValue", res_util_set_sz_value),
    (
        "ResUtilSetUnknownProperties",
        res_util_set_unknown_properties,
    ),
    (
        "ResUtilStartResourceService",
        res_util_start_resource_service,
    ),
    ("ResUtilStopResourceService", res_util_stop_resource_service),
    ("ResUtilStopService", res_util_stop_service),
    (
        "ResUtilTerminateServiceProcessFromResDll",
        res_util_terminate_service_process_from_res_dll,
    ),
    (
        "ResUtilVerifyPrivatePropertyList",
        res_util_verify_private_property_list,
    ),
    ("ResUtilVerifyPropertyTable", res_util_verify_property_table),
    (
        "ResUtilVerifyResourceService",
        res_util_verify_resource_service,
    ),
    ("ResUtilVerifyService", res_util_verify_service),
];

/// The stub record for `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, stub)| stub())
}
