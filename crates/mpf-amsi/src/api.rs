//! Port of `AmsiInitialize`, `AmsiOpenSession`, `AmsiCloseSession`, `AmsiScanBuffer`,
//! `AmsiScanString` and `AmsiUninitialize` from `dlls/amsi/main.c`.

/// `HRESULT`.
pub type Hresult = i32;
/// `HAMSICONTEXT` and `HAMSISESSION`: opaque pointer-sized handles.
pub type Handle = usize;
/// `AMSI_RESULT` from `include/amsi.idl`.
pub type AmsiResult = u32;

pub const S_OK: Hresult = 0;
/// The fixed value stored into every context and session handle.
pub const FAKE_HANDLE: Handle = 0xdead_beef;
pub const AMSI_RESULT_NOT_DETECTED: AmsiResult = 0x01;

/// Stores `FAKE_HANDLE` into `context` and returns `S_OK`.
pub fn amsi_initialize(context: &mut Handle) -> Hresult {
    *context = FAKE_HANDLE;
    S_OK
}

/// Stores `FAKE_HANDLE` into `session` and returns `S_OK`.
pub fn amsi_open_session(session: &mut Handle) -> Hresult {
    *session = FAKE_HANDLE;
    S_OK
}

/// Does nothing.
pub fn amsi_close_session() {}

/// Stores `AMSI_RESULT_NOT_DETECTED` into `result` and returns `S_OK`.
pub fn amsi_scan_buffer(result: &mut AmsiResult) -> Hresult {
    *result = AMSI_RESULT_NOT_DETECTED;
    S_OK
}

/// Stores `AMSI_RESULT_NOT_DETECTED` into `result` and returns `S_OK`.
pub fn amsi_scan_string(result: &mut AmsiResult) -> Hresult {
    *result = AMSI_RESULT_NOT_DETECTED;
    S_OK
}

/// Does nothing.
pub fn amsi_uninitialize() {}
