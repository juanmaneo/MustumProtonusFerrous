//! Port of the portable logic in Wine's `dlls/d3dx9_26/d3dx9_26.spec` and the `@ stub` exports it
//! declares.
//!
//! `dlls/d3dx9_26/Makefile.in` sets `PARENTSRC = ../d3dx9_36`, so the C bodies of the
//! `@ stdcall` exports live in `dlls/d3dx9_36`, not in this directory. Only the spec table and the
//! stub records are local to `d3dx9_26`.
//!
//! Ported:
//! - The export table of `d3dx9_26.spec`: [`EXPORTS`], [`ExportKind`] and [`export_kind`]. It
//!   has 327 entries: 259 `@ stdcall` and 68 `@ stub`.
//! - The 68 `@ stub` exports, which winebuild points at `__wine_spec_unimplemented_stub`:
//!   [`stub_exception`] returns the [`StubException`] record for each of them, with
//!   [`wine_spec_unimplemented_stub`] and the `EXCEPTION_*` constants (`stub.rs`).
//!
//! Skipped:
//! - The 259 `@ stdcall` bodies. They are in `dlls/d3dx9_36` through `PARENTSRC`, and
//!   `mpf-d3dx9_36` ports only its vector, matrix, plane and quaternion math. This crate has no
//!   dependency on that port, so the component is partial.
//! - The argument lists of the spec lines, which depend on the target ABI for the stack size.
//! - The ordinals, which winebuild assigns, and the `ExceptionRecord` and `ExceptionAddress`
//!   fields of the stub record. The stub record is modelled by its code, flags, parameter count
//!   and the two name parameters.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    stub_exception, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
