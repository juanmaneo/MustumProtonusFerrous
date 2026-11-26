/// How `dlls/msimtf/msimtf.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall -private`. Only `DllGetClassObject` and `DllCanUnloadNow` are ported, see the crate docs.
    Private,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `msimtf.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 6] = [
    ("DllCanUnloadNow", ExportKind::Private),
    ("DllGetClassObject", ExportKind::Private),
    ("DllRegisterServer", ExportKind::Private),
    ("DllUnregisterServer", ExportKind::Private),
    ("MsimtfIsGuidMapEnable", ExportKind::Stub),
    ("MsimtfIsWindowFiltered", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
