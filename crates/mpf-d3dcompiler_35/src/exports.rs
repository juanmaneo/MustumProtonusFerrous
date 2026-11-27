/// How `dlls/d3dcompiler_35/d3dcompiler_35.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function. The payload is the C function that implements it in the
    /// parent sources in `dlls/d3dcompiler_43`, which is the spec's `impl` name or the export name.
    Stdcall(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `d3dcompiler_35.spec` export table, in spec order.
pub const EXPORTS: [(&str, ExportKind); 10] = [
    ("D3DCompileFromMemory", ExportKind::Stdcall("D3DCompile")),
    ("D3DDisassembleCode", ExportKind::Stdcall("D3DDisassemble")),
    ("D3DDisassembleEffect", ExportKind::Stub),
    (
        "D3DGetCodeDebugInfo",
        ExportKind::Stdcall("D3DGetDebugInfo"),
    ),
    (
        "D3DGetInputAndOutputSignatureBlob",
        ExportKind::Stdcall("D3DGetInputAndOutputSignatureBlob"),
    ),
    (
        "D3DGetInputSignatureBlob",
        ExportKind::Stdcall("D3DGetInputSignatureBlob"),
    ),
    (
        "D3DGetOutputSignatureBlob",
        ExportKind::Stdcall("D3DGetOutputSignatureBlob"),
    ),
    (
        "D3DPreprocessFromMemory",
        ExportKind::Stdcall("D3DPreprocess"),
    ),
    ("D3DReflectCode", ExportKind::Stdcall("D3DReflect")),
    ("DebugSetMute", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
