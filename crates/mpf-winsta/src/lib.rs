//! Port of the portable logic in Wine's `dlls/winsta/main.c` and `dlls/winsta/winsta.spec`.
//!
//! Ported:
//! - The eight `@ stdcall` functions in `main.c`. Each one sets `ERROR_CALL_NOT_IMPLEMENTED`
//!   and returns `FALSE`; `WinStationGetAllProcesses` also writes `0` to `*process_count`:
//!   [`Winsta`], with `ERROR_CALL_NOT_IMPLEMENTED`
//! - `winsta.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 87 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`win_station_activate_license`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments, and the arguments the C bodies never read or write:
//!   the port accepts and ignores them.
//! - `WinStationGetProcessSid`'s `FILETIME` is passed as the packed `u64` the trace prints, since
//!   the body reads it only for the trace.
//! - The `__cdecl` and `WINAPI` calling conventions and the OS last-error slot: [`Winsta`]
//!   holds the slot.
//! - Every WinStation session, server, licensing, console and process implementation: Wine has
//!   none, so each export is either an explicit `ERROR_CALL_NOT_IMPLEMENTED` body or a stub.

mod calls;
mod exports;
mod stubs;

pub use calls::{Handle, Winsta, ERROR_CALL_NOT_IMPLEMENTED};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
