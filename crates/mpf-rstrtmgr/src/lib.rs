//! Port of the portable logic in Wine's `dlls/rstrtmgr/main.c` and `dlls/rstrtmgr/rstrtmgr.spec`.
//!
//! Ported:
//! - The out-parameter writes of `RmGetList` and `RmStartSession`: [`rm_get_list`],
//!   [`rm_start_session`], with the `0xdeadbeef` session handle
//! - The `ERROR_SUCCESS` returns of the other six explicit functions: [`rm_register_resources`],
//!   [`rm_end_session`], [`rm_restart`], [`rm_shutdown`], [`rm_add_filter`], [`rm_remove_filter`]
//! - `rstrtmgr.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The four `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`rm_cancel_current_task`], [`rm_get_filter_list`], [`rm_join_session`], [`rm_reserve_heap`],
//!   [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `FIXME` traces, which the port never evaluates.
//! - The parameters that the C bodies never read or write: the session flags and key, the
//!   resource, application and service lists, the reboot reasons, the status callbacks, the
//!   filter action and the module and process arguments. The `ERROR_SUCCESS` functions do not
//!   take them.
//! - The `RmGetList` application list and the reboot reasons, which the C body never writes.
//! - The `__wine_spec_unimplemented_stub` loop: the port returns the record instead.

mod api;
mod exports;
mod stub;

pub use api::{
    rm_add_filter, rm_end_session, rm_get_list, rm_register_resources, rm_remove_filter,
    rm_restart, rm_shutdown, rm_start_session, ERROR_SUCCESS,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    rm_cancel_current_task, rm_get_filter_list, rm_join_session, rm_reserve_heap,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
