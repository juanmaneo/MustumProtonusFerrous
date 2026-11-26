/// `ERROR_SUCCESS` from `include/winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;
/// `ERROR_FILE_NOT_FOUND` from `include/winerror.h`.
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
/// `ERROR_INVALID_HANDLE` from `include/winerror.h`.
pub const ERROR_INVALID_HANDLE: u32 = 6;
/// `ERROR_NO_MORE_FILES` from `include/winerror.h`.
pub const ERROR_NO_MORE_FILES: u32 = 18;
/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// A BOOL result together with the value Wine passes to `SetLastError`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome<T> {
    pub ret: T,
    /// `None` when Wine does not call `SetLastError`, so `GetLastError` keeps its old value.
    pub last_error: Option<u32>,
}

impl<T> Outcome<T> {
    /// A result that passes `code` to `SetLastError`.
    pub fn with_error(ret: T, code: u32) -> Self {
        Outcome {
            ret,
            last_error: Some(code),
        }
    }

    /// A result that leaves the last error alone.
    pub fn without_error(ret: T) -> Self {
        Outcome {
            ret,
            last_error: None,
        }
    }
}
