use crate::status::{
    ServiceStatus, SERVICE_ACCEPT_STOP, SERVICE_RUNNING, SERVICE_STOP_PENDING, SERVICE_WIN32,
};

/// `SERVICE_CONTROL_STOP` from `include/winsvc.h`.
pub const SERVICE_CONTROL_STOP: u32 = 0x0000_0001;

/// `SERVICE_CONTROL_SHUTDOWN` from `include/winsvc.h`.
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 0x0000_0005;

/// `ERROR_SUCCESS` from `include/winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;

/// What `service_handler` does for one control code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandlerOutcome {
    /// The status passed to `SetServiceStatus`.
    pub status: ServiceStatus,
    /// Whether the handler signals `stop_event` with `SetEvent`.
    pub sets_stop_event: bool,
    /// The value the handler returns.
    pub ret: u32,
}

/// `service_handler` from `dlls/scardsvr/scardsvr.c`. Stop and shutdown move the service to
/// `SERVICE_STOP_PENDING`, accept no further controls and signal the stop event. Every other
/// control reports `SERVICE_RUNNING` and still accepts `SERVICE_ACCEPT_STOP`.
pub fn service_handler(ctrl: u32) -> HandlerOutcome {
    match ctrl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => HandlerOutcome {
            status: handler_status(SERVICE_STOP_PENDING, 0),
            sets_stop_event: true,
            ret: ERROR_SUCCESS,
        },
        _ => HandlerOutcome {
            status: handler_status(SERVICE_RUNNING, SERVICE_ACCEPT_STOP),
            sets_stop_event: false,
            ret: ERROR_SUCCESS,
        },
    }
}

/// The `SERVICE_STATUS` the handler builds. Its exit codes, checkpoint and wait hint stay zero.
fn handler_status(current_state: u32, controls_accepted: u32) -> ServiceStatus {
    ServiceStatus {
        service_type: SERVICE_WIN32,
        current_state,
        controls_accepted,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }
}
