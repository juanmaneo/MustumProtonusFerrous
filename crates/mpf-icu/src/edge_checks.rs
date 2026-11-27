//! Edge-case tests for the ported icu stub records and export table.
//!
//! Not every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Oracle-derived: rows 01-15 of oracle_checks.rs print code=0x80000100 and flags=0x1.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn every_stub_helper_names_a_stub_export() {
    // Not oracle-derived: the helper names and the export table both come from icu.spec, and
    // the module name is MODULE = icu.dll in dlls/icu/Makefile.in.
    let helpers = [
        close_default_icu_grouping_letters(),
        get_default_icu_grouping_letters(),
        get_icu_grouping_letter(),
        sort_close_handle(),
        sort_get_handle(),
        sort_get_search_key(),
    ];
    for record in helpers {
        assert_eq!(record.module, "icu.dll");
        assert_eq!(record.number_parameters, 2);
        assert_eq!(export_kind(record.function), Some(ExportKind::Stub));
    }
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, helpers.len());
}

#[test]
fn export_table_counts_match_icu_spec() {
    // Not oracle-derived: counted from the 1059 lines of dlls/icu/icu.spec with grep: 6 @ stub,
    // 1046 @ cdecl and 7 @ varargs.
    let count = |kind| EXPORTS.iter().filter(|(_, k)| *k == kind).count();
    assert_eq!(EXPORTS.len(), 1059);
    assert_eq!(count(ExportKind::Stub), 6);
    assert_eq!(count(ExportKind::Cdecl), 1046);
    assert_eq!(count(ExportKind::Varargs), 7);
}

#[test]
fn export_table_names_are_unique() {
    // Not oracle-derived: an export name appears once in icu.spec, so the table has no repeats.
    let mut names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn export_table_endpoints_match_spec() {
    // Not oracle-derived: the first and last export lines of dlls/icu/icu.spec, and the first
    // @ cdecl line after the six stubs.
    assert_eq!(
        EXPORTS[0],
        ("CloseDefaultICUGroupingLetters", ExportKind::Stub)
    );
    assert_eq!(
        EXPORTS[6],
        ("UCNV_FROM_U_CALLBACK_ESCAPE", ExportKind::Cdecl)
    );
    assert_eq!(EXPORTS[1058], ("utrans_unregisterID", ExportKind::Cdecl));
}

#[test]
fn export_kind_lookup_is_case_sensitive_and_total() {
    // Not oracle-derived: lookup behaviour of export_kind. Names are matched exactly, so a case
    // change, a trailing space and the empty name are all unknown.
    assert_eq!(export_kind("SortGetSearchKey"), Some(ExportKind::Stub));
    assert_eq!(export_kind("u_formatMessage"), Some(ExportKind::Varargs));
    assert_eq!(export_kind("u_strlen"), Some(ExportKind::Cdecl));
    assert_eq!(export_kind("sortgetsearchkey"), None);
    assert_eq!(export_kind("u_strlen "), None);
    assert_eq!(export_kind(""), None);
}
