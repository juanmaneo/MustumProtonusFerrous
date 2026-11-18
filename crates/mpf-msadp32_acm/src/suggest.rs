//! Format suggestion (`ADPCM_FormatSuggest`).

use crate::format::{
    format_index, init_adpcm, WaveFormat, ADPCM_WAVEFORMAT_SIZE, PCM_WAVEFORMAT_SIZE,
};
use crate::{Error, WAVE_FORMAT_ADPCM, WAVE_FORMAT_PCM};

/// `ACM_FORMATSUGGESTF_WFORMATTAG`.
pub const SUGGEST_WFORMATTAG: u32 = 0x0001_0000;
/// `ACM_FORMATSUGGESTF_NCHANNELS`.
pub const SUGGEST_NCHANNELS: u32 = 0x0002_0000;
/// `ACM_FORMATSUGGESTF_NSAMPLESPERSEC`.
pub const SUGGEST_NSAMPLESPERSEC: u32 = 0x0004_0000;
/// `ACM_FORMATSUGGESTF_WBITSPERSAMPLE`.
pub const SUGGEST_WBITSPERSAMPLE: u32 = 0x0008_0000;

/// `ACMDRVFORMATSUGGEST`, the fields read by `ADPCM_FormatSuggest`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Suggest {
    pub fdw_suggest: u32,
    pub cb_src: u32,
    pub cb_dst: u32,
}

/// `ADPCM_FormatSuggest`: fills the destination format from the source format
/// and the suggestion flags. Only the destination is written.
pub fn format_suggest(s: &Suggest, src: &WaveFormat, dst: &mut WaveFormat) -> Result<(), Error> {
    if s.cb_src < PCM_WAVEFORMAT_SIZE
        || s.cb_dst < PCM_WAVEFORMAT_SIZE
        || src.format_tag == dst.format_tag
        || format_index(src).is_none()
    {
        return Err(Error::NotPossible);
    }
    let src_pcm = src.format_tag == WAVE_FORMAT_PCM;
    if s.fdw_suggest & SUGGEST_NCHANNELS == 0 {
        dst.channels = src.channels;
    }
    if s.fdw_suggest & SUGGEST_NSAMPLESPERSEC == 0 {
        dst.samples_per_sec = src.samples_per_sec;
    }
    if s.fdw_suggest & SUGGEST_WBITSPERSAMPLE == 0 {
        dst.bits_per_sample = if src_pcm { 4 } else { 16 };
    }
    if s.fdw_suggest & SUGGEST_WFORMATTAG == 0 {
        dst.format_tag = if src_pcm {
            WAVE_FORMAT_ADPCM
        } else {
            WAVE_FORMAT_PCM
        };
    }
    match dst.format_tag {
        WAVE_FORMAT_PCM => {
            dst.block_align =
                ((u32::from(dst.channels) * u32::from(dst.bits_per_sample)) / 8) as u16;
            dst.avg_bytes_per_sec = dst.samples_per_sec.wrapping_mul(u32::from(dst.block_align));
            if format_index(dst).is_none() {
                return Err(Error::NotPossible);
            }
        }
        WAVE_FORMAT_ADPCM => {
            if s.cb_dst < ADPCM_WAVEFORMAT_SIZE {
                return Err(Error::NotPossible);
            }
            init_adpcm(dst);
            // Wine divides by `wSamplesPerBlock` here, so a zero value is refused.
            if format_index(dst).is_none() || dst.samples_per_block == 0 {
                return Err(Error::NotPossible);
            }
        }
        _ => return Err(Error::NotPossible),
    }
    Ok(())
}
