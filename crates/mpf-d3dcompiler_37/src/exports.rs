use crate::stub::{d3d_disassemble_effect, debug_set_mute, StubException};

/// How `dlls/d3dcompiler_37/d3dcompiler_37.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: the export calls the C function named here, which lives in `dlls/d3dcompiler_43`.
    Stdcall(&'static str),
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `d3dcompiler_37.spec` export table, in spec order, with the C function each `stdcall`
/// export calls. Names are the exported names, and the second field is the C function name.
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

/// The stub record for `name` when it is a `@ stub` export, or `None` for any other name.
pub fn export_stub(name: &str) -> Option<StubException> {
    match name {
        "DebugSetMute" => Some(debug_set_mute()),
        "D3DDisassembleEffect" => Some(d3d_disassemble_effect()),
        _ => None,
    }
}
