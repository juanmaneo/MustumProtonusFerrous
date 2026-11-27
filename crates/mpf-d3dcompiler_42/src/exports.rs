/// How `dlls/d3dcompiler_42/d3dcompiler_42.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall Name`: implemented under the same name by the shared `d3dcompiler_43`
    /// sources (`PARENTSRC = ../d3dcompiler_43`), which are not in this directory.
    Stdcall,
    /// `@ stdcall -private Name`: built into the DLL but not in its export table.
    Private,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `d3dcompiler_42.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 13] = [
    ("D3DAssemble", ExportKind::Private),
    ("DebugSetMute", ExportKind::Stub),
    ("D3DCompile", ExportKind::Stdcall),
    ("D3DDisassemble10Effect", ExportKind::Stub),
    ("D3DDisassemble", ExportKind::Stdcall),
    ("D3DGetDebugInfo", ExportKind::Stdcall),
    ("D3DGetInputAndOutputSignatureBlob", ExportKind::Stdcall),
    ("D3DGetInputSignatureBlob", ExportKind::Stdcall),
    ("D3DGetOutputSignatureBlob", ExportKind::Stdcall),
    ("D3DPreprocess", ExportKind::Stdcall),
    ("D3DReflect", ExportKind::Stdcall),
    ("D3DReturnFailure1", ExportKind::Stub),
    ("D3DStripShader", ExportKind::Stdcall),
];

/// The kind of `name`, or `None` when the spec does not list it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
