/// `SERVICE_STOPPED` from `include/winsvc.h`.
pub const SERVICE_STOPPED: u32 = 0x0000_0001;
/// `SERVICE_START_PENDING` from `include/winsvc.h`.
pub const SERVICE_START_PENDING: u32 = 0x0000_0002;
/// `SERVICE_STOP_PENDING` from `include/winsvc.h`.
pub const SERVICE_STOP_PENDING: u32 = 0x0000_0003;
/// `SERVICE_RUNNING` from `include/winsvc.h`.
pub const SERVICE_RUNNING: u32 = 0x0000_0004;
/// `SERVICE_ACCEPT_STOP` from `include/winsvc.h`.
pub const SERVICE_ACCEPT_STOP: u32 = 0x0000_0001;
/// `SERVICE_ACCEPT_SHUTDOWN` from `include/winsvc.h`.
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 0x0000_0004;
/// `SERVICE_WIN32` from `include/winnt.h`: `SERVICE_WIN32_OWN_PROCESS | SERVICE_WIN32_SHARE_PROCESS`.
pub const SERVICE_WIN32: u32 = 0x0000_0010 | 0x0000_0020;

/// The `SERVICE_STATUS` record that `svc_update_status` fills in.
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

/// `svc_update_status` from `dlls/srvsvc/srvsvc.c`. The C function passes the record to
/// `SetServiceStatus`; the port returns it, and the caller records it where the C code sends it.
pub fn svc_update_status(state: u32) -> ServiceStatus {
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
