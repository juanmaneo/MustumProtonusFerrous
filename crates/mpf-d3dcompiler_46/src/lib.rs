//! Port of the local part of Wine's `dlls/d3dcompiler_46`.
//!
//! `d3dcompiler_46` sets `PARENTSRC = ../d3dcompiler_43` in `Makefile.in`, so its C bodies
//! live in `dlls/d3dcompiler_43`. The only local files are `Makefile.in`,
//! `d3dcompiler_46.spec` and `tests/`. This crate therefore models only the spec's stub
//! records. Verdict: `partial`.
//!
//! Ported:
//! - The nine `@ stub` exports of `d3dcompiler_46.spec`, which winebuild points at
//!   `__wine_spec_unimplemented_stub`: [`debug_set_mute`], [`d3d_compress_shaders`],
//!   [`d3d_decompress_shaders`], [`d3d_disassemble_10_effect`], [`d3d_disassemble_11_trace`],
//!   [`d3d_disassemble_region`], [`d3d_get_trace_instruction_offsets`],
//!   [`d3d_return_failure_1`] and [`d3d_set_blob_part`], with
//!   [`wine_spec_unimplemented_stub`] and [`StubException`] (`stub.rs`).
//!
//! Skipped:
//! - The non-stub exports of `d3dcompiler_46.spec` (`D3DAssemble`, `D3DCompile`,
//!   `D3DCompile2`, `D3DCompileFromFile`, `D3DCreateBlob`, `D3DDisassemble`,
//!   `D3DGetBlobPart`, `D3DGetDebugInfo`, the signature getters, `D3DPreprocess`,
//!   `D3DReadFileToBlob`, `D3DReflect`, `D3DStripShader` and `D3DWriteBlobToFile`).
//!   Their bodies are in `dlls/d3dcompiler_43`, which this crate does not port. The
//!   `d3dcompiler_43` crate covers part of that tree.
//! - The `TRACE` output that the C code writes through `wine_dbgstr_*`. It is diagnostics only.

mod stub;

pub use stub::{
    d3d_compress_shaders, d3d_decompress_shaders, d3d_disassemble_10_effect,
    d3d_disassemble_11_trace, d3d_disassemble_region, d3d_get_trace_instruction_offsets,
    d3d_return_failure_1, d3d_set_blob_part, debug_set_mute, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod edge_checks;

#[cfg(test)]
mod oracle_checks;
