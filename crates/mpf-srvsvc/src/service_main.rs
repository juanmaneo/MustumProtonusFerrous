use crate::control::svc_handler;
use crate::status::{
    svc_update_status, ServiceStatus, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STOPPED,
};

/// The name `ServiceMain` passes to `RegisterServiceCtrlHandlerW`.
pub const SERVICE_NAME: &str = "LanmanServer";

/// One run of `ServiceMain`.
///
/// `statuses` are the records sent with `SetServiceStatus`, in order. `returned` is false when
/// the run is still blocked in `WaitForSingleObject(done_event, INFINITE)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceRun {
    pub registered_name: &'static str,
    pub statuses: Vec<ServiceStatus>,
    pub returned: bool,
}

/// `ServiceMain` from `dlls/srvsvc/srvsvc.c`. `register_ok` is whether
/// `RegisterServiceCtrlHandlerW` returned a handle. `controls` are the control codes delivered
/// to the handler while the run waits, in order. The run ends at the first stop or shutdown.
pub fn service_main(register_ok: bool, controls: &[u32]) -> ServiceRun {
    let mut statuses = Vec::new();
    if !register_ok {
        return ServiceRun {
            registered_name: SERVICE_NAME,
            statuses,
            returned: true,
        };
    }

    statuses.push(svc_update_status(SERVICE_START_PENDING));
    statuses.push(svc_update_status(SERVICE_RUNNING));
    for &control in controls {
        let outcome = svc_handler(control);
        statuses.push(outcome.status);
        if outcome.set_done_event {
            statuses.push(svc_update_status(SERVICE_STOPPED));
            return ServiceRun {
                registered_name: SERVICE_NAME,
                statuses,
                returned: true,
            };
        }
    }
    ServiceRun {
        registered_name: SERVICE_NAME,
        statuses,
        returned: false,
    }
}
