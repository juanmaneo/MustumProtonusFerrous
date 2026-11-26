//! Edge-case tests for the ported feclient logic.
//!
//! These cover the constants, the export table and name lookup, the stub record's pass-through
//! names, and the derived impls of [`StubException`]. Not every expectation is oracle-derived;
//! the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the names of feclient.spec. The code, flags
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
fn stub_exception_derived_impls() {
    // Not oracle-derived: calls the derived Clone and Debug impls directly, because assert_eq!
    // only formats its operands when the assertion fails.
    let e = fe_client_initialize();
    assert_eq!(Clone::clone(&e), e);
    assert!(format!("{e:?}").contains("FeClientInitialize"));
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the first and last lines of dlls/feclient/feclient.spec. The full
    // order is checked against the oracle rows in oracle_checks.rs.
    assert_eq!(EXPORTS[0], "EfsUtilGetCurrentKey");
    assert_eq!(EXPORTS[49], "OefsCheckSupport");
}

#[test]
fn is_export_matches_only_spec_names() {
    // Not oracle-derived: every spec name is found, and names the spec does not export are not,
    // including a case variant, a trailing space, a misspelling, and a snake_case helper name.
    for name in EXPORTS {
        assert!(is_export(name), "{name}");
    }
    for name in [
        "efsutilgetcurrentkey",
        "EfsUtilGetCurrentKey ",
        "FeClientInitialise",
        "fe_client_initialize",
        "",
    ] {
        assert!(!is_export(name), "{name:?}");
    }
}
