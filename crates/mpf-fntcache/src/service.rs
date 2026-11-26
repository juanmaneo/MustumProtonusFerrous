//! Port of `ServiceMain` in Wine's `dlls/fntcache/main.c`.

use crate::control::{
    service_handler, ServiceStatus, SERVICE_ACCEPT_SHUTDOWN, SERVICE_ACCEPT_STOP, SERVICE_RUNNING,
    SERVICE_STOPPED, SERVICE_WIN32,
};

/// `dwWaitHint` of the RUNNING status that `ServiceMain` publishes.
const RUNNING_WAIT_HINT: u32 = 10000;

/// The outcome of one `ServiceMain` run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceMainRun {
    /// Every `SetServiceStatus` call in order: RUNNING, then the status of each handled control, then STOPPED.
    pub statuses: Vec<ServiceStatus>,
    /// False when the wait on the stop event never ends because no STOP or SHUTDOWN control arrives.
    pub returned: bool,
}

/// `ServiceMain`. `registered` is whether `RegisterServiceCtrlHandlerExW` succeeded. `controls` are the
/// control codes delivered while the main thread waits on the stop event.
pub fn service_main(registered: bool, controls: &[u32]) -> ServiceMainRun {
    if !registered {
        return ServiceMainRun {
            statuses: Vec::new(),
            returned: true,
        };
    }
    // The C code reuses one `status` for the final write, so STOPPED keeps the RUNNING wait hint.
    let mut status = ServiceStatus {
        service_type: SERVICE_WIN32,
        current_state: SERVICE_RUNNING,
        controls_accepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: RUNNING_WAIT_HINT,
    };
    let mut statuses = vec![status];
    // The stop event is CreateEventW(NULL, TRUE, FALSE, NULL): manual-reset and initially unsignaled.
    let mut stop_event_set = false;
    for &ctrl in controls {
        let result = service_handler(ctrl);
        statuses.push(result.status);
        if result.stop_event_set {
            stop_event_set = true;
            break;
        }
    }
    if !stop_event_set {
        return ServiceMainRun {
            statuses,
            returned: false,
        };
    }
    status.current_state = SERVICE_STOPPED;
    status.controls_accepted = 0;
    statuses.push(status);
    ServiceMainRun {
        statuses,
        returned: true,
    }
}
