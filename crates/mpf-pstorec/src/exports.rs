/// How `dlls/pstorec/pstorec.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall -private`: winebuild leaves it out of the import libraries
    /// (`tools/winebuild/import.c`).
    Private,
    /// `@ stdcall`: a real function, ported as [`crate::pstore_create_instance`].
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `pstorec.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 7] = [
    ("DllCanUnloadNow", ExportKind::Private),
    ("DllGetClassObject", ExportKind::Private),
    ("DllRegisterServer", ExportKind::Private),
    ("DllUnregisterServer", ExportKind::Private),
    ("GetPStoreProvider", ExportKind::Stub),
    ("PStoreCreateInstance", ExportKind::Stdcall),
    ("PStoreEnumProviders", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
