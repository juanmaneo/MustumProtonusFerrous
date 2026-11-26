//! Port of the portable logic in Wine's `dlls/scardsvr/scardsvr.c` and `dlls/scardsvr/scardsvr.spec`.
//!
//! Ported:
//! - `service_handler` control dispatch and the `SERVICE_STATUS` it reports: [`service_handler`],
//!   [`HandlerOutcome`], with the `SERVICE_CONTROL_*` and `ERROR_SUCCESS` values from the Wine headers
//! - `CalaisMain` call sequence: [`calais_main`], [`MainEffect`], [`SERVICE_NAME`], [`INFINITE`],
//!   [`start_status`] and [`stopped_status`]
//! - `scardsvr.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `InitSmartCardService` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`init_smart_card_service`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `TRACE` and `FIXME` output, the `WINE_DEFAULT_DEBUG_CHANNEL` declaration, and the `argc`
//!   and `argv` parameters of `CalaisMain`, which the C body never reads.
//! - The handles and the Win32 calls themselves: `stop_event`, `service_handle`, the handler
//!   function pointer and its `event_type`, `event_data` and `context` parameters. [`MainEffect`]
//!   models the order and the arguments of the calls.
//! - The `# @ stub SvchostPushServiceGlobals` line, which the spec file comments out.
//! - The `WINAPI` and `__stdcall` calling conventions.

mod exports;
mod handler;
mod main_loop;
mod status;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use handler::{
    service_handler, HandlerOutcome, ERROR_SUCCESS, SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP,
};
pub use main_loop::{calais_main, MainEffect, INFINITE, SERVICE_NAME};
pub use status::{
    start_status, stopped_status, ServiceStatus, SERVICE_ACCEPT_SHUTDOWN, SERVICE_ACCEPT_STOP,
    SERVICE_RUNNING, SERVICE_STOPPED, SERVICE_STOP_PENDING, SERVICE_WIN32,
};
pub use stub::{
    init_smart_card_service, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
