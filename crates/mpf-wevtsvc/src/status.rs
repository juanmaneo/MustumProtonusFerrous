/// `SERVICE_WIN32` from `include/winsvc.h`: `SERVICE_WIN32_OWN_PROCESS | SERVICE_WIN32_SHARE_PROCESS`.
pub const SERVICE_WIN32: u32 = 0x0000_0030;

/// `SERVICE_ACCEPT_STOP` from `include/winsvc.h`.
pub const SERVICE_ACCEPT_STOP: u32 = 0x0000_0001;

/// `SERVICE_ACCEPT_SHUTDOWN` from `include/winsvc.h`.
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 0x0000_0004;

/// `SERVICE_STOPPED` from `include/winsvc.h`.
pub const SERVICE_STOPPED: u32 = 0x0000_0001;

/// `SERVICE_STOP_PENDING` from `include/winsvc.h`.
pub const SERVICE_STOP_PENDING: u32 = 0x0000_0003;

/// `SERVICE_RUNNING` from `include/winsvc.h`.
pub const SERVICE_RUNNING: u32 = 0x0000_0004;

/// The `SERVICE_STATUS` record that `eventlog_update_status` fills in.
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

/// `eventlog_update_status` in `dlls/wevtsvc/wevtsvc.c`: the record it passes to
/// `SetServiceStatus` for `state`. Only `current_state` depends on the argument.
pub fn service_status(state: u32) -> ServiceStatus {
    ServiceStatus {
        service_type: SERVICE_WIN32,
        current_state: state,
        controls_accepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }
}
