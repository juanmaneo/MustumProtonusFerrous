//! Edge-case tests for the ported msvcp_win export table and stub records.
//!
//! These cover the constants, the counts and order of the export table, the exact-match
//! lookups, and the pass-through of the stub arguments. The 168 stub records are checked
//! against the oracle in `oracle_checks.rs`. Not every expectation here is oracle-derived; the
//! ones that aren't say so in a comment.

use super::oracle_checks::ROWS;
use super::*;

#[test]
fn constants_are_pinned() {
    // Oracle-derived: the harness printed code 0x80000100 and flags 0x1 for every row.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_arguments_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the names of msvcp_win.spec. The code, flags
    // and parameter count do not depend on the names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(
        other,
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "Foo",
        }
    );

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_counts_match_the_x86_64_build() {
    // Not oracle-derived: counted from dlls/msvcp_win/msvcp_win.spec. The x86_64 build keeps the
    // 289 entries that have no -arch flag and the 1203 entries with -arch=win64, 1492 in all.
    let count = |kind: ExportKind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(EXPORTS.len(), 1492);
    assert_eq!(count(ExportKind::Cdecl), 1203);
    assert_eq!(count(ExportKind::Extern), 121);
    assert_eq!(count(ExportKind::Stub), 168);
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: a name listed twice would make export_kind return only the first entry.
    let mut names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    names.sort_unstable();
    let total = names.len();
    names.dedup();
    assert_eq!(names.len(), total);
}

#[test]
fn export_table_ends_with_xtime_get() {
    // Not oracle-derived: the last line of dlls/msvcp_win/msvcp_win.spec.
    assert_eq!(EXPORTS.last(), Some(&("xtime_get", ExportKind::Cdecl)));
}

#[test]
fn oracle_rows_are_the_stub_exports_in_spec_order() {
    // Not oracle-derived: checks that the oracle table lists each Stub entry of EXPORTS once,
    // in the same order.
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let rows: Vec<&str> = ROWS.iter().map(|(name, _)| *name).collect();
    assert_eq!(rows, stubs);
}

#[test]
fn export_kind_matches_whole_names() {
    // Not oracle-derived: kinds of named entries in dlls/msvcp_win/msvcp_win.spec, one per kind.
    assert_eq!(
        export_kind("??0?$_Yarn@G@std@@QEAA@XZ"),
        Some(ExportKind::Stub)
    );
    assert_eq!(
        export_kind("??0?$_Yarn@D@std@@QEAA@AEBV01@@Z"),
        Some(ExportKind::Cdecl)
    );
    assert_eq!(
        export_kind("??_7?$basic_ios@DU?$char_traits@D@std@@@std@@6B@"),
        Some(ExportKind::Extern)
    );
    assert_eq!(export_kind("xtime_get"), Some(ExportKind::Cdecl));
}

#[test]
fn export_kind_rejects_prefixes_padding_and_other_case() {
    // Not oracle-derived: lookups are exact and case-sensitive, so a prefix, a padded name or
    // another case is not an export.
    assert_eq!(export_kind(""), None);
    assert_eq!(export_kind("??0?$_Yarn@G@std@@QEAA@X"), None);
    assert_eq!(export_kind("??0?$_Yarn@G@std@@QEAA@XZ "), None);
    assert_eq!(export_kind(" xtime_get"), None);
    assert_eq!(export_kind("XTIME_GET"), None);
}

#[test]
fn export_kind_omits_entries_of_other_builds() {
    // Not oracle-derived: each name is an entry of dlls/msvcp_win/msvcp_win.spec restricted to
    // another build: -arch=arm, -arch=i386 and -arch=win32.
    assert_eq!(export_kind("??0?$_Yarn@D@std@@QAA@ABV01@@Z"), None);
    assert_eq!(export_kind("??0?$_Yarn@D@std@@QAE@ABV01@@Z"), None);
    assert_eq!(
        export_kind("?CaptureCallstack@platform@details@Concurrency@@YAIPAPAXII@Z"),
        None
    );
}
