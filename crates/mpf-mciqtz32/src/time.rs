//! Time formats of `MCIQTZ_mciSet` and of the time format status item in `MCIQTZ_mciStatus`.

use crate::abi::{
    make_mci_resource, MCIERR_BAD_TIME_FORMAT, MCI_FORMAT_FRAMES, MCI_FORMAT_MILLISECONDS,
    MCI_FORMAT_RETURN_BASE, MCI_SET_TIME_FORMAT,
};

/// The time format of the device, `wma->time_format` in Wine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    Milliseconds,
    Frames,
}

impl TimeFormat {
    /// The MCI format code, as `MCI_FORMAT_MILLISECONDS` or `MCI_FORMAT_FRAMES`.
    pub const fn code(self) -> u32 {
        match self {
            Self::Milliseconds => MCI_FORMAT_MILLISECONDS,
            Self::Frames => MCI_FORMAT_FRAMES,
        }
    }
}

/// `MCI_SET` with `MCI_SET_TIME_FORMAT`. Returns the format the device has afterwards, or the
/// error for an unsupported format. Without the flag the format is unchanged.
pub fn set_time_format(flags: u32, requested: u32, current: TimeFormat) -> Result<TimeFormat, u32> {
    if flags & MCI_SET_TIME_FORMAT == 0 {
        return Ok(current);
    }
    match requested {
        MCI_FORMAT_MILLISECONDS => Ok(TimeFormat::Milliseconds),
        MCI_FORMAT_FRAMES => Ok(TimeFormat::Frames),
        _ => Err(MCIERR_BAD_TIME_FORMAT),
    }
}

/// The `dwReturn` of `MCI_STATUS_TIME_FORMAT`: the format code in the low word and its string
/// resource in the high word.
pub const fn time_format_value(format: TimeFormat) -> u32 {
    make_mci_resource(format.code(), MCI_FORMAT_RETURN_BASE + format.code())
}
