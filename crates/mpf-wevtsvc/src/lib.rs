//! Port of the portable logic in Wine's `dlls/wevtsvc/wevtsvc.c` and `dlls/wevtsvc/wevtsvc.spec`.
//!
//! Ported:
//! - `eventlog_update_status`: the `SERVICE_STATUS` record and the `SERVICE_*` state values,
//!   as [`service_status`], [`ServiceStatus`] and the `SERVICE_*` constants
//! - `eventlog_handler`: the control dispatch, as [`handler_action`], [`HandlerAction`] and the
//!   `SERVICE_CONTROL_*` values
//! - `ServiceMain`: the sequence of states it reports, as [`service_main_states`]
//! - `wevtsvc.spec`: the only entry is `@ stdcall -private ServiceMain(long ptr)`, so there is no
//!   public export table and no `@ stub` export to model
//!
//! Skipped:
//! - The Win32 calls `ServiceMain` makes: `RegisterServiceCtrlHandlerW`, `SetServiceStatus`,
//!   `CreateEventW`, `WaitForSingleObject` (the wait on `done_event` is assumed to return once a
//!   stop control arrives, as the handler's `SetEvent` does), and `CloseHandle`.
//! - The `TRACE` and `ERR` output, the `GetLastError` value in the error path, and the
//!   `svc_handle` and `done_event` statics.
//! - The `argc` and `argv` parameters, which the C body never reads.

mod control;
mod status;

pub use control::{
    handler_action, service_main_states, HandlerAction, SERVICE_CONTROL_SHUTDOWN,
    SERVICE_CONTROL_STOP,
};
pub use status::{
    service_status, ServiceStatus, SERVICE_ACCEPT_SHUTDOWN, SERVICE_ACCEPT_STOP, SERVICE_RUNNING,
    SERVICE_STOPPED, SERVICE_STOP_PENDING, SERVICE_WIN32,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
