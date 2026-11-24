//! `MCIAVI_ConvertFrameToTimeFormat` and `MCIAVI_ConvertTimeFormatToFrame`.

use crate::abi::{MCI_FORMAT_FRAMES, MCI_FORMAT_MILLISECONDS};
use crate::Device;

/// Converts a frame count to the device's time format. Milliseconds use `DWORD` arithmetic, and
/// a format that is neither milliseconds nor frames gives 0.
pub fn frame_to_time_format(device: &Device, val: u32) -> u32 {
    match device.time_format {
        MCI_FORMAT_MILLISECONDS => val.wrapping_mul(device.micro_sec_per_frame) / 1000,
        MCI_FORMAT_FRAMES => val,
        _ => 0,
    }
}

/// Converts a value in the device's time format to frames. Milliseconds divide by
/// `micro_sec_per_frame`, so that conversion gives `None` when the frame rate is zero.
pub fn time_format_to_frame(device: &Device, val: u32) -> Option<u32> {
    match device.time_format {
        MCI_FORMAT_MILLISECONDS => val
            .wrapping_mul(1000)
            .checked_div(device.micro_sec_per_frame),
        MCI_FORMAT_FRAMES => Some(val),
        _ => Some(0),
    }
}
