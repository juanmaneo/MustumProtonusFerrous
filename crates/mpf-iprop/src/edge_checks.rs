//! Edge-case tests for the ported iprop tables.
//!
//! The oracle rows cover the spec's eight exports and seven argument lists. These tests cover the
//! lookups that the spec never makes, and the stack sizes past the sweep. Expectations here are not
//! oracle-derived unless a comment says so.

use super::*;

#[test]
fn lookups_are_exact_and_case_sensitive() {
    // Not oracle-derived: winebuild's name table and the loader's export lookup compare names exactly.
    assert_eq!(find_export(""), None);
    assert_eq!(find_export("fmtidtopropstgname"), None);
    assert_eq!(find_export("FmtIdToPropStgName "), None);
    assert_eq!(find_export("PropVariantClear\0"), None);
}

#[test]
fn names_from_other_modules_are_not_forwarded() {
    // Not oracle-derived: MergeFontPackage is exported by fontsub, not by iprop.spec.
    assert_eq!(forward_target("MergeFontPackage"), None);
    assert_eq!(forward_target("NoSuchExport"), None);
}

#[test]
fn every_entry_forwards_to_ole32_under_its_own_name() {
    // Table invariant. The oracle rows pin the same values for all eight exports.
    for export in EXPORTS {
        assert_eq!(export.forward_module, "ole32");
        assert_eq!(export.forward_name, export.name);
    }
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a duplicate name would make find_export depend on table order.
    let mut names: Vec<&str> = EXPORTS.iter().map(|export| export.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn stack_bytes_is_linear_past_the_sweep() {
    // Not oracle-derived: the sweep stops at 16 arguments. These check the same i386 rule beyond it.
    assert_eq!(stack_bytes(&[ArgKind::Ptr; 100]), 400);
    assert_eq!(stack_bytes(&[ArgKind::Wstr; 17]), 68);
}
