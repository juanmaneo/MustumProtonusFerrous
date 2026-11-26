//! Export declarations from `dlls/msctfmonitor/msctfmonitor.spec`.

/// How `msctfmonitor.spec` declares an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall` with a body in `dlls/msctfmonitor/main.c`.
    Implemented,
    /// `@ stdcall -private` with the default body from `libs/winecrt0`.
    WinecrtDefault,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through `wine_spec_unimplemented_stub`.
    Stub,
}

/// The `msctfmonitor.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 7] = [
    ("DllCanUnloadNow", ExportKind::WinecrtDefault),
    ("DllGetClassObject", ExportKind::Stub),
    ("DllRegisterServer", ExportKind::WinecrtDefault),
    ("DllUnregisterServer", ExportKind::WinecrtDefault),
    ("DoMsCtfMonitor", ExportKind::Stub),
    ("InitLocalMsCtfMonitor", ExportKind::Implemented),
    ("UnInitLocalMsCtfMonitor", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
