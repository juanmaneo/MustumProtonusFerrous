/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// A handle or pointer argument. `HANDLE`, `HWND`, `PVOID`, `PSESSIONIDW` and
/// `PTS_ALL_PROCESSES_INFO` are pointers in C; the bodies never dereference them, so the port
/// treats them as opaque integers.
pub type Handle = usize;

/// The thread's last-error slot that the explicit `winsta` functions write to.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Winsta {
    last_error: u32,
}

impl Winsta {
    /// A slot that starts at `last_error`, like a thread whose earlier call set that value.
    pub fn with_last_error(last_error: u32) -> Self {
        Self { last_error }
    }

    /// The value of the last error after the most recent call.
    pub fn last_error(&self) -> u32 {
        self.last_error
    }

    /// `WinStationQueryInformationA`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns `FALSE`. The
    /// info buffer and the `ret_len` out-parameter are never read or written.
    pub fn win_station_query_information_a(
        &mut self,
        _server: Handle,
        _logon_id: u32,
        _class: u32,
        _info: Handle,
        _len: u32,
        _ret_len: Option<&mut u32>,
    ) -> u8 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WinStationQueryInformationW`: the same body as the `A` variant.
    pub fn win_station_query_information_w(
        &mut self,
        _server: Handle,
        _logon_id: u32,
        _class: u32,
        _info: Handle,
        _len: u32,
        _ret_len: Option<&mut u32>,
    ) -> u8 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WinStationRegisterConsoleNotification`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns
    /// `FALSE`.
    pub fn win_station_register_console_notification(
        &mut self,
        _server: Handle,
        _hwnd: Handle,
        _flags: u32,
    ) -> u8 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WinStationUnRegisterConsoleNotification`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns
    /// `FALSE`.
    pub fn win_station_un_register_console_notification(
        &mut self,
        _server: Handle,
        _hwnd: Handle,
    ) -> u8 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WinStationGetAllProcesses`: writes `0` to `*process_count`, sets
    /// `ERROR_CALL_NOT_IMPLEMENTED` and returns `FALSE`. The C body writes the count
    /// unconditionally, so the port takes a `&mut u32`; a NULL count would fault in C.
    pub fn win_station_get_all_processes(
        &mut self,
        _server: Handle,
        _level: u32,
        process_count: &mut u32,
        _info: Handle,
    ) -> u8 {
        *process_count = 0;
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WinStationGetProcessSid`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns `FALSE`. The
    /// `process_start_time` is the packed `FILETIME` the trace prints, and the SID buffer and
    /// `sid_size` are never read or written.
    pub fn win_station_get_process_sid(
        &mut self,
        _server: Handle,
        _process_id: u32,
        _process_start_time: u64,
        _process_user_sid: Handle,
        _sid_size: Option<&mut u32>,
    ) -> u8 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WinStationVirtualOpen`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns `FALSE`. Its
    /// `BOOL` result is an `i32`, as in C.
    pub fn win_station_virtual_open(&mut self, _a: Handle, _b: Handle, _c: Handle) -> i32 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WinStationEnumerateW`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns `FALSE`. The
    /// session list and the `count` out-parameter are never read or written.
    pub fn win_station_enumerate_w(
        &mut self,
        _server: Handle,
        _sessionids: Handle,
        _count: Option<&mut u32>,
    ) -> u8 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }
}
