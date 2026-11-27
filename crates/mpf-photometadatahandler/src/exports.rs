/// How `dlls/photometadatahandler/photometadatahandler.spec` declares a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: exported, and raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `stdcall -private`: not exported, and with no implementation in this directory.
    StdcallPrivate,
}

impl ExportKind {
    /// Whether the export table lists names of this kind. Winebuild skips `-private` functions
    /// (`FLAG_PRIVATE` in `tools/winebuild/build.h`), so they cannot be imported.
    pub fn in_export_table(self) -> bool {
        matches!(self, ExportKind::Stub)
    }
}

/// The `photometadatahandler.spec` entries, in spec order.
pub const EXPORTS: [(&str, ExportKind); 4] = [
    ("DllCanUnloadNow", ExportKind::StdcallPrivate),
    ("DllGetClassObject", ExportKind::Stub),
    ("DllRegisterServer", ExportKind::StdcallPrivate),
    ("DllUnregisterServer", ExportKind::StdcallPrivate),
];

/// The kind of `name`, or `None` when the spec does not declare it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}

/// Whether the DLL's export table lists `name`. Names are case-sensitive.
pub fn is_exported(name: &str) -> bool {
    export_kind(name).is_some_and(ExportKind::in_export_table)
}
