/// How `dlls/wpc/wpc.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `DllGetClassObject`, ported as [`crate::dll_get_class_object`].
    Ported,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::apply_parental_controls_w`].
    Stub,
    /// No C body in `dlls/wpc`. The `DllCanUnloadNow`, `DllRegisterServer` and
    /// `DllUnregisterServer` entry points come from the midl proxy code (`include/rpcproxy.h`
    /// `DLL*_ENTRY` macros), so they are not modelled.
    Generated,
}

/// The `wpc.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 5] = [
    ("ApplyParentalControlsW", ExportKind::Stub),
    ("DllCanUnloadNow", ExportKind::Generated),
    ("DllGetClassObject", ExportKind::Ported),
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
