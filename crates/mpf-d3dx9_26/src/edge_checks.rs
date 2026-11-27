//! Edge-case tests for the ported d3dx9_26 export table and stub records.
//!
//! The counts come from `grep -c` on `dlls/d3dx9_26/d3dx9_26.spec` (327 `@ stdcall` plus `@ stub`
//! lines, 259 `@ stdcall`, 68 `@ stub`), not from a Wine run. The record constants are checked
//! against the oracle rows in `oracle_checks.rs`.

use super::*;
use std::collections::HashSet;

fn count_of(kind: ExportKind) -> usize {
    EXPORTS.iter().filter(|(_, k)| *k == kind).count()
}

#[test]
fn export_table_matches_spec_counts() {
    assert_eq!(EXPORTS.len(), 327);
    assert_eq!(count_of(ExportKind::Stdcall), 259);
    assert_eq!(count_of(ExportKind::Stub), 68);
}

#[test]
fn export_names_are_unique() {
    let mut seen = HashSet::new();
    for (name, _) in EXPORTS {
        assert!(seen.insert(*name), "duplicate export {name}");
    }
}

#[test]
fn export_kind_reports_declared_kinds() {
    assert_eq!(export_kind("D3DXCheckVersion"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("D3DXGetTargetDescByName"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("D3DXNoSuchExport"), None);
}

#[test]
fn export_kind_is_case_sensitive() {
    assert_eq!(export_kind("d3dxcheckversion"), None);
}

#[test]
fn stub_exception_is_some_only_for_stub_exports() {
    let record = stub_exception("D3DXGetTargetDescByVersion").expect("@ stub export");
    assert_eq!(record.module, "d3dx9_26.dll");
    assert_eq!(record.function, "D3DXGetTargetDescByVersion");
    assert_eq!(stub_exception("D3DXCheckVersion"), None);
    assert_eq!(stub_exception("D3DXNoSuchExport"), None);
}

#[test]
fn stub_record_constants_match_headers() {
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    let record = wine_spec_unimplemented_stub("module.dll", "Function");
    assert_eq!(record.code, EXCEPTION_WINE_STUB);
    assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(record.number_parameters, 2);
    assert_eq!(record.module, "module.dll");
    assert_eq!(record.function, "Function");
}
