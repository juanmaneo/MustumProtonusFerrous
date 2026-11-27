//! Port of the local part of Wine's `dlls/d3dcompiler_41`: the export table of
//! `d3dcompiler_41.spec` and the `@ stub` exception records.
//!
//! `dlls/d3dcompiler_41/Makefile.in` sets `PARENTSRC = ../d3dcompiler_43`, so the C bodies
//! of its exports live in `dlls/d3dcompiler_43`, not in this directory. Only the spec table
//! and the stub records are local.
//!
//! Ported:
//! - The 13 entries of `d3dcompiler_41.spec`, with their kind (`stdcall`, `stdcall -private`
//!   or `stub`): [`EXPORTS`], [`Export`], [`Kind`] and [`export`].
//! - The three `@ stub` exports, `DebugSetMute`, `D3DDisassemble10Effect` and
//!   `D3DReturnFailure1`, which winebuild points at `__wine_spec_unimplemented_stub`:
//!   [`debug_set_mute`], [`d3d_disassemble10_effect`], [`d3d_return_failure1`] and
//!   [`stub_exception`], with [`wine_spec_unimplemented_stub`] and [`StubException`] (`stub.rs`).
//!
//! Skipped:
//! - The bodies of the nine `stdcall` exports and of the one `stdcall -private` export
//!   (`D3DAssemble`). They are in `dlls/d3dcompiler_43`, which this crate does not port, so
//!   this component is partial.
//! - `TRACE` output: diagnostics only.

mod spec;
mod stub;

pub use spec::{export, Export, Kind, EXPORTS};
pub use stub::{
    d3d_disassemble10_effect, d3d_return_failure1, debug_set_mute, stub_exception,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
