//! The format tables of Wine's `mpegl3.c`, and the `ACMDM_FORMAT_DETAILS` and
//! `ACMDM_FORMAT_SUGGEST` handlers. A format index is a row of these tables.

use crate::{
    word_tag, Error, WaveFormat, ACM_DRIVERDETAILS_SUPPORTF_CODEC, PCMWAVEFORMAT_SIZE,
    WAVE_FORMAT_MPEG, WAVE_FORMAT_MPEGLAYER3, WAVE_FORMAT_PCM,
};

pub const ACM_FORMATDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATDETAILSF_FORMAT: u32 = 1;

pub const ACM_FORMATSUGGESTF_WFORMATTAG: u32 = 0x0001_0000;
pub const ACM_FORMATSUGGESTF_NCHANNELS: u32 = 0x0002_0000;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: u32 = 0x0004_0000;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: u32 = 0x0008_0000;

/// One row of Wine's `PCM_Formats` or `MPEG3_Formats`. A `bits` of 0 matches
/// any sample size.
pub(crate) struct Format {
    channels: u16,
    bits: u16,
    rate: u32,
}

#[rustfmt::skip]
pub(crate) static PCM_FORMATS: [Format; 36] = [
    Format { channels: 1, bits: 8, rate: 8000 }, Format { channels: 2, bits: 8, rate: 8000 },
    Format { channels: 1, bits: 16, rate: 8000 }, Format { channels: 2, bits: 16, rate: 8000 },
    Format { channels: 1, bits: 8, rate: 11025 }, Format { channels: 2, bits: 8, rate: 11025 },
    Format { channels: 1, bits: 16, rate: 11025 }, Format { channels: 2, bits: 16, rate: 11025 },
    Format { channels: 1, bits: 8, rate: 12000 }, Format { channels: 2, bits: 8, rate: 12000 },
    Format { channels: 1, bits: 16, rate: 12000 }, Format { channels: 2, bits: 16, rate: 12000 },
    Format { channels: 1, bits: 8, rate: 16000 }, Format { channels: 2, bits: 8, rate: 16000 },
    Format { channels: 1, bits: 16, rate: 16000 }, Format { channels: 2, bits: 16, rate: 16000 },
    Format { channels: 1, bits: 8, rate: 22050 }, Format { channels: 2, bits: 8, rate: 22050 },
    Format { channels: 1, bits: 16, rate: 22050 }, Format { channels: 2, bits: 16, rate: 22050 },
    Format { channels: 1, bits: 8, rate: 24000 }, Format { channels: 2, bits: 8, rate: 24000 },
    Format { channels: 1, bits: 16, rate: 24000 }, Format { channels: 2, bits: 16, rate: 24000 },
    Format { channels: 1, bits: 8, rate: 32000 }, Format { channels: 2, bits: 8, rate: 32000 },
    Format { channels: 1, bits: 16, rate: 32000 }, Format { channels: 2, bits: 16, rate: 32000 },
    Format { channels: 1, bits: 8, rate: 44100 }, Format { channels: 2, bits: 8, rate: 44100 },
    Format { channels: 1, bits: 16, rate: 44100 }, Format { channels: 2, bits: 16, rate: 44100 },
    Format { channels: 1, bits: 8, rate: 48000 }, Format { channels: 2, bits: 8, rate: 48000 },
    Format { channels: 1, bits: 16, rate: 48000 }, Format { channels: 2, bits: 16, rate: 48000 },
];

#[rustfmt::skip]
pub(crate) static MPEG3_FORMATS: [Format; 18] = [
    Format { channels: 1, bits: 0, rate: 8000 }, Format { channels: 2, bits: 0, rate: 8000 },
    Format { channels: 1, bits: 0, rate: 11025 }, Format { channels: 2, bits: 0, rate: 11025 },
    Format { channels: 1, bits: 0, rate: 12000 }, Format { channels: 2, bits: 0, rate: 12000 },
    Format { channels: 1, bits: 0, rate: 16000 }, Format { channels: 2, bits: 0, rate: 16000 },
    Format { channels: 1, bits: 0, rate: 22050 }, Format { channels: 2, bits: 0, rate: 22050 },
    Format { channels: 1, bits: 0, rate: 24000 }, Format { channels: 2, bits: 0, rate: 24000 },
    Format { channels: 1, bits: 0, rate: 32000 }, Format { channels: 2, bits: 0, rate: 32000 },
    Format { channels: 1, bits: 0, rate: 44100 }, Format { channels: 2, bits: 0, rate: 44100 },
    Format { channels: 1, bits: 0, rate: 48000 }, Format { channels: 2, bits: 0, rate: 48000 },
];

