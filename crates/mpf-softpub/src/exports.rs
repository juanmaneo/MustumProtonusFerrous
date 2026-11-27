/// How `dlls/softpub/softpub.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `stdcall module.function`: forwarded to `function` in `module`.
    Forward {
        module: &'static str,
        function: &'static str,
    },
}

/// One line of `dlls/softpub/softpub.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub ordinal: u16,
    pub name: &'static str,
    /// `PRIVATE` in the `winebuild --def` output. The spec sets it with `-private`, and winebuild
    /// sets it for every `stub`.
    pub private: bool,
    pub kind: ExportKind,
}

/// The `softpub.spec` export table, in spec order. Twenty-one entries forward to `wintrust`.
pub const EXPORTS: [Export; 24] = [
    Export {
        ordinal: 1,
        name: "GenericChainCertificateTrust",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "GenericChainCertificateTrust",
        },
    },
    Export {
        ordinal: 2,
        name: "GenericChainFinalProv",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "GenericChainFinalProv",
        },
    },
    Export {
        ordinal: 3,
        name: "HTTPSCertificateTrust",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "HTTPSCertificateTrust",
        },
    },
    Export {
        ordinal: 4,
        name: "SoftpubDefCertInit",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubDefCertInit",
        },
    },
    Export {
        ordinal: 5,
        name: "SoftpubFreeDefUsageCallData",
        private: true,
        kind: ExportKind::Stub,
    },
    Export {
        ordinal: 6,
        name: "SoftpubLoadDefUsageCallData",
        private: true,
        kind: ExportKind::Stub,
    },
    Export {
        ordinal: 7,
        name: "AddPersonalTrustDBPages",
        private: true,
        kind: ExportKind::Stub,
    },
    Export {
        ordinal: 8,
        name: "DllRegisterServer",
        private: true,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubDllRegisterServer",
        },
    },
    Export {
        ordinal: 9,
        name: "DllUnregisterServer",
        private: true,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubDllUnregisterServer",
        },
    },
    Export {
        ordinal: 10,
        name: "DriverCleanupPolicy",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "DriverCleanupPolicy",
        },
    },
    Export {
        ordinal: 11,
        name: "DriverFinalPolicy",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "DriverFinalPolicy",
        },
    },
    Export {
        ordinal: 12,
        name: "DriverInitializePolicy",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "DriverInitializePolicy",
        },
    },
    Export {
        ordinal: 13,
        name: "FindCertsByIssuer",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "FindCertsByIssuer",
        },
    },
    Export {
        ordinal: 14,
        name: "HTTPSFinalProv",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "HTTPSFinalProv",
        },
    },
    Export {
        ordinal: 15,
        name: "OfficeCleanupPolicy",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "OfficeCleanupPolicy",
        },
    },
    Export {
        ordinal: 16,
        name: "OfficeInitializePolicy",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "OfficeInitializePolicy",
        },
    },
    Export {
        ordinal: 17,
        name: "OpenPersonalTrustDBDialog",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "OpenPersonalTrustDBDialog",
        },
    },
    Export {
        ordinal: 18,
        name: "SoftpubAuthenticode",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubAuthenticode",
        },
    },
    Export {
        ordinal: 19,
        name: "SoftpubCheckCert",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubCheckCert",
        },
    },
    Export {
        ordinal: 20,
        name: "SoftpubCleanup",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubCleanup",
        },
    },
    Export {
        ordinal: 21,
        name: "SoftpubDumpStructure",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubDumpStructure",
        },
    },
    Export {
        ordinal: 22,
        name: "SoftpubInitialize",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubInitialize",
        },
    },
    Export {
        ordinal: 23,
        name: "SoftpubLoadMessage",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubLoadMessage",
        },
    },
    Export {
        ordinal: 24,
        name: "SoftpubLoadSignature",
        private: false,
        kind: ExportKind::Forward {
            module: "wintrust",
            function: "SoftpubLoadSignature",
        },
    },
];

/// The export named `name`, or `None` when `softpub.spec` does not export it. Names are
/// case-sensitive.
pub fn find_export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}

/// The `(module, function)` that a call to `name` is forwarded to, or `None` when `name` is not
/// exported or is a `stub`.
pub fn forward_target(name: &str) -> Option<(&'static str, &'static str)> {
    match find_export(name)?.kind {
        ExportKind::Forward { module, function } => Some((module, function)),
        ExportKind::Stub => None,
    }
}
