//! Edge-case checks for version comparison and exact-name export lookup.
//!
//! These expectations come from the local C condition and spec table, and exercise both sides of
//! the short-circuit comparison and each export kind.

use super::*;

#[test]
fn check_version_accepts_only_the_exact_pair() {
    assert!(d3dx10_check_version(D3D10_SDK_VERSION, 37));
    assert!(!d3dx10_check_version(D3D10_SDK_VERSION, 36));
    assert!(!d3dx10_check_version(D3D10_SDK_VERSION, 38));
    assert!(!d3dx10_check_version(28, 37));
    assert!(!d3dx10_check_version(30, 37));
    assert!(!d3dx10_check_version(0, 0));
}

#[test]
fn export_lookup_preserves_each_spec_kind() {
    assert_eq!(export_kind("D3DX10CreateReduction"), Some(ExportKind::Stub));
    assert_eq!(export_kind("D3DX10CheckVersion"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("D3DX10CreateDevice"),
        Some(ExportKind::Forward("d3dx10_43"))
    );
    assert_eq!(
        export_kind("D3DX10DisassembleEffect"),
        Some(ExportKind::Forward("d3dx10_39"))
    );
}

#[test]
fn export_lookup_is_case_sensitive_and_reports_misses() {
    assert_eq!(export_kind("D3DX10Missing"), None);
    assert_eq!(export_kind("D3DX10CheckVersio"), None);
    assert_eq!(export_kind("d3dx10checkversion"), None);
    assert_eq!(export_kind(""), None);
}
