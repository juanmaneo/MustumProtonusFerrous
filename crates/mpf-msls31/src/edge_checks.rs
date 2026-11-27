//! Edge-case tests for the ported msls31 export table and stub records.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The oracle shim
    // defines the same values, so its rows confirm the record, not the headers.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    // Not oracle-derived: the MODULE line of dlls/msls31/Makefile.in.
    assert_eq!(
        stub_exception("LsCompressSubline").unwrap().module,
        "msls31.dll"
    );
}

#[test]
fn export_table_matches_spec() {
    // Not oracle-derived: the 79 `@ stub` lines of dlls/msls31/msls31.spec, in order. Checked
    // here: the count, the first and last names, and that no name is repeated.
    assert_eq!(EXPORTS.len(), 79);
    assert_eq!(EXPORTS[0].0, "LsAppendRunToCurrentSubline");
    assert_eq!(EXPORTS[78].0, "LssbGetVisibleDcpInSubline");
    let names: HashSet<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(names.len(), EXPORTS.len());

    // Not oracle-derived: each export's helper raises a record whose function is the export name.
    for (name, helper) in EXPORTS {
        assert_eq!(helper().function, name);
        assert_eq!(stub_exception(name), Some(helper()));
    }
}

#[test]
fn unknown_names_are_not_exports() {
    // Not oracle-derived: names that msls31.spec does not export. Names are case-sensitive, and
    // MergeFontPackage is an export of fontsub, not of msls31.
    assert_eq!(stub_exception("lscompresssubline"), None);
    assert_eq!(stub_exception("LsCompressSubline "), None);
    assert_eq!(stub_exception("MergeFontPackage"), None);
    assert_eq!(stub_exception(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of msls31.spec. The code and
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
