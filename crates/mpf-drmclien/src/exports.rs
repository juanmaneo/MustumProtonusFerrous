/// How `dlls/drmclien/drmclien.spec` declares a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through a helper in the `stub` module.
    Stub,
    /// `@ stdcall -private`: the reference has no C body for it, so nothing is modelled.
    Private,
}

/// The `drmclien.spec` entry table, in spec order, including the `-private` entries.
pub const EXPORTS: [(&str, ExportKind); 31] = [
    ("DllMain", ExportKind::Private),
    ("CanDecrypt", ExportKind::Stub),
    ("Decrypt", ExportKind::Stub),
    ("RequestLicense", ExportKind::Stub),
    ("ProcessResponse", ExportKind::Stub),
    ("Bind", ExportKind::Stub),
    ("KeyExchange", ExportKind::Stub),
    ("GetVersion", ExportKind::Stub),
    ("CanDecryptEx", ExportKind::Stub),
    ("BindEx", ExportKind::Stub),
    ("??0CDRMLiteCrypto@@QAE@XZ", ExportKind::Stub),
    ("??1CDRMLiteCrypto@@QAE@XZ", ExportKind::Stub),
    ("SetAppSec", ExportKind::Stub),
    ("SetLicenseStore", ExportKind::Stub),
    ("GetPMLicenseFileName", ExportKind::Stub),
    ("GetPMLicenseSize", ExportKind::Stub),
    ("CreatePMLicense", ExportKind::Stub),
    ("GenerateNewLicense", ExportKind::Stub),
    ("Encrypt", ExportKind::Stub),
    ("InitAppCerts", ExportKind::Stub),
    ("QueryXferToPM", ExportKind::Stub),
    (
        "?BackupLicenses@CDRMLiteCrypto@@QAEJKPAGPAUIUnknown@@PAHPAX@Z",
        ExportKind::Stub,
    ),
    (
        "?EncryptFast@CDRMLiteCrypto@@QAEJPBDKPAE1@Z",
        ExportKind::Stub,
    ),
    (
        "?EncryptIndirectFast@CDRMLiteCrypto@@QAEJPBDKPAE1@Z",
        ExportKind::Stub,
    ),
    (
        "?GenerateNewLicenseEx@CDRMLiteCrypto@@QAEJKPAE00PAPAD10@Z",
        ExportKind::Stub,
    ),
    (
        "?GetLicenses@CDRMLiteCrypto@@QAEJPBDPAUPMLICENSE@@PAKKPAX2PAE@Z",
        ExportKind::Stub,
    ),
    (
        "?GetPublicKey@CDRMLiteCrypto@@QAEJPAUPKCERT@@@Z",
        ExportKind::Stub,
    ),
    (
        "?QueryXferToPMEx@CDRMLiteCrypto@@QAEJPBDKPAKPAEK2K12@Z",
        ExportKind::Stub,
    ),
    (
        "?RestoreLicenses@CDRMLiteCrypto@@QAEJKPAEPAGPAUIUnknown@@PAHPAX@Z",
        ExportKind::Stub,
    ),
    ("DllRegisterServer", ExportKind::Private),
    ("DllUnregisterServer", ExportKind::Private),
];

/// The kind of `name`, or `None` when the spec does not declare it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
