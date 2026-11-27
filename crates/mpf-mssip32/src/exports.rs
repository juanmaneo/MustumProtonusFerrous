/// How `dlls/mssip32/mssip32.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall -private`: `-private` keeps the name out of the import and export tables
    /// (`FLAG_PRIVATE` in `tools/winebuild`). No source in `dlls/mssip32` defines it.
    StdcallPrivate,
}

/// The `mssip32.spec` table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 9] = [
    ("CryptSIPGetInfo", ExportKind::Stub),
    ("CryptSIPGetRegWorkingFlags", ExportKind::Stub),
    ("CryptSIPCreateIndirectData", ExportKind::Stub),
    ("CryptSIPGetSignedDataMsg", ExportKind::Stub),
    ("CryptSIPPutSignedDataMsg", ExportKind::Stub),
    ("CryptSIPRemoveSignedDataMsg", ExportKind::Stub),
    ("CryptSIPVerifyIndirectData", ExportKind::Stub),
    ("DllRegisterServer", ExportKind::StdcallPrivate),
    ("DllUnregisterServer", ExportKind::StdcallPrivate),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
