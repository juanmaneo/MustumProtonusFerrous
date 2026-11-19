//! The device boundary. `mcicda.c` reaches the disc through `DeviceIoControl` on a CD-ROM handle,
//! and finds drives with `GetDriveTypeW` and `CreateFileW`. Those calls sit behind `CdRom`, and the
//! logic above them is pure.

use crate::error::device_error;
use crate::frame::Msf;
use crate::toc::Toc;

/// The current-position fields of `SUB_Q_CHANNEL_DATA` that `mcicda.c` reads.
#[derive(Clone, Copy)]
pub struct QChannel {
    /// `Header.AudioStatus`, one of the `AUDIO_STATUS_*` values.
    pub audio_status: u8,
    /// `TrackNumber`.
    pub track: u8,
    /// `AbsoluteAddress`.
    pub absolute: Msf,
}

/// The device operations behind the handle. A failed call leaves the Win32 last error, which
/// `device_error` maps to an `MCIERR_*` code.
pub trait CdRom {
    /// `IOCTL_CDROM_READ_TOC`.
    fn read_toc(&mut self) -> Result<Toc, u32>;
    /// `IOCTL_CDROM_READ_Q_CHANNEL` for the current position.
    fn read_q_channel(&mut self) -> Result<QChannel, u32>;
    /// `IOCTL_CDROM_SEEK_AUDIO_MSF`.
    fn seek_audio(&mut self, to: Msf) -> bool;
    /// `IOCTL_CDROM_STOP_AUDIO`. Every caller ignores the result, so none is returned.
    fn stop_audio(&mut self);
    /// `IOCTL_STORAGE_EJECT_MEDIA` when `open`, otherwise `IOCTL_STORAGE_LOAD_MEDIA`. Every caller
    /// ignores the result, so none is returned.
    fn set_door(&mut self, open: bool);
    /// Whether `GetDriveTypeW` reports `DRIVE_CDROM` for the root of drive `letter`.
    fn is_cd_rom(&self, letter: u8) -> bool;
    /// `CreateFileW` on the device path of drive `letter`.
    fn open_handle(&mut self, letter: u8) -> bool;
    /// `CloseHandle` on the device handle.
    fn close_handle(&mut self);
}

pub(crate) fn toc_or_error(dev: &mut dyn CdRom) -> Result<Toc, u32> {
    dev.read_toc().map_err(device_error)
}

pub(crate) fn q_or_error(dev: &mut dyn CdRom) -> Result<QChannel, u32> {
    dev.read_q_channel().map_err(device_error)
}
