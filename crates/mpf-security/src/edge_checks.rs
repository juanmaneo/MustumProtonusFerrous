//! Edge-case tests for the ported `security.spec` export table.
//!
//! These cover the constants, the ordinal and name lookups, the forward split, and the table
//! invariants. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from MODULE and IMPORTS in dlls/security/Makefile.in.
    assert_eq!(MODULE, "security.dll");
    assert_eq!(IMPORTS, "secur32");
}

#[test]
fn table_size_and_dense_ordinals() {
    // Oracle-derived: the harness summary says rows=36 base=1 limit=36, and row N has ordinal N.
    assert_eq!(EXPORTS.len(), 36);
    for (index, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), index + 1);
    }
}

#[test]
fn ordinal_lookup_boundaries() {
    // Oracle-derived for 1 and 36 (rows 01 and 36). The 0, 37 and u16::MAX cases are not:
    // ordinal 0 is never assigned, and anything above the limit has no export.
    assert_eq!(export_by_ordinal(0), None);
    assert_eq!(
        export_by_ordinal(1).map(|e| e.name),
        Some("AcceptSecurityContext")
    );
    assert_eq!(
        export_by_ordinal(36).map(|e| e.name),
        Some("VerifySignature")
    );
    assert_eq!(export_by_ordinal(37), None);
    assert_eq!(export_by_ordinal(u16::MAX), None);
}

#[test]
fn name_lookup_is_case_sensitive() {
    // Not oracle-derived: PE export names are matched exactly.
    assert_eq!(
        export_by_name("AcceptSecurityContext").map(|e| e.ordinal),
        Some(1)
    );
    assert_eq!(export_by_name("acceptsecuritycontext"), None);
    assert_eq!(export_by_name(""), None);
    assert_eq!(export_by_name("NoSuchExport"), None);
}

#[test]
fn zero_argument_exports() {
    // Oracle-derived: rows 21 and 22 (InitSecurityInterfaceA and W) have nb_args=0.
    assert_eq!(
        export_by_name("InitSecurityInterfaceA").map(|e| e.nb_args),
        Some(0)
    );
    assert_eq!(
        export_by_name("InitSecurityInterfaceW").map(|e| e.nb_args),
        Some(0)
    );
}

#[test]
fn forwards_split_at_first_dot() {
    // Not oracle-derived: the split rule is winebuild's test for a forward, a '.' in the link name.
    assert_eq!(
        split_forward("secur32.AcceptSecurityContext"),
        Some(("secur32", "AcceptSecurityContext"))
    );
    assert_eq!(split_forward("a.b.c"), Some(("a", "b.c")));
    assert_eq!(split_forward("NoDot"), None);
    assert_eq!(split_forward(""), None);
}

#[test]
fn every_export_forwards_to_imports() {
    // Not oracle-derived: it re-derives the rule from the table. The oracle rows agree (forward=1,
    // link=secur32.<name> on every row).
    for export in EXPORTS.iter() {
        assert_eq!(split_forward(export.forward), Some((IMPORTS, export.name)));
    }
}
