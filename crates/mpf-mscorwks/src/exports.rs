/// How `dlls/mscorwks/mscorwks.spec` defines an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ stdcall ... <dll>.<name>`: forwards to the export named by the target, such as
    /// `mscoree.LoadStringRC`.
    Forward(&'static str),
}

/// One entry of the `dlls/mscorwks/mscorwks.spec` export table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The exported name, from the spec line.
    pub name: &'static str,
    /// The ordinal `winebuild` assigns. The spec gives no explicit ordinals, so each entry takes
    /// its 1-based line position (`assign_ordinals` in `tools/winebuild/parser.c`).
    pub ordinal: u16,
    pub kind: ExportKind,
}

/// The `mscorwks.spec` export table, in spec order.
pub const EXPORTS: [Export; 110] = [
    Export {
        name: "LoadStringRC",
        ordinal: 1,
        kind: ExportKind::Forward("mscoree.LoadStringRC"),
    },
    Export {
        name: "ReOpenMetaDataWithMemory",
        ordinal: 2,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CorDllMainForThunk",
        ordinal: 3,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateActContextInternal",
        ordinal: 4,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateCMSFromXmlInternal",
        ordinal: 5,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetAddrOfContractShutoffFlag",
        ordinal: 6,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetCLRFunction",
        ordinal: 7,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetUserStateManagerInternal",
        ordinal: 8,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetUserStoreInternal",
        ordinal: 9,
        kind: ExportKind::Stub,
    },
    Export {
        name: "ParseManifestInternal",
        ordinal: 10,
        kind: ExportKind::Stub,
    },
    Export {
        name: "_AxlGetIssuerPublicKeyHash",
        ordinal: 11,
        kind: ExportKind::Stub,
    },
    Export {
        name: "_AxlPublicKeyBlobToPublicKeyToken",
        ordinal: 12,
        kind: ExportKind::Stub,
    },
    Export {
        name: "_AxlRSAKeyValueToPublicKeyToken",
        ordinal: 13,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CertCreateAuthenticodeLicense",
        ordinal: 14,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CertFreeAuthenticodeSignerInfo",
        ordinal: 15,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CertFreeAuthenticodeTimestamperInfo",
        ordinal: 16,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CertTimestampAuthenticodeLicense",
        ordinal: 17,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CertVerifyAuthenticodeLicense",
        ordinal: 18,
        kind: ExportKind::Stub,
    },
    Export {
        name: "ClearDownloadCache",
        ordinal: 19,
        kind: ExportKind::Stub,
    },
    Export {
        name: "ClrCreateManagedInstance",
        ordinal: 20,
        kind: ExportKind::Forward("mscoree.ClrCreateManagedInstance"),
    },
    Export {
        name: "CoEEShutDownCOM",
        ordinal: 21,
        kind: ExportKind::Forward("mscoree.CoEEShutDownCOM"),
    },
    Export {
        name: "CoInitializeCor",
        ordinal: 22,
        kind: ExportKind::Forward("mscoree.CoInitializeCor"),
    },
    Export {
        name: "CoInitializeEE",
        ordinal: 23,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CoUninitializeCor",
        ordinal: 24,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CoUninitializeEE",
        ordinal: 25,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CompareAssemblyIdentity",
        ordinal: 26,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CopyPDBs",
        ordinal: 27,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CorExitProcess",
        ordinal: 28,
        kind: ExportKind::Forward("mscoree.CorExitProcess"),
    },
    Export {
        name: "CorLaunchApplication",
        ordinal: 29,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CorMarkThreadInThreadPool",
        ordinal: 30,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateActContext",
        ordinal: 31,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateApplicationContext",
        ordinal: 32,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateAssemblyCache",
        ordinal: 33,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateAssemblyEnum",
        ordinal: 34,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateAssemblyNameObject",
        ordinal: 35,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateCMSFromXml",
        ordinal: 36,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateHistoryReader",
        ordinal: 37,
        kind: ExportKind::Stub,
    },
    Export {
        name: "CreateInstallReferenceEnum",
        ordinal: 38,
        kind: ExportKind::Stub,
    },
    Export {
        name: "DeleteShadowCache",
        ordinal: 39,
        kind: ExportKind::Stub,
    },
    Export {
        name: "DllCanUnloadNowInternal",
        ordinal: 40,
        kind: ExportKind::Stub,
    },
    Export {
        name: "DllGetClassObjectInternal",
        ordinal: 41,
        kind: ExportKind::Stub,
    },
    Export {
        name: "DllRegisterServerInternal",
        ordinal: 42,
        kind: ExportKind::Stub,
    },
    Export {
        name: "DllUnregisterServerInternal",
        ordinal: 43,
        kind: ExportKind::Stub,
    },
    Export {
        name: "EEDllRegisterServer",
        ordinal: 44,
        kind: ExportKind::Stub,
    },
    Export {
        name: "EEDllUnregisterServer",
        ordinal: 45,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetAppIdAuthority",
        ordinal: 46,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetAssemblyIdentityFromFile",
        ordinal: 47,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetAssemblyMDImport",
        ordinal: 48,
        kind: ExportKind::Forward("mscoree.GetAssemblyMDImport"),
    },
    Export {
        name: "GetCLRIdentityManager",
        ordinal: 49,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetCachePath",
        ordinal: 50,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetHashFromAssemblyFile",
        ordinal: 51,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetHashFromAssemblyFileW",
        ordinal: 52,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetHashFromBlob",
        ordinal: 53,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetHashFromFile",
        ordinal: 54,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetHashFromFileW",
        ordinal: 55,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetHashFromHandle",
        ordinal: 56,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetHistoryFileDirectory",
        ordinal: 57,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetIdentityAuthority",
        ordinal: 58,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetMetaDataInternalInterface",
        ordinal: 59,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetMetaDataInternalInterfaceFromPublic",
        ordinal: 60,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetMetaDataPublicInterfaceFromInternal",
        ordinal: 61,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetPermissionRequests",
        ordinal: 62,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetPrivateContextsPerfCounters",
        ordinal: 63,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetUserStateManager",
        ordinal: 64,
        kind: ExportKind::Stub,
    },
    Export {
        name: "GetUserStore",
        ordinal: 65,
        kind: ExportKind::Stub,
    },
    Export {
        name: "IEE",
        ordinal: 66,
        kind: ExportKind::Stub,
    },
    Export {
        name: "InitializeFusion",
        ordinal: 67,
        kind: ExportKind::Stub,
    },
    Export {
        name: "InstallCustomAssembly",
        ordinal: 68,
        kind: ExportKind::Stub,
    },
    Export {
        name: "InstallCustomModule",
        ordinal: 69,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LegacyNGenCompile",
        ordinal: 70,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LegacyNGenCreateZapper",
        ordinal: 71,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LegacyNGenFreeZapper",
        ordinal: 72,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LegacyNGenTryEnumerateFusionCache",
        ordinal: 73,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LoadStringRCEx",
        ordinal: 74,
        kind: ExportKind::Forward("mscoree.LoadStringRCEx"),
    },
    Export {
        name: "LogHelp_LogAssert",
        ordinal: 75,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LogHelp_NoGuiOnAssert",
        ordinal: 76,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LogHelp_TerminateOnAssert",
        ordinal: 77,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LookupHistoryAssembly",
        ordinal: 78,
        kind: ExportKind::Stub,
    },
    Export {
        name: "MetaDataGetDispenser",
        ordinal: 79,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NGenCreateNGenWorker",
        ordinal: 80,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NukeDownloadedCache",
        ordinal: 81,
        kind: ExportKind::Stub,
    },
    Export {
        name: "ParseManifest",
        ordinal: 82,
        kind: ExportKind::Stub,
    },
    Export {
        name: "PostErrorVA",
        ordinal: 83,
        kind: ExportKind::Stub,
    },
    Export {
        name: "PreBindAssembly",
        ordinal: 84,
        kind: ExportKind::Stub,
    },
    Export {
        name: "PreBindAssemblyEx",
        ordinal: 85,
        kind: ExportKind::Stub,
    },
    Export {
        name: "ReOpenMetaDataWithMemoryEx",
        ordinal: 86,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameCompareAssemblies",
        ordinal: 87,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameErrorInfo",
        ordinal: 88,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameFreeBuffer",
        ordinal: 89,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameGetBlob",
        ordinal: 90,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameGetBlobFromImage",
        ordinal: 91,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameGetPublicKey",
        ordinal: 92,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameHashSize",
        ordinal: 93,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameKeyDelete",
        ordinal: 94,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameKeyGen",
        ordinal: 95,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameKeyGenEx",
        ordinal: 96,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameKeyInstall",
        ordinal: 97,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameSignatureGeneration",
        ordinal: 98,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameSignatureGenerationEx",
        ordinal: 99,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameSignatureSize",
        ordinal: 100,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameSignatureVerification",
        ordinal: 101,
        kind: ExportKind::Forward("mscoree.StrongNameSignatureVerification"),
    },
    Export {
        name: "StrongNameSignatureVerificationEx",
        ordinal: 102,
        kind: ExportKind::Forward("mscoree.StrongNameSignatureVerificationEx"),
    },
    Export {
        name: "StrongNameSignatureVerificationFromImage",
        ordinal: 103,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameTokenFromAssembly",
        ordinal: 104,
        kind: ExportKind::Forward("mscoree.StrongNameTokenFromAssembly"),
    },
    Export {
        name: "StrongNameTokenFromAssemblyEx",
        ordinal: 105,
        kind: ExportKind::Stub,
    },
    Export {
        name: "StrongNameTokenFromPublicKey",
        ordinal: 106,
        kind: ExportKind::Stub,
    },
    Export {
        name: "TranslateSecurityAttributes",
        ordinal: 107,
        kind: ExportKind::Stub,
    },
    Export {
        name: "_CorDllMain",
        ordinal: 108,
        kind: ExportKind::Forward("mscoree._CorDllMain"),
    },
    Export {
        name: "_CorExeMain2",
        ordinal: 109,
        kind: ExportKind::Forward("mscoree._CorExeMain2"),
    },
    Export {
        name: "_CorExeMain",
        ordinal: 110,
        kind: ExportKind::Forward("mscoree._CorExeMain"),
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are matched exactly.
pub fn export_by_name(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}
