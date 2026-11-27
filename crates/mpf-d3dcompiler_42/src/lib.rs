//! Port of the portable logic in Wine's `dlls/d3dcompiler_42/d3dcompiler_42.spec` and
//! `dlls/d3dcompiler_42/Makefile.in`.
//!
//! The directory has no C sources of its own. `Makefile.in` sets `PARENTSRC = ../d3dcompiler_43`,
//! so the C bodies of the exported functions are the `d3dcompiler_43` sources built with
//! `D3D_COMPILER_VERSION=42`. Only the spec table and the `@ stub` records are local.
//!
//! Ported:
//! - `d3dcompiler_42.spec` export table: [`EXPORTS`], [`ExportKind`] and [`export_kind`], with
//!   the thirteen names in spec order, the `@ stdcall` exports, the `-private` export and the
//!   `@ stub` entries
//! - `@ stub` exports, modelled as the `__wine_spec_unimplemented_stub` record they raise:
//!   [`debug_set_mute`], [`d3d_disassemble10_effect`], [`d3d_return_failure1`],
//!   [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The bodies of the ten non-stub exports (`D3DAssemble`, `D3DCompile`, `D3DDisassemble`,
//!   `D3DGetDebugInfo`, the three `D3DGetInput*`/`D3DGetOutput*` signature blob functions,
//!   `D3DPreprocess`, `D3DReflect` and `D3DStripShader`). They live in `dlls/d3dcompiler_43`
//!   and allocate blobs or read DXBC containers, so they are not ported here. The portable part
//!   of that code is in `mpf-d3dcompiler_43`.
//! - The `__stdcall` calling convention and the `stdcall` argument layout of the exports.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    d3d_disassemble10_effect, d3d_return_failure1, debug_set_mute, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
