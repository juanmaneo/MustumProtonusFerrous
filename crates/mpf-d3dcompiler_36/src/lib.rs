//! Port of the portable logic in Wine's `dlls/d3dcompiler_36`, which has no sources of its own.
//! Its `Makefile.in` sets `PARENTSRC = ../d3dcompiler_43`, so the C bodies come from
//! `dlls/d3dcompiler_43/blob.c` and `dlls/d3dcompiler_43/compiler.c`, and the stub body from
//! `dlls/ntdll/exception.c`.
//!
//! Ported:
//! - `D3DGetInputSignatureBlob`, `D3DGetOutputSignatureBlob`, `D3DGetInputAndOutputSignatureBlob`
//!   and `D3DGetCodeDebugInfo` (`D3DGetDebugInfo`): the argument checks, the part-to-tag selection
//!   and the section-count rules of `d3dcompiler_get_blob_part`: [`get_blob_part`], [`BlobPart`],
//!   [`Section`], with the `D3D_BLOB_*` and `TAG_*` values
//! - `hresult_from_vkd3d_result`: [`hresult_from_vkd3d_result`], with the `E_*` and `VKD3D_*` values
//! - `D3DDisassembleCode` (`D3DDisassemble`): the source choice [`disassemble_source_type`]
//! - `D3DPreprocessFromMemory` (`D3DPreprocess`): the argument gate [`check_preprocess_args`]
//! - The include path that `d3dcompiler_include_from_file_open` builds: [`include_full_path`]
//! - `d3dcompiler_36.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports `D3DDisassembleEffect` and `DebugSetMute`, and the `__wine_spec_unimplemented_stub`
//!   record they raise: [`d3d_disassemble_effect`], [`debug_set_mute`], [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//!
//! Skipped:
//! - The shader compiler and reflection behind `D3DCompileFromMemory`, `D3DDisassembleCode`,
//!   `D3DPreprocessFromMemory` and `D3DReflectCode`. Those calls go to vkd3d
//!   (`vkd3d_D3DCompile2VKD3D`, `vkd3d_shader_compile`, `vkd3d_D3DPreprocess`, `vkd3d_D3DReflectVKD3D`),
//!   which is not ported. Only the argument checks and the choices made before those calls are ported.
//! - The DXBC container parse (`vkd3d_shader_parse_dxbc`) and the packing of selected sections into a
//!   container (`vkd3d_shader_serialize_dxbc`). The caller supplies the parsed sections, and
//!   [`BlobPart::Container`] returns the selected sections without packing them.
//! - The blob allocation (`D3DCreateBlob`), the file reads of the include open, and the `malloc` failures.
//! - `get_line`, which only splits vkd3d messages for `ERR` logs, and the `TRACE`, `FIXME`, `WARN` and `ERR` calls.
//! - The `WINAPI` and `__cdecl` calling conventions. Include names are `&str`, not bytes.

mod blob;
mod compiler;
mod exports;
mod hresult;
mod stub;

pub use blob::{
    get_blob_part, BlobPart, Section, D3D_BLOB_ALL_SIGNATURE_BLOB, D3D_BLOB_DEBUG_INFO,
    D3D_BLOB_DEBUG_NAME, D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB, D3D_BLOB_INPUT_SIGNATURE_BLOB,
    D3D_BLOB_LEGACY_SHADER, D3D_BLOB_OUTPUT_SIGNATURE_BLOB, D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB,
    D3D_BLOB_PDB, D3D_BLOB_TEST_ALTERNATE_SHADER, D3D_BLOB_TEST_COMPILE_REPORT,
    D3D_BLOB_XNA_PREPASS_SHADER, D3D_BLOB_XNA_SHADER, DXBC_HEADER_SIZE,
};
pub use compiler::{
    check_preprocess_args, disassemble_source_type, include_full_path, SourceType, MAX_PATH,
    TAG_DXBC,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use hresult::{
    hresult_from_vkd3d_result, D3DERR_INVALIDCALL, E_FAIL, E_INVALIDARG, E_NOTIMPL, E_OUTOFMEMORY,
    S_OK,
};
pub use stub::{
    d3d_disassemble_effect, debug_set_mute, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
