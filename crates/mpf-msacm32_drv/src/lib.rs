//! Port of the portable logic in Wine's `dlls/msacm32.drv/wavemap.c`.
//!
//! Ported:
//! - `MulDiv` (`dlls/kernelbase/main.c`), used by the position math: [`mul_div`]
//! - `wodGetPosition` and `widGetPosition` (identical bodies), including the
//!   `TIME_MS` request mapping: [`position_query`], [`get_position`], [`MmTime`], [`Rates`]
//! - Byte count and average rate arithmetic of `wodOpenHelper` / `widOpenHelper`: [`PcmFormat`]
//! - The format search of `wodOpen` / `widOpen`: the direct stage, the PCM and
//!   non-PCM probe ladders, and the error mapping: [`wod_open`], [`wid_open`]
//! - `WAVE_MAPPED` device selection and its `MMSYSERR_INVALPARAM` error: [`device_range`]
//!
//! Skipped:
//! - Every `waveOut*`, `waveIn*` and `acmStream*` call (OS audio). The `WAVE_FORMAT_QUERY`
//!   flag, which only decides whether the stream handle is kept, is not modelled.
//! - `HeapAlloc` / `HeapFree` and `WAVEMAPDATA` allocation, and `hAcmStream` bookkeeping.
//! - `wodCallback` / `widCallback`, `WHDR` flag updates, and the write, prepare,
//!   unprepare, add-buffer and close entries.
//! - `wodGetDevCaps` / `widGetDevCaps`: forward to `waveOutGetDevCapsW` / `waveInGetDevCapsW`.
//! - Volume, pause, restart, reset, break-loop, stop and start entries.
//! - Mapper status and reconfigure entries (`wodMapperStatus`, `wodMapperReconfigure` and the
//!   wave-in counterparts).
//! - `WAVEMAP_wodMessage` / `widMessage` dispatch, `WAVEMAP_drvOpen` / `drvClose`, `DriverProc`.

mod format;
mod open;
mod position;

pub use format::PcmFormat;
pub use open::{wid_open, wod_open, Opened, SourceFormat};
pub use position::{get_position, mul_div, position_query, MmTime, Rates};

use std::ops::Range;

/// `MMSYSERR_*` and `WAVERR_*` return codes from `mmsystem.h`.
pub const MMSYSERR_NOERROR: u32 = 0;
pub const MMSYSERR_NOMEM: u32 = 7;
pub const MMSYSERR_INVALPARAM: u32 = 11;
pub const WAVERR_BADFORMAT: u32 = 32;

/// `ACMERR_NOTPOSSIBLE` from `msacm.h`.
pub const ACMERR_NOTPOSSIBLE: u32 = 512;

/// `WAVE_FORMAT_PCM`, `WAVE_MAPPED` and `WAVE_FORMAT_DIRECT` from `mmsystem.h`.
pub const WAVE_FORMAT_PCM: u16 = 1;
pub const WAVE_MAPPED: u32 = 0x0004;
pub const WAVE_FORMAT_DIRECT: u32 = 0x0008;

/// `TIME_*` types from `mmsystem.h`.
pub const TIME_MS: u32 = 0x0001;
pub const TIME_SAMPLES: u32 = 0x0002;
pub const TIME_BYTES: u32 = 0x0004;
pub const TIME_SMPTE: u32 = 0x0008;

/// The device ids that `wodOpen` / `widOpen` try, given the caller's `flags` and
/// `uMappedDeviceID`. Without `WAVE_MAPPED` every device is tried. With it, only the mapped
/// device is tried, and it must be below `num_devices`.
pub fn device_range(flags: u32, mapped_device: u32, num_devices: u32) -> Result<Range<u32>, u32> {
    if (flags & WAVE_MAPPED) == 0 {
        return Ok(0..num_devices);
    }
    if mapped_device >= num_devices {
        return Err(MMSYSERR_INVALPARAM);
    }
    Ok(mapped_device..mapped_device + 1)
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
