//! Edge-case tests for the ported olesvr32 logic.
//!
//! These cover the handle counter's boundaries, the out-parameter writes, and the stub and
//! export tables. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from the C enum and the include/ headers. The oracle prints the
    // same numbers (ret=0 for OLE_OK, code and flags for the stub exception).
    assert_eq!(OLE_OK, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn handle_counter_starts_at_zero_and_counts_up() {
    // Not oracle-derived: the first three values of the same counter, checked in order.
    let mut counter = HandleCounter::default();
    assert_eq!(counter.current_handle, 0);
    assert_eq!(counter.next_handle(), 1);
    assert_eq!(counter.next_handle(), 2);
    assert_eq!(counter.next_handle(), 3);
    assert_eq!(counter.current_handle, 3);
}

#[test]
fn handle_counter_wraps_at_i32_max() {
    // Not oracle-derived for the checks: the value repeats excluded row 17, which UBSan flags as
    // a signed overflow at `++OLE_current_handle`.
    let mut counter = HandleCounter {
        current_handle: i32::MAX,
    };
    let mut h = 0;
    assert_eq!(ole_register_server(&mut counter, &mut h), OLE_OK);
    assert_eq!(h, i32::MIN);
    assert_eq!(counter.current_handle, i32::MIN);
}

#[test]
fn unblock_server_stores_zero_over_any_value() {
    // Not oracle-derived: the oracle stores FALSE over 1 and -7. These are the extremes.
    let mut block = i32::MIN;
    assert_eq!(ole_unblock_server(&mut block), OLE_OK);
    assert_eq!(block, 0);

    let mut block = i32::MAX;
    assert_eq!(ole_unblock_server(&mut block), OLE_OK);
    assert_eq!(block, 0);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of olesvr32.spec. The code and
    // flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the twelve lines of dlls/olesvr32/olesvr32.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("WEP", ExportKind::Stub),
            ("OleRegisterServer", ExportKind::Stdcall),
            ("OleRevokeServer", ExportKind::Stdcall),
            ("OleBlockServer", ExportKind::Stdcall),
            ("OleUnblockServer", ExportKind::Stdcall),
            ("OleRegisterServerDoc", ExportKind::Stdcall),
            ("OleRevokeServerDoc", ExportKind::Stdcall),
            ("OleRenameServerDoc", ExportKind::Stdcall),
            ("OleRevertServerDoc", ExportKind::Stdcall),
            ("OleSavedServerDoc", ExportKind::Stdcall),
            ("OleRevokeObject", ExportKind::Stub),
            ("OleQueryServerVersion", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("WEP"), Some(ExportKind::Stub));
    assert_eq!(export_kind("OleRegisterServer"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("OleQueryServerVersion"), Some(ExportKind::Stub));
    assert_eq!(export_kind("oleregisterserver"), None);
    assert_eq!(export_kind(""), None);
}
