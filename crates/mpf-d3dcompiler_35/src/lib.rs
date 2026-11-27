//! Port of the portable logic in Wine's `dlls/d3dcompiler_35/d3dcompiler_35.spec` and the
//! version gates of the sources it builds. `dlls/d3dcompiler_35/Makefile.in` sets
//! `PARENTSRC = ../d3dcompiler_43` and `EXTRADEFS = -DD3D_COMPILER_VERSION=35`, so the C code is
//! the parent's, compiled for version 35.
//!
//! Ported:
//! - `d3dcompiler_35.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports `D3DDisassembleEffect` and `DebugSetMute`, and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`d3d_disassemble_effect`],
//!   [`debug_set_mute`], [`wine_spec_unimplemented_stub`], [`StubException`]
//! - the `D3D_COMPILER_VERSION` gates: [`disassemble_precheck`], the empty-input check at the top
//!   of `D3DDisassemble` (`compiler.c`, compiled from version 46), and [`reflect_forward`], the
//!   choice between `D3DReflect` and `D3D10ReflectShader` (`reflection.c`)
//!
//! Skipped:
//! - The shared bodies of the parent sources (blob, token, opcode, assembler, compiler, linker
//!   and reflection). Those are ported in `mpf-d3dcompiler_43`, which this DLL compiles unchanged.
//! - The vkd3d calls (`vkd3d_shader_compile`, `vkd3d_D3DCompile2VKD3D` and
//!   `vkd3d_D3DReflectVKD3D`). [`reflect_forward`] reports the version they receive, and the
//!   port does not model their results.
//! - The `TRACE` and `FIXME` output, the `D3DCompile2` include and dummy-blob handling, and the
//!   `D3DDisassemble` blob copy after the vkd3d call.
//! - `RtlRaiseException`: the C stub loops forever, so the port returns the record it would raise.
//!
//! Oracle provenance: the stub, the `D3DDisassemble` gate and the reflection block are copied
//! verbatim from Wine's `dlls/ntdll/exception.c`, `dlls/d3dcompiler_43/compiler.c` and
//! `dlls/d3dcompiler_43/reflection.c`. The harness compiles under `gcc -fsanitize=address,undefined`
//! with `-DD3D_COMPILER_VERSION` set to 35, 46 and 0 (`target/canon/d3dcompiler_35/`, not tracked in git).

mod exports;
mod stub;
mod version;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    d3d_disassemble_effect, debug_set_mute, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use version::{
    disassemble_precheck, reflect_forward, ReflectForward, D3D_COMPILER_VERSION, E_INVALIDARG,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
