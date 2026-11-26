//! Port of the portable logic in Wine's `dlls/avrt/main.c` and the `@ stub`
//! exports of `dlls/avrt/avrt.spec`.
//!
//! Ported:
//! - Argument checks, error codes and the fixed handle of
//!   `AvSetMmThreadCharacteristicsW` and `AvSetMmMaxThreadCharacteristicsW`:
//!   [`av_set_mm_thread_characteristics_w`], [`av_set_mm_max_thread_characteristics_w`],
//!   [`AvrtError`], [`AVRT_STUB_HANDLE`]
//! - NULL and conversion-failure handling of the `...A` wrappers around `strdupAW`:
//!   [`av_set_mm_thread_characteristics_a`], [`av_set_mm_max_thread_characteristics_a`],
//!   [`AnsiArg`]
//! - Stub return values: [`av_query_system_responsiveness`] (FALSE),
//!   [`av_revert_mm_thread_characteristics`] (TRUE), [`av_set_mm_thread_priority`] (TRUE)
//! - The seven `@ stub` ordering-group exports. They never return: the generated stub
//!   calls `__wine_spec_unimplemented_stub`, which raises a non-continuable
//!   `EXCEPTION_WINE_STUB`. Modelled as [`StubRaise`] values from [`unimplemented_stub`]
//!   and the `av_rt_*` functions.
//!
//! Skipped:
//! - `MultiByteToWideChar(CP_ACP, ...)` in `strdupAW`: code-page conversion is OS behaviour.
//!   The wrappers take its result as [`AnsiArg`].
//! - `malloc` in `strdupAW`: only its failure is modelled, as [`AnsiArg::OutOfMemory`].
//! - `FIXME` tracing and `debugstr_w`.
//! - The module-name argument of `__wine_spec_unimplemented_stub`: it is the build's spec file name.

mod error;
mod ordering;
mod stubs;
mod thread;

pub use error::AvrtError;
pub use ordering::{
    av_rt_create_thread_ordering_group, av_rt_create_thread_ordering_group_ex_a,
    av_rt_create_thread_ordering_group_ex_w, av_rt_delete_thread_ordering_group,
    av_rt_join_thread_ordering_group, av_rt_leave_thread_ordering_group,
    av_rt_wait_on_thread_ordering_group, unimplemented_stub, StubRaise,
};
pub use stubs::{
    av_query_system_responsiveness, av_revert_mm_thread_characteristics, av_set_mm_thread_priority,
};
pub use thread::{
    av_set_mm_max_thread_characteristics_a, av_set_mm_max_thread_characteristics_w,
    av_set_mm_thread_characteristics_a, av_set_mm_thread_characteristics_w, AnsiArg,
    AVRT_STUB_HANDLE,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
