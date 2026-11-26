/// `ERROR_CALL_NOT_IMPLEMENTED` from `winerror.h`, the code the apphelp stubs pass to `SetLastError`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// A stub's return value and the error code it leaves for `GetLastError`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubResult<T> {
    pub value: T,
    pub last_error: u32,
}

impl<T> StubResult<T> {
    /// A stub that calls `SetLastError(ERROR_CALL_NOT_IMPLEMENTED)` and returns `value`.
    pub fn not_implemented(value: T) -> Self {
        Self {
            value,
            last_error: ERROR_CALL_NOT_IMPLEMENTED,
        }
    }
}
