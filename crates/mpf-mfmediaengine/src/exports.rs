/// How `dlls/mfmediaengine/mfmediaengine.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall -private` with a definition in `dlls/mfmediaengine/main.c`, ported as
    /// [`crate::dll_get_class_object`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall -private` with no definition anywhere under `dlls/mfmediaengine`, so there is
    /// no C body to port and no return value to model.
    Undefined,
}

/// The `mfmediaengine.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 3] = [
    ("DllCanUnloadNow", ExportKind::Undefined),
    ("DllGetActivationFactory", ExportKind::Stub),
    ("DllGetClassObject", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
