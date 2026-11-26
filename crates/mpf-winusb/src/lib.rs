//! Port of the portable logic in Wine's `dlls/winusb/main.c` and `dlls/winusb/winusb.spec`.
//!
//! Ported:
//! - `WinUsb_Free`: the stub body, which returns `TRUE`: [`winusb_free`], with [`TRUE`]
//! - `winusb.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 21 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`winusb_abort_pipe`]
//!
//! Skipped:
//! - The `FIXME` trace of the handle, and the handle itself: the C body never reads it, so the
//!   port accepts any type and ignores it.
//! - The `WINAPI` calling convention: the functions are plain Rust functions.
//! - Ordinal names: every `winusb.spec` entry has a name, so no stub is raised by ordinal.

mod exports;
mod free;
mod stubs;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use free::{winusb_free, TRUE};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
