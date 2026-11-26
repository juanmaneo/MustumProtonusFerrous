//! NTSTATUS values returned by the ported functions, and the conversion errors that map onto them.

pub const STATUS_SUCCESS: u32 = 0x0000_0000;
pub const STATUS_NOT_IMPLEMENTED: u32 = 0xc000_0002;
pub const STATUS_BUFFER_TOO_SMALL: u32 = 0xc000_0023;
pub const STATUS_NOT_FOUND: u32 = 0xc000_0225;
pub const STATUS_NOT_SUPPORTED: u32 = 0xc000_00bb;

/// Errors from the format conversions. `BufferTooSmall` carries the size the caller has to provide.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatError {
    BufferTooSmall(u32),
    NotImplemented,
}

impl FormatError {
    /// The NTSTATUS that the C function returns.
    pub fn status(self) -> u32 {
        match self {
            Self::BufferTooSmall(_) => STATUS_BUFFER_TOO_SMALL,
            Self::NotImplemented => STATUS_NOT_IMPLEMENTED,
        }
    }

    /// The size written back through the size out-parameter, or `None` when the C code leaves it alone.
    pub fn required_size(self) -> Option<u32> {
        match self {
            Self::BufferTooSmall(size) => Some(size),
            Self::NotImplemented => None,
        }
    }
}

/// Fails with `BufferTooSmall(size)` when a format of `size` bytes does not fit in `capacity`.
pub(crate) fn check_capacity(size: usize, capacity: u32) -> Result<(), FormatError> {
    if size > capacity as usize {
        return Err(FormatError::BufferTooSmall(size as u32));
    }
    Ok(())
}
