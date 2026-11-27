/// How `dlls/d3dcompiler_38/d3dcompiler_38.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall Name`: implemented under the same name by the shared `d3dcompiler_43`
    /// sources (`PARENTSRC = ../d3dcompiler_43`), which are not in this directory.
    Stdcall,
    /// `@ stdcall Name(args) Target`: exported as `Name` and forwarded to `Target`.
    Forward(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `d3dcompiler_38.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 11] = [
    ("D3DCompileFromMemory", ExportKind::Forward("D3DCompile")),
    ("D3DDisassembleCode", ExportKind::Forward("D3DDisassemble")),
    ("D3DDisassembleEffect", ExportKind::Stub),
    (
        "D3DGetCodeDebugInfo",
        ExportKind::Forward("D3DGetDebugInfo"),
    ),
    ("D3DGetInputAndOutputSignatureBlob", ExportKind::Stdcall),
    ("D3DGetInputSignatureBlob", ExportKind::Stdcall),
    ("D3DGetOutputSignatureBlob", ExportKind::Stdcall),
    (
        "D3DPreprocessFromMemory",
        ExportKind::Forward("D3DPreprocess"),
    ),
    ("D3DReflectCode", ExportKind::Forward("D3DReflect")),
    ("D3DReturnFailure1", ExportKind::Stub),
    ("DebugSetMute", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
