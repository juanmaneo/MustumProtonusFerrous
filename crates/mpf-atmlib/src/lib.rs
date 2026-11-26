//! Port of the portable logic in Wine's `dlls/atmlib/main.c` and `dlls/atmlib/atmlib.spec`.
//!
//! Ported:
//! - `ATMProperlyLoaded` (returns `FALSE`) and `ATMFinish` (returns `0`):
//!   [`atm_properly_loaded`], [`atm_finish`]
//! - The export table of `atmlib.spec` (`@ stdcall` or `@ stub` per name):
//!   [`EXPORTS`], [`export_kind`], [`ExportKind`]
//! - `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`), which every `@ stub` export
//!   calls. It raises `EXCEPTION_WINE_STUB` and never returns, so the port returns the raised
//!   record: [`unimplemented_stub`], [`StubException`], [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - `FIXME("stub\n")` tracing in `main.c`: debug-channel output, no state.
//! - The raise loop and `RtlRaiseException`/`RaiseException`: OS exception delivery.
//! - The record's `ExceptionRecord` (always NULL) and `ExceptionAddress` (address of the stub
//!   function) are not modelled.

mod entry;
mod exports;
mod stub;

pub use entry::{atm_finish, atm_properly_loaded};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
