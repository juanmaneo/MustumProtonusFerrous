//! Port of the portable logic in Wine's `dlls/sas/sas.spec` and `dlls/sas/Makefile.in`.
//!
//! `dlls/sas` has no C sources, so this is a partial port: the export table and the stub record
//! that its only export raises.
//!
//! Ported:
//! - `sas.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `SendSAS` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`send_sas`], [`wine_spec_unimplemented_stub`], [`StubException`], with the constants
//!   [`EXCEPTION_WINE_STUB`] and [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The `__cdecl` calling convention and the `for (;;)` loop around `RtlRaiseException`: the port
//!   returns the record instead, because the C function never returns.
//! - A body for `SendSAS`: the reference defines none, only the stub entry that winebuild emits.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    send_sas, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
