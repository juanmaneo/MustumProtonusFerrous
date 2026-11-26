//! Port of the portable logic in Wine's `dlls/vdmdbg/vdmdbg.c` and `dlls/vdmdbg/vdmdbg.spec`.
//!
//! Ported:
//! - `VDMEnumProcessWOW` and `VDMEnumTaskWOW` (`@ stdcall`), which return 0 without calling
//!   their callback: [`vdm_enum_process_wow`], [`vdm_enum_task_wow`]
//! - `vdmdbg.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 17 `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per stub export
//!
//! Skipped:
//! - The `FIXME` traces of the arguments, and the `fp`, `lparam` and `dwProcessId` parameters,
//!   which the C bodies never read or call.
//! - The `WINAPI` calling convention.

mod exports;
mod stub;
mod wow;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    vdm_break_thread, vdm_detect_wow, vdm_enum_task_wow_ex, vdm_get_module_selector,
    vdm_get_pointer, vdm_get_selector_module, vdm_get_thread_context,
    vdm_get_thread_selector_entry, vdm_global_first, vdm_global_next, vdm_kill_wow,
    vdm_module_first, vdm_module_next, vdm_process_exception, vdm_set_thread_context,
    vdm_start_task_in_wow, vdm_terminate_task_wow, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use wow::{vdm_enum_process_wow, vdm_enum_task_wow};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
