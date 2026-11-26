/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// A handle or pointer argument. `HANDLE`, `FARPROC` and `PVOID` are pointers in C; the bodies
/// only log them, so the port treats them as opaque integers.
pub type Handle = usize;

/// `NULL` handle, the value `WIMCreateFile` returns.
pub const NULL_HANDLE: Handle = 0;

/// The thread's last-error slot that the explicit `wimgapi` functions write to.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Wimgapi {
    last_error: u32,
}

impl Wimgapi {
    /// A slot that starts at `last_error`, like a thread whose earlier call set that value.
    pub fn with_last_error(last_error: u32) -> Self {
        Self { last_error }
    }

    /// The value of the last error after the most recent call.
    pub fn last_error(&self) -> u32 {
        self.last_error
    }

    /// `WIMRegisterMessageCallback`: returns 0. Unlike the other two bodies it does not call
    /// `SetLastError`, so the last error is left as it was.
    pub fn wim_register_message_callback(
        &mut self,
        _wim: Handle,
        _callback: Handle,
        _data: Handle,
    ) -> u32 {
        0
    }

    /// `WIMGetMountedImages`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns `FALSE` (0). The
    /// list and the length are never read or written.
    pub fn wim_get_mounted_images(&mut self, _list: Handle, _length: Option<&mut u32>) -> i32 {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        0
    }

    /// `WIMCreateFile`: sets `ERROR_CALL_NOT_IMPLEMENTED` and returns `NULL`. The path and the
    /// `result` out-parameter are never read or written.
    pub fn wim_create_file(
        &mut self,
        _path: Option<&[u16]>,
        _access: u32,
        _creation: u32,
        _flags: u32,
        _compression: u32,
        _result: Option<&mut u32>,
    ) -> Handle {
        self.last_error = ERROR_CALL_NOT_IMPLEMENTED;
        NULL_HANDLE
    }
}
