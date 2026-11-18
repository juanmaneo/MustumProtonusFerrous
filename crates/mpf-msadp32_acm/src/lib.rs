//! Port of the deterministic logic in Wine's `dlls/msadp32.acm/msadp32.c`, the
//! Microsoft ADPCM codec of the Audio Compression Manager.
//!
//! Ported: the format tables and index lookup, the driver details, the format
//! tag and format queries, format suggestion, stream size arithmetic, the
//! stream open checks, and the block decoders from 4-bit MS ADPCM to 8- and
//! 16-bit PCM with the convert loop.
//!
//! Skipped: the driver entry point and its message dispatch, driver open and
//! close, stream allocation and close, the `szFormat` output, and the
//! MultiByteToWideChar conversion (the driver strings are `&'static str`).
//! Wine has no encoder, so PCM to ADPCM streams are refused.
//!
//! Deviations: where Wine divides by zero (a zero channel count, zero block
//! alignment or zero samples per block), which is undefined behaviour in C, the
//! port returns [`Error::NotPossible`]. An ASYNC stream open, which Wine asserts
//! on, returns [`Error::NotSupported`]. Layouts whose blocks would not fit their
//! stated block alignment are refused at open, since Wine would read or write
//! past them. Unknown convert flags are ignored, as in Wine. Arithmetic that
//! overflows a C `int` is done with wrapping operations.

pub mod decode;
pub mod details;
pub mod format;
pub mod size;
pub mod suggest;

#[cfg(test)]
mod oracle_checks;

/// `WAVE_FORMAT_UNKNOWN`.
pub const WAVE_FORMAT_UNKNOWN: u16 = 0x0000;
/// `WAVE_FORMAT_PCM`.
pub const WAVE_FORMAT_PCM: u16 = 0x0001;
/// `WAVE_FORMAT_ADPCM`.
pub const WAVE_FORMAT_ADPCM: u16 = 0x0002;

/// The error codes Wine returns, as `MMSYSERR_*` and `ACMERR_*` values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `MMSYSERR_INVALPARAM`.
    InvalidParam,
    /// `MMSYSERR_NOTSUPPORTED`.
    NotSupported,
    /// `ACMERR_NOTPOSSIBLE`.
    NotPossible,
}

impl Error {
    /// The return value Wine gives for this error.
    pub const fn code(self) -> u32 {
        match self {
            Self::InvalidParam => 11,
            Self::NotSupported => 8,
            Self::NotPossible => 512,
        }
    }
}
