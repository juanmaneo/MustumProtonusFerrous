//! Port of the portable logic in Wine's `dlls/d3d12core/d3d12core.spec`. The DLL has no C
//! sources, so the logic is its export table and the `@ stub` records that winebuild emits.
//!
//! Ported:
//! - `d3d12core.spec` export table: [`EXPORTS`], [`export_stub`]
//! - `D3D12GetInterface` and `D3D12SDKVersion` (both `@ stub`): [`d3d12_get_interface`],
//!   [`d3d12_sdk_version`]
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The stubs never return, so no return value is modelled. The C body loops on
//!   `RtlRaiseException`; the port returns the record instead.
//! - The `__cdecl` calling convention and the `RtlRaiseException` call.
//! - NULL `module` or `function` pointers and embedded NUL bytes. The port takes `&'static str`,
//!   so it cannot hold a NULL pointer, and it keeps the whole string where C stops at the first
//!   NUL. The oracle does not drive either case.
//! - Ordinal names: both exports are named in the spec, so winebuild never passes an ordinal.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::{
    d3d12_get_interface, d3d12_sdk_version, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
