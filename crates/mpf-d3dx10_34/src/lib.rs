//! Port of the portable logic in Wine's `dlls/d3dx10_34`.
//!
//! The C bodies for this DLL are in the parent directory, `dlls/d3dx9_36`, because
//! `dlls/d3dx10_34/Makefile.in` sets `PARENTSRC = ../d3dx9_36`. The local directory holds only
//! `d3dx10_34_main.c` and `d3dx10_34.spec`.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_34_main.c`: [`d3dx10_check_version`].
//!
//! Skipped:
//! - The `@ stub` exports. `d3dx10_34.spec` has no `@ stub` lines, so there are no
//!   `StubException` records to model.
//! - The exports that the spec forwards to `d3dx10_37`, `d3dx10_39` and `d3dx10_43`. Their
//!   bodies are in other DLLs, not in this component.
//! - The bodies of `D3DX10CompileFromMemory`, `D3DX10CreateEffectFromFileA/W`,
//!   `D3DX10CreateEffectFromMemory`, `D3DX10CreateEffectFromResourceA/W` and
//!   `D3DX10PreprocessShaderFromMemory`. They are in `dlls/d3dx9_36/d3dx10_compiler.c`, which
//!   this DLL compiles through `PARENTSRC`. They are wrappers around `D3DCompile`,
//!   `D3DPreprocess` and `D3D10CreateEffectFromMemory`, plus file and resource loading, so the
//!   portable part is not separable from those calls. The verdict for this component is therefore
//!   `partial`.

mod version;

pub use version::d3dx10_check_version;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
