//! Port of the portable logic in Wine's `dlls/srclient/srclient.spec` and `dlls/srclient/Makefile.in`.
//!
//! `dlls/srclient` has no `.c`, `.idl`, `.rc` or `.tlb` source, so this port is partial: it models
//! what the DLL defines, and it has no logic of its own.
//!
//! Ported:
//! - `srclient.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`], with the ordinals
//!   that `winebuild` assigns (`assign_ordinals` in `tools/winebuild/parser.c`)
//! - the eleven `@ stub` exports and the `__wine_spec_unimplemented_stub` record each raises:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, such as
//!   [`sysprep_cleanup`]
//! - the `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE` values
//! - the `MODULE = srclient.dll` file name that `winebuild` passes to every stub
//!
//! Skipped:
//! - The `@ stdcall` forwards `SRSetRestorePointA` and `SRSetRestorePointW` to `sfc_os`. Their code
//!   is in sfc_os, so the port records the forward targets and does not implement them.
//! - The `__cdecl` calling convention and the assembly that `winebuild` generates for each stub.
//! - The `for (;;)` loop in `__wine_spec_unimplemented_stub`: the port returns the record instead.
//! - The ordinal-as-function form of the record. Every `@ stub` in `srclient.spec` has a name, so
//!   the function argument is always the export name.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    disable_sr, disable_sr_internal, enable_sr, enable_sr_ex, enable_sr_internal,
    set_sr_state_after_setup, sr_new_system_id, sr_remove_restore_point,
    sr_set_restore_point_internal, sysprep_cleanup, sysprep_generalize,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
