/// How `dlls/srvcli/srvcli.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through the helper of the same name in [`crate::stub`].
    Stub,
    /// `@ stdcall ... module.function`: `winebuild` writes a forwarder, so calls to the name resolve
    /// in `module`.
    Forward {
        module: &'static str,
        function: &'static str,
    },
}

/// One line of `dlls/srvcli/srvcli.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The exported name.
    pub name: &'static str,
    /// The ordinal `winebuild` assigns: the line's position in the spec, counted from 1.
    pub ordinal: u16,
    pub kind: ExportKind,
}

/// The `srvcli.spec` export table, in spec order: 52 `@ stub` exports and 9 forwards.
pub const EXPORTS: [Export; 61] = [
    Export {
        name: "I_NetDfsGetVersion",
        ordinal: 1,
        kind: ExportKind::Stub,
    },
    Export {
        name: "I_NetServerSetServiceBits",
        ordinal: 2,
        kind: ExportKind::Stub,
    },
    Export {
        name: "I_NetServerSetServiceBitsEx",
        ordinal: 3,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalAliasGet",
        ordinal: 4,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalFileClose",
        ordinal: 5,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalFileEnum",
        ordinal: 6,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalFileEnumEx",
        ordinal: 7,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalFileGetInfo",
        ordinal: 8,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalFileGetInfoEx",
        ordinal: 9,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalServerCertificateMappingAdd",
        ordinal: 10,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalServerCertificateMappingEnum",
        ordinal: 11,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalServerCertificateMappingGet",
        ordinal: 12,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalServerCertificateMappingRemove",
        ordinal: 13,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalSessionDel",
        ordinal: 14,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalSessionEnum",
        ordinal: 15,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalSessionEnumEx",
        ordinal: 16,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalSessionGetInfo",
        ordinal: 17,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalSessionGetInfoEx",
        ordinal: 18,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalShareAdd",
        ordinal: 19,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalShareDelEx",
        ordinal: 20,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalShareEnum",
        ordinal: 21,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalShareEnumEx",
        ordinal: 22,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalShareGetInfo",
        ordinal: 23,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalShareGetInfoEx",
        ordinal: 24,
        kind: ExportKind::Stub,
    },
    Export {
        name: "LocalShareSetInfo",
        ordinal: 25,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetConnectionEnum",
        ordinal: 26,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetFileClose",
        ordinal: 27,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetFileEnum",
        ordinal: 28,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetFileEnum",
        },
    },
    Export {
        name: "NetFileGetInfo",
        ordinal: 29,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetRemoteTOD",
        ordinal: 30,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetRemoteTOD",
        },
    },
    Export {
        name: "NetServerAliasAdd",
        ordinal: 31,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerAliasDel",
        ordinal: 32,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerAliasEnum",
        ordinal: 33,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerComputerNameAdd",
        ordinal: 34,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerComputerNameDel",
        ordinal: 35,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerDiskEnum",
        ordinal: 36,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetServerDiskEnum",
        },
    },
    Export {
        name: "NetServerGetInfo",
        ordinal: 37,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetServerGetInfo",
        },
    },
    Export {
        name: "NetServerSetInfo",
        ordinal: 38,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerStatisticsGet",
        ordinal: 39,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerTransportAdd",
        ordinal: 40,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerTransportAddEx",
        ordinal: 41,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerTransportDel",
        ordinal: 42,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetServerTransportEnum",
        ordinal: 43,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetSessionDel",
        ordinal: 44,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetSessionEnum",
        ordinal: 45,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetSessionEnum",
        },
    },
    Export {
        name: "NetSessionGetInfo",
        ordinal: 46,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetShareAdd",
        ordinal: 47,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetShareAdd",
        },
    },
    Export {
        name: "NetShareCheck",
        ordinal: 48,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetShareDel",
        ordinal: 49,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetShareDel",
        },
    },
    Export {
        name: "NetShareDelEx",
        ordinal: 50,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetShareDelSticky",
        ordinal: 51,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetShareEnum",
        ordinal: 52,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetShareEnum",
        },
    },
    Export {
        name: "NetShareEnumSticky",
        ordinal: 53,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetShareGetInfo",
        ordinal: 54,
        kind: ExportKind::Forward {
            module: "netapi32",
            function: "NetShareGetInfo",
        },
    },
    Export {
        name: "NetShareSetInfo",
        ordinal: 55,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetpsNameCanonicalize",
        ordinal: 56,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetpsNameCompare",
        ordinal: 57,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetpsNameValidate",
        ordinal: 58,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetpsPathCanonicalize",
        ordinal: 59,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetpsPathCompare",
        ordinal: 60,
        kind: ExportKind::Stub,
    },
    Export {
        name: "NetpsPathType",
        ordinal: 61,
        kind: ExportKind::Stub,
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are matched exactly.
pub fn export_by_name(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}

/// The `(module, function)` that a call to `name` is forwarded to, or `None` when `name` is not a
/// forward.
pub fn forward_target(name: &str) -> Option<(&'static str, &'static str)> {
    match export_by_name(name)?.kind {
        ExportKind::Forward { module, function } => Some((module, function)),
        ExportKind::Stub => None,
    }
}
