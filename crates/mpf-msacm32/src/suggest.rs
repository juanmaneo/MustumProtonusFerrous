//! Format suggestion (`PCM_FormatSuggest`).

use crate::format::{fill_block_fields, format_index, WaveFormat, PCM_WAVEFORMAT_SIZE};
use crate::{Error, WAVE_FORMAT_PCM};

/// `ACM_FORMATSUGGESTF_WFORMATTAG`.
pub const SUGGEST_WFORMATTAG: u32 = 0x0001_0000;
/// `ACM_FORMATSUGGESTF_NCHANNELS`.
pub const SUGGEST_NCHANNELS: u32 = 0x0002_0000;
/// `ACM_FORMATSUGGESTF_NSAMPLESPERSEC`.
pub const SUGGEST_NSAMPLESPERSEC: u32 = 0x0004_0000;
/// `ACM_FORMATSUGGESTF_WBITSPERSAMPLE`.
pub const SUGGEST_WBITSPERSAMPLE: u32 = 0x0008_0000;

/// The `ACMDRVFORMATSUGGEST` fields: the flags and the sizes of the two formats.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Suggest {
    pub fdw_suggest: u32,
    pub cb_src: u32,
    pub cb_dst: u32,
}

/// `PCM_FormatSuggest`. Fields of `dst` the flags do not set are copied from
/// `src`. A refusal can leave the fields already copied in `dst`, as in Wine.
pub fn format_suggest(s: &Suggest, src: &WaveFormat, dst: &mut WaveFormat) -> Result<(), Error> {
    if s.cb_src < PCM_WAVEFORMAT_SIZE
        || s.cb_dst < PCM_WAVEFORMAT_SIZE
        || format_index(src).is_none()
    {
        return Err(Error::NotPossible);
    }
    if s.fdw_suggest & SUGGEST_NCHANNELS == 0 {
        dst.channels = src.channels;
    }
    if s.fdw_suggest & SUGGEST_NSAMPLESPERSEC == 0 {
        dst.samples_per_sec = src.samples_per_sec;
    }
    if s.fdw_suggest & SUGGEST_WBITSPERSAMPLE == 0 {
        dst.bits_per_sample = src.bits_per_sample;
    }
    if s.fdw_suggest & SUGGEST_WFORMATTAG == 0 {
        if src.format_tag != WAVE_FORMAT_PCM {
            return Err(Error::NotPossible);
        }
        dst.format_tag = src.format_tag;
    } else if dst.format_tag != WAVE_FORMAT_PCM {
        return Err(Error::NotPossible);
    }
    if format_index(dst).is_none() {
        return Err(Error::NotPossible);
    }
    fill_block_fields(dst);
    Ok(())
}
