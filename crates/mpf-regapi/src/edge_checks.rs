//! Edge-case tests for the ported regapi logic.
//!
//! These cover the constants, the stub record for arbitrary names, and the export table with its
//! lookup. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the stub copies whatever names it is given. The code, flags and
    // parameter count do not depend on the names.
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
fn export_table_matches_spec() {
    // Not oracle-derived: the length, the kinds, and the first and last names of the 69 `@ stub`
    // lines of dlls/regapi/regapi.spec. The full order is checked by `oracle_stub_rows`.
    assert_eq!(EXPORTS.len(), 69);
    assert!(EXPORTS.iter().all(|&(_, kind)| kind == ExportKind::Stub));
    assert_eq!(EXPORTS[0], ("RegBuildNumberQuery", ExportKind::Stub));
    assert_eq!(
        EXPORTS[68],
        ("WaitForTSConnectionsPolicyChanges", ExportKind::Stub)
    );
    assert_eq!(export_kind("RegSAMUserConfig"), Some(ExportKind::Stub));
    assert_eq!(export_kind("reg_build_number_query"), None);
    assert_eq!(export_kind("regbuildnumberquery"), None);
    assert_eq!(export_kind(""), None);
}
