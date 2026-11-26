//! Port of `service_handler` in Wine's `dlls/fntcache/main.c`.

/// `SERVICE_WIN32`: `SERVICE_WIN32_OWN_PROCESS | SERVICE_WIN32_SHARE_PROCESS` from `winnt.h`.
pub const SERVICE_WIN32: u32 = 0x0000_0030;
pub const SERVICE_STOPPED: u32 = 0x0000_0001;
pub const SERVICE_STOP_PENDING: u32 = 0x0000_0003;
pub const SERVICE_RUNNING: u32 = 0x0000_0004;
pub const SERVICE_ACCEPT_STOP: u32 = 0x0000_0001;
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 0x0000_0004;
pub const SERVICE_CONTROL_STOP: u32 = 0x0000_0001;
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 0x0000_0005;
/// `NO_ERROR` from `winerror.h`.
pub const NO_ERROR: u32 = 0;

/// The `SERVICE_STATUS` fields, in declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ServiceStatus {
    pub service_type: u32,
    pub current_state: u32,
    pub controls_accepted: u32,
    pub win32_exit_code: u32,
    pub service_specific_exit_code: u32,
    pub check_point: u32,
    pub wait_hint: u32,
}

/// What `service_handler` does besides returning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandlerResult {
    /// The status passed to `SetServiceStatus`.
    pub status: ServiceStatus,
    /// Whether the handler calls `SetEvent(stop_event)`.
    pub stop_event_set: bool,
    /// The handler's return value.
    pub ret: u32,
}

/// `service_handler`. Builds a fresh status on every call.
pub fn service_handler(ctrl: u32) -> HandlerResult {
    let mut status = ServiceStatus {
        service_type: SERVICE_WIN32,
        current_state: 0,
        // The default branch keeps this STOP-only value, not the STOP|SHUTDOWN that ServiceMain advertises.
        controls_accepted: SERVICE_ACCEPT_STOP,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    };
    let stop_event_set = match ctrl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            status.current_state = SERVICE_STOP_PENDING;
            status.controls_accepted = 0;
            true
        }
        _ => {
            status.current_state = SERVICE_RUNNING;
            false
        }
    };
    HandlerResult {
        status,
        stop_event_set,
        ret: NO_ERROR,
    }
}
