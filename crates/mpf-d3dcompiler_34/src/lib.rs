//! Port of the portable logic in Wine's `dlls/d3dcompiler_34/d3dcompiler_34.spec` and the
//! `@ stub` exports it declares.
//!
//! Ported:
//! - `d3dcompiler_34.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`] and
//!   [`implementation_name`], which maps each `@ stdcall` export to its target name.
//! - `@ stub` exports `D3DDisassembleEffect` and `DebugSetMute`: [`d3d_disassemble_effect`] and
//!   [`debug_set_mute`], which raise the record of [`wine_spec_unimplemented_stub`] (see
//!   [`StubException`]). The record carries the `MODULE` name of `Makefile.in`,
//!   `d3dcompiler_34.dll`.
//!
//! Skipped:
//! - The C sources. `Makefile.in` sets `PARENTSRC = ../d3dcompiler_43` and compiles those
//!   sources as this DLL, so the exported `D3D*` functions live in the `d3dcompiler_43` tree.
//!   That tree is ported by `mpf-d3dcompiler_43`. This crate has no dependencies, so it does
//!   not reuse that port. `EXTRADEFS = -DD3D_COMPILER_VERSION=34` has no effect on the
//!   non-test code of that tree.
//! - The argument lists of the spec lines, which depend on the target ABI for the stack size.
//! - The ordinal form of the stub record, used only for exports without a name. Every export
//!   here has a name.

mod exports;
mod stub;

pub use exports::{export_kind, implementation_name, ExportKind, EXPORTS};
pub use stub::{
    d3d_disassemble_effect, debug_set_mute, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
