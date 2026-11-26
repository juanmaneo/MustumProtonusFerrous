//! Port of the portable logic in Wine's `dlls/msimg32/msimg32_main.c` and `dlls/msimg32/msimg32.spec`.
//!
//! Ported:
//! - `vSetDdrawflag` (`1 stdcall`, ordinal 1): [`v_set_ddrawflag`], which leaves the thread's
//!   last error at [`ERROR_CALL_NOT_IMPLEMENTED`]
//! - `DllInitialize` (`-private`, implemented by `DllMain`): [`dll_main`], [`DllMainOutcome`] and
//!   [`DLL_PROCESS_ATTACH`], from the default `libs/winecrt0/dll_main.c` that msimg32 links
//! - `msimg32.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - The `FIXME` trace and the function-local `vDrawflag` static, which only the trace reads.
//! - The bodies of `AlphaBlend`, `GradientFill` and `TransparentBlt`. The spec forwards them to
//!   `gdi32`, so their code is not in msimg32.
//! - The `reserved` argument of `DllMain`, which the C body never reads.
//! - The `@ stub` model: `msimg32.spec` has no `@ stub` export.

mod ddrawflag;
mod entry;
mod exports;

pub use ddrawflag::{v_set_ddrawflag, ERROR_CALL_NOT_IMPLEMENTED};
pub use entry::{dll_main, DllMainOutcome, DLL_PROCESS_ATTACH};
pub use exports::{export_kind, ExportKind, EXPORTS};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
