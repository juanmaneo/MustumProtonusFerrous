/// How `dlls/msports/msports.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function in `msports.c`, ported as [`crate::com_db_open`] or
    /// [`crate::com_db_close`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `msports.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 11] = [
    ("ComDBClaimNextFreePort", ExportKind::Stub),
    ("ComDBClaimPort", ExportKind::Stub),
    ("ComDBClose", ExportKind::Stdcall),
    ("ComDBGetCurrentPortUsage", ExportKind::Stub),
    ("ComDBOpen", ExportKind::Stdcall),
    ("ComDBReleasePort", ExportKind::Stub),
    ("ComDBResizeDatabase", ExportKind::Stub),
    ("ParallelPortPropPageProvider", ExportKind::Stub),
    ("PortsClassInstaller", ExportKind::Stub),
    ("SerialDisplayAdvancedSettings", ExportKind::Stub),
    ("SerialPortPropPageProvider", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
