//! Edge-case tests for the ported wbemdisp stub exports.
//!
//! These cover the stub record shape, the export table, and the name lookup. Not every
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
    // Not oracle-derived: the harness only passed the fixed names of wbemdisp.spec. The code and
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
    // Not oracle-derived: the four lines of dlls/wbemdisp/wbemdisp.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("DllCanUnloadNow", ExportKind::Stub),
            ("DllGetClassObject", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::Stub),
            ("DllUnregisterServer", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("DllCanUnloadNow"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DllGetClassObject"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DllRegisterServer"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DllUnregisterServer"), Some(ExportKind::Stub));
    assert_eq!(export_kind("dllcanunloadnow"), None);
    assert_eq!(export_kind(""), None);
}
