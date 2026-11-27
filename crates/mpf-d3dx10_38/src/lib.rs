//! Port of the portable logic in Wine's `dlls/d3dx10_38/d3dx10_38_main.c` and
//! `dlls/d3dx10_38/d3dx10_38.spec`.
//!
//! Ported:
//! - `D3DX10CheckVersion` from `d3dx10_38_main.c`.
//! - The `D3DX10CreateReduction` `@ stub` from `d3dx10_38.spec`, modelled with
//!   [`wine_spec_unimplemented_stub`] and [`StubException`].
//!
//! Skipped:
//! - The bodies for `D3DX10CompileFromMemory`, `D3DX10CreateEffectFrom*` and
//!   `D3DX10PreprocessShaderFromMemory`, because `Makefile.in` sets
//!   `PARENTSRC = ../d3dx9_36` and these bodies live in `dlls/d3dx9_36`.
//! - Exports forwarded by the spec to `d3dx10_43.dll` or `d3dx10_39.dll`; they are implemented
//!   by those DLLs rather than by this component.
//! - The exception address and pointer-valued exception information in the stub record: this
//!   portable model preserves the exception code, flags, parameter count, module and function.
//!
//! Oracle vectors were produced by compiling the unmodified local C body and a verbatim copy of
//! `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c` against shims with
//! `gcc -fsanitize=address,undefined -g`; the stub harness uses `longjmp` to leave the real
//! function's non-returning loop.

mod check_version;
mod stub;

pub use check_version::d3dx10_check_version;
pub use stub::{
    d3dx10_create_reduction, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
