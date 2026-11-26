/// How `dlls/wmp/wmp.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// An explicit C function, ported: [`crate::dll_get_class_object`].
    Explicit,
    /// `@ stub` with no name: raises `EXCEPTION_WINE_STUB` through
    /// [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `-private` with no body in `dlls/wmp`, so it is not modelled.
    Generated,
}

/// The `dlls/wmp/wmp.spec` export table, in spec order. Unnamed exports use their decimal
/// ordinal as the name.
pub const EXPORTS: [(&str, ExportKind); 8] = [
    ("3000", ExportKind::Stub),
    ("3002", ExportKind::Stub),
    ("3003", ExportKind::Stub),
    ("3004", ExportKind::Stub),
    ("DllCanUnloadNow", ExportKind::Generated),
    ("DllGetClassObject", ExportKind::Explicit),
    ("DllRegisterServer", ExportKind::Generated),
    ("DllUnregisterServer", ExportKind::Generated),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
