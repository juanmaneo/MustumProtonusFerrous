/// `HANDLE`, at pointer width. `INVALID_HANDLE_VALUE` is all ones at that width.
pub type Handle = usize;

/// `INVALID_HANDLE_VALUE`, `((HANDLE)(LONG_PTR)-1)` from `include/winbase.h`.
pub const INVALID_HANDLE_VALUE: Handle = usize::MAX;

/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `TcRegisterClient` from `dlls/traffic/traffic_main.c`. A non-NULL `buffer` receives
/// `INVALID_HANDLE_VALUE`, and the call returns `ERROR_CALL_NOT_IMPLEMENTED`.
pub fn tc_register_client(buffer: Option<&mut Handle>) -> u32 {
    if let Some(buffer) = buffer {
        *buffer = INVALID_HANDLE_VALUE;
    }
    ERROR_CALL_NOT_IMPLEMENTED
}

/// `TcDeregisterClient` from `dlls/traffic/traffic_main.c`. It always returns
/// `ERROR_CALL_NOT_IMPLEMENTED`, whatever handle it is given.
pub fn tc_deregister_client() -> u32 {
    ERROR_CALL_NOT_IMPLEMENTED
}
