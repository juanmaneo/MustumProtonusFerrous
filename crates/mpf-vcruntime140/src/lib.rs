//! Port of the portable logic in Wine's `dlls/vcruntime140/`: the `vcruntime140.spec` export table,
//! the `__wine_spec_unimplemented_stub` records of its `@ stub` exports, `misc.c`, the `_Init_thread_*`
//! once state machine in `init_thread.c`, and the allocation rules of `new.cpp`.
//!
//! Ported:
//! - `vcruntime140.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 17 `@ stub` exports, one helper each, and the `__wine_spec_unimplemented_stub` record they
//!   raise: [`wine_spec_unimplemented_stub`], [`stub_for_export`], [`StubException`] and the helpers
//! - `__uncaught_exceptions` and the `__telemetry_main_*` triggers: [`__uncaught_exceptions`],
//!   [`__telemetry_main_invoke_trigger`], [`__telemetry_main_return_trigger`]
//! - `_Init_thread_header`, `_Init_thread_footer` and `_Init_thread_abort`: [`InitThreadState`],
//!   [`HeaderOutcome`]
//! - `operator new`, its nothrow and aligned forms, and the `operator delete` forms: [`operator_new`],
//!   [`operator_new_nothrow`], [`operator_new_aligned`], [`operator_new_aligned_nothrow`],
//!   [`operator_delete`], [`operator_delete_aligned`], [`BadAlloc`], [`normalize_size`],
//!   [`normalize_alignment`], [`POINTER_SIZE`]
//!
//! Skipped:
//! - `__vcrt_InitializeCriticalSectionEx`: it returns the OS result of `InitializeCriticalSectionEx`.
//! - The 67 forwards to `ucrtbase`: this DLL has no code for them. [`ExportKind::Forward`] records them.
//! - `_Init_thread_lock`, `_Init_thread_wait` and `_Init_thread_notify`: SRW lock and condition
//!   variable calls. [`InitThreadState::header`] returns [`HeaderOutcome::Wait`] where the C loop waits.
//! - `typeinfo.c` (a vtable alias in asm) and `typeinfo_root.cpp` (glue that passes a zeroed
//!   `SLIST_HEADER` to `__std_type_info_destroy_list`).
//! - `operator new[]` and `operator delete[]`: the C bodies only delegate to the scalar operators.
//! - The `FIXME` and `TRACE` output, the calling conventions, the `-arch` and `-norelay` spec
//!   attributes, and `EXCEPTION_RECORD.ExceptionAddress`.

mod exports;
mod init_thread;
mod misc;
mod new_delete;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use init_thread::{HeaderOutcome, InitThreadState};
pub use misc::{
    __telemetry_main_invoke_trigger, __telemetry_main_return_trigger, __uncaught_exceptions,
};
pub use new_delete::{
    normalize_alignment, normalize_size, operator_delete, operator_delete_aligned, operator_new,
    operator_new_aligned, operator_new_aligned_nothrow, operator_new_nothrow, BadAlloc,
    POINTER_SIZE,
};
pub use stub::{
    __build_catch_object, __build_catch_object_helper, __c_specific_handler_noexcept,
    __frame_unwind_filter, __get_platform_exception_info, __nlg_dispatch2, __nlg_return2,
    __report_gsfailure, __type_match, __vcrt_get_module_file_name_w, __vcrt_get_module_handle_w,
    __vcrt_load_library_ex_w, _is_exception_typeof, _nlg_dispatch2, _nlg_return, _nlg_return2,
    stub_for_export, unexpected, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
