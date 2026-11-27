/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = mscorwks.dll` in `dlls/mscorwks/Makefile.in`.
const MSCORWKS_DLL_NAME: &str = "mscorwks.dll";

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

/// `ReOpenMetaDataWithMemory`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn re_open_meta_data_with_memory() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "ReOpenMetaDataWithMemory")
}

/// `CorDllMainForThunk`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cor_dll_main_for_thunk() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CorDllMainForThunk")
}

/// `CreateActContextInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_act_context_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateActContextInternal")
}

/// `CreateCMSFromXmlInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_cms_from_xml_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateCMSFromXmlInternal")
}

/// `GetAddrOfContractShutoffFlag`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_addr_of_contract_shutoff_flag() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetAddrOfContractShutoffFlag")
}

/// `GetCLRFunction`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_clr_function() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetCLRFunction")
}

/// `GetUserStateManagerInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_user_state_manager_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetUserStateManagerInternal")
}

/// `GetUserStoreInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_user_store_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetUserStoreInternal")
}

/// `ParseManifestInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn parse_manifest_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "ParseManifestInternal")
}

/// `_AxlGetIssuerPublicKeyHash`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn _axl_get_issuer_public_key_hash() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "_AxlGetIssuerPublicKeyHash")
}

/// `_AxlPublicKeyBlobToPublicKeyToken`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn _axl_public_key_blob_to_public_key_token() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "_AxlPublicKeyBlobToPublicKeyToken")
}

/// `_AxlRSAKeyValueToPublicKeyToken`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn _axl_rsa_key_value_to_public_key_token() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "_AxlRSAKeyValueToPublicKeyToken")
}

/// `CertCreateAuthenticodeLicense`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cert_create_authenticode_license() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CertCreateAuthenticodeLicense")
}

/// `CertFreeAuthenticodeSignerInfo`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cert_free_authenticode_signer_info() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CertFreeAuthenticodeSignerInfo")
}

/// `CertFreeAuthenticodeTimestamperInfo`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cert_free_authenticode_timestamper_info() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CertFreeAuthenticodeTimestamperInfo")
}

/// `CertTimestampAuthenticodeLicense`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cert_timestamp_authenticode_license() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CertTimestampAuthenticodeLicense")
}

/// `CertVerifyAuthenticodeLicense`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cert_verify_authenticode_license() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CertVerifyAuthenticodeLicense")
}

/// `ClearDownloadCache`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn clear_download_cache() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "ClearDownloadCache")
}

/// `CoInitializeEE`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn co_initialize_ee() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CoInitializeEE")
}

/// `CoUninitializeCor`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn co_uninitialize_cor() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CoUninitializeCor")
}

/// `CoUninitializeEE`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn co_uninitialize_ee() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CoUninitializeEE")
}

/// `CompareAssemblyIdentity`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn compare_assembly_identity() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CompareAssemblyIdentity")
}

/// `CopyPDBs`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn copy_pdbs() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CopyPDBs")
}

/// `CorLaunchApplication`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cor_launch_application() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CorLaunchApplication")
}

/// `CorMarkThreadInThreadPool`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn cor_mark_thread_in_thread_pool() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CorMarkThreadInThreadPool")
}

/// `CreateActContext`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_act_context() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateActContext")
}

/// `CreateApplicationContext`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_application_context() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateApplicationContext")
}

/// `CreateAssemblyCache`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_assembly_cache() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateAssemblyCache")
}

/// `CreateAssemblyEnum`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_assembly_enum() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateAssemblyEnum")
}

/// `CreateAssemblyNameObject`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_assembly_name_object() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateAssemblyNameObject")
}

/// `CreateCMSFromXml`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_cms_from_xml() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateCMSFromXml")
}

/// `CreateHistoryReader`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_history_reader() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateHistoryReader")
}

/// `CreateInstallReferenceEnum`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn create_install_reference_enum() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "CreateInstallReferenceEnum")
}

/// `DeleteShadowCache`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn delete_shadow_cache() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "DeleteShadowCache")
}

/// `DllCanUnloadNowInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn dll_can_unload_now_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "DllCanUnloadNowInternal")
}

/// `DllGetClassObjectInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn dll_get_class_object_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "DllGetClassObjectInternal")
}

/// `DllRegisterServerInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn dll_register_server_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "DllRegisterServerInternal")
}

/// `DllUnregisterServerInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn dll_unregister_server_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "DllUnregisterServerInternal")
}

/// `EEDllRegisterServer`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn ee_dll_register_server() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "EEDllRegisterServer")
}

/// `EEDllUnregisterServer`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn ee_dll_unregister_server() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "EEDllUnregisterServer")
}

/// `GetAppIdAuthority`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_app_id_authority() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetAppIdAuthority")
}

/// `GetAssemblyIdentityFromFile`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_assembly_identity_from_file() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetAssemblyIdentityFromFile")
}

/// `GetCLRIdentityManager`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_clr_identity_manager() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetCLRIdentityManager")
}

/// `GetCachePath`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_cache_path() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetCachePath")
}

/// `GetHashFromAssemblyFile`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_hash_from_assembly_file() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetHashFromAssemblyFile")
}

/// `GetHashFromAssemblyFileW`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_hash_from_assembly_file_w() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetHashFromAssemblyFileW")
}

/// `GetHashFromBlob`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_hash_from_blob() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetHashFromBlob")
}

/// `GetHashFromFile`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_hash_from_file() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetHashFromFile")
}

