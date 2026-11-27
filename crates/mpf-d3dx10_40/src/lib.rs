//! Port of the local portable logic in Wine's `dlls/d3dx10_40`.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_40_main.c`, which checks the D3D10 SDK version and the
//!   D3DX10 SDK version this DLL was built against (`version.rs`).
//!
//! Skipped:
//! - The inherited export bodies: `Makefile.in` sets `PARENTSRC = ../d3dx9_36`, so those C bodies
//!   are shared from `dlls/d3dx9_36`; the spec also forwards selected exports to `d3dx10_43`.
//!   They are mostly COM, graphics, file, and decoder operations, not local portable logic.
//! - There are no `@ stub` exports in `d3dx10_40.spec`.

mod version;

pub use version::d3dx10_check_version;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
