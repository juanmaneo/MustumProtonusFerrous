/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = msdrm.dll` in `dlls/msdrm/Makefile.in`.
const MSDRM_DLL_NAME: &str = "msdrm.dll";

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

/// Defines one helper per `@ stub` export in `dlls/msdrm/msdrm.spec`. Each helper passes the
/// module name and the export name, as the spec writes it, to `wine_spec_unimplemented_stub`.
macro_rules! stub_exports {
    ($($fn_name:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/msdrm/msdrm.spec`.")]
            pub fn $fn_name() -> StubException {
                wine_spec_unimplemented_stub(MSDRM_DLL_NAME, $export)
            }
        )*
    };
}

stub_exports! {
    drm_acquire_advisories => "DRMAcquireAdvisories",
    drm_acquire_issuance_license_template => "DRMAcquireIssuanceLicenseTemplate",
    drm_acquire_license => "DRMAcquireLicense",
    drm_activate => "DRMActivate",
    drm_add_license => "DRMAddLicense",
    drm_add_right_with_user => "DRMAddRightWithUser",
    drm_attest => "DRMAttest",
    drm_check_security => "DRMCheckSecurity",
    drm_clear_all_rights => "DRMClearAllRights",
    drm_close_environment_handle => "DRMCloseEnvironmentHandle",
    drm_close_handle => "DRMCloseHandle",
    drm_close_pub_handle => "DRMClosePubHandle",
    drm_close_query_handle => "DRMCloseQueryHandle",
    drm_close_session => "DRMCloseSession",
    drm_construct_certificate_chain => "DRMConstructCertificateChain",
    drm_create_bound_license => "DRMCreateBoundLicense",
    drm_create_client_session => "DRMCreateClientSession",
    drm_create_enabling_bits_decryptor => "DRMCreateEnablingBitsDecryptor",
    drm_create_enabling_bits_encryptor => "DRMCreateEnablingBitsEncryptor",
    drm_create_enabling_principal => "DRMCreateEnablingPrincipal",
    drm_create_issuance_license => "DRMCreateIssuanceLicense",
    drm_create_license_storage_session => "DRMCreateLicenseStorageSession",
    drm_create_right => "DRMCreateRight",
    drm_create_user => "DRMCreateUser",
    drm_decode => "DRMDecode",
    drm_deconstruct_certificate_chain => "DRMDeconstructCertificateChain",
    drm_decrypt => "DRMDecrypt",
    drm_delete_license => "DRMDeleteLicense",
    drm_duplicate_environment_handle => "DRMDuplicateEnvironmentHandle",
    drm_duplicate_handle => "DRMDuplicateHandle",
    drm_duplicate_pub_handle => "DRMDuplicatePubHandle",
    drm_duplicate_session => "DRMDuplicateSession",
    drm_encode => "DRMEncode",
    drm_encrypt => "DRMEncrypt",
    drm_enumerate_license => "DRMEnumerateLicense",
    drm_get_application_specific_data => "DRMGetApplicationSpecificData",
    drm_get_bound_license_attribute => "DRMGetBoundLicenseAttribute",
    drm_get_bound_license_attribute_count => "DRMGetBoundLicenseAttributeCount",
    drm_get_bound_license_object => "DRMGetBoundLicenseObject",
    drm_get_bound_license_object_count => "DRMGetBoundLicenseObjectCount",
    drm_get_certificate_chain_count => "DRMGetCertificateChainCount",
    drm_get_client_version => "DRMGetClientVersion",
    drm_get_environment_info => "DRMGetEnvironmentInfo",
    drm_get_info => "DRMGetInfo",
    drm_get_interval_time => "DRMGetIntervalTime",
    drm_get_issuance_license_info => "DRMGetIssuanceLicenseInfo",
    drm_get_issuance_license_template => "DRMGetIssuanceLicenseTemplate",
    drm_get_meta_data => "DRMGetMetaData",
    drm_get_name_and_description => "DRMGetNameAndDescription",
    drm_get_owner_license => "DRMGetOwnerLicense",
    drm_get_proc_address => "DRMGetProcAddress",
    drm_get_revocation_point => "DRMGetRevocationPoint",
    drm_get_right_extended_info => "DRMGetRightExtendedInfo",
    drm_get_right_info => "DRMGetRightInfo",
    drm_get_security_provider => "DRMGetSecurityProvider",
    drm_get_service_location => "DRMGetServiceLocation",
    drm_get_signed_issuance_license => "DRMGetSignedIssuanceLicense",
    drm_get_signed_issuance_license_ex => "DRMGetSignedIssuanceLicenseEx",
    drm_get_time => "DRMGetTime",
    drm_get_unbound_license_attribute => "DRMGetUnboundLicenseAttribute",
    drm_get_unbound_license_attribute_count => "DRMGetUnboundLicenseAttributeCount",
    drm_get_unbound_license_object => "DRMGetUnboundLicenseObject",
    drm_get_unbound_license_object_count => "DRMGetUnboundLicenseObjectCount",
    drm_get_usage_policy => "DRMGetUsagePolicy",
    drm_get_user_info => "DRMGetUserInfo",
    drm_get_user_rights => "DRMGetUserRights",
    drm_get_users => "DRMGetUsers",
    drm_init_environment => "DRMInitEnvironment",
    drm_is_activated => "DRMIsActivated",
    drm_is_window_protected => "DRMIsWindowProtected",
    drm_load_library => "DRMLoadLibrary",
    drm_parse_unbound_license => "DRMParseUnboundLicense",
    drm_register_protected_window => "DRMRegisterProtectedWindow",
    drm_register_revocation_list => "DRMRegisterRevocationList",
    drm_repair => "DRMRepair",
    drm_set_application_specific_data => "DRMSetApplicationSpecificData",
    drm_set_global_options => "DRMSetGlobalOptions",
    drm_set_interval_time => "DRMSetIntervalTime",
    drm_set_meta_data => "DRMSetMetaData",
    drm_set_name_and_description => "DRMSetNameAndDescription",
    drm_set_revocation_point => "DRMSetRevocationPoint",
    drm_set_usage_policy => "DRMSetUsagePolicy",
    drm_verify => "DRMVerify",
    dll_get_class_object => "DllGetClassObject",
    __add_machine_cert_to_license_store => "__AddMachineCertToLicenseStore",
}
