//! Portable subset of Wine's `dlls/d3dx10_35`.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_35_main.c`, as [`d3dx10_check_version`]. It checks
//!   against `D3D10_SDK_VERSION` 29 and the D3DX10 SDK version 35.
//!
//! Skipped:
//! - The component sets `PARENTSRC = ../d3dx9_36` in `Makefile.in`. The bodies for
//!   `d3dx10_compiler.c` and `d3dx_helpers.c` are in `dlls/d3dx9_36`, and are not ported here.
//! - The other `d3dx10_35.spec` entries forward to `d3dx10_43`, `d3dx10_39`, or `d3dx10_37`;
//!   non-forwarded compiler and effect bodies are in the parent source. There are no `@ stub`
//!   exports in this spec.
//! - DLL, COM, compiler, graphics-device, and test behavior requiring the inherited bodies or
//!   Windows runtime state.

mod version;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use version::d3dx10_check_version;
