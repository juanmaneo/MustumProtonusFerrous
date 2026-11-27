/// How `dlls/itircl/itircl.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall -private`: a function in Wine's C code. This reference has no C body for it,
    /// so only the table entry is ported.
    PrivateStdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `itircl.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 4] = [
    ("DllCanUnloadNow", ExportKind::PrivateStdcall),
    ("DllGetClassObject", ExportKind::Stub),
    ("DllRegisterServer", ExportKind::PrivateStdcall),
    ("DllUnregisterServer", ExportKind::PrivateStdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