/// `MPEG3_GetFormatIndex`: the row of the table for `format`, or `None` when
/// the tag, channels, rate or sample size match no row.
pub fn get_format_index(format: &WaveFormat) -> Option<u32> {
    let table: &[Format] = match format.format_tag {
        WAVE_FORMAT_PCM => &PCM_FORMATS,
        WAVE_FORMAT_MPEG | WAVE_FORMAT_MPEGLAYER3 => &MPEG3_FORMATS,
        _ => return None,
    };
    table
        .iter()
        .position(|row| {
            format.channels == row.channels
                && format.samples_per_sec == row.rate
                && (format.bits_per_sample == row.bits || row.bits == 0)
        })
        .map(|index| index as u32)
}

/// `ACMFORMATDETAILSW`: the query's tag and index, and the `WAVEFORMATEX` it fills.
pub struct FormatDetails {
    pub format_tag: u32,
    pub format_index: u32,
    pub support: u32,
    pub format: WaveFormat,
}

/// `MPEG3_FormatDetails`: for a format query, checks `details.format`. For an
/// index query, fills `details.format` from the PCM row `details.format_index`.
pub fn format_details(details: &mut FormatDetails, query: u32) -> Result<(), Error> {
    match query {
        ACM_FORMATDETAILSF_FORMAT => {
            if get_format_index(&details.format).is_none() {
                return Err(Error::NotPossible);
            }
        }
        ACM_FORMATDETAILSF_INDEX => {
            details.format.format_tag = details.format_tag as u16;
            match word_tag(details.format_tag) {
                Some(WAVE_FORMAT_PCM) => {
                    let row = PCM_FORMATS
                        .get(details.format_index as usize)
                        .ok_or(Error::NotPossible)?;
                    let format = &mut details.format;
                    format.channels = row.channels;
                    format.samples_per_sec = row.rate;
                    format.bits_per_sample = row.bits;
                    format.block_align = row.channels * row.bits / 8;
                    format.avg_bytes_per_sec = row.rate * u32::from(format.block_align);
                }
                Some(WAVE_FORMAT_MPEG | WAVE_FORMAT_MPEGLAYER3) => return Err(Error::NotPossible),
                _ => return Err(Error::InvalidParam),
            }
        }
        _ => return Err(Error::NotSupported),
    }
    details.support = ACM_DRIVERDETAILS_SUPPORTF_CODEC;
    Ok(())
}

/// `MPEG3_FormatSuggest`: fills the destination fields that the suggestion
/// flags leave open from `src`, then recomputes the PCM block fields. Only PCM
/// can be suggested as the destination. Fields of `dst` changed before an
/// error are kept, as in Wine.
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
    if (fdw_suggest & ACM_FORMATSUGGESTF_NCHANNELS) == 0 {
        dst.channels = src.channels;
    }
    if (fdw_suggest & ACM_FORMATSUGGESTF_NSAMPLESPERSEC) == 0 {
        dst.samples_per_sec = src.samples_per_sec;
    }
    if (fdw_suggest & ACM_FORMATSUGGESTF_WBITSPERSAMPLE) == 0 {
        dst.bits_per_sample = 16;
    }
    if (fdw_suggest & ACM_FORMATSUGGESTF_WFORMATTAG) == 0 {
        if src.format_tag == WAVE_FORMAT_PCM {
            return Err(Error::NotPossible);
        }
        dst.format_tag = WAVE_FORMAT_PCM;
    }
    if get_format_index(dst).is_none() || dst.format_tag != WAVE_FORMAT_PCM {
        return Err(Error::NotPossible);
    }
    dst.block_align = dst.channels * dst.bits_per_sample / 8;
    dst.avg_bytes_per_sec = dst.samples_per_sec * u32::from(dst.block_align);
    Ok(())
}
