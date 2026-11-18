//! The format tables and the format queries: `ADPCM_GetFormatIndex`,
//! `init_wfx_ima_adpcm`, `ADPCM_FormatDetails` and `ADPCM_FormatSuggest`.

use crate::word_tag;
use crate::{
    Error, WaveFormat, ACM_DRIVERDETAILS_SUPPORTF_CODEC, IMAADPCMWAVEFORMAT_SIZE,
    PCMWAVEFORMAT_SIZE, WAVE_FORMAT_IMA_ADPCM, WAVE_FORMAT_PCM,
};

pub const ACM_FORMATDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATDETAILSF_FORMAT: u32 = 1;

pub const ACM_FORMATSUGGESTF_WFORMATTAG: u32 = 0x0001_0000;
pub const ACM_FORMATSUGGESTF_NCHANNELS: u32 = 0x0002_0000;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: u32 = 0x0004_0000;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: u32 = 0x0008_0000;

/// One row of `PCM_Formats` or `ADPCM_Formats`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Format {
    pub(crate) channels: u16,
    pub(crate) bits: u16,
    pub(crate) rate: u32,
}

pub(crate) const PCM_FORMATS: [Format; 16] = [
    Format {
        channels: 1,
        bits: 8,
        rate: 8000,
    },
    Format {
        channels: 2,
        bits: 8,
        rate: 8000,
    },
    Format {
        channels: 1,
        bits: 16,
        rate: 8000,
    },
    Format {
        channels: 2,
        bits: 16,
        rate: 8000,
    },
    Format {
        channels: 1,
        bits: 8,
        rate: 11025,
    },
    Format {
        channels: 2,
        bits: 8,
        rate: 11025,
    },
    Format {
        channels: 1,
        bits: 16,
        rate: 11025,
    },
    Format {
        channels: 2,
        bits: 16,
        rate: 11025,
    },
    Format {
        channels: 1,
        bits: 8,
        rate: 22050,
    },
    Format {
        channels: 2,
        bits: 8,
        rate: 22050,
    },
    Format {
        channels: 1,
        bits: 16,
        rate: 22050,
    },
    Format {
        channels: 2,
        bits: 16,
        rate: 22050,
    },
    Format {
        channels: 1,
        bits: 8,
        rate: 44100,
    },
    Format {
        channels: 2,
        bits: 8,
        rate: 44100,
    },
    Format {
        channels: 1,
        bits: 16,
        rate: 44100,
    },
    Format {
        channels: 2,
        bits: 16,
        rate: 44100,
    },
];

pub(crate) const ADPCM_FORMATS: [Format; 8] = [
    Format {
        channels: 1,
        bits: 4,
        rate: 8000,
    },
    Format {
        channels: 2,
        bits: 4,
        rate: 8000,
    },
    Format {
        channels: 1,
        bits: 4,
        rate: 11025,
    },
    Format {
        channels: 2,
        bits: 4,
        rate: 11025,
    },
    Format {
        channels: 1,
        bits: 4,
        rate: 22050,
    },
    Format {
        channels: 2,
        bits: 4,
        rate: 22050,
    },
    Format {
        channels: 1,
        bits: 4,
        rate: 44100,
    },
    Format {
        channels: 2,
        bits: 4,
        rate: 44100,
    },
];

/// The formats that `ADPCM_GetFormatIndex` accepts without a table row: a
/// mono or stereo PCM format whose average rate and block align follow from
/// the rate, or an ADPCM format with a 2-byte extension.
fn accepts_unlisted(wfx: &WaveFormat) -> bool {
    let channels = u32::from(wfx.channels);
    if !(1..3).contains(&channels) {
        return false;
    }
    match wfx.format_tag {
        WAVE_FORMAT_PCM => {
            wfx.avg_bytes_per_sec
                == 2u32
                    .wrapping_mul(wfx.samples_per_sec)
                    .wrapping_mul(channels)
                && u32::from(wfx.block_align) == 2 * channels
                && wfx.bits_per_sample == 16
        }
        _ => wfx.bits_per_sample == 4 && wfx.cb_size == 2,
    }
}

/// `ADPCM_GetFormatIndex`: the row of the PCM or ADPCM table that `wfx`
/// names, or the index one past the table for an unlisted format that
/// [`accepts_unlisted`] allows. `None` for any other format tag.
pub fn get_format_index(wfx: &WaveFormat) -> Option<u32> {
    let table: &[Format] = match wfx.format_tag {
        WAVE_FORMAT_PCM => &PCM_FORMATS,
        WAVE_FORMAT_IMA_ADPCM => &ADPCM_FORMATS,
        _ => return None,
    };
    table
        .iter()
        .position(|row| {
            row.channels == wfx.channels
                && row.rate == wfx.samples_per_sec
                && row.bits == wfx.bits_per_sample
        })
        .or_else(|| accepts_unlisted(wfx).then_some(table.len()))
        .map(|index| index as u32)
}

/// The bytes that one ADPCM block of `wfx` reads or writes: its header and its
/// nibbles. A block whose footprint is larger than its block align overlaps the
/// next block.
pub(crate) fn footprint(wfx: &WaveFormat) -> u32 {
    (u32::from(wfx.samples_per_block).wrapping_sub(1) / 2 + 4).wrapping_mul(u32::from(wfx.channels))
}

