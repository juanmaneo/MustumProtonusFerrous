//! The format tables, the format index lookup (`ADPCM_GetFormatIndex`) and the
//! ADPCM header initialisation (`init_wfx_adpcm`).

use crate::{WAVE_FORMAT_ADPCM, WAVE_FORMAT_PCM};

/// `sizeof(ADPCMWAVEFORMAT) + 6 * sizeof(ADPCMCOEFSET)`: the header with all seven coefficient pairs.
pub const ADPCM_WAVEFORMAT_SIZE: u32 = 50;
/// `sizeof(PCMWAVEFORMAT)`.
pub const PCM_WAVEFORMAT_SIZE: u32 = 16;

/// One `ADPCMCOEFSET`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coef {
    pub i_coef1: i16,
    pub i_coef2: i16,
}

/// `WAVEFORMATEX` with the ADPCM fields, holding all seven coefficient pairs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
    pub samples_per_block: u16,
    pub num_coef: u16,
    pub coef: [Coef; 7],
}

/// `(channels, bits, rate)` rows of `PCM_Formats`.
pub(crate) const PCM_FORMATS: [(u16, u16, u32); 16] = [
    (1, 8, 8000),
    (2, 8, 8000),
    (1, 16, 8000),
    (2, 16, 8000),
    (1, 8, 11025),
    (2, 8, 11025),
    (1, 16, 11025),
    (2, 16, 11025),
    (1, 8, 22050),
    (2, 8, 22050),
    (1, 16, 22050),
    (2, 16, 22050),
    (1, 8, 44100),
    (2, 8, 44100),
    (1, 16, 44100),
    (2, 16, 44100),
];

/// `(channels, bits, rate)` rows of `ADPCM_Formats`.
pub(crate) const ADPCM_FORMATS: [(u16, u16, u32); 8] = [
    (1, 4, 8000),
    (2, 4, 8000),
    (1, 4, 11025),
    (2, 4, 11025),
    (1, 4, 22050),
    (2, 4, 22050),
    (1, 4, 44100),
    (2, 4, 44100),
];

/// `MSADPCM_CoeffSet`.
pub(crate) const MS_COEFS: [Coef; 7] = [
    Coef {
        i_coef1: 256,
        i_coef2: 0,
    },
    Coef {
        i_coef1: 512,
        i_coef2: -256,
    },
    Coef {
        i_coef1: 0,
        i_coef2: 0,
    },
    Coef {
        i_coef1: 192,
        i_coef2: 64,
    },
    Coef {
        i_coef1: 240,
        i_coef2: 0,
    },
    Coef {
        i_coef1: 460,
        i_coef2: -208,
    },
    Coef {
        i_coef1: 392,
        i_coef2: -232,
    },
];

/// `ADPCM_GetFormatIndex`: the table row matching the channels, rate and bits,
/// or the row past the table for a mono or stereo layout that Wine accepts
/// without a table row. `None` for other tags and other layouts.
pub fn format_index(fmt: &WaveFormat) -> Option<u32> {
    let table: &[(u16, u16, u32)] = match fmt.format_tag {
        WAVE_FORMAT_PCM => &PCM_FORMATS,
        WAVE_FORMAT_ADPCM => &ADPCM_FORMATS,
        _ => return None,
    };
    let row = table.iter().position(|&(channels, bits, rate)| {
        fmt.channels == channels && fmt.samples_per_sec == rate && fmt.bits_per_sample == bits
    });
    if let Some(row) = row {
        return Some(row as u32);
    }
    let mono_or_stereo = fmt.channels > 0 && fmt.channels < 3;
    let accepted = if fmt.format_tag == WAVE_FORMAT_PCM {
        mono_or_stereo
            && fmt
                .samples_per_sec
                .wrapping_mul(2)
                .wrapping_mul(u32::from(fmt.channels))
                == fmt.avg_bytes_per_sec
            && fmt.block_align == 2 * fmt.channels
            && fmt.bits_per_sample == 16
    } else {
        mono_or_stereo && fmt.bits_per_sample == 4 && fmt.cb_size == 32
    };
    accepted.then_some(table.len() as u32)
}

/// `init_wfx_adpcm`: fills the ADPCM fields of a format whose tag, channels,
/// rate and bits are set. A format without a valid index is left unchanged.
/// A zero samples-per-block leaves the average unchanged; callers reject it.
pub(crate) fn init_adpcm(fmt: &mut WaveFormat) {
    if format_index(fmt).is_none() {
        return;
    }
    let unit = match fmt.samples_per_sec {
        8000 | 11025 => Some(256),
        22050 => Some(512),
        44100 => Some(1024),
        _ => None,
    };
    if let Some(unit) = unit {
        fmt.block_align = unit * fmt.channels;
    }
    fmt.cb_size = 32;
    fmt.samples_per_block = samples_per_block(fmt.block_align, fmt.channels).unwrap_or(0);
    if let Some(avg) = fmt
        .samples_per_sec
        .wrapping_mul(u32::from(fmt.block_align))
        .checked_div(u32::from(fmt.samples_per_block))
    {
        fmt.avg_bytes_per_sec = avg;
    }
    fmt.num_coef = 7;
    fmt.coef = MS_COEFS;
}

/// `block_align * 2 / channels - 12`, stored as a WORD. `None` without channels.
pub(crate) fn samples_per_block(block_align: u16, channels: u16) -> Option<u16> {
    let half = (u32::from(block_align) * 2).checked_div(u32::from(channels))?;
    Some(half.wrapping_sub(12) as u16)
}
