//! Port of the portable logic in Wine's `dlls/wimgapi/main.c` and `dlls/wimgapi/wimgapi.spec`.
//!
//! Ported:
//! - `WIMRegisterMessageCallback`, `WIMGetMountedImages` and `WIMCreateFile`: the stub bodies
//!   (return value and last error): [`Wimgapi`], with `ERROR_CALL_NOT_IMPLEMENTED`
//! - `wimgapi.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 37 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`STUB_EXPORTS`], [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per
//!   export such as [`wim_apply_image`]
//!
//! Skipped:
//! - The `FIXME` traces of the arguments, and the arguments themselves: the C bodies never read
//!   them, so the port accepts and ignores them.
//! - `DllCanUnloadNow` and `DllMain` (`-private`, no body in `main.c`).
//! - The WIM image, mount and file I/O implementation: Wine has none.
//! - The `__cdecl` calling convention and the OS last-error slot: [`Wimgapi`] holds the slot.

mod calls;
mod exports;
mod stubs;

pub use calls::{Handle, Wimgapi, ERROR_CALL_NOT_IMPLEMENTED, NULL_HANDLE};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
