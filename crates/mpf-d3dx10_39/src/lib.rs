//! Port of the local portable logic in Wine's `dlls/d3dx10_39`.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_39_main.c`, as [`d3dx10_check_version`].
//! - The four `@ stub` exports in `d3dx10_39.spec`, as exception records returned by
//!   [`wine_spec_unimplemented_stub`] and the corresponding export helpers (`stub.rs`).
//!
//! Skipped:
//! - `Makefile.in` sets `PARENTSRC = ../d3dx9_36`. Those shared C bodies live in
//!   `dlls/d3dx9_36`, and are not ported in this crate.
//! - The other spec exports forward to `d3dx10_43`; their implementations are not local to this
//!   DLL. Diagnostics, Windows/COM state, and the non-portable behavior of the stub's infinite
//!   exception-raising loop are not reproduced.
//!
//! This is a partial port because the parent-source bodies and forwarded implementations remain
//! outside this crate.

mod stub;
mod version;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use stub::{
    d3dx10_create_reduction, d3dx10_disassemble_effect, d3dx10_disassemble_shader,
    d3dx10_reflect_shader, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};
pub use version::d3dx10_check_version;
