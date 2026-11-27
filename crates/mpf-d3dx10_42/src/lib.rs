//! Port of the portable logic in Wine's `dlls/d3dx10_42`.
//!
//! `Makefile.in` sets `PARENTSRC = ../d3dx9_36`. Its `SOURCES` are the local
//! `d3dx10_42_main.c` plus `d3dx10_compiler.c` and `d3dx_helpers.c`, which live in
//! `dlls/d3dx9_36`. The local `d3dx10_42.spec` forwards most exports to `d3dx10_43` and leaves
//! the rest unforwarded, so their bodies come from the parent directory. This crate ports only
//! the body that is local to this directory.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_42_main.c`: [`d3dx10_check_version`].
//!
//! Skipped, parent-source bodies (`dlls/d3dx9_36`):
//! - `d3dx10_compiler.c`: `D3DX10CreateEffectFromMemory`, `D3DX10CreateEffectFromFileA` and
//!   `W`, `D3DX10CreateEffectFromResourceA` and `W`, `D3DX10PreprocessShaderFromMemory`,
//!   `D3DX10CompileFromMemory` and the static `create_effect`. They are thin wrappers over
//!   `D3DCompile`, `D3DPreprocess` and `D3D10CreateEffectFromMemory` from d3dcompiler and d3d10,
//!   plus the `d3dx_load_*` file and resource loaders. None of those can be ported without
//!   the shader compiler and the D3D10 effect framework.
//! - `d3dx_helpers.c`: about 3.5k lines of WIC, DDS, TGA, pixel format and filter code shared
//!   with `d3dx9_36`. Its port belongs to the `d3dx9_36` component.
//! - Trace output and `FIXME` diagnostics.
//!
//! Oracle provenance: see `oracle_checks.rs`.

mod version;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use version::d3dx10_check_version;
