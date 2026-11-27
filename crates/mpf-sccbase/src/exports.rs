/// How `dlls/sccbase/sccbase.spec` declares a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `stdcall -private`: kept out of the export table, and not a `@ stub`. The reference has
    /// no body for it.
    StdcallPrivate,
}

/// The `sccbase.spec` declarations, in spec order, including the two `-private` entries.
pub const EXPORTS: [(&str, ExportKind); 26] = [
    ("CPAcquireContext", ExportKind::Stub),
    ("CPAcquireContextW", ExportKind::Stub),
    ("CPCreateHash", ExportKind::Stub),
    ("CPDecrypt", ExportKind::Stub),
    ("CPDeriveKey", ExportKind::Stub),
    ("CPDestroyHash", ExportKind::Stub),
    ("CPDestroyKey", ExportKind::Stub),
    ("CPEncrypt", ExportKind::Stub),
    ("CPExportKey", ExportKind::Stub),
    ("CPGenKey", ExportKind::Stub),
    ("CPGenRandom", ExportKind::Stub),
    ("CPGetHashParam", ExportKind::Stub),
    ("CPGetKeyParam", ExportKind::Stub),
    ("CPGetProvParam", ExportKind::Stub),
    ("CPGetUserKey", ExportKind::Stub),
    ("CPHashData", ExportKind::Stub),
    ("CPHashSessionKey", ExportKind::Stub),
    ("CPImportKey", ExportKind::Stub),
    ("CPReleaseContext", ExportKind::Stub),
    ("CPSetHashParam", ExportKind::Stub),
    ("CPSetKeyParam", ExportKind::Stub),
    ("CPSetProvParam", ExportKind::Stub),
    ("CPSignHash", ExportKind::Stub),
    ("CPVerifySignature", ExportKind::Stub),
    ("DllRegisterServer", ExportKind::StdcallPrivate),
    ("DllUnregisterServer", ExportKind::StdcallPrivate),
];

/// How `sccbase.spec` declares `name`, or `None` when it declares no such name.
/// Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
