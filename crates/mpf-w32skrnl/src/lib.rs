//! Port of the portable logic in Wine's `dlls/w32skrnl/w32skernel.c` and
//! `dlls/w32skrnl/w32skrnl.spec`.
//!
//! Ported:
//! - `GetWin32sDirectory` string concatenation: [`get_win32s_directory`], [`WIN32S_SUFFIX`]
//! - `w32skrnl.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per stub export.
//!
//! Skipped:
//! - `GetCurrentTask32`: calls `GetCurrentTask()` and returns an OS task handle.
//! - `GetWin32sDirectory`'s `GetSystemDirectoryA` and `HeapAlloc` calls: the port takes the
//!   system directory as an argument and returns the concatenated string.
//! - The forwarded exports `W32S_BackTo32` and `_RtlNtStatusToDosError`, which Wine forwards
//!   to `kernel32` and `ntdll` instead of implementing them here.

mod exports;
mod stub;
mod win32s;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::*;
pub use win32s::{get_win32s_directory, WIN32S_SUFFIX};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
