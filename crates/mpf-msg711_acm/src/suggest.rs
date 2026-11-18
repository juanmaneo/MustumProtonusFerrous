//! The `ACMDM_FORMAT_SUGGEST` query (`G711_FormatSuggest`).

use crate::format::get_format_index;
use crate::{Error, WaveFormat, PCMWAVEFORMAT_SIZE, WAVE_FORMAT_ALAW, WAVE_FORMAT_PCM};

pub const ACM_FORMATSUGGESTF_WFORMATTAG: u32 = 0x0001_0000;
pub const ACM_FORMATSUGGESTF_NCHANNELS: u32 = 0x0002_0000;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: u32 = 0x0004_0000;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: u32 = 0x0008_0000;

/// `G711_FormatSuggest`. Each field that `fdw_suggest` does not name is taken
/// from the source, or derived from it. Both format buffers must hold at least
/// a `PCMWAVEFORMAT`, and the source must be listed. The destination is
/// written before its own check, so a `NotPossible` result can leave it
/// partly updated, as in Wine.
pub fn format_suggest(
    fdw_suggest: u32,
    cb_src: u32,
    cb_dst: u32,
    src: &WaveFormat,
    dst: &mut WaveFormat,
) -> Result<(), Error> {
    if cb_src < PCMWAVEFORMAT_SIZE || cb_dst < PCMWAVEFORMAT_SIZE || get_format_index(src).is_none()
    {
        return Err(Error::NotPossible);
    }
    if fdw_suggest & ACM_FORMATSUGGESTF_NCHANNELS == 0 {
        dst.channels = src.channels;
    }
    if fdw_suggest & ACM_FORMATSUGGESTF_NSAMPLESPERSEC == 0 {
        dst.samples_per_sec = src.samples_per_sec;
    }
    if fdw_suggest & ACM_FORMATSUGGESTF_WBITSPERSAMPLE == 0 {
        dst.bits_per_sample = if src.format_tag == WAVE_FORMAT_PCM {
            8
        } else {
            16
        };
    }
    if fdw_suggest & ACM_FORMATSUGGESTF_WFORMATTAG == 0 {
        dst.format_tag = if src.format_tag == WAVE_FORMAT_PCM {
            WAVE_FORMAT_ALAW
        } else {
            WAVE_FORMAT_PCM
        };
    }
    if get_format_index(dst).is_none() {
        return Err(Error::NotPossible);
    }
    if dst.format_tag == WAVE_FORMAT_PCM {
        dst.block_align = dst.channels * 2;
        dst.avg_bytes_per_sec = dst.samples_per_sec * u32::from(dst.block_align);
    } else {
        dst.block_align = dst.channels;
        dst.avg_bytes_per_sec = dst.samples_per_sec * u32::from(src.channels);
    }
    Ok(())
}
