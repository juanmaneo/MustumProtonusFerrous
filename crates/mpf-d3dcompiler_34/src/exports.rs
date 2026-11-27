/// How `dlls/d3dcompiler_34/d3dcompiler_34.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function in the shared `d3dcompiler_43` sources.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `d3dcompiler_34.spec` export table, in spec order, as (export name, kind, implementation
/// name). The implementation name is the target after the argument list of an `@ stdcall` line,
/// or the export name when the line has no target. Stub exports have no target.
pub const EXPORTS: [(&str, ExportKind, &str); 10] = [
    ("D3DCompileFromMemory", ExportKind::Stdcall, "D3DCompile"),
    ("D3DDisassembleCode", ExportKind::Stdcall, "D3DDisassemble"),
    (
        "D3DDisassembleEffect",
        ExportKind::Stub,
        "D3DDisassembleEffect",
    ),
    (
        "D3DGetCodeDebugInfo",
        ExportKind::Stdcall,
        "D3DGetDebugInfo",
    ),
    (
        "D3DGetInputAndOutputSignatureBlob",
        ExportKind::Stdcall,
        "D3DGetInputAndOutputSignatureBlob",
    ),
    (
        "D3DGetInputSignatureBlob",
        ExportKind::Stdcall,
        "D3DGetInputSignatureBlob",
    ),
    (
        "D3DGetOutputSignatureBlob",
        ExportKind::Stdcall,
        "D3DGetOutputSignatureBlob",
    ),
    (
        "D3DPreprocessFromMemory",
        ExportKind::Stdcall,
        "D3DPreprocess",
    ),
    ("D3DReflectCode", ExportKind::Stdcall, "D3DReflect"),
    ("DebugSetMute", ExportKind::Stub, "DebugSetMute"),
];

fn find(name: &str) -> Option<&'static (&'static str, ExportKind, &'static str)> {
    EXPORTS.iter().find(|(n, _, _)| *n == name)
}

/// The kind of `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    find(name).map(|&(_, kind, _)| kind)
}

/// The implementation name `name` maps to, or `None` when the spec does not export it.
pub fn implementation_name(name: &str) -> Option<&'static str> {
    find(name).map(|&(_, _, target)| target)
}
