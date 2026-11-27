/// One entry of `sfc.spec`, with the ordinal winebuild gave it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The line of the entry in `sfc.spec`.
    pub line: u32,
    /// The ordinal. An entry without one takes the lowest free ordinal after the lowest explicit
    /// one, in spec order (`assign_ordinals` in `tools/winebuild/parser.c`).
    pub ordinal: u16,
    /// The exported name. `@` in the spec gives `None`: the entry is exported by ordinal only.
    pub name: Option<&'static str>,
    /// The forward target written after the function name, `module.function`.
    pub link_name: &'static str,
}

impl Export {
    /// Splits the forward target at its last `.`, as `find_forwarded_export` does with `strrchr`
    /// in `dlls/ntdll/loader.c`. Returns `None` when the link name has no `.`.
    pub fn forward_target(&self) -> Option<(&'static str, &'static str)> {
        self.link_name.rsplit_once('.')
    }
}

/// The `dlls/sfc/sfc.spec` export table, in spec order.
pub const EXPORTS: [Export; 16] = [
    Export {
        line: 1,
        ordinal: 1,
        name: None,
        link_name: "sfc_os.SfcInitProt",
    },
    Export {
        line: 2,
        ordinal: 2,
        name: None,
        link_name: "sfc_os.SfcTerminateWatcherThread",
    },
    Export {
        line: 3,
        ordinal: 3,
        name: None,
        link_name: "sfc_os.SfcConnectToServer",
    },
    Export {
        line: 4,
        ordinal: 4,
        name: None,
        link_name: "sfc_os.SfcClose",
    },
    Export {
        line: 5,
        ordinal: 5,
        name: None,
        link_name: "sfc_os.SfcFileException",
    },
    Export {
        line: 6,
        ordinal: 6,
        name: None,
        link_name: "sfc_os.SfcInitiateScan",
    },
    Export {
        line: 7,
        ordinal: 7,
        name: None,
        link_name: "sfc_os.SfcInstallProtectedFiles",
    },
    Export {
        line: 8,
        ordinal: 8,
        name: None,
        link_name: "sfc_os.SfpInstallCatalog",
    },
    Export {
        line: 9,
        ordinal: 9,
        name: None,
        link_name: "sfc_os.SfpDeleteCatalog",
    },
    Export {
        line: 10,
        ordinal: 10,
        name: Some("SRSetRestorePoint"),
        link_name: "sfc_os.SRSetRestorePointA",
    },
    Export {
        line: 11,
        ordinal: 11,
        name: Some("SRSetRestorePointA"),
        link_name: "sfc_os.SRSetRestorePointA",
    },
    Export {
        line: 12,
        ordinal: 12,
        name: Some("SRSetRestorePointW"),
        link_name: "sfc_os.SRSetRestorePointW",
    },
    Export {
        line: 13,
        ordinal: 13,
        name: Some("SfcGetNextProtectedFile"),
        link_name: "sfc_os.SfcGetNextProtectedFile",
    },
    Export {
        line: 14,
        ordinal: 14,
        name: Some("SfcIsFileProtected"),
        link_name: "sfc_os.SfcIsFileProtected",
    },
    Export {
        line: 15,
        ordinal: 15,
        name: Some("SfcIsKeyProtected"),
        link_name: "sfc_os.SfcIsKeyProtected",
    },
    Export {
        line: 16,
        ordinal: 16,
        name: Some("SfpVerifyFile"),
        link_name: "sfc_os.SfpVerifyFile",
    },
];

/// The name table, sorted by name the way winebuild sorts it with `strcmp`. The index of an entry
/// in the returned table is its hint.
pub fn name_table() -> Vec<Export> {
    let mut named: Vec<Export> = EXPORTS
        .iter()
        .copied()
        .filter(|e| e.name.is_some())
        .collect();
    named.sort_by_key(|e| e.name);
    named
}

/// The entry exported under `name`. Names are case-sensitive, and nameless entries never match.
pub fn export_by_name(name: &str) -> Option<Export> {
    EXPORTS.iter().copied().find(|e| e.name == Some(name))
}

/// The entry with `ordinal`.
pub fn export_by_ordinal(ordinal: u16) -> Option<Export> {
    EXPORTS.iter().copied().find(|e| e.ordinal == ordinal)
}
