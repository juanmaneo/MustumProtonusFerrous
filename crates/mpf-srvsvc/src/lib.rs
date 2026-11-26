//! Port of the portable logic in Wine's `dlls/srvsvc/srvsvc.c`.
//!
//! Ported:
//! - `svc_update_status`: the `SERVICE_STATUS` record it sends: [`svc_update_status`],
//!   [`ServiceStatus`], with the `SERVICE_*` state and accept values from `include/winsvc.h`
//! - `svc_handler`: control code to reported state and done-event signal: [`svc_handler`],
//!   [`ControlOutcome`], [`SERVICE_CONTROL_STOP`], [`SERVICE_CONTROL_SHUTDOWN`]
//! - `ServiceMain`: the start, run and stop sequence, with the wait on the done event modelled
//!   as a list of control codes: [`service_main`], [`ServiceRun`], [`SERVICE_NAME`]
//!
//! Skipped:
//! - The `TRACE` and `ERR` output, and the `GetLastError` value that `ERR` prints.
//! - `RegisterServiceCtrlHandlerW`, `SetServiceStatus`, `CreateEventW`, `SetEvent`,
//!   `WaitForSingleObject` and `CloseHandle` themselves. The port takes the registration result
//!   as `register_ok`, the controls as a slice, and returns the records the C code would send.
//! - The failure path of `CreateEventW`. The C code does not check the handle, so its effect
//!   depends on how `WaitForSingleObject` treats a NULL handle, which this file does not show.
//! - The file-scope `svc_handle` and `done_event` globals. Each run starts with fresh state.
//! - Control codes after the first stop or shutdown. `ServiceMain` has returned by then, so
//!   [`service_main`] does not consume them.

mod control;
mod service_main;
mod status;

pub use control::{svc_handler, ControlOutcome, SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP};
pub use service_main::{service_main, ServiceRun, SERVICE_NAME};
pub use status::{
    svc_update_status, ServiceStatus, SERVICE_ACCEPT_SHUTDOWN, SERVICE_ACCEPT_STOP,
    SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STOPPED, SERVICE_STOP_PENDING, SERVICE_WIN32,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
