/// `SERVICE_WIN32` from `include/winsvc.h`.
pub const SERVICE_WIN32: u32 = 0x0000_0010;

/// `SERVICE_STOPPED` from `include/winsvc.h`.
pub const SERVICE_STOPPED: u32 = 0x0000_0001;

/// `SERVICE_STOP_PENDING` from `include/winsvc.h`.
pub const SERVICE_STOP_PENDING: u32 = 0x0000_0003;

/// `SERVICE_RUNNING` from `include/winsvc.h`.
pub const SERVICE_RUNNING: u32 = 0x0000_0004;

/// `SERVICE_ACCEPT_STOP` from `include/winsvc.h`.
pub const SERVICE_ACCEPT_STOP: u32 = 0x0000_0001;

/// `SERVICE_ACCEPT_SHUTDOWN` from `include/winsvc.h`.
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 0x0000_0004;

/// The `wait_hint` that `CalaisMain` sets in its start status.
const START_WAIT_HINT: u32 = 10000;

/// `SERVICE_STATUS` from `include/winsvc.h`, with every field the C code sets.
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

/// The status `CalaisMain` reports once its control handler is registered.
pub fn start_status() -> ServiceStatus {
    ServiceStatus {
        service_type: SERVICE_WIN32,
        current_state: SERVICE_RUNNING,
        controls_accepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: START_WAIT_HINT,
    }
}

/// The status `CalaisMain` reports after the stop event. The C code changes only the state and
/// the accepted controls on the start status, so `wait_hint` stays at 10000.
pub fn stopped_status() -> ServiceStatus {
    ServiceStatus {
        current_state: SERVICE_STOPPED,
        controls_accepted: 0,
        ..start_status()
    }
}
