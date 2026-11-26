//! Boundary and branch checks beyond the oracle inputs.

use super::*;

#[test]
fn constants_and_general_stub_builder() {
    // Not oracle-derived: values pinned to the Windows headers.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 1);
    let custom = wine_spec_unimplemented_stub("elsewhere.dll", "CustomExport");
    assert_eq!(custom.module, "elsewhere.dll");
    assert_eq!(custom.function, "CustomExport");
    assert_eq!(custom.code, EXCEPTION_WINE_STUB);
    assert_eq!(custom.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(custom.number_parameters, 2);
}

#[test]
fn export_table_includes_spec_exports_and_case_sensitive_lookup() {
    // Not oracle-derived: table size/order and lookup behavior are checked against vssapi.spec.
    assert_eq!(EXPORTS.len(), 216);
    assert_eq!(STUB_EXPORTS.len(), 197);
    assert_eq!(export_kind("IsVolumeSnapshotted"), Some(ExportKind::Stub));
    assert_eq!(
        export_kind("?CreateVssBackupComponents@@YGJPAPAVIVssBackupComponents@@@Z"),
        Some(ExportKind::Cdecl)
    );
    assert_eq!(export_kind("isvolumesnapshotted"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn default_writer_state_is_null() {
    // Not oracle-derived: Default is a safe initial Rust representation.
    assert_eq!(VssWriter::default(), VssWriter { vtable: None });
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
}
