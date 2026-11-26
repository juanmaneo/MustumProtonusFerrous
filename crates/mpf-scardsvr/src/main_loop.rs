use crate::status::{start_status, stopped_status, ServiceStatus};

/// The name `CalaisMain` registers its control handler under, `L"scardsvr"` in the C source.
pub const SERVICE_NAME: &str = "scardsvr";

/// `INFINITE` from `include/winbase.h`.
pub const INFINITE: u32 = 0xffff_ffff;

/// One call that `CalaisMain` makes, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainEffect {
    /// `CreateEventW(NULL, TRUE, FALSE, NULL)`, which creates the manual-reset `stop_event`.
    CreateStopEvent,
    /// `RegisterServiceCtrlHandlerExW(name, service_handler, NULL)`.
    RegisterHandler { name: &'static str },
    /// `SetServiceStatus(service_handle, &status)`.
    SetStatus(ServiceStatus),
    /// `WaitForSingleObject(stop_event, timeout)`. It returns once the handler signals the event.
    WaitForStopEvent { timeout: u32 },
}

/// `CalaisMain` from `dlls/scardsvr/scardsvr.c`, as the calls it makes. `registered` says whether
/// `RegisterServiceCtrlHandlerExW` returned a handle. A NULL handle makes the C code return
/// straight after the registration.
pub fn calais_main(registered: bool) -> Vec<MainEffect> {
    let mut effects = vec![
        MainEffect::CreateStopEvent,
        MainEffect::RegisterHandler { name: SERVICE_NAME },
    ];
    if registered {
        effects.push(MainEffect::SetStatus(start_status()));
        effects.push(MainEffect::WaitForStopEvent { timeout: INFINITE });
        effects.push(MainEffect::SetStatus(stopped_status()));
    }
    effects
}
