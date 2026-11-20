//! Boundary and constant checks beyond the sanitizer oracle rows.

use super::{
    sp_get_info_schannel, sp_get_info_unified, sp_lsa_mode_initialize, sp_user_mode_initialize,
    Status, SCHANNEL_CAPABILITIES, SCHANNEL_CB_MAX_TOKEN, SCHANNEL_TABLE_COUNT, SCHANNEL_W_VERSION,
    SECPKG_FLAG_ACCEPT_WIN32_NAME, SECPKG_FLAG_CONNECTION, SECPKG_FLAG_EXTENDED_ERROR,
    SECPKG_FLAG_IMPERSONATION, SECPKG_FLAG_INTEGRITY, SECPKG_FLAG_MULTI_REQUIRED,
    SECPKG_FLAG_MUTUAL_AUTH, SECPKG_FLAG_PRIVACY, SECPKG_FLAG_STREAM, SECPKG_INTERFACE_VERSION,
    SECPKG_INTERFACE_VERSION_8, UNISP_RPC_ID,
};

#[test]
fn status_codes_match_ntstatus() {
    assert_eq!(Status::Success.code(), 0x0000_0000);
    assert_eq!(Status::InvalidParameter.code(), 0xC000_000D);
}

#[test]
fn user_mode_accepts_only_the_exact_interface_version() {
    assert_eq!(
        sp_user_mode_initialize(SECPKG_INTERFACE_VERSION),
        (
            Status::Success,
            SECPKG_INTERFACE_VERSION,
            SCHANNEL_TABLE_COUNT
        )
    );
    for version in [
        SECPKG_INTERFACE_VERSION - 1,
        SECPKG_INTERFACE_VERSION + 1,
        SECPKG_INTERFACE_VERSION_8,
    ] {
        assert_eq!(
            sp_user_mode_initialize(version),
            (Status::InvalidParameter, 0, 0),
            "version {version:#x}"
        );
    }
}

#[test]
fn lsa_mode_reports_version_8_and_two_tables() {
    assert_eq!(SECPKG_INTERFACE_VERSION_8, 0x0080_0000);
    assert_eq!(SCHANNEL_TABLE_COUNT, 2);
    assert_eq!(
        sp_lsa_mode_initialize(),
        (Status::Success, SECPKG_INTERFACE_VERSION_8, 2)
    );
}

#[test]
fn capability_mask_is_the_union_of_its_flags() {
    let flags = [
        SECPKG_FLAG_MUTUAL_AUTH,
        SECPKG_FLAG_INTEGRITY,
        SECPKG_FLAG_PRIVACY,
        SECPKG_FLAG_CONNECTION,
        SECPKG_FLAG_MULTI_REQUIRED,
        SECPKG_FLAG_EXTENDED_ERROR,
        SECPKG_FLAG_IMPERSONATION,
        SECPKG_FLAG_ACCEPT_WIN32_NAME,
        SECPKG_FLAG_STREAM,
    ];
    assert_eq!(
        flags.iter().fold(0, |acc, flag| acc | flag),
        SCHANNEL_CAPABILITIES
    );
    assert_eq!(SCHANNEL_CAPABILITIES.count_ones(), 9);
}

#[test]
fn info_constants_match_wine_headers() {
    assert_eq!(UNISP_RPC_ID, 14);
    assert_eq!(SCHANNEL_W_VERSION, 1);
    assert_eq!(SCHANNEL_CB_MAX_TOKEN, 0x4000);
}

#[test]
fn both_info_functions_share_numeric_fields_but_differ_in_names() {
    let (status_unified, unified) = sp_get_info_unified();
    let (status_schannel, schannel) = sp_get_info_schannel();
    assert_eq!(status_unified, Status::Success);
    assert_eq!(status_schannel, Status::Success);
    assert_eq!(unified.f_capabilities, schannel.f_capabilities);
    assert_eq!(unified.w_version, schannel.w_version);
    assert_eq!(unified.w_rpc_id, schannel.w_rpc_id);
    assert_eq!(unified.cb_max_token, schannel.cb_max_token);
    assert_ne!(unified.name, schannel.name);
    assert_ne!(unified.comment, schannel.comment);
    assert_eq!(unified.name, unified.comment);
}
