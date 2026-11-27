//! Edge-case tests for the npptools export table and the stub records.
//!
//! The table and the record are fixed, so these tests pin the constants, the lookup misses, and
//! the record each table entry produces. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h and MODULE in
    // dlls/npptools/Makefile.in.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(NPPTOOLS_DLL_NAME, "npptools.dll");
}

#[test]
fn export_table_has_the_spec_exports_in_order() {
    // Not oracle-derived: the count and the first and last lines of dlls/npptools/npptools.spec.
    assert_eq!(STUB_EXPORTS.len(), 60);
    assert_eq!(STUB_EXPORTS[0].0, "ClearEventData");
    assert_eq!(STUB_EXPORTS[59].0, "setKeyAndValue");
    assert!(is_export("ClearEventData"));
    assert!(is_export("setKeyAndValue"));
}

#[test]
fn every_table_entry_raises_its_own_name() {
    // Not oracle-derived: the same check as the oracle rows, run over every table entry, so a
    // helper that names the wrong export fails here even without a row.
    for (name, stub) in STUB_EXPORTS {
        let raised = stub();
        assert_eq!(raised.code, EXCEPTION_WINE_STUB);
        assert_eq!(raised.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(raised.number_parameters, 2);
        assert_eq!(raised.module, "npptools.dll");
        assert_eq!(raised.function, *name);
    }
}

#[test]
fn lookup_returns_the_helper_record_for_a_hit() {
    // Not oracle-derived: a hit by name returns the same record as calling the helper directly.
    assert_eq!(call_export("CreateBlob"), Some(create_blob()));
}

#[test]
fn lookups_miss_unknown_and_differently_cased_names() {
    // Not oracle-derived: the lookup is exact and case-sensitive, and an empty name matches nothing.
    assert!(!is_export("createblob"));
    assert!(!is_export("CreateBlob "));
    assert!(!is_export(""));
    assert_eq!(call_export("setkeyandvalue"), None);
    assert_eq!(call_export(""), None);
    assert_eq!(call_export("NotAnExport"), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: wine_spec_unimplemented_stub copies its arguments into the record and
    // uses the same code, flags and parameter count for any names.
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
