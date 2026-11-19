//! Headless port of Wine's `dlls/mcicda`, the MCI audio CD driver. The C reference is `mcicda.c`.
//! Its state lives in `WINE_MCICDAUDIO`, and each command reaches the disc through a CD-ROM handle.
//! The deterministic parts run here on `Driver` and the `CdRom` trait, which stands in for the
//! handle, so the control flow of the C code is kept.
//!
//! Ported (names in `dlls/mcicda/mcicda.c`):
//! - `MCICDA_CalcFrame` and `MCICDA_CalcTime`: `calc_frame` and `calc_time`.
//! - `MCICDA_GetError`: `error::device_error`.
//! - `MCICDA_Status`, and the mode that `MCICDA_GetStatus` reads from the Q channel: `status`.
//! - `MCICDA_GetDevCaps`: `get_dev_caps`.
//! - `MCICDA_Info` and `CDROM_Audio_GetSerial`: `info` and `Toc::serial`.
//! - `MCICDA_Open` and `MCICDA_Close`: `open` and `close`.
//! - `MCICDA_Seek` and `MCICDA_SkipDataTracks`: `seek`.
//! - `MCICDA_Set` and `MCICDA_SetDoor`: `set`.
//! - The `CDROM_TOC` table, `FRAME_OF_TOC` and the `MCI_MAKE_MSF` and `MCI_MAKE_TMSF` packing:
//!   `Toc`, `Msf`, `pack_tmsf` and `unpack_tmsf`.
//!
//! Skipped:
//! - The device-ID lookup (`mciGetDriverData` and `MCICDA_GetOpenDrv`). The caller passes the
//!   `Driver` it owns.
//! - `MCI_NOTIFY` and `MCICDA_Notify`. The callbacks are not modelled.
//! - `MCICDA_Play`, `MCICDA_Pause`, `MCICDA_Resume` and the playback thread with its DirectSound
//!   buffer. `MCICDA_Stop` keeps only the `IOCTL_CDROM_STOP_AUDIO` branch, as `CdRom::stop_audio`.
//!   The mode in `MCICDA_GetStatus` comes from the Q channel alone.
//! - `MCICDA_drvOpen`, `MCICDA_drvClose` and `MCICDA_DriverProc`: driver instance management.
//! - The file, handle and device I/O (`CreateFileW`, `GetDriveTypeW`, `DeviceIoControl` and
//!   `CloseHandle`). They sit behind `CdRom`.
//!
//! Deviations from Wine, where Wine reads out of bounds or has dead code:
//! - A TOC index past the 100-entry table reads a zero entry. The C code reads out of bounds.
//! - The `== (WORD)-1` check after `MCI_STATUS_NUMBER_OF_TRACKS` is dropped. The track count is at
//!   most 256, so it cannot match.

pub mod error;
pub mod mci;

mod devcaps;
mod device;
mod frame;
mod info;
mod open;
mod seek;
mod set;
mod status;
mod time;
mod toc;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use devcaps::{get_dev_caps, DevCapsParms};
pub use device::{CdRom, QChannel};
pub use frame::Msf;
pub use info::info;
pub use open::{close, open, OpenParms};
pub use seek::{seek, SeekParms};
pub use set::{set, SetParms};
pub use status::{status, StatusParms};
pub use time::{calc_frame, calc_time};
pub use toc::{Toc, TrackData};

/// The state that `WINE_MCICDAUDIO` keeps for an open device.
pub struct Driver {
    /// `nUseCount`: the opens that hold the device. Zero when it is closed.
    pub use_count: i32,
    /// `fShareable`: whether the first open asked for a shareable device.
    pub shareable: bool,
    /// `dwTimeFormat`: `MCI_FORMAT_MILLISECONDS`, `MCI_FORMAT_MSF` or `MCI_FORMAT_TMSF`.
    pub time_format: u32,
}

impl Driver {
    /// `MCICDA_GetOpenDrv`: a device whose use count is zero is not open.
    pub fn is_open(&self) -> bool {
        self.use_count != 0
    }
}
