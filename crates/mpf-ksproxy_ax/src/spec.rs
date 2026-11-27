/// How `dlls/ksproxy.ax/ksproxy.ax.spec` declares an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: calls `__wine_spec_unimplemented_stub`, see the helpers in `stub.rs`.
    Stub,
    /// `@ stdcall -private` with no body in `dlls/ksproxy.ax`.
    PrivateWithoutBody,
}

/// The `ksproxy.ax.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 10] = [
    ("DllCanUnloadNow", ExportKind::PrivateWithoutBody),
    ("DllGetClassObject", ExportKind::Stub),
    ("DllRegisterServer", ExportKind::PrivateWithoutBody),
    ("DllUnregisterServer", ExportKind::PrivateWithoutBody),
    ("KsGetMediaType", ExportKind::Stub),
    ("KsGetMediaTypeCount", ExportKind::Stub),
    ("KsGetMultiplePinFactoryItems", ExportKind::Stub),
    ("KsOpenDefaultDevice", ExportKind::Stub),
    ("KsResolveRequiredAttributes", ExportKind::Stub),
    ("KsSynchronousDeviceControl", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
