//! Port of the stub records of Wine's `dlls/d3dx9_29/d3dx9_29.spec`.
//!
//! `dlls/d3dx9_29` has no C sources of its own. `Makefile.in` sets `PARENTSRC = ../d3dx9_36`,
//! so the bodies of the 264 `@ stdcall` exports live in `dlls/d3dx9_36`. Only the spec table
//! and the stub records are local to this directory.
//!
//! Ported:
//! - The 68 `@ stub` exports of `d3dx9_29.spec`, listed in [`STUB_EXPORTS`]. winebuild points
//!   them at `__wine_spec_unimplemented_stub`, which is modelled by
//!   [`wine_spec_unimplemented_stub`], with [`d3dx9_29_stub`] for the `d3dx9_29.dll` module and
//!   [`StubException`] as the record (`stub.rs`).
//!
//! Skipped:
//! - The 264 `@ stdcall` exports. Their bodies are in `dlls/d3dx9_36` through `PARENTSRC`, not
//!   in this directory, and the `d3dx9_36` crate ports only part of them.
//! - `TRACE` output and the other diagnostics of `dlls/d3dx9_36`.

mod stub;

pub use stub::{
    d3dx9_29_stub, wine_spec_unimplemented_stub, StubException, D3DX9_29_DLL_NAME,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, STUB_EXPORTS,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
