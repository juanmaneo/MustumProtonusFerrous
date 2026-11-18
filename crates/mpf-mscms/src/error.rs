//! Failures of the ported entry points, with the Win32 error code Wine leaves in `GetLastError`.

/// A failed call. Variants name the Win32 error they stand for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McsError {
    /// `ERROR_INVALID_PARAMETER`.
    InvalidParameter,
    /// `ERROR_INVALID_TRANSFORM`.
    InvalidTransform,
    /// `ERROR_NOT_SUPPORTED`.
    NotSupported,
    /// `ERROR_FILE_NOT_FOUND`.
    FileNotFound,
    /// `ERROR_INSUFFICIENT_BUFFER`.
    InsufficientBuffer,
    /// `ERROR_MORE_DATA`, with the size in bytes the caller must supply.
    MoreData(u32),
    /// `CreateMultiProfileTransform` rejects more than two profiles without setting an error.
    MoreThanTwoProfiles,
}

impl McsError {
    /// The error code Wine leaves in `GetLastError`, or `None` when Wine sets none.
    pub fn win32_code(self) -> Option<u32> {
        match self {
            Self::InvalidParameter => Some(87),
            Self::InvalidTransform => Some(2020),
            Self::NotSupported => Some(50),
            Self::FileNotFound => Some(2),
            Self::InsufficientBuffer => Some(122),
            Self::MoreData(_) => Some(234),
            Self::MoreThanTwoProfiles => None,
        }
    }
}
