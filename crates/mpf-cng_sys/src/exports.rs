//! Export table of `dlls/cng.sys/cng.sys.spec`, in spec order.

/// What a `cng.sys.spec` line says its export does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall ... bcrypt.<name>`: the export is forwarded to that DLL, so `cng.sys` has no code for it.
    Forward(&'static str),
    /// `@ stub`: the export calls `__wine_spec_unimplemented_stub`. See [`crate::unimplemented_stub`].
    Stub,
}

/// One line of `cng.sys.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub kind: ExportKind,
}

/// Every entry of `cng.sys.spec`, in spec order. Ordinals are not listed because the spec does not state them.
pub const EXPORTS: &[Export] = &[
    Export {
        name: "BCryptAddContextFunctionProvider",
        kind: ExportKind::Forward("bcrypt.BCryptAddContextFunctionProvider"),
    },
    Export {
        name: "BCryptCloseAlgorithmProvider",
        kind: ExportKind::Forward("bcrypt.BCryptCloseAlgorithmProvider"),
    },
    Export {
        name: "BCryptCreateHash",
        kind: ExportKind::Forward("bcrypt.BCryptCreateHash"),
    },
    Export {
        name: "BCryptCreateMultiHash",
        kind: ExportKind::Stub,
    },
    Export {
        name: "BCryptDecrypt",
        kind: ExportKind::Forward("bcrypt.BCryptDecrypt"),
    },
    Export {
        name: "BCryptDeriveKey",
        kind: ExportKind::Forward("bcrypt.BCryptDeriveKey"),
    },
    Export {
        name: "BCryptDeriveKeyCapi",
        kind: ExportKind::Forward("bcrypt.BCryptDeriveKeyCapi"),
    },
    Export {
        name: "BCryptDeriveKeyPBKDF2",
        kind: ExportKind::Forward("bcrypt.BCryptDeriveKeyPBKDF2"),
    },
    Export {
        name: "BCryptDestroyHash",
        kind: ExportKind::Forward("bcrypt.BCryptDestroyHash"),
    },
    Export {
        name: "BCryptDestroyKey",
        kind: ExportKind::Forward("bcrypt.BCryptDestroyKey"),
    },
    Export {
        name: "BCryptDestroySecret",
        kind: ExportKind::Forward("bcrypt.BCryptDestroySecret"),
    },
    Export {
        name: "BCryptDuplicateHash",
        kind: ExportKind::Forward("bcrypt.BCryptDuplicateHash"),
    },
    Export {
        name: "BCryptDuplicateKey",
        kind: ExportKind::Forward("bcrypt.BCryptDuplicateKey"),
    },
    Export {
        name: "BCryptEncrypt",
        kind: ExportKind::Forward("bcrypt.BCryptEncrypt"),
    },
    Export {
        name: "BCryptEnumAlgorithms",
        kind: ExportKind::Forward("bcrypt.BCryptEnumAlgorithms"),
    },
    Export {
        name: "BCryptEnumProviders",
        kind: ExportKind::Stub,
    },
    Export {
        name: "BCryptExportKey",
        kind: ExportKind::Forward("bcrypt.BCryptExportKey"),
    },
    Export {
        name: "BCryptFinalizeKeyPair",
        kind: ExportKind::Forward("bcrypt.BCryptFinalizeKeyPair"),
    },
    Export {
        name: "BCryptFinishHash",
        kind: ExportKind::Forward("bcrypt.BCryptFinishHash"),
    },
    Export {
        name: "BCryptFreeBuffer",
        kind: ExportKind::Forward("bcrypt.BCryptFreeBuffer"),
    },
    Export {
        name: "BCryptGenRandom",
        kind: ExportKind::Forward("bcrypt.BCryptGenRandom"),
    },
    Export {
        name: "BCryptGenerateKeyPair",
        kind: ExportKind::Forward("bcrypt.BCryptGenerateKeyPair"),
    },
    Export {
        name: "BCryptGenerateSymmetricKey",
        kind: ExportKind::Forward("bcrypt.BCryptGenerateSymmetricKey"),
    },
    Export {
        name: "BCryptGetFipsAlgorithmMode",
        kind: ExportKind::Forward("bcrypt.BCryptGetFipsAlgorithmMode"),
    },
    Export {
        name: "BCryptGetProperty",
        kind: ExportKind::Forward("bcrypt.BCryptGetProperty"),
    },
    Export {
        name: "BCryptHash",
        kind: ExportKind::Forward("bcrypt.BCryptHash"),
    },
    Export {
        name: "BCryptHashData",
        kind: ExportKind::Forward("bcrypt.BCryptHashData"),
    },
    Export {
        name: "BCryptImportKey",
        kind: ExportKind::Forward("bcrypt.BCryptImportKey"),
    },
    Export {
        name: "BCryptImportKeyPair",
        kind: ExportKind::Forward("bcrypt.BCryptImportKeyPair"),
    },
    Export {
        name: "BCryptKeyDerivation",
        kind: ExportKind::Forward("bcrypt.BCryptKeyDerivation"),
    },
    Export {
        name: "BCryptOpenAlgorithmProvider",
        kind: ExportKind::Forward("bcrypt.BCryptOpenAlgorithmProvider"),
    },
    Export {
        name: "BCryptProcessMultiOperations",
        kind: ExportKind::Stub,
    },
    Export {
        name: "BCryptRegisterConfigChangeNotify",
        kind: ExportKind::Stub,
    },
    Export {
        name: "BCryptRegisterProvider",
        kind: ExportKind::Forward("bcrypt.BCryptRegisterProvider"),
    },
    Export {
        name: "BCryptResolveProviders",
        kind: ExportKind::Stub,
    },
    Export {
        name: "BCryptSecretAgreement",
        kind: ExportKind::Forward("bcrypt.BCryptSecretAgreement"),
    },
    Export {
        name: "BCryptSetProperty",
        kind: ExportKind::Forward("bcrypt.BCryptSetProperty"),
    },
    Export {
        name: "BCryptSignHash",
        kind: ExportKind::Forward("bcrypt.BCryptSignHash"),
    },
    Export {
        name: "BCryptUnregisterConfigChangeNotify",
        kind: ExportKind::Stub,
    },
    Export {
        name: "BCryptUnregisterProvider",
        kind: ExportKind::Forward("bcrypt.BCryptUnregisterProvider"),
    },
    Export {
        name: "BCryptVerifySignature",
        kind: ExportKind::Forward("bcrypt.BCryptVerifySignature"),
    },
    Export {
        name: "CngGetFipsAlgorithmMode",
        kind: ExportKind::Stub,
    },
    Export {
        name: "EntropyPoolTriggerReseedForIum",
        kind: ExportKind::Stub,
    },
    Export {
        name: "EntropyProvideData",
        kind: ExportKind::Stub,
    },
    Export {
        name: "EntropyRegisterCallback",
        kind: ExportKind::Stub,
    },
    Export {
        name: "EntropyRegisterSource",
        kind: ExportKind::Stub,
    },
    Export {
        name: "EntropyUnregisterSource",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslDecrementProviderReferenceCount",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslDecryptPacket",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslEncryptPacket",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslExportKey",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslExportKeyingMaterial",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslFreeObject",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslImportKey",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslIncrementProviderReferenceCount",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslLookupCipherLengths",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslLookupCipherSuiteInfo",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SslOpenProvider",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SymCrypt802_11SaeCustomCommitCreate",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SymCrypt802_11SaeCustomCommitProcess",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SymCrypt802_11SaeCustomDestroy",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SymCrypt802_11SaeCustomInit",
        kind: ExportKind::Stub,
    },
    Export {
        name: "SystemPrng",
        kind: ExportKind::Stub,
    },
];

/// The entry named `name`, or `None` if `cng.sys` does not export it. Matching is case-sensitive.
pub fn find_export(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}
