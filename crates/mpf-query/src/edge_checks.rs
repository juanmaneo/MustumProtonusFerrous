//! Edge-case tests for the ported mpf-query logic.
//!
//! These cover the constants, the spec table and its lookup, the stub record and the out-parameter
//! write. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use crate::oracle_checks::STUB_ROWS;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values transcribed from include/winerror.h, include/cierror.h,
    // include/winternl.h and include/winnt.h.
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    assert_eq!(CI_E_NOT_RUNNING as u32, 0x8004_1820);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(crate::stub::QUERY_DLL_NAME, "query.dll");
}

#[test]
fn export_table_has_the_spec_counts_and_unique_names() {
    // Not oracle-derived: counts of the export lines in dlls/query/query.spec.
    assert_eq!(EXPORTS.len(), 46);
    let count = |kind: ExportKind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(count(ExportKind::Stub), 38);
    assert_eq!(count(ExportKind::Ported), 5);
    assert_eq!(count(ExportKind::NoBody), 3);
    for (i, (name, _)) in EXPORTS.iter().enumerate() {
        assert!(EXPORTS[..i].iter().all(|(other, _)| other != name));
    }
}

#[test]
fn export_kind_needs_the_exact_spec_name() {
    // Not oracle-derived: lookups against the spec names, including the decorated ones.
    assert_eq!(export_kind("_AbortMerges@16"), Some(ExportKind::Stub));
    assert_eq!(export_kind("_AbortMerges"), None);
    assert_eq!(export_kind("abortmerges@16"), None);
    assert_eq!(export_kind("LoadIFilter"), Some(ExportKind::Ported));
    assert_eq!(export_kind("loadifilter"), None);
    assert_eq!(export_kind("DllRegisterServer"), Some(ExportKind::NoBody));
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_rows_are_the_stub_exports_in_spec_order() {
    // Not oracle-derived: checks the oracle vector against the spec table.
    let spec_stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let row_functions: Vec<&str> = STUB_ROWS.iter().map(|row| row.5).collect();
    assert_eq!(row_functions, spec_stubs);
}

#[test]
fn stub_record_copies_its_arguments() {
    // Not oracle-derived: the C harness never passed empty names. The record copies both
    // arguments as they are, with the constant code, flags and parameter count.
    assert_eq!(
        wine_spec_unimplemented_stub("", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "",
            function: "",
        }
    );
}

#[test]
fn load_ifilter_overwrites_a_previous_value() {
    // Not oracle-derived: the oracle rows use an integer. Here the previous value is an owned
    // String, to show the out-parameter is replaced with NULL and the old value is released.
    let mut out = Some(String::from("previous"));
    assert_eq!(load_ifilter(&mut out), E_NOTIMPL);
    assert_eq!(out, None);
}
