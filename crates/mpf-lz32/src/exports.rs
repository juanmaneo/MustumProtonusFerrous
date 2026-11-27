/// An argument type in a `.spec` signature. `winebuild` parses these names: `long`, `ptr`,
/// `str` and `wstr`. A signature written `long long` is two `long` arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgKind {
    Long,
    Ptr,
    Str,
    WStr,
}

impl ArgKind {
    /// The bytes the argument takes on the i386 stack, as `winebuild`'s `get_args_size` counts
    /// them. Each kind is one pointer-sized word on i386.
    fn i386_bytes(self) -> u32 {
        match self {
            ArgKind::Long | ArgKind::Ptr | ArgKind::Str | ArgKind::WStr => 4,
        }
    }
}

/// One `@ stdcall` line of `dlls/lz32/lz32.spec`. Each line forwards to `kernel32`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    /// Assigned in spec order, because every line uses `@` instead of an explicit ordinal.
    pub ordinal: u16,
    pub args: &'static [ArgKind],
    /// The forward target written after the signature, `module.function`.
    pub target: &'static str,
}

impl Export {
    /// The `N` in the `name@N` decoration, which is the i386 argument byte count.
    pub fn stdcall_bytes(&self) -> u32 {
        self.args.iter().map(|arg| arg.i386_bytes()).sum()
    }

    /// The module and function of the forward target, split at the first `.` as `winebuild`'s
    /// `strchr` does. `None` when the target has no `.`.
    pub fn forward(&self) -> Option<(&'static str, &'static str)> {
        self.target.split_once('.')
    }

    /// The `.def` line that `winebuild --def -m32` writes for this export.
    pub fn def_line(&self) -> String {
        format!(
            "  {}@{}={} @{}",
            self.name,
            self.stdcall_bytes(),
            self.target,
            self.ordinal
        )
    }
}

/// The `lz32.spec` export table, in spec order.
pub const EXPORTS: [Export; 12] = [
    Export {
        name: "CopyLZFile",
        ordinal: 1,
        args: &[ArgKind::Long, ArgKind::Long],
        target: "kernel32.CopyLZFile",
    },
    Export {
        name: "GetExpandedNameA",
        ordinal: 2,
        args: &[ArgKind::Str, ArgKind::Ptr],
        target: "kernel32.GetExpandedNameA",
    },
    Export {
        name: "GetExpandedNameW",
        ordinal: 3,
        args: &[ArgKind::WStr, ArgKind::Ptr],
        target: "kernel32.GetExpandedNameW",
    },
    Export {
        name: "LZClose",
        ordinal: 4,
        args: &[ArgKind::Long],
        target: "kernel32.LZClose",
    },
    Export {
        name: "LZCopy",
        ordinal: 5,
        args: &[ArgKind::Long, ArgKind::Long],
        target: "kernel32.LZCopy",
    },
    Export {
        name: "LZDone",
        ordinal: 6,
        args: &[],
        target: "kernel32.LZDone",
    },
    Export {
        name: "LZInit",
        ordinal: 7,
        args: &[ArgKind::Long],
        target: "kernel32.LZInit",
    },
    Export {
        name: "LZOpenFileA",
        ordinal: 8,
        args: &[ArgKind::Str, ArgKind::Ptr, ArgKind::Long],
        target: "kernel32.LZOpenFileA",
    },
    Export {
        name: "LZOpenFileW",
        ordinal: 9,
        args: &[ArgKind::WStr, ArgKind::Ptr, ArgKind::Long],
        target: "kernel32.LZOpenFileW",
    },
    Export {
        name: "LZRead",
        ordinal: 10,
        args: &[ArgKind::Long, ArgKind::Ptr, ArgKind::Long],
        target: "kernel32.LZRead",
    },
    Export {
        name: "LZSeek",
        ordinal: 11,
        args: &[ArgKind::Long, ArgKind::Long, ArgKind::Long],
        target: "kernel32.LZSeek",
    },
    Export {
        name: "LZStart",
        ordinal: 12,
        args: &[],
        target: "kernel32.LZStart",
    },
];

/// The export named `name`, or `None` when `lz32.spec` does not export it. Names are
/// case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}
