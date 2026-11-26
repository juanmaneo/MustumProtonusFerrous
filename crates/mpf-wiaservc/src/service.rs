//! The service entry point from `dlls/wiaservc/service.c`: `ServiceMain`, `UpdateStatus`,
//! `ServiceHandler` and `StartCount`. The Win32 and COM calls go through [`ServiceHost`].

use crate::com::{failed, Guid, Hresult, CLSID_WIADEVMGR};

pub(crate) const SERVICE_STOPPED: u32 = 1;
pub(crate) const SERVICE_START_PENDING: u32 = 2;
pub(crate) const SERVICE_STOP_PENDING: u32 = 3;
pub(crate) const SERVICE_RUNNING: u32 = 4;
pub(crate) const SERVICE_WIN32_OWN_PROCESS: u32 = 0x10;
pub(crate) const SERVICE_ACCEPT_STOP: u32 = 0x1;
pub(crate) const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 0x2;
pub(crate) const SERVICE_ACCEPT_SHUTDOWN: u32 = 0x4;
pub(crate) const SERVICE_CONTROL_STOP: u32 = 1;
pub(crate) const SERVICE_CONTROL_SHUTDOWN: u32 = 5;
pub(crate) const NO_ERROR: u32 = 0;
/// The wait hint `ServiceMain` passes with `SERVICE_START_PENDING`.
pub(crate) const SERVICE_START_WAIT_HINT: u32 = 3000;

/// `SERVICE_STATUS`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ServiceStatus {
    pub service_type: u32,
    pub current_state: u32,
    pub controls_accepted: u32,
    pub win32_exit_code: u32,
    pub service_specific_exit_code: u32,
    pub check_point: u32,
    pub wait_hint: u32,
}

/// The Win32 and COM calls that `service.c` makes. Each returns the value the C call returns:
/// a `BOOL` as `bool`, an `HRESULT` as [`Hresult`].
pub trait ServiceHost {
    /// `CreateEventW(NULL, TRUE, FALSE, NULL)`.
    fn create_stop_event(&mut self) -> bool;
    /// `RegisterServiceCtrlHandlerExW(L"stisvc", ServiceHandler, NULL)`.
    fn register_service_ctrl_handler(&mut self) -> bool;
    /// `SetServiceStatus(status_handle, &status)`.
    fn set_service_status(&mut self, status: &ServiceStatus) -> bool;
    /// `SetEvent(stop_event)`.
    fn set_stop_event(&mut self);
    /// `CoInitializeEx(NULL, COINIT_MULTITHREADED)`.
    fn co_initialize_ex(&mut self) -> Hresult;
    /// `CoInitializeSecurity(NULL, -1, ...)`.
    fn co_initialize_security(&mut self) -> Hresult;
    /// `CoRegisterClassObject(clsid, WIASERVC_ClassFactory, CLSCTX_LOCAL_SERVER, REGCLS_MULTIPLEUSE, &dwReg)`.
    fn co_register_class_object(&mut self, clsid: &Guid) -> Hresult;
    /// `WaitForSingleObject(stop_event, INFINITE)`.
    fn wait_for_stop_event(&mut self);
    /// `CoRevokeClassObject(dwReg)`.
    fn co_revoke_class_object(&mut self);
    /// `CloseHandle(stop_event)`.
    fn close_stop_event(&mut self);
    /// `CoUninitialize()`.
    fn co_uninitialize(&mut self);
}

/// The service state that `service.c` keeps in its file-scope `status` and `stop_event` globals.
#[derive(Debug)]
pub struct Service<H: ServiceHost> {
    host: H,
    status: ServiceStatus,
}

impl<H: ServiceHost> Service<H> {
    pub fn new(host: H) -> Self {
        Self {
            host,
            status: ServiceStatus::default(),
        }
    }

    pub fn host(&self) -> &H {
        &self.host
    }

    pub fn host_mut(&mut self) -> &mut H {
        &mut self.host
    }

    pub fn status(&self) -> &ServiceStatus {
        &self.status
    }

    /// `UpdateStatus`. The C body ignores `dwWin32ExitCode` and always writes 0.
    pub fn update_status(&mut self, current_state: u32, _win32_exit_code: u32, wait_hint: u32) {
        let controls_accepted = if current_state == SERVICE_START_PENDING {
            0
        } else {
            SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_PAUSE_CONTINUE | SERVICE_ACCEPT_SHUTDOWN
        };
        self.status = ServiceStatus {
            service_type: SERVICE_WIN32_OWN_PROCESS,
            current_state,
            controls_accepted,
            win32_exit_code: 0,
            service_specific_exit_code: 0,
            check_point: 0,
            wait_hint,
        };
        if !self.host.set_service_status(&self.status) {
            self.host.set_stop_event();
        }
    }

    /// `ServiceHandler`. The C body ignores the event type, event data and context, and always
    /// returns `NO_ERROR`.
    pub fn service_handler(&mut self, ctrl: u32) -> u32 {
        match ctrl {
            SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
                self.update_status(SERVICE_STOP_PENDING, NO_ERROR, 0);
                self.host.set_stop_event();
            }
            _ => self.update_status(self.status.current_state, NO_ERROR, 0),
        }
        NO_ERROR
    }

    /// `StartCount`: returns false at the first failed COM call.
    fn start_count(&mut self) -> bool {
        if failed(self.host.co_initialize_ex()) {
            return false;
        }
        if failed(self.host.co_initialize_security()) {
            return false;
        }
        !failed(self.host.co_register_class_object(&CLSID_WIADEVMGR))
    }

    /// `ServiceMain`. Each failure returns without the cleanup that follows it, as in the C body.
    /// The `dwArgc` and `lpszArgv` arguments are never read.
    pub fn service_main(&mut self) {
        if !self.host.create_stop_event() {
            return;
        }
        if !self.host.register_service_ctrl_handler() {
            return;
        }
        self.update_status(SERVICE_START_PENDING, NO_ERROR, SERVICE_START_WAIT_HINT);
        if !self.start_count() {
            self.update_status(SERVICE_STOPPED, NO_ERROR, 0);
            return;
        }
        self.update_status(SERVICE_RUNNING, NO_ERROR, 0);
        self.host.wait_for_stop_event();
        self.host.co_revoke_class_object();
        self.update_status(SERVICE_STOPPED, NO_ERROR, 0);
        self.host.close_stop_event();
        self.host.co_uninitialize();
    }
}
