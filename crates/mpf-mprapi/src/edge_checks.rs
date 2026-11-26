//! Edge-case tests for the ported mprapi logic.
//!
//! These cover the constants, the out-pointer writes, the export table and the stub record.
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(ERROR_MR_MID_NOT_FOUND, 317);
    assert_eq!(ERROR_NOT_SUPPORTED, 50);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn error_string_writes_null_over_an_existing_null() {
    // Not oracle-derived: the oracle only passed a non-NULL out-pointer. The C body writes NULL
    // unconditionally, so an out-pointer that is already NULL stays NULL.
    let mut localstr = None;
    assert_eq!(
        mpr_admin_get_error_string(0, &mut localstr),
        ERROR_MR_MID_NOT_FOUND
    );
    assert_eq!(localstr, None);
}

#[test]
fn error_string_fails_for_codes_next_to_the_known_one() {
    // Not oracle-derived: boundary codes either side of ERROR_MR_MID_NOT_FOUND. The C body has no
    // code-specific branch, so every code takes the same path.
    for code in [316, 318, u32::MAX] {
        let mut localstr = Some(vec![0x41]);
        assert_eq!(
            mpr_admin_get_error_string(code, &mut localstr),
            ERROR_MR_MID_NOT_FOUND
        );
        assert_eq!(localstr, None);
    }
}

#[test]
fn config_server_connect_writes_null_over_an_existing_handle() {
    // Not oracle-derived: the oracle only passed a non-NULL handle. Same reasoning as above.
    let mut hmprconfig = Some(7);
    assert_eq!(
        mpr_config_server_connect(None, &mut hmprconfig),
        ERROR_NOT_SUPPORTED
    );
    assert_eq!(hmprconfig, None);
}

#[test]
fn stub_record_carries_the_given_names() {
    // Not oracle-derived: the stub is called with names the oracle did not use. The C body copies
    // both pointers into ExceptionInformation without reading them.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Other"),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "Other",
        }
    );
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: checked against the order of dlls/mprapi/mprapi.spec.
    assert_eq!(EXPORTS.len(), 132);
    assert_eq!(EXPORTS[0], ("CompressPhoneNumber", ExportKind::Stub));
    assert_eq!(EXPORTS[8], ("MprAdminGetErrorString", ExportKind::Stdcall));
    assert_eq!(
        EXPORTS[131],
        ("RasPrivilegeAndCallBackNumber", ExportKind::Stub)
    );
    let stdcall: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stdcall)
        .map(|(name, _)| *name)
        .collect();
    assert_eq!(
        stdcall,
        [
            "MprAdminGetErrorString",
            "MprAdminIsServiceRunning",
            "MprConfigServerConnect"
        ]
    );
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a duplicate name would make export_kind ambiguous.
    for (index, (name, _)) in EXPORTS.iter().enumerate() {
        assert!(
            EXPORTS[index + 1..].iter().all(|(other, _)| other != name),
            "{name} is listed twice"
        );
    }
}

#[test]
fn export_kind_finds_every_entry() {
    // Not oracle-derived: the lookup must agree with the table.
    for &(name, kind) in EXPORTS.iter() {
        assert_eq!(export_kind(name), Some(kind));
    }
}

#[test]
fn export_kind_rejects_unknown_and_inexact_names() {
    // Not oracle-derived: names are compared exactly, so case, spacing and unknown names all miss.
    assert_eq!(export_kind("mprAdminIsServiceRunning"), None);
    assert_eq!(export_kind("MprAdminIsServiceRunning "), None);
    assert_eq!(export_kind(""), None);
}
