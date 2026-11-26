//! Edge-case tests for the atmlib port.
//!
//! Expectations marked "not oracle-derived" come from the port's own contract, not from a
//! harness row.

use super::*;

#[test]
fn export_kind_resolves_boundaries_and_both_kinds() {
    // Oracle-derived rows: first and last table entries, plus both kinds.
    assert_eq!(export_kind("ATMAddFont"), Some(ExportKind::Stub));
    assert_eq!(export_kind("ATMXYShowTextW"), Some(ExportKind::Stub));
    assert_eq!(export_kind("ATMFinish"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("ATMProperlyLoaded"), Some(ExportKind::Stdcall));
}

#[test]
fn export_kind_rejects_unknown_and_inexact_names() {
    // Not oracle-derived: lookup is exact and case-sensitive.
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("atmfinish"), None);
    assert_eq!(export_kind("ATMAddFont "), None);
    assert_eq!(export_kind("NoSuchExport"), None);
}

#[test]
fn export_table_names_are_unique() {
    // Not oracle-derived: the table must not repeat a name.
    let mut names: Vec<&str> = EXPORTS.iter().map(|&(name, _)| name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn unimplemented_stub_passes_names_through() {
    // Not oracle-derived: the harness only passed fixed, non-empty names.
    let empty = unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
    assert_eq!(empty.code, EXCEPTION_WINE_STUB);
    assert_eq!(empty.flags, EXCEPTION_NONCONTINUABLE);

    let named = unimplemented_stub("other.dll", "ATMGetVersion");
    assert_eq!(named.module, "other.dll");
    assert_eq!(named.function, "ATMGetVersion");
    assert_eq!(named.number_parameters, 2);
}
