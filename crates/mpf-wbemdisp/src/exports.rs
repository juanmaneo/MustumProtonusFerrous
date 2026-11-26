/// How `dlls/wbemdisp/wbemdisp.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::stub::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `wbemdisp.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 4] = [
    ("DllCanUnloadNow", ExportKind::Stub),
    ("DllGetClassObject", ExportKind::Stub),
    ("DllRegisterServer", ExportKind::Stub),
    ("DllUnregisterServer", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
