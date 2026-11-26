use crate::status::{SERVICE_RUNNING, SERVICE_STOPPED, SERVICE_STOP_PENDING};

/// `SERVICE_CONTROL_STOP` from `include/winsvc.h`.
pub const SERVICE_CONTROL_STOP: u32 = 0x0000_0001;

/// `SERVICE_CONTROL_SHUTDOWN` from `include/winsvc.h`.
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 0x0000_0005;

/// What `eventlog_handler` does for one control code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandlerAction {
    /// The state passed to `eventlog_update_status`.
    pub reported_state: u32,
    /// Whether the handler calls `SetEvent(done_event)`.
    pub signal_done: bool,
}

/// `eventlog_handler` in `dlls/wevtsvc/wevtsvc.c`. STOP and SHUTDOWN report `STOP_PENDING` and
/// signal `done_event`. Every other control reports `RUNNING` and signals nothing.
pub fn handler_action(control: u32) -> HandlerAction {
    match control {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => HandlerAction {
            reported_state: SERVICE_STOP_PENDING,
            signal_done: true,
        },
        _ => HandlerAction {
            reported_state: SERVICE_RUNNING,
            signal_done: false,
        },
    }
}

/// The states `ServiceMain` passes to `eventlog_update_status`, in order. When the control
/// handler cannot be registered it returns before reporting anything. Otherwise it reports
/// `RUNNING`, waits for `done_event`, and then reports `STOPPED`.
pub fn service_main_states(registered: bool) -> Vec<u32> {
    if registered {
        vec![SERVICE_RUNNING, SERVICE_STOPPED]
    } else {
        Vec::new()
    }
}
