//! Edge-case tests for the ported rasdlg logic.
//!
//! These cover the constants, the stub record, and the export table and its lookups. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of rasdlg.spec. The code, flags
    // and parameter count are the same for any names.
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
fn export_table_matches_spec_shape() {
    // Not oracle-derived: the 36 export lines of dlls/rasdlg/rasdlg.spec, 35 @ stub and one
    // @ stdcall, in spec order.
    assert_eq!(EXPORTS.len(), 36);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stub)
            .count(),
        35
    );
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, kind)| *kind == ExportKind::Stdcall)
            .count(),
        1
    );
    assert_eq!(EXPORTS[0], ("DwTerminalDlg", ExportKind::Stub));
    assert_eq!(EXPORTS[9], ("RasEntryDlgW", ExportKind::Stdcall));
    assert_eq!(EXPORTS[35], ("RouterEntryDlgW", ExportKind::Stub));
}

#[test]
fn export_kind_is_exact_and_case_sensitive() {
    // Not oracle-derived: lookups against the names of rasdlg.spec.
    assert_eq!(export_kind("RasEntryDlgW"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("RasEntryDlgA"), Some(ExportKind::Stub));
    assert_eq!(export_kind("RouterEntryDlgW"), Some(ExportKind::Stub));
    assert_eq!(export_kind("rasentrydlgw"), None);
    assert_eq!(export_kind("RasEntryDlgW "), None);
    assert_eq!(export_kind(""), None);
}
