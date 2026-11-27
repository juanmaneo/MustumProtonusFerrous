//! Edge-case tests for the resutils stub model.
//!
//! The oracle covers the 71 spec names with the fixed module name. These tests cover the export
//! lookup and the pass-through of other names. Expectations that are not oracle-derived say so in
//! a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The oracle rows
    // print the same values.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the 71 names of resutils.spec. The code, flags
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
fn export_table_matches_spec_order() {
    // Not oracle-derived: the first and last lines of dlls/resutils/resutils.spec.
    assert_eq!(EXPORTS.len(), 71);
    assert_eq!(EXPORTS[0].0, "ClusWorkerCheckTerminate");
    assert_eq!(EXPORTS[70].0, "ResUtilVerifyService");
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: the spec lists each name once, so the table has no duplicates.
    let names: HashSet<&str> = EXPORTS.iter().map(|(n, _)| *n).collect();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn export_stub_looks_up_names_case_sensitively() {
    // Not oracle-derived: lookups for names the spec does not export return None.
    assert_eq!(export_stub("resutildupstring"), None);
    assert_eq!(export_stub("ResUtilDupString "), None);
    assert_eq!(export_stub(""), None);
    assert_eq!(export_stub("ResUtilDupString"), Some(res_util_dup_string()));
}
