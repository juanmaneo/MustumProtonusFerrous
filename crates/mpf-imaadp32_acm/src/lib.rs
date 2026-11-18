//! Port of the deterministic logic in Wine's `dlls/imaadp32.acm/imaadp32.c`,
//! the IMA ADPCM codec of the Audio Compression Manager.
//!
//! Ported: the PCM and ADPCM format tables and index lookup, the format-tag and
//! format details queries, format suggestion, the IMA step and index tables,
//! the nibble encoder and decoder, stream size arithmetic, the stream open
//! checks, and the block loop of the convert step.
//!
//! Skipped: the driver entry points, driver details (which need
//! MultiByteToWideChar), stream close, driver open and close (heap allocation),
//! the `DriverProc` dispatch, the `szFormat` output, and the Wine debug
//! channel.
//!
//! Deviations, where Wine's C is undefined behaviour or reads past a buffer:
//! a zero divisor in a size query, an ADPCM format with a zero block align, or
//! a stereo format whose samples per block is not `1 mod 8`, is rejected with an
//! error. A block whose footprint overlaps the next block is only converted when
//! the buffer holds the last block's footprint, otherwise the port returns
//! [`Error::InvalidParam`]. The port works in `u32` where Wine uses C `int`
//! products. Wine's `DWORD` fields are `u32` with wrapping arithmetic, and
//! `WORD` fields are `u16`.

mod codec;
pub mod convert;
pub mod format;
pub mod open;
pub mod size;
pub mod tag;

#[cfg(test)]
mod oracle_checks;

pub const WAVE_FORMAT_UNKNOWN: u16 = 0x0000;
pub const WAVE_FORMAT_PCM: u16 = 0x0001;
pub const WAVE_FORMAT_IMA_ADPCM: u16 = 0x0011;

pub const PCMWAVEFORMAT_SIZE: u32 = 16;
pub const IMAADPCMWAVEFORMAT_SIZE: u32 = 20;

pub const ACM_DRIVERDETAILS_SUPPORTF_CODEC: u32 = 0x0000_0001;

const MMSYSERR_NOTSUPPORTED: u32 = 8;
const MMSYSERR_INVALPARAM: u32 = 11;
const ACMERR_NOTPOSSIBLE: u32 = 512;

/// The fields of `WAVEFORMATEX` and `IMAADPCMWAVEFORMAT` that the codec reads.
/// `samples_per_block` (`wSamplesPerBlock`) is only meaningful for ADPCM.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
    pub samples_per_block: u16,
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
