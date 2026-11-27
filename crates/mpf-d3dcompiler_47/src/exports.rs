/// How `dlls/d3dcompiler_47/d3dcompiler_47.spec` exports a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ stdcall`: a real function in the shared `d3dcompiler_43` sources.
    Stdcall,
    /// `@ stdcall -private`: a real function that is not in the DLL's export table.
    Private,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
}

/// The `d3dcompiler_47.spec` export table, in spec order, as (export name, kind). Every line
/// names its export and has no target, so the export name is the implementation name.
pub const EXPORTS: [(&str, ExportKind); 29] = [
    ("D3DAssemble", ExportKind::Private),
    ("D3DCompile", ExportKind::Stdcall),
    ("D3DCompile2", ExportKind::Stdcall),
    ("D3DCompileFromFile", ExportKind::Stdcall),
    ("D3DCompressShaders", ExportKind::Stub),
    ("D3DCreateBlob", ExportKind::Stdcall),
    ("D3DCreateFunctionLinkingGraph", ExportKind::Stdcall),
    ("D3DCreateLinker", ExportKind::Stdcall),
    ("D3DDecompressShaders", ExportKind::Stub),
    ("D3DDisassemble", ExportKind::Stdcall),
    ("D3DDisassemble10Effect", ExportKind::Stub),
    ("D3DDisassemble11Trace", ExportKind::Stub),
    ("D3DDisassembleRegion", ExportKind::Stub),
    ("D3DGetBlobPart", ExportKind::Stdcall),
    ("D3DGetDebugInfo", ExportKind::Stdcall),
    ("D3DGetInputAndOutputSignatureBlob", ExportKind::Stdcall),
    ("D3DGetInputSignatureBlob", ExportKind::Stdcall),
    ("D3DGetOutputSignatureBlob", ExportKind::Stdcall),
    ("D3DGetTraceInstructionOffsets", ExportKind::Stub),
    ("D3DLoadModule", ExportKind::Stdcall),
    ("D3DPreprocess", ExportKind::Stdcall),
    ("D3DReadFileToBlob", ExportKind::Stdcall),
    ("D3DReflect", ExportKind::Stdcall),
    ("D3DReflectLibrary", ExportKind::Stub),
    ("D3DReturnFailure1", ExportKind::Stub),
    ("D3DSetBlobPart", ExportKind::Stub),
    ("D3DStripShader", ExportKind::Stdcall),
    ("D3DWriteBlobToFile", ExportKind::Stdcall),
    ("DebugSetMute", ExportKind::Stub),
];

/// The kind of `name`, or `None` when the spec does not declare it. Names are case-sensitive.
pub fn export_kind(name: &str) -> Option<ExportKind> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, kind)| kind)
}
