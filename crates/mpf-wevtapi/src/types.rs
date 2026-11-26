/// `EVT_HANDLE` from `include/winevt.h`, a pointer-sized `HANDLE`. The port never dereferences it.
pub type EvtHandle = usize;

/// A NULL `EVT_HANDLE`.
pub const NULL_HANDLE: EvtHandle = 0;

/// `ERROR_CALL_NOT_IMPLEMENTED` from `include/winerror.h`.
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `ERROR_INSUFFICIENT_BUFFER` from `include/winerror.h`.
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

/// A return value, plus the last-error code the C code set with `SetLastError`, if it did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outcome<T> {
    pub ret: T,
    pub last_error: Option<u32>,
}

impl<T> Outcome<T> {
    /// A return that leaves the last error alone.
    pub fn returned(ret: T) -> Self {
        Self {
            ret,
            last_error: None,
        }
    }

    /// A return that also calls `SetLastError(last_error)`.
    pub fn failed(ret: T, last_error: u32) -> Self {
        Self {
            ret,
            last_error: Some(last_error),
        }
    }
}
