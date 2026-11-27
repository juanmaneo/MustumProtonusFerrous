//! Edge-case tests for the ported itircl logic.
//!
//! These cover the stub record's constants, the export table and its lookup, and the link between
//! the `DllGetClassObject` helper and the table. Not every expectation is oracle-derived; the ones
//! that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_is_in_spec_order() {
    // Not oracle-derived: the table is transcribed from dlls/itircl/itircl.spec. No harness reads
    // the spec file. The spec has exactly one `@ stub` line, `DllGetClassObject`.
    assert_eq!(
        EXPORTS,
        [
            ("DllCanUnloadNow", ExportKind::PrivateStdcall),
            ("DllGetClassObject", ExportKind::Stub),
            ("DllRegisterServer", ExportKind::PrivateStdcall),
            ("DllUnregisterServer", ExportKind::PrivateStdcall),
        ]
    );
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, 1);
}

#[test]
fn export_kind_finds_each_name() {
    // Not oracle-derived: lookup over the transcribed spec table.
    for (name, kind) in EXPORTS {
        assert_eq!(export_kind(name), Some(kind));
    }
}

#[test]
fn export_kind_rejects_unknown_and_differently_cased_names() {
    // Not oracle-derived: names the spec does not export. Lookup is case-sensitive, as in the
    // fontsub table.
    for name in [
        "",
        "DllMain",
        "dllgetclassobject",
        "DllGetClassObject ",
        "DllGetClassObjectX",
    ] {
        assert_eq!(export_kind(name), None);
    }
}

#[test]
fn stub_helper_matches_its_stub_entry() {
    // Not oracle-derived: the helper's export name must be the table's stub entry, and its module
    // must be `MODULE = itircl.dll` from dlls/itircl/Makefile.in.
    let stub = dll_get_class_object();
    assert_eq!(export_kind(stub.function), Some(ExportKind::Stub));
    assert_eq!(stub.module, "itircl.dll");
    assert_eq!(stub.function, "DllGetClassObject");
}
