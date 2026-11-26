use crate::status::{svc_update_status, ServiceStatus, SERVICE_RUNNING, SERVICE_STOP_PENDING};

/// `SERVICE_CONTROL_STOP` from `include/winsvc.h`.
pub const SERVICE_CONTROL_STOP: u32 = 0x0000_0001;
/// `SERVICE_CONTROL_SHUTDOWN` from `include/winsvc.h`.
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 0x0000_0005;

/// What `svc_handler` does for one control code: the status it reports, and whether it signals
/// the done event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControlOutcome {
    pub status: ServiceStatus,
    pub set_done_event: bool,
}

/// `svc_handler` from `dlls/srvsvc/srvsvc.c`. Stop and shutdown report `SERVICE_STOP_PENDING`
/// and then signal the done event. Every other control, including unknown codes, reports
/// `SERVICE_RUNNING`.
pub fn svc_handler(control: u32) -> ControlOutcome {
    match control {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => ControlOutcome {
            status: svc_update_status(SERVICE_STOP_PENDING),
            set_done_event: true,
        },
        _ => ControlOutcome {
            status: svc_update_status(SERVICE_RUNNING),
            set_done_event: false,
        },
    }
}
