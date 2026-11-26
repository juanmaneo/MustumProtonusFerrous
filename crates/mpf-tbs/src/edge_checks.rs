//! Edge-case tests for the ported tbs logic.
//!
//! These cover the constant values, the stub record fields, the export table, and the pairing of
//! each `@ stub` export with its helper. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/tbs.h, include/winerror.h, include/winternl.h
    // and include/winnt.h. The oracle rows print the same values for the functions that return them.
    assert_eq!(TBS_E_TPM_NOT_FOUND, 0x8028_400F);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn fixed_results_are_failures() {
    // Not oracle-derived: both `TBS_E_TPM_NOT_FOUND` and `E_NOTIMPL` have the HRESULT severity bit
    // set, so the signed values are negative and `FAILED()` holds for each.
    assert!((tbsi_context_create() as i32) < 0);
    assert!((tbsi_get_device_info() as i32) < 0);
    assert!(get_device_id_string() < 0);
}

#[test]
fn fixed_results_ignore_their_arguments() {
    // Not oracle-derived: the C bodies only trace their arguments. Repeated calls return the same
    // value, since none of the functions keeps state.
    for _ in 0..3 {
        assert_eq!(tbsi_context_create(), TBS_E_TPM_NOT_FOUND);
        assert_eq!(tbsi_get_device_info(), TBS_E_TPM_NOT_FOUND);
        assert_eq!(get_device_id_string(), E_NOTIMPL);
    }
}

#[test]
fn wine_spec_unimplemented_stub_passes_names_through() {
    // Not oracle-derived: the C function copies its two pointers into the record without reading
    // the strings. Empty and unusual names therefore pass through unchanged.
    let record = wine_spec_unimplemented_stub("", "");
    assert_eq!(record.module, "");
    assert_eq!(record.function, "");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);

    let record = wine_spec_unimplemented_stub("other.dll", "Name With Spaces");
    assert_eq!(record.module, "other.dll");
    assert_eq!(record.function, "Name With Spaces");
}

#[test]
fn every_stub_helper_names_its_export() {
    // Not oracle-derived: the 18 `@ stub` exports of `dlls/tbs/tbs.spec` and the helper for each.
    // Every helper must report the module `tbs.dll` and its own export name, and the export
    // table must mark that name as a stub.
    type StubHelper = fn() -> StubException;
    let helpers: [(&str, StubHelper); 18] = [
        (
            "Tbsi_Create_Attestation_From_Log",
            tbsi_create_attestation_from_log,
        ),
        ("Tbsi_Get_TCG_Logs", tbsi_get_tcg_logs),
        ("GetDeviceID", get_device_id),
        ("GetDeviceIDWithTimeout", get_device_id_with_timeout),
        ("Tbsi_Create_Windows_Key", tbsi_create_windows_key),
        ("Tbsi_FilterLog", tbsi_filter_log),
        ("Tbsi_Get_OwnerAuth", tbsi_get_owner_auth),
        ("Tbsi_Get_TCG_Log", tbsi_get_tcg_log),
        ("Tbsi_Get_TCG_Log_Ex", tbsi_get_tcg_log_ex),
        (
            "Tbsi_Physical_Presence_Command",
            tbsi_physical_presence_command,
        ),
        ("Tbsi_Revoke_Attestation", tbsi_revoke_attestation),
        ("Tbsi_ShaHash", tbsi_sha_hash),
        ("Tbsip_Cancel_Commands", tbsip_cancel_commands),
        ("Tbsip_Context_Close", tbsip_context_close),
        ("Tbsip_Submit_Command", tbsip_submit_command),
        (
            "Tbsip_Submit_Command_NonBlocking",
            tbsip_submit_command_non_blocking,
        ),
        (
            "Tbsip_TestInterruptInformation",
            tbsip_test_interrupt_information,
        ),
        ("Tbsip_TestMorBit", tbsip_test_mor_bit),
    ];
    for (name, helper) in helpers {
        let record = helper();
        assert_eq!(record.module, "tbs.dll");
        assert_eq!(record.function, name);
        assert_eq!(record.number_parameters, 2);
        assert_eq!(export_kind(name), Some(ExportKind::Stub));
    }
}

#[test]
fn export_table_matches_spec_order_and_counts() {
    // Not oracle-derived: the 21 names of `dlls/tbs/tbs.spec`, in spec order, with 18 stubs and
    // 3 stdcall functions.
    assert_eq!(EXPORTS.len(), 21);
    assert_eq!(
        EXPORTS[0],
        ("Tbsi_Create_Attestation_From_Log", ExportKind::Stub)
    );
    assert_eq!(EXPORTS[3], ("GetDeviceIDString", ExportKind::Stdcall));
    assert_eq!(EXPORTS[5], ("Tbsi_Context_Create", ExportKind::Stdcall));
    assert_eq!(EXPORTS[8], ("Tbsi_GetDeviceInfo", ExportKind::Stdcall));
    assert_eq!(EXPORTS[20], ("Tbsip_TestMorBit", ExportKind::Stub));

    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    let stdcalls = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .count();
    assert_eq!(stubs, 18);
    assert_eq!(stdcalls, 3);
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a duplicate name would make `export_kind` ambiguous.
    let names: HashSet<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn export_kind_lookups() {
    // Not oracle-derived: the lookup is exact and case-sensitive. Unknown and empty names are
    // not exports.
    assert_eq!(export_kind("GetDeviceIDString"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("Tbsi_ShaHash"), Some(ExportKind::Stub));
    assert_eq!(export_kind("tbsi_shahash"), None);
    assert_eq!(export_kind("TBSI_SHAHASH"), None);
    assert_eq!(export_kind("Tbsi_ShaHash "), None);
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("NoSuchExport"), None);
}
