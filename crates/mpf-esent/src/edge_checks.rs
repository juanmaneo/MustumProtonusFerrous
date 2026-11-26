//! Edge-case tests for the ported esent logic.
//!
//! These cover the constants, the stub record, and the export table. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;
use crate::stub::ESENT_DLL_NAME;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h and
    // MODULE = esent.dll in dlls/esent/Makefile.in.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(ESENT_DLL_NAME, "esent.dll");
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the oracle only passed the 336 export names. The code, flags and
    // parameter count are the same for any names.
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
    // Not oracle-derived: the count and the first and last lines of dlls/esent/esent.spec.
    assert_eq!(EXPORTS.len(), 336);
    assert_eq!(EXPORTS[0], ("DebugExtensionInitialize", ExportKind::Stub));
    assert_eq!(EXPORTS[335], ("JetUpgradeDatabaseW", ExportKind::Stub));
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a duplicate name would make the lookup ambiguous.
    let mut names: Vec<&str> = EXPORTS.iter().map(|&(name, _)| name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn export_kind_matches_exact_spelling() {
    // Not oracle-derived: names are case-sensitive, and only esent.spec names are exports.
    assert_eq!(export_kind("JetAddColumn"), Some(ExportKind::Stub));
    assert_eq!(export_kind("jetaddcolumn"), None);
    assert_eq!(export_kind("JetAddColumn "), None);
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("CreateFontPackage"), None);
}

#[test]
fn every_table_entry_is_found_by_its_name() {
    // Not oracle-derived: the lookup agrees with the table for all 336 entries.
    for (name, kind) in EXPORTS {
        assert_eq!(export_kind(name), Some(kind));
    }
}
