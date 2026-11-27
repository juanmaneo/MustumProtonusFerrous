//! Edge-case tests for the ported odbccu32 export table and stub.
//!
//! These cover the spec's entry counts, the forward targets, the lookup rules, and the stub
//! record's pass-through of its names. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_has_36_forwards_and_one_stub() {
    // Not oracle-derived: the counts of the lines of odbccu32.spec, 36 stdcall and 1 stub.
    assert_eq!(EXPORTS.len(), 37);
    let forwards = EXPORTS
        .iter()
        .filter(|export| matches!(export.kind, ExportKind::Forward(_)))
        .count();
    let stubs = EXPORTS
        .iter()
        .filter(|export| export.kind == ExportKind::Stub)
        .count();
    assert_eq!((forwards, stubs), (36, 1));
}

#[test]
fn ordinals_increase_in_spec_order() {
    // Not oracle-derived: the spec numbers its lines in increasing order, so the ordinals are
    // unique as well.
    assert!(EXPORTS
        .windows(2)
        .all(|pair| pair[0].ordinal < pair[1].ordinal));
}

#[test]
fn forward_targets_name_the_same_export_in_odbc32() {
    // Not oracle-derived: every forward in the spec is `odbc32.<same name>`. The oracle rows
    // print each target, so this only restates them as a property.
    for export in EXPORTS {
        if let ExportKind::Forward(target) = export.kind {
            assert_eq!(target, format!("odbc32.{}", export.name));
        }
    }
}

#[test]
fn find_export_returns_each_kind_and_none_for_unknown_names() {
    // Not oracle-derived: one forward, the stub, and a name the spec does not export.
    assert_eq!(
        find_export("SQLBindCol").map(|export| export.kind),
        Some(ExportKind::Forward("odbc32.SQLBindCol"))
    );
    assert_eq!(
        find_export("ReleaseCLStmtResources").map(|export| export.kind),
        Some(ExportKind::Stub)
    );
    assert_eq!(find_export("SQLFoo"), None);
}

#[test]
fn find_export_is_case_sensitive() {
    // Not oracle-derived: lookups compare names exactly, as the doc of find_export says.
    assert_eq!(find_export("sqlcancel"), None);
    assert_eq!(
        find_export("SQLCancel").map(|export| export.ordinal),
        Some(5)
    );
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of odbccu32.spec. The code
    // and flags are the same for any names.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", "Func"),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Func",
        }
    );
}

#[test]
fn stub_function_name_matches_its_export_entry() {
    // Not oracle-derived: cross-checks the name the stub helper passes against the export table.
    let stub = release_cl_stmt_resources();
    assert_eq!(stub.module, "odbccu32.dll");
    assert_eq!(
        Some(stub.function),
        find_export("ReleaseCLStmtResources").map(|export| export.name)
    );
}
