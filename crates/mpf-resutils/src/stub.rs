/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = resutils.dll` in `dlls/resutils/Makefile.in`.
const RESUTILS_DLL_NAME: &str = "resutils.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

// One helper per `@ stub` export of `dlls/resutils/resutils.spec`, in spec order. Each returns the
// record that `wine_spec_unimplemented_stub` raises for the export's name.
pub fn clus_worker_check_terminate() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ClusWorkerCheckTerminate")
}

pub fn clus_worker_create() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ClusWorkerCreate")
}

pub fn clus_worker_start() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ClusWorkerStart")
}

pub fn clus_worker_terminate() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ClusWorkerTerminate")
}

pub fn res_util_add_unknown_properties() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilAddUnknownProperties")
}

pub fn res_util_create_directory_tree() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilCreateDirectoryTree")
}

pub fn res_util_dup_parameter_block() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilDupParameterBlock")
}

pub fn res_util_dup_string() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilDupString")
}

pub fn res_util_enum_private_properties() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilEnumPrivateProperties")
}

pub fn res_util_enum_properties() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilEnumProperties")
}

pub fn res_util_enum_resources() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilEnumResources")
}

pub fn res_util_enum_resources_ex() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilEnumResourcesEx")
}

pub fn res_util_expand_environment_strings() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilExpandEnvironmentStrings")
}

pub fn res_util_find_binary_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFindBinaryProperty")
}

pub fn res_util_find_dependent_disk_resource_drive_letter() -> StubException {
    wine_spec_unimplemented_stub(
        RESUTILS_DLL_NAME,
        "ResUtilFindDependentDiskResourceDriveLetter",
    )
}

pub fn res_util_find_dword_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFindDwordProperty")
}

pub fn res_util_find_expand_sz_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFindExpandSzProperty")
}

pub fn res_util_find_expanded_sz_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFindExpandedSzProperty")
}

pub fn res_util_find_long_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFindLongProperty")
}

pub fn res_util_find_multi_sz_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFindMultiSzProperty")
}

pub fn res_util_find_sz_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFindSzProperty")
}

pub fn res_util_free_environment() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFreeEnvironment")
}

pub fn res_util_free_parameter_block() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilFreeParameterBlock")
}

pub fn res_util_get_all_properties() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetAllProperties")
}

pub fn res_util_get_binary_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetBinaryProperty")
}

pub fn res_util_get_binary_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetBinaryValue")
}

pub fn res_util_get_core_cluster_resources() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetCoreClusterResources")
}

pub fn res_util_get_dword_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetDwordProperty")
}

pub fn res_util_get_dword_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetDwordValue")
}

pub fn res_util_get_environment_with_net_name() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetEnvironmentWithNetName")
}

pub fn res_util_get_multi_sz_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetMultiSzProperty")
}

pub fn res_util_get_private_properties() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetPrivateProperties")
}

pub fn res_util_get_properties() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetProperties")
}

pub fn res_util_get_properties_to_parameter_block() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetPropertiesToParameterBlock")
}

pub fn res_util_get_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetProperty")
}

pub fn res_util_get_property_formats() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetPropertyFormats")
}

pub fn res_util_get_property_size() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetPropertySize")
}

pub fn res_util_get_resource_dependency() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetResourceDependency")
}

pub fn res_util_get_resource_dependency_by_class() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetResourceDependencyByClass")
}

pub fn res_util_get_resource_dependency_by_name() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetResourceDependencyByName")
}

pub fn res_util_get_resource_dependent_ip_address_props() -> StubException {
    wine_spec_unimplemented_stub(
        RESUTILS_DLL_NAME,
        "ResUtilGetResourceDependentIPAddressProps",
    )
}

pub fn res_util_get_resource_name() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetResourceName")
}

pub fn res_util_get_resource_name_dependency() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetResourceNameDependency")
}

pub fn res_util_get_sz_property() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetSzProperty")
}

pub fn res_util_get_sz_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilGetSzValue")
}

pub fn res_util_is_path_valid() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilIsPathValid")
}

pub fn res_util_is_resource_class_equal() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilIsResourceClassEqual")
}

pub fn res_util_property_list_from_parameter_block() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilPropertyListFromParameterBlock")
}

pub fn res_util_resource_types_equal() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilResourceTypesEqual")
}

pub fn res_util_resources_equal() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilResourcesEqual")
}

pub fn res_util_set_binary_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetBinaryValue")
}

pub fn res_util_set_dword_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetDwordValue")
}

pub fn res_util_set_expand_sz_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetExpandSzValue")
}

pub fn res_util_set_multi_sz_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetMultiSzValue")
}

pub fn res_util_set_private_property_list() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetPrivatePropertyList")
}

pub fn res_util_set_property_parameter_block() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetPropertyParameterBlock")
}

pub fn res_util_set_property_parameter_block_ex() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetPropertyParameterBlockEx")
}

pub fn res_util_set_property_table() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetPropertyTable")
}

pub fn res_util_set_property_table_ex() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetPropertyTableEx")
}

pub fn res_util_set_resource_service_environment() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetResourceServiceEnvironment")
}

pub fn res_util_set_resource_service_start_parameters() -> StubException {
    wine_spec_unimplemented_stub(
        RESUTILS_DLL_NAME,
        "ResUtilSetResourceServiceStartParameters",
    )
}

pub fn res_util_set_sz_value() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetSzValue")
}

pub fn res_util_set_unknown_properties() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilSetUnknownProperties")
}

pub fn res_util_start_resource_service() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilStartResourceService")
}

pub fn res_util_stop_resource_service() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilStopResourceService")
}

pub fn res_util_stop_service() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilStopService")
}

pub fn res_util_terminate_service_process_from_res_dll() -> StubException {
    wine_spec_unimplemented_stub(
        RESUTILS_DLL_NAME,
        "ResUtilTerminateServiceProcessFromResDll",
    )
}

pub fn res_util_verify_private_property_list() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilVerifyPrivatePropertyList")
}

pub fn res_util_verify_property_table() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilVerifyPropertyTable")
}

pub fn res_util_verify_resource_service() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilVerifyResourceService")
}

pub fn res_util_verify_service() -> StubException {
    wine_spec_unimplemented_stub(RESUTILS_DLL_NAME, "ResUtilVerifyService")
}
