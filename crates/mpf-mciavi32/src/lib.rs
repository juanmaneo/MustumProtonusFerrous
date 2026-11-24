//! Headless port of Wine's `dlls/mciavi32`, the digital video MCI driver.
//!
//! The C reference is an MCI driver. Its commands arrive through `mciSendCommand`, find the open
//! device in a table and write their replies through `DWORD` out parameters. The deterministic
//! parts run here on a [Device] value and follow the C control flow, so the replies match Wine's.
//!
//! Ported (names in `dlls/mciavi32/info.c`):
//! - `MCIAVI_ConvertFrameToTimeFormat` and `MCIAVI_ConvertTimeFormatToFrame`: [frame_to_time_format]
//!   and [time_format_to_frame].
//! - `MCIAVI_mciGetDevCaps`: [mci_get_dev_caps].
//! - `MCIAVI_mciInfo`: [mci_info].
//! - `MCIAVI_mciSet`: [mci_set].
//! - `MCIAVI_mciStatus`: [mci_status]. Its window items return `None`, see below.
//! - `MulDiv`, which `MCI_DGV_STATUS_FRAME_RATE` uses: [mul_div].
//! - `MAKEMCIRESOURCE`: [make_resource] and [bool_resource].
//!
//! Skipped: `DllMain`; the device table behind `MCIAVI_mciGetOpenDev`, so callers pass in the open
//! device; the open, close, play, seek and other commands of `mciavi.c`; the `mciDriverNotify` call
//! for `MCI_NOTIFY` in `mciStatus`, which the caller sends on the success paths; the window items
//! `MCI_DGV_STATUS_HWND`, `MCI_DGV_STATUS_WINDOW_VISIBLE`, `MCI_DGV_STATUS_WINDOW_MINIMIZED` and
//! `MCI_DGV_STATUS_WINDOW_MAXIMIZED`, which query OS windows; the `MCI_DGV_SET_FILEFORMAT`,
//! `MCI_DGV_SET_SPEED` and `MCI_DGV_SET_SEEK_EXACTLY` flags of `mciSet`, which only log in Wine;
//! and all `TRACE` and `FIXME` output.
//!
//! Deviations from Wine:
//! - [time_format_to_frame] returns `None` where the millisecond conversion divides by a zero
//!   `dwMicroSecPerFrame`. The C code faults there (undefined behaviour, flagged by the sanitizers).
//! - [mci_info] writes into a slice whose length is the buffer size. Wine passes `dwRetSize` to
//!   `lstrcpynW` as an `INT`, so a size above `INT_MAX` makes its copy unbounded.
//! - [mul_div] works in `i64`, so the negation of `INT_MIN`, which is undefined in C, does not arise.
//! - [mci_set] keeps Wine's `MCI_SET_OFF` with `MCI_SET_AUDIO_LEFT`, which clears the right channel
//!   bit (`&= ~2`). This is a Wine bug, kept so that the state matches.
//! - Replies are 32 bits wide, as `dwReturn` is. [frame_to_time_format] wraps like `DWORD` arithmetic.

mod abi;
mod devcaps;
mod info;
mod set;
mod status;
mod time;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use abi::*;
pub use devcaps::mci_get_dev_caps;
pub use info::mci_info;
pub use set::{mci_set, SetParams};
pub use status::{mci_status, mul_div};
pub use time::{frame_to_time_format, time_format_to_frame};

/// The open device, holding the fields of `WINE_MCIAVI` that the commands read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    /// `dwMciTimeFormat`: [MCI_FORMAT_MILLISECONDS] or [MCI_FORMAT_FRAMES].
    pub time_format: u32,
    /// `mah.dwMicroSecPerFrame`.
    pub micro_sec_per_frame: u32,
    /// `mah.dwTotalFrames`.
    pub total_frames: u32,
    /// `dwCurrVideoFrame`.
    pub current_frame: u32,
    /// `dwStatus`, one of the `MCI_MODE_*` values.
    pub status: u32,
    /// `dwSet`, built from [SET_VIDEO_BIT], [SET_AUDIO_LEFT_BIT] and [SET_AUDIO_RIGHT_BIT].
    pub set: u32,
    /// Whether `hFile` is open, which `LENGTH` and `POSITION` need.
    pub has_file: bool,
    /// `lpFileName` as UTF-16 without its NUL. It is empty where Wine has no name.
    pub file_name: Vec<u16>,
    /// `inbih->biBitCount`.
    pub bit_count: u16,
    /// `audio_stream_n`.
    pub audio_stream: u32,
}

/// The reply of an MCI command: its return code and, when the C code writes it, `lpParms->dwReturn`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reply {
    /// The value the command returns.
    pub ret: u32,
    /// The `dwReturn` that the command writes, or `None` when the C code leaves it untouched.
    pub dw_return: Option<u32>,
}

impl Reply {
    /// A reply that returns `ret` and writes no `dwReturn`.
    pub const fn code(ret: u32) -> Self {
        Self {
            ret,
            dw_return: None,
        }
    }

    /// A successful reply that writes a plain `dwReturn`.
    pub const fn value(dw_return: u32) -> Self {
        Self {
            ret: 0,
            dw_return: Some(dw_return),
        }
    }

    /// A successful reply that writes a packed resource in `dwReturn`.
    pub const fn resource(dw_return: u32) -> Self {
        Self {
            ret: MCI_RESOURCE_RETURNED,
            dw_return: Some(dw_return),
        }
    }
}
