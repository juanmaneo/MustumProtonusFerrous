//! Edge-case tests for the ported mscorwks logic.
//!
//! These cover the stub record, the export table and its forwards, and the name lookup. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The oracle shim
    // uses the same values, but the rows only check the record they produce.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of mscorwks.spec. The code and
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

#[test]
fn export_table_has_spec_counts() {
    // Not oracle-derived: the spec has 110 lines, 97 `@ stub` and 13 `@ stdcall` forwards.
    assert_eq!(EXPORTS.len(), 110);
    let stubs = EXPORTS
        .iter()
        .filter(|export| export.kind == ExportKind::Stub)
        .count();
    let forwards = EXPORTS
        .iter()
        .filter(|export| matches!(export.kind, ExportKind::Forward(_)))
        .count();
    assert_eq!(stubs, 97);
    assert_eq!(forwards, 13);
}

#[test]
fn ordinals_follow_spec_position() {
    // Not oracle-derived: winebuild's assign_ordinals (tools/winebuild/parser.c) gives each entry
    // the lowest free ordinal from 1, in spec order. The spec has no explicit ordinals.
    for (index, export) in EXPORTS.iter().enumerate() {
        assert_eq!(usize::from(export.ordinal), index + 1, "{}", export.name);
    }
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a repeated name would make `export_by_name` ambiguous.
    let names: HashSet<&str> = EXPORTS.iter().map(|export| export.name).collect();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn forwards_target_the_same_name_in_mscoree() {
    // Not oracle-derived: every `@ stdcall` line in the spec reads `... mscoree.<name>`.
    for export in EXPORTS.iter() {
        if let ExportKind::Forward(target) = export.kind {
            assert_eq!(target, format!("mscoree.{}", export.name));
        }
    }
}

#[test]
fn export_by_name_finds_first_stub_and_last_forward() {
    // Not oracle-derived: the first and last lines of mscorwks.spec, and the ordinal of CopyPDBs
    // at line 27.
    let first = export_by_name("LoadStringRC").unwrap();
    assert_eq!(first.ordinal, 1);
    assert_eq!(first.kind, ExportKind::Forward("mscoree.LoadStringRC"));

    let copy_pdbs = export_by_name("CopyPDBs").unwrap();
    assert_eq!(copy_pdbs.ordinal, 27);
    assert_eq!(copy_pdbs.kind, ExportKind::Stub);

    let last = export_by_name("_CorExeMain").unwrap();
    assert_eq!(last.ordinal, 110);
    assert_eq!(last.kind, ExportKind::Forward("mscoree._CorExeMain"));
}

#[test]
fn export_by_name_matches_exactly() {
    // Not oracle-derived: lookups are case-sensitive and match the whole name.
    assert_eq!(export_by_name("copypdbs"), None);
    assert_eq!(export_by_name("CopyPDB"), None);
    assert_eq!(export_by_name(""), None);
}