/// `GetHashFromFileW`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_hash_from_file_w() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetHashFromFileW")
}

/// `GetHashFromHandle`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_hash_from_handle() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetHashFromHandle")
}

/// `GetHistoryFileDirectory`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_history_file_directory() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetHistoryFileDirectory")
}

/// `GetIdentityAuthority`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_identity_authority() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetIdentityAuthority")
}

/// `GetMetaDataInternalInterface`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_meta_data_internal_interface() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetMetaDataInternalInterface")
}

/// `GetMetaDataInternalInterfaceFromPublic`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_meta_data_internal_interface_from_public() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetMetaDataInternalInterfaceFromPublic")
}

/// `GetMetaDataPublicInterfaceFromInternal`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_meta_data_public_interface_from_internal() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetMetaDataPublicInterfaceFromInternal")
}

/// `GetPermissionRequests`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_permission_requests() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetPermissionRequests")
}

/// `GetPrivateContextsPerfCounters`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_private_contexts_perf_counters() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetPrivateContextsPerfCounters")
}

/// `GetUserStateManager`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_user_state_manager() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetUserStateManager")
}

/// `GetUserStore`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn get_user_store() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "GetUserStore")
}

/// `IEE`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn iee() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "IEE")
}

/// `InitializeFusion`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn initialize_fusion() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "InitializeFusion")
}

/// `InstallCustomAssembly`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn install_custom_assembly() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "InstallCustomAssembly")
}

/// `InstallCustomModule`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn install_custom_module() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "InstallCustomModule")
}

/// `LegacyNGenCompile`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn legacy_n_gen_compile() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LegacyNGenCompile")
}

/// `LegacyNGenCreateZapper`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn legacy_n_gen_create_zapper() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LegacyNGenCreateZapper")
}

/// `LegacyNGenFreeZapper`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn legacy_n_gen_free_zapper() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LegacyNGenFreeZapper")
}

/// `LegacyNGenTryEnumerateFusionCache`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn legacy_n_gen_try_enumerate_fusion_cache() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LegacyNGenTryEnumerateFusionCache")
}

/// `LogHelp_LogAssert`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn log_help_log_assert() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LogHelp_LogAssert")
}

/// `LogHelp_NoGuiOnAssert`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn log_help_no_gui_on_assert() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LogHelp_NoGuiOnAssert")
}

/// `LogHelp_TerminateOnAssert`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn log_help_terminate_on_assert() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LogHelp_TerminateOnAssert")
}

/// `LookupHistoryAssembly`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn lookup_history_assembly() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "LookupHistoryAssembly")
}

/// `MetaDataGetDispenser`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn meta_data_get_dispenser() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "MetaDataGetDispenser")
}

/// `NGenCreateNGenWorker`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn n_gen_create_n_gen_worker() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "NGenCreateNGenWorker")
}

/// `NukeDownloadedCache`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn nuke_downloaded_cache() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "NukeDownloadedCache")
}

/// `ParseManifest`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn parse_manifest() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "ParseManifest")
}

/// `PostErrorVA`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn post_error_va() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "PostErrorVA")
}

/// `PreBindAssembly`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn pre_bind_assembly() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "PreBindAssembly")
}

/// `PreBindAssemblyEx`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn pre_bind_assembly_ex() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "PreBindAssemblyEx")
}

/// `ReOpenMetaDataWithMemoryEx`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn re_open_meta_data_with_memory_ex() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "ReOpenMetaDataWithMemoryEx")
}

/// `StrongNameCompareAssemblies`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_compare_assemblies() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameCompareAssemblies")
}

/// `StrongNameErrorInfo`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_error_info() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameErrorInfo")
}

/// `StrongNameFreeBuffer`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_free_buffer() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameFreeBuffer")
}

/// `StrongNameGetBlob`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_get_blob() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameGetBlob")
}

/// `StrongNameGetBlobFromImage`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_get_blob_from_image() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameGetBlobFromImage")
}

/// `StrongNameGetPublicKey`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_get_public_key() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameGetPublicKey")
}

/// `StrongNameHashSize`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_hash_size() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameHashSize")
}

/// `StrongNameKeyDelete`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_key_delete() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameKeyDelete")
}

/// `StrongNameKeyGen`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_key_gen() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameKeyGen")
}

/// `StrongNameKeyGenEx`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_key_gen_ex() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameKeyGenEx")
}

/// `StrongNameKeyInstall`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_key_install() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameKeyInstall")
}

/// `StrongNameSignatureGeneration`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_signature_generation() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameSignatureGeneration")
}

/// `StrongNameSignatureGenerationEx`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_signature_generation_ex() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameSignatureGenerationEx")
}

/// `StrongNameSignatureSize`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_signature_size() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameSignatureSize")
}

/// `StrongNameSignatureVerificationFromImage`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_signature_verification_from_image() -> StubException {
    wine_spec_unimplemented_stub(
        MSCORWKS_DLL_NAME,
        "StrongNameSignatureVerificationFromImage",
    )
}

/// `StrongNameTokenFromAssemblyEx`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_token_from_assembly_ex() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameTokenFromAssemblyEx")
}

/// `StrongNameTokenFromPublicKey`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn strong_name_token_from_public_key() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "StrongNameTokenFromPublicKey")
}

/// `TranslateSecurityAttributes`, exported as `@ stub` in `dlls/mscorwks/mscorwks.spec`.
pub fn translate_security_attributes() -> StubException {
    wine_spec_unimplemented_stub(MSCORWKS_DLL_NAME, "TranslateSecurityAttributes")
}
