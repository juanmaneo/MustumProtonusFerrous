//! Port of the portable logic in Wine's `dlls/winnls32/winnls.c` and `dlls/winnls32/winnls32.spec`.
//!
//! Ported:
//! - `WINNLS32EnableIME` and `WINNLS32GetEnableStatus`, the two explicit bodies:
//!   [`winnls32_enable_ime`] and [`winnls32_get_enable_status`], with the `BOOL` values
//!   [`TRUE`] and [`FALSE`]
//! - `winnls32.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The five `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`winnls32_get_ime_hot_key`]
//!
//! Skipped:
//! - The `hWnd` argument of both explicit bodies, which the C code never reads: the port
//!   accepts it as an opaque [`Hwnd`] and ignores it.
//! - The `WINAPI` calling convention, and the `winebuild` thunks that call the stub.

mod calls;
mod exports;
mod stubs;

pub use calls::{winnls32_enable_ime, winnls32_get_enable_status, Bool, Hwnd, FALSE, TRUE};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stubs::{
    ime32_send_ime_message_ex, imp32_get_ime, imp32_query_ime, imp32_set_ime,
    wine_spec_unimplemented_stub, winnls32_get_ime_hot_key, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB, STUB_EXPORTS, WINNLS32_DLL_NAME,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