/// `init_wfx_ima_adpcm`: fills the block align, the samples per block and the
/// average rate of an IMA ADPCM format from its rate and channels. Wine returns
/// without a change when the tag or the format is wrong, which is an error here.
///
/// A block align that is too small gives zero samples per block, which Wine
/// divides by. That case sets the fields and then returns an error.
pub(crate) fn init_ima_adpcm(wfx: &mut WaveFormat) -> Result<(), Error> {
    if wfx.format_tag != WAVE_FORMAT_IMA_ADPCM || get_format_index(wfx).is_none() {
        return Err(Error::NotPossible);
    }
    let channels = wfx.channels;
    wfx.block_align = match wfx.samples_per_sec {
        8000 | 11025 => 256 * channels,
        22050 => 512 * channels,
        44100 => 1024 * channels,
        _ => wfx.block_align,
    };
    wfx.cb_size = 2;
    let samples =
        (i32::from(wfx.block_align) - 4 * i32::from(channels)) * (2 / i32::from(channels)) + 1;
    wfx.samples_per_block = samples as u16;
    if wfx.samples_per_block == 0 {
        return Err(Error::NotPossible);
    }
    wfx.avg_bytes_per_sec = wfx.samples_per_sec.wrapping_mul(u32::from(wfx.block_align))
        / u32::from(wfx.samples_per_block);
    Ok(())
}

/// `ADPCM_FormatDetails` arguments: the tag and index of a row to fill, and the
/// format that receives it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatDetails {
    pub format_tag: u32,
    pub format_index: u32,
    pub support: u32,
    pub format: WaveFormat,
}

/// `ADPCM_FormatDetails`: with [`ACM_FORMATDETAILSF_INDEX`], fills `format`
/// from the row `format_index` of the table for `format_tag`. With
/// [`ACM_FORMATDETAILSF_FORMAT`], checks that `format` has a table index.
pub fn format_details(details: &mut FormatDetails, query: u32) -> Result<(), Error> {
    match query {
        ACM_FORMATDETAILSF_FORMAT => {
            if get_format_index(&details.format).is_none() {
                return Err(Error::NotPossible);
            }
        }
        ACM_FORMATDETAILSF_INDEX => {
            details.format.format_tag = details.format_tag as u16;
            fill_by_index(details)?;
        }
        _ => return Err(Error::NotSupported),
    }
    details.support = ACM_DRIVERDETAILS_SUPPORTF_CODEC;
    Ok(())
}

fn fill_by_index(details: &mut FormatDetails) -> Result<(), Error> {
    let index = details.format_index as usize;
    match word_tag(details.format_tag) {
        Some(WAVE_FORMAT_PCM) => {
            let row = PCM_FORMATS.get(index).ok_or(Error::NotPossible)?;
            let wfx = &mut details.format;
            wfx.channels = row.channels;
            wfx.samples_per_sec = row.rate;
            wfx.bits_per_sample = row.bits;
            wfx.block_align = row.channels * row.bits / 8;
            wfx.avg_bytes_per_sec = wfx.samples_per_sec.wrapping_mul(u32::from(wfx.block_align));
        }
        Some(WAVE_FORMAT_IMA_ADPCM) => {
            let row = ADPCM_FORMATS.get(index).ok_or(Error::NotPossible)?;
            let wfx = &mut details.format;
            wfx.channels = row.channels;
            wfx.samples_per_sec = row.rate;
            wfx.bits_per_sample = row.bits;
            init_ima_adpcm(wfx).expect("table rows are valid IMA ADPCM formats");
        }
        _ => return Err(Error::InvalidParam),
    }
    Ok(())
}

/// `ADPCM_FormatSuggest`: fills the fields of `dst` that the flags in `fdw` do
/// not fix from `src`, then checks that the result has a table index.
/// `cb_src` and `cb_dst` are the sizes of the two format structures.
pub fn format_suggest(
    fdw: u32,
    cb_src: u32,
    cb_dst: u32,
    src: &WaveFormat,
    dst: &mut WaveFormat,
) -> Result<(), Error> {
    if cb_src < PCMWAVEFORMAT_SIZE
        || cb_dst < PCMWAVEFORMAT_SIZE
        || src.format_tag == dst.format_tag
        || get_format_index(src).is_none()
    {
        return Err(Error::NotPossible);
    }
    if fdw & ACM_FORMATSUGGESTF_NCHANNELS == 0 {
        dst.channels = src.channels;
    }
    if fdw & ACM_FORMATSUGGESTF_NSAMPLESPERSEC == 0 {
        dst.samples_per_sec = src.samples_per_sec;
    }
    if fdw & ACM_FORMATSUGGESTF_WBITSPERSAMPLE == 0 {
        dst.bits_per_sample = if src.format_tag == WAVE_FORMAT_PCM {
            4
        } else {
            16
        };
    }
    if fdw & ACM_FORMATSUGGESTF_WFORMATTAG == 0 {
        dst.format_tag = if src.format_tag == WAVE_FORMAT_PCM {
            WAVE_FORMAT_IMA_ADPCM
        } else {
            WAVE_FORMAT_PCM
        };
    }
    match dst.format_tag {
        WAVE_FORMAT_PCM => {
            if cb_src < IMAADPCMWAVEFORMAT_SIZE {
                return Err(Error::NotPossible);
            }
            dst.block_align = (u32::from(dst.channels) * u32::from(dst.bits_per_sample) / 8) as u16;
            dst.avg_bytes_per_sec = dst.samples_per_sec.wrapping_mul(u32::from(dst.block_align));
            if get_format_index(dst).is_none() {
                return Err(Error::NotPossible);
            }
        }
        WAVE_FORMAT_IMA_ADPCM => {
            if cb_dst < IMAADPCMWAVEFORMAT_SIZE {
                return Err(Error::NotPossible);
            }
            init_ima_adpcm(dst)?;
        }
        _ => return Err(Error::NotPossible),
    }
    Ok(())
}
