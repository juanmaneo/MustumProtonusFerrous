//! Port of the portable logic in Wine's `dlls/d3dx10_36`, which has almost no sources of its own.
//! Its `Makefile.in` sets `PARENTSRC = ../d3dx9_36`, so the C bodies of the non-forwarded exports
//! come from `dlls/d3dx9_36/d3dx10_compiler.c`, and the shared helpers come from
//! `dlls/d3dx9_36/d3dx_helpers.c`. Only the spec table and `d3dx10_36_main.c` are local.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_36_main.c`: [`d3dx10_check_version`], with
//!   [`D3D10_SDK_VERSION`].
//! - `d3dx10_36.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]. The table has
//!   172 forwards (to `d3dx10_43`, `d3dx10_39` and `d3dx10_37`) and 8 `@ stdcall` names without a
//!   forward.
//!
//! Skipped:
//! - The `@ stdcall` names without a forward that have bodies in `dlls/d3dx9_36`:
//!   `D3DX10CompileFromMemory`, `D3DX10CreateEffectFromFileA`, `D3DX10CreateEffectFromFileW`,
//!   `D3DX10CreateEffectFromMemory`, `D3DX10CreateEffectFromResourceA`,
//!   `D3DX10CreateEffectFromResourceW` and `D3DX10PreprocessShaderFromMemory`. Those bodies
//!   call the D3DCompiler and effect code that is not ported here.
//! - `d3dx_helpers.c`, which the parent directory also compiles into this DLL.
//! - The `WINAPI` calling convention and the `BOOL` return type. The port returns `bool`.
//!
//! The spec has no `@ stub` entries, so there is no `__wine_spec_unimplemented_stub` record to model.

mod exports;
mod version;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use version::{d3dx10_check_version, D3D10_SDK_VERSION};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
