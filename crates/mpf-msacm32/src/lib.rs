//! Port of the deterministic logic in Wine's `dlls/msacm32/pcmconverter.c`, the
//! PCM converter of the Audio Compression Manager.
//!
//! Ported: the PCM format table and index lookup, the driver details, the
//! format tag and format queries, format suggestion, stream size arithmetic,
//! the stream open checks, and the convert loop with its sample readers,
//! writers and mixers for 8-, 16- and packed 24-bit samples.
//!
//! Skipped: the driver entry point and its message dispatch, driver open and
//! close, stream allocation and close, the `szFormat` and `szFormatTag` output,
//! and the MultiByteToWideChar conversion (the driver strings are `&'static str`).
//!
//! Deviations: where Wine asserts (an ASYNC stream open, a format index past the
//! table, a zero average bytes per second on the divisor side of a size query)
//! the port returns an error instead. A layout whose blocks do not hold one
//! frame is refused at open, since Wine would read or write past them. Samples
//! are held left-justified in an `i32`, so the negative left shifts and the
//! sign-bit shifts that Wine performs on `int` are defined here. Wine adds two
//! 24-bit samples in an `int`, which can overflow; the port wraps and then
//! clamps. A size result beyond `u32` saturates.

pub mod convert;
pub mod details;
pub mod format;
pub mod sample;
pub mod size;
pub mod suggest;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `WAVE_FORMAT_UNKNOWN`.
pub const WAVE_FORMAT_UNKNOWN: u16 = 0x0000;
/// `WAVE_FORMAT_PCM`.
pub const WAVE_FORMAT_PCM: u16 = 0x0001;

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
