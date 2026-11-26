/// How `dlls/olesvr32/olesvr32.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stdcall`: a real function in `olesvr_main.c`, ported as the `ole_*` function of the same name.
    Stdcall,
    /// `stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `olesvr32.spec` export table, in spec order (ordinals 1 to 12).
pub const EXPORTS: [(&str, ExportKind); 12] = [
    ("WEP", ExportKind::Stub),
    ("OleRegisterServer", ExportKind::Stdcall),
    ("OleRevokeServer", ExportKind::Stdcall),
    ("OleBlockServer", ExportKind::Stdcall),
    ("OleUnblockServer", ExportKind::Stdcall),
    ("OleRegisterServerDoc", ExportKind::Stdcall),
    ("OleRevokeServerDoc", ExportKind::Stdcall),
    ("OleRenameServerDoc", ExportKind::Stdcall),
    ("OleRevertServerDoc", ExportKind::Stdcall),
    ("OleSavedServerDoc", ExportKind::Stdcall),
    ("OleRevokeObject", ExportKind::Stub),
    ("OleQueryServerVersion", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
