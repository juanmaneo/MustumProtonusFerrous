//! Port of the portable logic in Wine's `dlls/d3dcompiler_47/d3dcompiler_47.spec` and the
//! `@ stub` exports it declares.
//!
//! Ported:
//! - `d3dcompiler_47.spec` export table: [`EXPORTS`], [`ExportKind`] and [`export_kind`], which
//!   record each of the 29 lines as `@ stdcall`, `@ stdcall -private` or `@ stub`.
//! - `@ stub` exports, the ten `@ stub` lines: [`export_stub`] returns the record for a stub
//!   export, and [`wine_spec_unimplemented_stub`] is the shared stub body (see
//!   [`StubException`]). The record carries the `MODULE` name of `Makefile.in`,
//!   `d3dcompiler_47.dll`.
//!
//! Skipped:
//! - The C bodies of the `@ stdcall` and `@ stdcall -private` exports. `Makefile.in` sets
//!   `PARENTSRC = ../d3dcompiler_43`, so those functions are compiled from the
//!   `dlls/d3dcompiler_43` tree. That tree is ported by `mpf-d3dcompiler_43`, which covers only
//!   part of it. This crate has no dependencies, so it does not reuse that port.
//!   `EXTRADEFS = -DD3D_COMPILER_VERSION=47` only affects that shared source.
//! - The argument lists of the spec lines, which depend on the target ABI for the stack size.
//! - The ordinal form of the stub record, used only for exports without a name. Every export
//!   here has a name.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    export_stub, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
