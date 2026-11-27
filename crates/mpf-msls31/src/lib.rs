//! Port of the portable logic in Wine's `dlls/msls31/msls31.spec` and `dlls/msls31/Makefile.in`.
//!
//! `dlls/msls31` has no C, IDL, resource or typelib files. Every export in its spec is `@ stub`,
//! so the port covers the export table and the stub records the exports raise.
//!
//! Ported:
//! - `msls31.spec` export table: [`EXPORTS`], [`stub_exception`]
//! - `__wine_spec_unimplemented_stub` and the record it raises, with the `EXCEPTION_WINE_STUB`
//!   and `EXCEPTION_NONCONTINUABLE` values: [`wine_spec_unimplemented_stub`], [`StubException`],
//!   [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//! - One helper per `@ stub` export, `ls_*`, `lsdn_*` and `lssb_*`, in `stub.rs`. Each raises the
//!   record for its export name, and [`EXPORTS`] maps every name to its helper.
//!
//! Skipped:
//! - The Line Services (`LS*`) code itself. The reference tree has no source for it, so there is
//!   no return value or logic to port. Every export is a stub, and the stub body never returns.
//! - The `__cdecl` calling convention and the `RtlRaiseException` loop: the port returns the record
//!   instead, as `mpf-fontsub` does.
//! - Ordinal function names: the spec names every export, so the function argument is always a name.

mod exports;
mod stub;

pub use exports::{stub_exception, EXPORTS};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
