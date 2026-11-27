use crate::ArgKind;

/// One `@ stdcall` line of `dlls/iprop/iprop.spec`. The line's target `forward_module.forward_name`
/// is the `ole32` export that calls to `name` are forwarded to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub args: &'static [ArgKind],
    pub forward_module: &'static str,
    pub forward_name: &'static str,
}

/// The `iprop.spec` export table, in spec order. Every entry is a forward.
pub const EXPORTS: [Export; 8] = [
    Export {
        name: "FmtIdToPropStgName",
        args: &[ArgKind::Ptr, ArgKind::Wstr],
        forward_module: "ole32",
        forward_name: "FmtIdToPropStgName",
    },
    Export {
        name: "FreePropVariantArray",
        args: &[ArgKind::Long, ArgKind::Ptr],
        forward_module: "ole32",
        forward_name: "FreePropVariantArray",
    },
    Export {
        name: "PropStgNameToFmtId",
        args: &[ArgKind::Wstr, ArgKind::Ptr],
        forward_module: "ole32",
        forward_name: "PropStgNameToFmtId",
    },
    Export {
        name: "PropVariantClear",
        args: &[ArgKind::Ptr],
        forward_module: "ole32",
        forward_name: "PropVariantClear",
    },
    Export {
        name: "PropVariantCopy",
        args: &[ArgKind::Ptr, ArgKind::Ptr],
        forward_module: "ole32",
        forward_name: "PropVariantCopy",
    },
    Export {
        name: "StgCreatePropSetStg",
        args: &[ArgKind::Ptr, ArgKind::Long, ArgKind::Ptr],
        forward_module: "ole32",
        forward_name: "StgCreatePropSetStg",
    },
    Export {
        name: "StgCreatePropStg",
        args: &[
            ArgKind::Ptr,
            ArgKind::Ptr,
            ArgKind::Ptr,
            ArgKind::Long,
            ArgKind::Long,
            ArgKind::Ptr,
        ],
        forward_module: "ole32",
        forward_name: "StgCreatePropStg",
    },
    Export {
        name: "StgOpenPropStg",
        args: &[
            ArgKind::Ptr,
            ArgKind::Ptr,
            ArgKind::Long,
            ArgKind::Long,
            ArgKind::Ptr,
        ],
        forward_module: "ole32",
        forward_name: "StgOpenPropStg",
    },
];

/// The export named `name`, or `None` when `iprop.spec` does not export it. Names are
/// case-sensitive.
pub fn find_export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}

/// The `(module, function)` that a call to `name` on `iprop.dll` is forwarded to, or `None` when
/// `iprop.dll` does not export `name`.
pub fn forward_target(name: &str) -> Option<(&'static str, &'static str)> {
    find_export(name).map(|export| (export.forward_module, export.forward_name))
}
