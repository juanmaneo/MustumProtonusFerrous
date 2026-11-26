//! Edge-case tests for the ported sppc logic.
//!
//! These cover the NULL and handle boundaries, the export table, and the stub record. Expectations
//! that are not oracle-derived say so in a comment.

use super::*;

const APP: Slid = [0x11; 16];
const PRODUCT: Slid = [0x22; 16];

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/slerror.h, include/winternl.h
    // and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(E_INVALIDARG, 0x8007_0057);
    assert_eq!(SL_E_RIGHT_NOT_CONSUMED, 0xC004_F002);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn sl_open_overwrites_any_previous_handle() {
    // Not oracle-derived: the C body does not read the old value, so a live handle is replaced
    // and the call still succeeds.
    let mut handle = SLC_HANDLE;
    assert_eq!(sl_open(Some(&mut handle)), S_OK);
    assert_eq!(handle, SLC_HANDLE);

    let mut handle = usize::MAX;
    assert_eq!(sl_open(Some(&mut handle)), S_OK);
    assert_eq!(handle, SLC_HANDLE);
}

#[test]
fn sl_open_null_is_the_only_failure() {
    // Not oracle-derived: repeated calls with a NULL handle keep returning E_INVALIDARG, and
    // there is no handle to write.
    assert_eq!(sl_open(None), E_INVALIDARG);
    assert_eq!(sl_open(None), E_INVALIDARG);
}

#[test]
fn sl_close_ignores_its_handle() {
    // Not oracle-derived: the boundary values 0 and usize::MAX, which the C body never checks.
    assert_eq!(sl_close(0), S_OK);
    assert_eq!(sl_close(SLC_HANDLE), S_OK);
    assert_eq!(sl_close(usize::MAX), S_OK);
}

#[test]
fn persist_only_tests_app_for_null() {
    // Not oracle-derived: product and flags are never read, so every combination with a
    // non-NULL app succeeds, and a NULL app fails whatever the other arguments are.
    for product in [None, Some(&PRODUCT)] {
        for flags in [0, 1, 0x8000_0000, u32::MAX] {
            assert_eq!(
                sl_persist_application_policies(Some(&APP), product, flags),
                S_OK
            );
            assert_eq!(
                sl_persist_application_policies(None, product, flags),
                E_INVALIDARG
            );
        }
    }
}

#[test]
fn licensing_status_is_a_constant() {
    // Not oracle-derived: the C body has no state, so repeated calls agree.
    assert_eq!(
        sl_get_licensing_status_information(),
        SL_E_RIGHT_NOT_CONSUMED
    );
    assert_eq!(
        sl_get_licensing_status_information(),
        SL_E_RIGHT_NOT_CONSUMED
    );
}

#[test]
fn stub_record_passes_names_through() {
    // Not oracle-derived: the oracle only passes "sppc.dll". This checks that the module and the
    // function name are returned unchanged and that the other fields are fixed.
    let s = wine_spec_unimplemented_stub("other.dll", "Anything");
    assert_eq!(
        s,
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "other.dll",
            function: "Anything",
        }
    );
}

#[test]
fn export_table_matches_the_spec() {
    // Not oracle-derived: counts and order come from dlls/sppc/sppc.spec (67 lines: 4 @ stdcall,
    // 63 @ stub).
    assert_eq!(EXPORTS.len(), 67);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, k)| *k == ExportKind::Stdcall)
            .count(),
        4
    );
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, k)| *k == ExportKind::Stub)
            .count(),
        63
    );
    assert_eq!(EXPORTS[0], ("SLCallServer", ExportKind::Stub));
    assert_eq!(EXPORTS[66], ("SLUnregisterPlugin", ExportKind::Stub));
}

#[test]
fn export_kind_lookup() {
    // Not oracle-derived: the four real functions, one stub, and names that must not match.
    assert_eq!(export_kind("SLOpen"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("SLClose"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("SLGetLicensingStatusInformation"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("SLPersistApplicationPolicies"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("SLCallServer"), Some(ExportKind::Stub));
    assert_eq!(export_kind("SLpVLActivateProduct"), Some(ExportKind::Stub));
    assert_eq!(export_kind("slopen"), None);
    assert_eq!(export_kind("SLOpenX"), None);
    assert_eq!(export_kind(""), None);
}
