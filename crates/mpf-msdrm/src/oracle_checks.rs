//! Oracle-derived test vectors for the ported msdrm logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DRMRegisterContent`
//! (`dlls/msdrm/main.c`) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/msdrm/`. The harness printed 90 rows:
//! 5 for `DRMRegisterContent` and one per `@ stub` export (85). ASan and UBSan reported nothing,
//! so no vectors are excluded. The 85 stub rows share the code, flags, parameter count and module,
//! and differ only in the function name, so [`stub_row`] builds the expected record from the name.

use super::*;

type StubFn = fn() -> StubException;

/// The `@ stub` exports in `msdrm.spec` order, each with the helper that models it.
const STUB_ROWS: &[(StubFn, &str)] = &[
    (drm_acquire_advisories, "DRMAcquireAdvisories"),
    (
        drm_acquire_issuance_license_template,
        "DRMAcquireIssuanceLicenseTemplate",
    ),
    (drm_acquire_license, "DRMAcquireLicense"),
    (drm_activate, "DRMActivate"),
    (drm_add_license, "DRMAddLicense"),
    (drm_add_right_with_user, "DRMAddRightWithUser"),
    (drm_attest, "DRMAttest"),
    (drm_check_security, "DRMCheckSecurity"),
    (drm_clear_all_rights, "DRMClearAllRights"),
    (drm_close_environment_handle, "DRMCloseEnvironmentHandle"),
    (drm_close_handle, "DRMCloseHandle"),
    (drm_close_pub_handle, "DRMClosePubHandle"),
    (drm_close_query_handle, "DRMCloseQueryHandle"),
    (drm_close_session, "DRMCloseSession"),
    (
        drm_construct_certificate_chain,
        "DRMConstructCertificateChain",
    ),
    (drm_create_bound_license, "DRMCreateBoundLicense"),
    (drm_create_client_session, "DRMCreateClientSession"),
    (
        drm_create_enabling_bits_decryptor,
        "DRMCreateEnablingBitsDecryptor",
    ),
    (
        drm_create_enabling_bits_encryptor,
        "DRMCreateEnablingBitsEncryptor",
    ),
    (drm_create_enabling_principal, "DRMCreateEnablingPrincipal"),
    (drm_create_issuance_license, "DRMCreateIssuanceLicense"),
    (
        drm_create_license_storage_session,
        "DRMCreateLicenseStorageSession",
    ),
    (drm_create_right, "DRMCreateRight"),
    (drm_create_user, "DRMCreateUser"),
    (drm_decode, "DRMDecode"),
    (
        drm_deconstruct_certificate_chain,
        "DRMDeconstructCertificateChain",
    ),
    (drm_decrypt, "DRMDecrypt"),
    (drm_delete_license, "DRMDeleteLicense"),
    (
        drm_duplicate_environment_handle,
        "DRMDuplicateEnvironmentHandle",
    ),
    (drm_duplicate_handle, "DRMDuplicateHandle"),
    (drm_duplicate_pub_handle, "DRMDuplicatePubHandle"),
    (drm_duplicate_session, "DRMDuplicateSession"),
    (drm_encode, "DRMEncode"),
    (drm_encrypt, "DRMEncrypt"),
    (drm_enumerate_license, "DRMEnumerateLicense"),
    (
        drm_get_application_specific_data,
        "DRMGetApplicationSpecificData",
    ),
    (
        drm_get_bound_license_attribute,
        "DRMGetBoundLicenseAttribute",
    ),
    (
        drm_get_bound_license_attribute_count,
        "DRMGetBoundLicenseAttributeCount",
    ),
    (drm_get_bound_license_object, "DRMGetBoundLicenseObject"),
    (
        drm_get_bound_license_object_count,
        "DRMGetBoundLicenseObjectCount",
    ),
    (
        drm_get_certificate_chain_count,
        "DRMGetCertificateChainCount",
    ),
    (drm_get_client_version, "DRMGetClientVersion"),
    (drm_get_environment_info, "DRMGetEnvironmentInfo"),
    (drm_get_info, "DRMGetInfo"),
    (drm_get_interval_time, "DRMGetIntervalTime"),
    (drm_get_issuance_license_info, "DRMGetIssuanceLicenseInfo"),
    (
        drm_get_issuance_license_template,
        "DRMGetIssuanceLicenseTemplate",
    ),
    (drm_get_meta_data, "DRMGetMetaData"),
    (drm_get_name_and_description, "DRMGetNameAndDescription"),
    (drm_get_owner_license, "DRMGetOwnerLicense"),
    (drm_get_proc_address, "DRMGetProcAddress"),
    (drm_get_revocation_point, "DRMGetRevocationPoint"),
    (drm_get_right_extended_info, "DRMGetRightExtendedInfo"),
    (drm_get_right_info, "DRMGetRightInfo"),
    (drm_get_security_provider, "DRMGetSecurityProvider"),
    (drm_get_service_location, "DRMGetServiceLocation"),
    (
        drm_get_signed_issuance_license,
        "DRMGetSignedIssuanceLicense",
    ),
    (
        drm_get_signed_issuance_license_ex,
        "DRMGetSignedIssuanceLicenseEx",
    ),
    (drm_get_time, "DRMGetTime"),
    (
        drm_get_unbound_license_attribute,
        "DRMGetUnboundLicenseAttribute",
    ),
    (
        drm_get_unbound_license_attribute_count,
        "DRMGetUnboundLicenseAttributeCount",
    ),
    (drm_get_unbound_license_object, "DRMGetUnboundLicenseObject"),
    (
        drm_get_unbound_license_object_count,
        "DRMGetUnboundLicenseObjectCount",
    ),
    (drm_get_usage_policy, "DRMGetUsagePolicy"),
    (drm_get_user_info, "DRMGetUserInfo"),
    (drm_get_user_rights, "DRMGetUserRights"),
    (drm_get_users, "DRMGetUsers"),
    (drm_init_environment, "DRMInitEnvironment"),
    (drm_is_activated, "DRMIsActivated"),
    (drm_is_window_protected, "DRMIsWindowProtected"),
    (drm_load_library, "DRMLoadLibrary"),
    (drm_parse_unbound_license, "DRMParseUnboundLicense"),
    (drm_register_protected_window, "DRMRegisterProtectedWindow"),
    (drm_register_revocation_list, "DRMRegisterRevocationList"),
    (drm_repair, "DRMRepair"),
    (
        drm_set_application_specific_data,
        "DRMSetApplicationSpecificData",
    ),
    (drm_set_global_options, "DRMSetGlobalOptions"),
    (drm_set_interval_time, "DRMSetIntervalTime"),
    (drm_set_meta_data, "DRMSetMetaData"),
    (drm_set_name_and_description, "DRMSetNameAndDescription"),
    (drm_set_revocation_point, "DRMSetRevocationPoint"),
    (drm_set_usage_policy, "DRMSetUsagePolicy"),
    (drm_verify, "DRMVerify"),
    (dll_get_class_object, "DllGetClassObject"),
    (
        __add_machine_cert_to_license_store,
        "__AddMachineCertToLicenseStore",
    ),
];

fn stub_row(function: &'static str) -> StubException {
    StubException {
        code: 0x8000_0100,
        flags: 0x01,
        number_parameters: 2,
        module: "msdrm.dll",
        function,
    }
}

#[test]
fn oracle_drm_register_content_rows() {
    // row 01 value=0 -> ret=0
    assert_eq!(drm_register_content(0), 0);
    // row 02 value=1 -> ret=0
    assert_eq!(drm_register_content(1), 0);
    // row 03 value=-1 -> ret=0
    assert_eq!(drm_register_content(-1), 0);
    // row 04 value=2147483647 -> ret=0
    assert_eq!(drm_register_content(0x7fff_ffff), 0);
    // row 05 value=-2147483648 -> ret=0
    assert_eq!(drm_register_content(i32::MIN), 0);
}

#[test]
fn oracle_stub_rows() {
    // rows 06-90, one per @ stub export in msdrm.spec order
    for (row, &(export, function)) in STUB_ROWS.iter().enumerate() {
        assert_eq!(export(), stub_row(function), "row {:02}", row + 6);
    }
}
