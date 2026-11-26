//! Deterministic portions of `dlls/vssapi/main.c`.

/// Minimal state represented by Wine's private `struct CVssWriter`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VssWriter {
    pub vtable: Option<usize>,
}

/// `S_OK` from the Windows headers.
pub const S_OK: u32 = 0;
/// `E_NOTIMPL` from the Windows headers.
pub const E_NOTIMPL: u32 = 0x8000_4001;

/// `VSSAPI_CVssWriter_default_ctor`: Wine initializes the writer's vtable to NULL.
pub fn cvss_writer_default_ctor(writer: &mut VssWriter) -> &mut VssWriter {
    writer.vtable = None;
    writer
}

/// `VSSAPI_CVssWriter_dtor`: the Wine body has no observable effect.
pub fn cvss_writer_dtor(_writer: &mut VssWriter) {}

/// `VSSAPI_CVssWriter_Initialize`: the Wine body always returns `S_OK`.
pub fn cvss_writer_initialize(_writer: &mut VssWriter) -> u32 {
    S_OK
}

/// `VSSAPI_CVssWriter_Subscribe`: the Wine body always returns `S_OK`.
pub fn cvss_writer_subscribe(_writer: &mut VssWriter, _flags: u32) -> u32 {
    S_OK
}

/// `VSSAPI_CVssWriter_Unsubscribe`: the Wine body always returns `S_OK`.
pub fn cvss_writer_unsubscribe(_writer: &mut VssWriter) -> u32 {
    S_OK
}

/// `CreateVssBackupComponentsInternal`: Wine always returns `E_NOTIMPL`.
pub fn create_vss_backup_components_internal() -> u32 {
    E_NOTIMPL
}

/// `VSSAPI_CreateVssBackupComponents` delegates directly to the internal function.
pub fn create_vss_backup_components() -> u32 {
    create_vss_backup_components_internal()
}
