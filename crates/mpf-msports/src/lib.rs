//! Port of the portable logic in Wine's `dlls/msports/msports.c` and `dlls/msports/msports.spec`.
//!
//! Ported:
//! - `ComDBOpen` and `ComDBClose`, the two explicit C functions: [`com_db_open`] and
//!   [`com_db_close`], with [`HCOMDB`], [`INVALID_HANDLE_VALUE`], [`ERROR_ACCESS_DENIED`] and
//!   [`ERROR_INVALID_PARAMETER`]
//! - `msports.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The nine `@ stub` exports, each with a helper that raises the record:
//!   [`wine_spec_unimplemented_stub`], [`StubException`] and the `*` helpers in `stub.rs`
//!
//! Skipped:
//! - The `FIXME` trace in both C functions.
//! - The `HCOMDB` argument of `ComDBClose`, which the C body never reads.
//! - `ComDBOpen(NULL)`: the C body stores through the pointer and would crash. `&mut` cannot be
//!   null, so the port has no such call.
//! - The `WINAPI` and `__cdecl` calling conventions.
//! - The ordinal parameter of `__wine_spec_unimplemented_stub`: every `msports.spec` export has a
//!   name, so `winebuild` never passes an ordinal.

mod comdb;
mod exports;
mod stub;

pub use comdb::{
    com_db_close, com_db_open, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER, HCOMDB,
    INVALID_HANDLE_VALUE,
};
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    com_db_claim_next_free_port, com_db_claim_port, com_db_get_current_port_usage,
    com_db_release_port, com_db_resize_database, parallel_port_prop_page_provider,
    ports_class_installer, serial_display_advanced_settings, serial_port_prop_page_provider,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
