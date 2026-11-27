/// How `dlls/mscat32/mscat32.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// Forwarded to another module, so this DLL has no body for it. The string is the target
    /// that winebuild writes into the export table. Every target here is in `wintrust`.
    Forwarded(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::stub_for_export`].
    Stub,
}

/// One row of the `mscat32.spec` export table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    /// The ordinal winebuild assigns. The first entry sets 1 explicitly, and the rest follow in
    /// spec order.
    pub ordinal: u16,
    pub kind: ExportKind,
    /// `PRIVATE` in the `.def` that `winebuild --def` writes: every stub, and the two `-private`
    /// forwards.
    pub private: bool,
}

/// The `mscat32.spec` export table, in spec order.
pub const EXPORTS: [Export; 35] = [
    Export {
        name: "CryptCATVerifyMember",
        ordinal: 1,
        kind: ExportKind::Stub,
        private: true,
    },
    Export {
        name: "CryptCATAdminAcquireContext",
        ordinal: 2,
        kind: ExportKind::Forwarded("wintrust.CryptCATAdminAcquireContext"),
        private: false,
    },
    Export {
        name: "CryptCATAdminAddCatalog",
        ordinal: 3,
        kind: ExportKind::Forwarded("wintrust.CryptCATAdminAddCatalog"),
        private: false,
    },
    Export {
        name: "CryptCATAdminCalcHashFromFileHandle",
        ordinal: 4,
        kind: ExportKind::Forwarded("wintrust.CryptCATAdminCalcHashFromFileHandle"),
        private: false,
    },
    Export {
        name: "CryptCATAdminEnumCatalogFromHash",
        ordinal: 5,
        kind: ExportKind::Forwarded("wintrust.CryptCATAdminEnumCatalogFromHash"),
        private: false,
    },
    Export {
        name: "CryptCATAdminReleaseCatalogContext",
        ordinal: 6,
        kind: ExportKind::Forwarded("wintrust.CryptCATAdminReleaseCatalogContext"),
        private: false,
    },
    Export {
        name: "CryptCATAdminReleaseContext",
        ordinal: 7,
        kind: ExportKind::Forwarded("wintrust.CryptCATAdminReleaseContext"),
        private: false,
    },
    Export {
        name: "CryptCATCDFClose",
        ordinal: 8,
        kind: ExportKind::Forwarded("wintrust.CryptCATCDFClose"),
        private: false,
    },
    Export {
        name: "CryptCATCDFEnumAttributes",
        ordinal: 9,
        kind: ExportKind::Stub,
        private: true,
    },
    Export {
        name: "CryptCATCDFEnumAttributesWithCDFTag",
        ordinal: 10,
        kind: ExportKind::Forwarded("wintrust.CryptCATCDFEnumAttributesWithCDFTag"),
        private: false,
    },
    Export {
        name: "CryptCATCDFEnumCatAttributes",
        ordinal: 11,
        kind: ExportKind::Forwarded("wintrust.CryptCATCDFEnumCatAttributes"),
        private: false,
    },
    Export {
        name: "CryptCATCDFEnumMembers",
        ordinal: 12,
        kind: ExportKind::Stub,
        private: true,
    },
    Export {
        name: "CryptCATCDFEnumMembersByCDFTag",
        ordinal: 13,
        kind: ExportKind::Forwarded("wintrust.CryptCATCDFEnumMembersByCDFTag"),
        private: false,
    },
    Export {
        name: "CryptCATCDFEnumMembersByCDFTagEx",
        ordinal: 14,
        kind: ExportKind::Forwarded("wintrust.CryptCATCDFEnumMembersByCDFTagEx"),
        private: false,
    },
    Export {
        name: "CryptCATCDFOpen",
        ordinal: 15,
        kind: ExportKind::Forwarded("wintrust.CryptCATCDFOpen"),
        private: false,
    },
    Export {
        name: "CryptCATCatalogInfoFromContext",
        ordinal: 16,
        kind: ExportKind::Forwarded("wintrust.CryptCATCatalogInfoFromContext"),
        private: false,
    },
    Export {
        name: "CryptCATClose",
        ordinal: 17,
        kind: ExportKind::Forwarded("wintrust.CryptCATClose"),
        private: false,
    },
    Export {
        name: "CryptCATEnumerateAttr",
        ordinal: 18,
        kind: ExportKind::Forwarded("wintrust.CryptCATEnumerateAttr"),
        private: false,
    },
    Export {
        name: "CryptCATEnumerateCatAttr",
        ordinal: 19,
        kind: ExportKind::Forwarded("wintrust.CryptCATEnumerateCatAttr"),
        private: false,
    },
    Export {
        name: "CryptCATEnumerateMember",
        ordinal: 20,
        kind: ExportKind::Forwarded("wintrust.CryptCATEnumerateMember"),
        private: false,
    },
    Export {
        name: "CryptCATGetAttrInfo",
        ordinal: 21,
        kind: ExportKind::Forwarded("wintrust.CryptCATGetAttrInfo"),
        private: false,
    },
    Export {
        name: "CryptCATGetCatAttrInfo",
        ordinal: 22,
        kind: ExportKind::Forwarded("wintrust.CryptCATGetCatAttrInfo"),
        private: false,
    },
    Export {
        name: "CryptCATGetMemberInfo",
        ordinal: 23,
        kind: ExportKind::Forwarded("wintrust.CryptCATGetMemberInfo"),
        private: false,
    },
    Export {
        name: "CryptCATHandleFromStore",
        ordinal: 24,
        kind: ExportKind::Forwarded("wintrust.CryptCATHandleFromStore"),
        private: false,
    },
    Export {
        name: "CryptCATOpen",
        ordinal: 25,
        kind: ExportKind::Forwarded("wintrust.CryptCATOpen"),
        private: false,
    },
    Export {
        name: "CryptCATPersistStore",
        ordinal: 26,
        kind: ExportKind::Forwarded("wintrust.CryptCATPersistStore"),
        private: false,
    },
    Export {
        name: "CryptCATPutAttrInfo",
        ordinal: 27,
        kind: ExportKind::Forwarded("wintrust.CryptCATPutAttrInfo"),
        private: false,
    },
    Export {
        name: "CryptCATPutCatAttrInfo",
        ordinal: 28,
        kind: ExportKind::Forwarded("wintrust.CryptCATPutCatAttrInfo"),
        private: false,
    },
    Export {
        name: "CryptCATPutMemberInfo",
        ordinal: 29,
        kind: ExportKind::Forwarded("wintrust.CryptCATPutMemberInfo"),
        private: false,
    },
    Export {
        name: "CryptCATStoreFromHandle",
        ordinal: 30,
        kind: ExportKind::Forwarded("wintrust.CryptCATStoreFromHandle"),
        private: false,
    },
    Export {
        name: "DllRegisterServer",
        ordinal: 31,
        kind: ExportKind::Forwarded("wintrust.mscat32DllRegisterServer"),
        private: true,
    },
    Export {
        name: "DllUnregisterServer",
        ordinal: 32,
        kind: ExportKind::Forwarded("wintrust.mscat32DllUnregisterServer"),
        private: true,
    },
    Export {
        name: "IsCatalogFile",
        ordinal: 33,
        kind: ExportKind::Forwarded("wintrust.IsCatalogFile"),
        private: false,
    },
    Export {
        name: "MsCatConstructHashTag",
        ordinal: 34,
        kind: ExportKind::Stub,
        private: true,
    },
    Export {
        name: "MsCatFreeHashTag",
        ordinal: 35,
        kind: ExportKind::Stub,
        private: true,
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|e| e.name == name).copied()
}
