//! The format table, the index lookup (`G711_GetFormatIndex`) and the
//! `ACMDM_FORMAT_DETAILS` query (`G711_FormatDetails`).

use crate::{word_tag, Error, WaveFormat, WAVE_FORMAT_ALAW, WAVE_FORMAT_MULAW, WAVE_FORMAT_PCM};

pub const ACM_FORMATDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATDETAILSF_FORMAT: u32 = 1;

/// One listed format: its channel count, sample size in bits and sample rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Format {
    pub channels: u16,
    pub bits: u16,
    pub rate: u32,
}

/// Wine's `PCM_Formats`. Its 8-bit rows are commented out in the source.
pub const PCM_FORMATS: [Format; 8] = [
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
        bits: 16,
        rate: 44100,
    },
    Format {
        channels: 2,
        bits: 16,
        rate: 44100,
    },
];

/// Wine's `ALaw_Formats`.
pub const ALAW_FORMATS: [Format; 8] = [
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
        bits: 8,
        rate: 44100,
    },
    Format {
        channels: 2,
        bits: 8,
        rate: 44100,
    },
];

/// Wine's `ULaw_Formats`.
pub const ULAW_FORMATS: [Format; 8] = [
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
        bits: 8,
        rate: 44100,
    },
    Format {
        channels: 2,
        bits: 8,
        rate: 44100,
    },
];

/// The table that lists the G.711 format tag `format_tag`, if it is one.
pub fn table_for(format_tag: u32) -> Option<&'static [Format]> {
    match word_tag(format_tag)? {
        WAVE_FORMAT_PCM => Some(&PCM_FORMATS),
        WAVE_FORMAT_ALAW => Some(&ALAW_FORMATS),
        WAVE_FORMAT_MULAW => Some(&ULAW_FORMATS),
        _ => None,
    }
}

/// `G711_GetFormatIndex`: the position of `wfx` in its format table, if listed.
pub fn get_format_index(wfx: &WaveFormat) -> Option<usize> {
    table_for(u32::from(wfx.format_tag))?
        .iter()
        .position(|format| {
            format.channels == wfx.channels
                && format.rate == wfx.samples_per_sec
                && format.bits == wfx.bits_per_sample
        })
}

/// `G711_FormatDetails`. `ACM_FORMATDETAILSF_FORMAT` checks that `wfx` is
/// listed. `ACM_FORMATDETAILSF_INDEX` writes the listed format `format_index`
/// of `format_tag` into `wfx`. As in Wine, `wFormatTag` is stored before the
/// tag is checked, so an error can leave it changed.
pub fn format_details(
    query: u32,
    format_tag: u32,
    format_index: u32,
    wfx: &mut WaveFormat,
) -> Result<(), Error> {
    match query {
        ACM_FORMATDETAILSF_FORMAT => {
            if get_format_index(wfx).is_none() {
                return Err(Error::NotPossible);
            }
        }
        ACM_FORMATDETAILSF_INDEX => {
            wfx.format_tag = format_tag as u16;
            let table = table_for(format_tag).ok_or(Error::InvalidParam)?;
            let format = table.get(format_index as usize).ok_or(Error::NotPossible)?;
            wfx.channels = format.channels;
            wfx.samples_per_sec = format.rate;
            wfx.bits_per_sample = format.bits;
            if wfx.format_tag == WAVE_FORMAT_PCM {
                wfx.block_align = wfx.channels * 2;
                wfx.avg_bytes_per_sec =
                    wfx.samples_per_sec.wrapping_mul(u32::from(wfx.block_align));
            } else {
                wfx.block_align = wfx.channels;
                wfx.avg_bytes_per_sec = wfx.samples_per_sec.wrapping_mul(u32::from(wfx.channels));
                wfx.cb_size = 0;
            }
        }
        _ => return Err(Error::NotSupported),
    }
    Ok(())
}
