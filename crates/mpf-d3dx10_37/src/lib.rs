//! Port of the local portable logic in Wine's `dlls/d3dx10_37`.
//!
//! `Makefile.in` sets `PARENTSRC = ../d3dx9_36`, so most implementation bodies are in the parent
//! directory; `d3dx10_37_main.c` and `d3dx10_37.spec` are local.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_37_main.c`: [`d3dx10_check_version`] and
//!   [`D3D10_SDK_VERSION`] (`version.rs`).
//! - The full `d3dx10_37.spec` export table: [`EXPORTS`], [`ExportKind`], and [`export_kind`]
//!   (`exports.rs`), including its 171 forwards, 8 local stdcalls, and two `@ stub` records.
//! - `D3DX10CreateReduction` and `D3DX10GetDriverLevel` stub exception records, modeled after
//!   Wine's `__wine_spec_unimplemented_stub`: [`StubException`],
//!   [`wine_spec_unimplemented_stub`], [`d3dx10_create_reduction`], and
//!   [`d3dx10_get_driver_level`] (`stub.rs`).
//!
//! Skipped:
//! - The seven non-forwarded stdcall bodies other than `D3DX10CheckVersion`. They are compiled
//!   from `dlls/d3dx9_36` via `PARENTSRC` and call effect/compiler logic not ported here.
//! - The forwarded implementation bodies, which belong to the destination DLLs in the spec.
//! - The Windows `WINAPI` calling convention and `BOOL` return type; the version port returns
//!   Rust `bool`.

mod exports;
mod stub;
mod version;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    d3dx10_create_reduction, d3dx10_get_driver_level, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use version::{d3dx10_check_version, D3D10_SDK_VERSION};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
