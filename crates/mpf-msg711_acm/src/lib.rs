//! Port of the deterministic logic in Wine's `dlls/msg711.acm/msg711.c`, the
//! G.711 codec of the Audio Compression Manager (A-law and mu-law).
//!
//! Ported: the sample codecs ([`law`]), the format table and index lookup and
//! the format details query ([`format`]), the format-tag query ([`tag`]),
//! format suggestion ([`suggest`]), stream size arithmetic ([`size`]), the
//! stream open checks ([`open`]) and the convert loops ([`convert`]).
//!
//! Skipped: the driver entry point (`G711_DriverProc`), `drvOpen` and
//! `drvClose`, `DefDriverProc`, driver details (MultiByteToWideChar and the
//! icon handle), the configure dialog (MessageBoxA), the per-stream allocation
//! (HeapAlloc and HeapFree) and the `szFormat` and `fdwSupport` output fields.
//!
//! Deviations: Wine's DWORD fields are `u32` with wrapping arithmetic and its
//! WORD fields are `u16`. The decoders use Wine's statically generated tables,
//! as Wine does without `NO_FASTDECODE`. The `fdwConvert` flags do not change
//! the output (see [`convert`]).

pub mod convert;
pub mod format;
pub mod law;
pub mod open;
pub mod size;
pub mod suggest;
pub mod tag;

#[cfg(test)]
mod oracle_checks;

pub const WAVE_FORMAT_UNKNOWN: u16 = 0x0000;
pub const WAVE_FORMAT_PCM: u16 = 0x0001;
pub const WAVE_FORMAT_ALAW: u16 = 0x0006;
pub const WAVE_FORMAT_MULAW: u16 = 0x0007;

pub const PCMWAVEFORMAT_SIZE: u32 = 16;
pub const WAVEFORMATEX_SIZE: u32 = 18;

const MMSYSERR_NOTSUPPORTED: u32 = 8;
const MMSYSERR_INVALPARAM: u32 = 11;
const ACMERR_NOTPOSSIBLE: u32 = 512;

/// The fields of `WAVEFORMATEX` that the codec reads or writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

/// A failed query, with the result code Wine returns for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// `dwFormatTag` as the `WORD` that Wine's `switch` compares it with. A tag
/// wider than 16 bits matches no known format.
pub(crate) fn word_tag(format_tag: u32) -> Option<u16> {
    u16::try_from(format_tag).ok()
}
