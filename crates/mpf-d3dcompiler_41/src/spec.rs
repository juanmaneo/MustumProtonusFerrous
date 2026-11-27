/// How `d3dcompiler_41.spec` declares an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `@ stdcall`: the body is in `dlls/d3dcompiler_43`.
    Stdcall,
    /// `@ stdcall -private`: the body is in `dlls/d3dcompiler_43`, and the DLL does not export it.
    StdcallPrivate,
    /// `@ stub`: winebuild points the export at `__wine_spec_unimplemented_stub`.
    Stub,
}

/// One line of `d3dcompiler_41.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub kind: Kind,
}

/// The entries of `dlls/d3dcompiler_41/d3dcompiler_41.spec`, in spec order.
pub const EXPORTS: [Export; 13] = [
    Export {
        name: "D3DAssemble",
        kind: Kind::StdcallPrivate,
    },
    Export {
        name: "DebugSetMute",
        kind: Kind::Stub,
    },
    Export {
        name: "D3DCompile",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DDisassemble10Effect",
        kind: Kind::Stub,
    },
    Export {
        name: "D3DDisassemble",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DGetDebugInfo",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DGetInputAndOutputSignatureBlob",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DGetInputSignatureBlob",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DGetOutputSignatureBlob",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DPreprocess",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DReflect",
        kind: Kind::Stdcall,
    },
    Export {
        name: "D3DReturnFailure1",
        kind: Kind::Stub,
    },
    Export {
        name: "D3DStripShader",
        kind: Kind::Stdcall,
    },
];

/// The spec entry called `name`, if there is one.
pub fn export(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|e| e.name == name)
}
