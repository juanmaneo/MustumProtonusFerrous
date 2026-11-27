//! Port of the portable logic in Wine's `dlls/d3dcompiler_37/d3dcompiler_37.spec` and
//! `dlls/d3dcompiler_37/Makefile.in`.
//!
//! The directory has no `.c` sources. `Makefile.in` sets `PARENTSRC = ../d3dcompiler_43`, so the
//! exports link to C functions in that tree. The shared blob and token logic is ported in
//! `mpf-d3dcompiler_43`, not here.
//!
//! Ported:
//! - `d3dcompiler_37.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The two `@ stub` exports, `DebugSetMute` and `D3DDisassembleEffect`, and the
//!   `__wine_spec_unimplemented_stub` record they raise: [`debug_set_mute`],
//!   [`d3d_disassemble_effect`], [`export_stub`], [`wine_spec_unimplemented_stub`],
//!   [`StubException`], with `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE` from
//!   `include/winternl.h` and `include/winnt.h`
//!
//! Skipped:
//! - The bodies of the eight `@ stdcall` exports. They are C functions in `dlls/d3dcompiler_43`
//!   (compiler, blob, disassembler, reflection and preprocessor code), not in this directory. The
//!   export table records which C function each one calls.
//! - The ordinal form of the function argument. Every export has a name, so winebuild always
//!   passes the name.
//! - The `__cdecl` calling convention, and the `for (;;)` loop around `RtlRaiseException`. The port
//!   returns the record instead.
//! - The stdcall stack sizes and the `VER_*` version resource values in `Makefile.in`, which are
//!   build data, not logic.

mod exports;
mod stub;

pub use exports::{export_kind, export_stub, ExportKind, EXPORTS};
pub use stub::{
    d3d_disassemble_effect, debug_set_mute, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
