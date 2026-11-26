//! Port of the portable logic in Wine's `dlls/fntcache/main.c` and `fntcache.spec`.
//!
//! Ported:
//! - `service_handler` (control code to `SERVICE_STATUS` and stop-event request):
//!   [`service_handler`], [`HandlerResult`], [`ServiceStatus`]
//! - `ServiceMain` status sequence and its wait on the stop event: [`service_main`], [`ServiceMainRun`]
//! - `SERVICE_*` and `NO_ERROR` constants from `winsvc.h`, `winnt.h` and `winerror.h`
//! - Export binding of the `fntcache.spec` entries against `main.c`: [`SPEC_EXPORTS`], [`defined_in_main_c`]
//!
//! Skipped:
//! - `CreateEventW`, `RegisterServiceCtrlHandlerExW`, `SetServiceStatus`, `SetEvent`, `WaitForSingleObject`:
//!   OS calls. The port takes the registration result and the control stream as arguments.
//! - `service_handle` and `stop_event` statics: state is passed explicitly instead.
//! - `TRACE` and `FIXME` logging, and the `WINE_DEFAULT_DEBUG_CHANNEL` declaration.
//! - `DllRegisterServer` and `DllUnregisterServer`: declared in the spec but never defined in `main.c`,
//!   so the reference has no behaviour or return value to port. See [`defined_in_main_c`].
//! - `main.c` never closes the stop event handle, so there is no handle lifetime to model.

mod control;
mod exports;
mod service;

pub use control::{
    service_handler, HandlerResult, ServiceStatus, NO_ERROR, SERVICE_ACCEPT_SHUTDOWN,
    SERVICE_ACCEPT_STOP, SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP, SERVICE_RUNNING,
    SERVICE_STOPPED, SERVICE_STOP_PENDING, SERVICE_WIN32,
};
pub use exports::{defined_in_main_c, SPEC_EXPORTS};
pub use service::{service_main, ServiceMainRun};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
