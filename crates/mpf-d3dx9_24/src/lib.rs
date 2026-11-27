//! d3dx9_24 subset for MPF: the `@ stub` exports of Wine's `dlls/d3dx9_24/d3dx9_24.spec` and
//! the records that `__wine_spec_unimplemented_stub` raises for them.
//!
//! `dlls/d3dx9_24/Makefile.in` sets `PARENTSRC = ../d3dx9_36`. The C bodies of this DLL are in
//! `dlls/d3dx9_36`, and `dlls/d3dx9_24` has no `.c` or `.h` files of its own. This crate models
//! only the spec table and the stub records.
//!
//! Ported:
//! - The 61 `@ stub` exports of `d3dx9_24.spec`: [`STUB_EXPORTS`], in spec order.
//! - [`stub_export`], which returns the record for a stub export and `None` for any other name.
//! - `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`: [`wine_spec_unimplemented_stub`]
//!   and the [`StubException`] record (`stub.rs`).
//!
//! Skipped:
//! - The 259 `@ stdcall` exports of `d3dx9_24.spec`, from `D3DXAssembleShader` through
//!   `D3DXWeldVertices`. Their bodies are in `dlls/d3dx9_36`, which `PARENTSRC` compiles into
//!   this DLL. The sibling crate `mpf-d3dx9_36` ports part of that code. This crate does not
//!   reuse it.
//! - The loop in `__wine_spec_unimplemented_stub`. The C function re-raises the record forever,
//!   so the port returns the record instead.

mod stub;

pub use stub::{
    stub_export, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB, STUB_EXPORTS,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
