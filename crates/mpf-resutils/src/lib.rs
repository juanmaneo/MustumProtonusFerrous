//! Port of the portable logic in Wine's `dlls/resutils/resutils.spec` and `dlls/resutils/Makefile.in`.
//!
//! `dlls/resutils` has no C, IDL, RC or TLB sources. Its only content is the spec's export table,
//! which lists 71 `@ stub` exports and nothing else, so the port models those stubs.
//!
//! Ported:
//! - `resutils.spec` export table: [`EXPORTS`], [`export_stub`]
//! - the 71 `@ stub` exports, one helper each, named in snake_case after the C names (for example
//!   [`res_util_dup_string`] for `ResUtilDupString`)
//! - `__wine_spec_unimplemented_stub` and the record it raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - Function bodies: the spec exports no real code, so no export has a return value to port.
//! - The `for (;;)` loop in `__wine_spec_unimplemented_stub`: the port returns the record instead.
//! - The ordinal form of the stub call: every entry in the spec has a name.
//! - The `__cdecl` calling convention.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
