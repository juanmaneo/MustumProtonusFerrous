//! Port of the deterministic logic in Wine's `dlls/l3codeca.acm/mpegl3.c`, the
//! MPEG Layer-3 decoder of the Audio Compression Manager.
//!
//! Ported: the format tables and index lookup, the format-tag and format
//! details queries, format suggestion, stream size arithmetic, the stream open
//! checks, and the convert loop around a libmpg123-style [`convert::Decoder`].
//!
//! Skipped: the driver entry points, driver details (which need
//! MultiByteToWideChar), stream close, driver open and close (mpg123_init and
//! mpg123_exit), the libmpg123 binding and the mpg123_getformat trace, the
//! instance handle, and the szFormat output.
//!
//! Deviations: where Wine divides by a zero rate or block size, which is
//! undefined behaviour in C, the port returns [`Error::NotPossible`]. Wine's
//! DWORD fields are `u32` with wrapping arithmetic, and WORD fields are `u16`.

pub mod convert;
pub mod format;
pub mod open;
pub mod size;
pub mod tag;

#[cfg(test)]
mod oracle_checks;

pub const WAVE_FORMAT_UNKNOWN: u16 = 0x0000;
pub const WAVE_FORMAT_PCM: u16 = 0x0001;
pub const WAVE_FORMAT_MPEG: u16 = 0x0050;
pub const WAVE_FORMAT_MPEGLAYER3: u16 = 0x0055;

pub const PCMWAVEFORMAT_SIZE: u32 = 16;
pub const MPEGLAYER3WAVEFORMAT_SIZE: u32 = 30;
pub const MPEG1WAVEFORMAT_SIZE: u32 = 40;

pub const ACM_DRIVERDETAILS_SUPPORTF_CODEC: u32 = 0x0000_0001;

const MMSYSERR_NOTSUPPORTED: u32 = 8;
const MMSYSERR_INVALPARAM: u32 = 11;
const ACMERR_NOTPOSSIBLE: u32 = 512;

/// The fields of `WAVEFORMATEX` and `MPEGLAYER3WAVEFORMAT` that the codec reads.
/// `cb_size` and `mpeg_id` (`wID`) are the MPEG Layer-3 extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
    pub mpeg_id: u16,
}

/// A failed query, with the result code Wine returns for it.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// `ACMERR_NOTPOSSIBLE`
    NotPossible,
    /// `MMSYSERR_NOTSUPPORTED`
    NotSupported,
    /// `MMSYSERR_INVALPARAM`
    InvalidParam,
}

impl Error {
    /// The `LRESULT` value that Wine returns for this error.
    pub fn code(self) -> u32 {
        match self {
            Error::NotPossible => ACMERR_NOTPOSSIBLE,
            Error::NotSupported => MMSYSERR_NOTSUPPORTED,
            Error::InvalidParam => MMSYSERR_INVALPARAM,
        }
    }
}

/// `dwFormatTag` as the `WORD` that Wine compares it with. A tag wider than 16
/// bits matches no known format.
pub(crate) fn word_tag(tag: u32) -> Option<u16> {
    u16::try_from(tag).ok()
}
