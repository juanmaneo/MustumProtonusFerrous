/// An argument type in a `.spec` signature. `psapi.spec` uses only `long` and `ptr`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgKind {
    Long,
    Ptr,
}

impl ArgKind {
    /// The bytes the argument takes on the i386 stack, as `winebuild`'s `get_args_size` counts
    /// them. Each kind is one pointer-sized word on i386.
    fn i386_bytes(self) -> u32 {
        match self {
            ArgKind::Long | ArgKind::Ptr => 4,
        }
    }
}

/// One `@ stdcall -import` line of `dlls/psapi/psapi.spec`. Each line imports the `K32` symbol
/// named by `link_name`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    /// Assigned in spec order, because every line uses `@` instead of an explicit ordinal.
    pub ordinal: u16,
    pub args: &'static [ArgKind],
    /// The symbol written after the signature, which the import thunk jumps to.
    pub link_name: &'static str,
}

impl Export {
    /// The `N` in the `name@N` decoration, which is the i386 argument byte count.
    pub fn stdcall_bytes(&self) -> u32 {
        self.args.iter().map(|arg| arg.i386_bytes()).sum()
    }

    /// The `.def` line that `winebuild --def -m32` writes for this export. The `=link` part
    /// is written only when the link name differs from the export name.
    pub fn def_line(&self) -> String {
        let decorated = format!("{}@{}", self.name, self.stdcall_bytes());
        if self.name == self.link_name {
            format!("  {} @{}", decorated, self.ordinal)
        } else {
            format!("  {}={} @{}", decorated, self.link_name, self.ordinal)
        }
    }
}

/// The `psapi.spec` export table, in spec order.
pub const EXPORTS: [Export; 27] = [
    Export {
        name: "EmptyWorkingSet",
        ordinal: 1,
        args: &[ArgKind::Long],
        link_name: "K32EmptyWorkingSet",
    },
    Export {
        name: "EnumDeviceDrivers",
        ordinal: 2,
        args: &[ArgKind::Ptr, ArgKind::Long, ArgKind::Ptr],
        link_name: "K32EnumDeviceDrivers",
    },
    Export {
        name: "EnumPageFilesA",
        ordinal: 3,
        args: &[ArgKind::Ptr, ArgKind::Ptr],
        link_name: "K32EnumPageFilesA",
    },
    Export {
        name: "EnumPageFilesW",
        ordinal: 4,
        args: &[ArgKind::Ptr, ArgKind::Ptr],
        link_name: "K32EnumPageFilesW",
    },
    Export {
        name: "EnumProcessModules",
        ordinal: 5,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long, ArgKind::Ptr],
        link_name: "K32EnumProcessModules",
    },
    Export {
        name: "EnumProcessModulesEx",
        ordinal: 6,
        args: &[
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
        ],
        link_name: "K32EnumProcessModulesEx",
    },
    Export {
        name: "EnumProcesses",
        ordinal: 7,
        args: &[ArgKind::Ptr, ArgKind::Long, ArgKind::Ptr],
        link_name: "K32EnumProcesses",
    },
    Export {
        name: "GetDeviceDriverBaseNameA",
        ordinal: 8,
        args: &[ArgKind::Ptr, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetDeviceDriverBaseNameA",
    },
    Export {
        name: "GetDeviceDriverBaseNameW",
        ordinal: 9,
        args: &[ArgKind::Ptr, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetDeviceDriverBaseNameW",
    },
    Export {
        name: "GetDeviceDriverFileNameA",
        ordinal: 10,
        args: &[ArgKind::Ptr, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetDeviceDriverFileNameA",
    },
    Export {
        name: "GetDeviceDriverFileNameW",
        ordinal: 11,
        args: &[ArgKind::Ptr, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetDeviceDriverFileNameW",
    },
    Export {
        name: "GetMappedFileNameA",
        ordinal: 12,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetMappedFileNameA",
    },
    Export {
        name: "GetMappedFileNameW",
        ordinal: 13,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetMappedFileNameW",
    },
    Export {
        name: "GetModuleBaseNameA",
        ordinal: 14,
        args: &[ArgKind::Long, ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetModuleBaseNameA",
    },
    Export {
        name: "GetModuleBaseNameW",
        ordinal: 15,
        args: &[ArgKind::Long, ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetModuleBaseNameW",
    },
    Export {
        name: "GetModuleFileNameExA",
        ordinal: 16,
        args: &[ArgKind::Long, ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetModuleFileNameExA",
    },
    Export {
        name: "GetModuleFileNameExW",
        ordinal: 17,
        args: &[ArgKind::Long, ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetModuleFileNameExW",
    },
    Export {
        name: "GetModuleInformation",
        ordinal: 18,
        args: &[ArgKind::Long, ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetModuleInformation",
    },
    Export {
        name: "GetPerformanceInfo",
        ordinal: 19,
        args: &[ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetPerformanceInfo",
    },
    Export {
        name: "GetProcessImageFileNameA",
        ordinal: 20,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetProcessImageFileNameA",
    },
    Export {
        name: "GetProcessImageFileNameW",
        ordinal: 21,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetProcessImageFileNameW",
    },
    Export {
        name: "GetProcessMemoryInfo",
        ordinal: 22,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetProcessMemoryInfo",
    },
    Export {
        name: "GetWsChanges",
        ordinal: 23,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32GetWsChanges",
    },
    Export {
        name: "GetWsChangesEx",
        ordinal: 24,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Ptr],
        link_name: "K32GetWsChangesEx",
    },
    Export {
        name: "InitializeProcessForWsWatch",
        ordinal: 25,
        args: &[ArgKind::Long],
        link_name: "K32InitializeProcessForWsWatch",
    },
    Export {
        name: "QueryWorkingSet",
        ordinal: 26,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32QueryWorkingSet",
    },
    Export {
        name: "QueryWorkingSetEx",
        ordinal: 27,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        link_name: "K32QueryWorkingSetEx",
    },
];

/// The export named `name`, or `None` when `psapi.spec` does not export it. Names are
/// case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}
