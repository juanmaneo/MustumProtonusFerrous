/// How `dlls/slbcsp/slbcsp.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall -private`: a function the spec names but the reference never defines, so no
    /// record or return value is modelled for it.
    StdcallPrivate,
}

/// The `slbcsp.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 27] = [
    ("CPAcquireContext", ExportKind::Stub),
    ("CPSetHashParam", ExportKind::Stub),
    ("CPSetKeyParam", ExportKind::Stub),
    ("CPSetProvParam", ExportKind::Stub),
    ("CPCreateHash", ExportKind::Stub),
    ("CPDecrypt", ExportKind::Stub),
    ("CPDeriveKey", ExportKind::Stub),
    ("CPDestroyHash", ExportKind::Stub),
    ("CPDestroyKey", ExportKind::Stub),
    ("CPDuplicateHash", ExportKind::Stub),
    ("CPDuplicateKey", ExportKind::Stub),
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
    ("CPSignHash", ExportKind::Stub),
    ("CPVerifySignature", ExportKind::Stub),
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
