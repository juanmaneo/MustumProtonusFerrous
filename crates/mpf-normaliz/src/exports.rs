use crate::ArgKind;

/// One `@ stdcall` line of `dlls/normaliz/normaliz.spec`. The line's target
/// `forward_module.forward_name` is the `kernel32` export that calls to `name` are forwarded to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub args: &'static [ArgKind],
    pub forward_module: &'static str,
    pub forward_name: &'static str,
}

/// The `normaliz.spec` export table, in spec order. Every entry is a forward.
pub const EXPORTS: [Export; 5] = [
    Export {
        name: "IdnToAscii",
        args: &[
            ArgKind::Long,
            ArgKind::Wstr,
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
        ],
        forward_module: "kernel32",
        forward_name: "IdnToAscii",
    },
    Export {
        name: "IdnToNameprepUnicode",
        args: &[
            ArgKind::Long,
            ArgKind::Wstr,
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
        ],
        forward_module: "kernel32",
        forward_name: "IdnToNameprepUnicode",
    },
    Export {
        name: "IdnToUnicode",
        args: &[
            ArgKind::Long,
            ArgKind::Wstr,
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
        ],
        forward_module: "kernel32",
        forward_name: "IdnToUnicode",
    },
    Export {
        name: "IsNormalizedString",
        args: &[ArgKind::Long, ArgKind::Wstr, ArgKind::Long],
        forward_module: "kernel32",
        forward_name: "IsNormalizedString",
    },
    Export {
        name: "NormalizeString",
        args: &[
            ArgKind::Long,
            ArgKind::Wstr,
            ArgKind::Long,
            ArgKind::Ptr,
            ArgKind::Long,
        ],
        forward_module: "kernel32",
        forward_name: "NormalizeString",
    },
];

/// The export named `name`, or `None` when `normaliz.spec` does not export it. Names are
/// case-sensitive.
pub fn find_export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}

/// The `(module, function)` that a call to `name` on `normaliz.dll` is forwarded to, or `None`
/// when `normaliz.dll` does not export `name`.
pub fn forward_target(name: &str) -> Option<(&'static str, &'static str)> {
    find_export(name).map(|export| (export.forward_module, export.forward_name))
}
