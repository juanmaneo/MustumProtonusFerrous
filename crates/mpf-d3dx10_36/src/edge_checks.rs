//! Edge-case tests for the ported d3dx10_36 logic.
//!
//! These cover the version boundaries around each argument, the lookup misses, and the kind of
//! the names whose bodies are in `dlls/d3dx9_36`. These expectations come from reading the spec
//! and `d3dx10_36_main.c`, not from the oracle.

use super::*;

#[test]
fn check_version_accepts_only_the_exact_pair() {
    assert!(d3dx10_check_version(D3D10_SDK_VERSION, 36));
    assert!(!d3dx10_check_version(D3D10_SDK_VERSION, 35));
    assert!(!d3dx10_check_version(D3D10_SDK_VERSION, 37));
    assert!(!d3dx10_check_version(28, 36));
    assert!(!d3dx10_check_version(30, 36));
    assert!(!d3dx10_check_version(36, 36));
    assert!(!d3dx10_check_version(0, 0));
}

#[test]
fn export_kind_reports_forwards_and_local_names() {
    assert_eq!(export_kind("D3DX10CheckVersion"), Some(ExportKind::Stdcall));
    assert_eq!(
        export_kind("D3DX10CompileFromMemory"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(
        export_kind("D3DX10CreateDevice"),
        Some(ExportKind::Forward("d3dx10_43"))
    );
    assert_eq!(
        export_kind("D3DX10DisassembleEffect"),
        Some(ExportKind::Forward("d3dx10_39"))
    );
    assert_eq!(
        export_kind("D3DX10GetDriverLevel"),
        Some(ExportKind::Forward("d3dx10_37"))
    );
}

#[test]
fn export_kind_misses_unknown_names() {
    assert_eq!(export_kind("D3DX10Missing"), None);
    assert_eq!(export_kind("D3DX10CheckVersio"), None);
    assert_eq!(export_kind("d3dx10checkversion"), None);
    assert_eq!(export_kind(""), None);
}
