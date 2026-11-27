//! Port of the portable logic in Wine's `dlls/d3dx10_41/d3dx10_41_main.c` and `dlls/d3dx10_41/d3dx10_41.spec`.
//!
//! `dlls/d3dx10_41/Makefile.in` sets `PARENTSRC = ../d3dx9_36`. The C bodies of
//! `d3dx10_compiler.c` and `d3dx_helpers.c` (listed in `SOURCES`) live in `dlls/d3dx9_36`, not in
//! this directory. The local C tree contains only `d3dx10_41_main.c` and the spec table.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_41_main.c`: [`check_version`], with the SDK versions
//!   [`D3D10_SDK_VERSION`] and [`D3DX10_SDK_VERSION`] (`version.rs`).
//!
//! Skipped:
//! - The `@ stub` records: `d3dx10_41.spec` has none, so there is no `stub.rs` and no
//!   `__wine_spec_unimplemented_stub` model.
//! - The 168 forwarded exports. Their target is `d3dx10_43`, a different DLL.
//! - The 7 exports without a forward target whose bodies are in the parent's
//!   `dlls/d3dx9_36/d3dx10_compiler.c`: `D3DX10CompileFromMemory`, `D3DX10CreateEffectFromFileA`,
//!   `D3DX10CreateEffectFromFileW`, `D3DX10CreateEffectFromMemory`,
//!   `D3DX10CreateEffectFromResourceA`, `D3DX10CreateEffectFromResourceW` and
//!   `D3DX10PreprocessShaderFromMemory`. Their bodies are not in this crate, so the component is
//!   partial.

mod version;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use version::{check_version, D3D10_SDK_VERSION, D3DX10_SDK_VERSION};
