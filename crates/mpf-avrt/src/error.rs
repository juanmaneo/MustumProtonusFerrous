const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_OUTOFMEMORY: u32 = 14;
const ERROR_INVALID_TASK_NAME: u32 = 1550;

/// Error that the `AvSetMm*ThreadCharacteristics*` functions pass to `SetLastError`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvrtError {
    /// The `TaskIndex` pointer is NULL.
    InvalidHandle,
    /// `strdupAW` could not allocate a wide copy of an ANSI argument.
    OutOfMemory,
    /// A task name is NULL, or the second task name of the MAX variant is given.
    InvalidTaskName,
}

impl AvrtError {
    /// The Win32 error code.
    pub fn code(self) -> u32 {
        match self {
            AvrtError::InvalidHandle => ERROR_INVALID_HANDLE,
            AvrtError::OutOfMemory => ERROR_OUTOFMEMORY,
            AvrtError::InvalidTaskName => ERROR_INVALID_TASK_NAME,
        }
    }
}
