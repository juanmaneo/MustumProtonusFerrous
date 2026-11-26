/// How `dlls/tiptsf/tiptsf.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall -private`. `DllGetClassObject` is ported as [`crate::dll_get_class_object`];
    /// the other three have no body in `dlls/tiptsf` (see the crate docs).
    Stdcall,
}

/// The `tiptsf.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 11] = [
    ("AdviseHook", ExportKind::Stub),
    ("EndCaretTracking", ExportKind::Stub),
    ("ProcessCaretEvents", ExportKind::Stub),
    ("ProcessCiceroCaretEvent", ExportKind::Stub),
    ("RestrictedModeMsgWndProc", ExportKind::Stub),
    ("StartCaretTracking", ExportKind::Stub),
    ("UnadviseHook", ExportKind::Stub),
    ("DllCanUnloadNow", ExportKind::Stdcall),
    ("DllGetClassObject", ExportKind::Stdcall),
    ("DllRegisterServer", ExportKind::Stdcall),
    ("DllUnregisterServer", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
