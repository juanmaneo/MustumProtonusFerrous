//! The DMO media types: the GUIDs they are matched by, `WAVEFORMATEX`, and the
//! `SetInputType`, `GetOutputType` and `SetOutputType` checks.

use crate::{check_stream, hresult, Hresult};

/// A 128-bit GUID, laid out as Windows' `GUID` struct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const GUID_NULL: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

/// `WMMEDIATYPE_Audio`.
pub const WMMEDIATYPE_AUDIO: Guid = Guid {
    data1: 0x7364_7561,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `WMMEDIASUBTYPE_MP3`.
pub const WMMEDIASUBTYPE_MP3: Guid = Guid {
    data1: 0x0000_0055,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

/// `WMFORMAT_WaveFormatEx`.
pub const WMFORMAT_WAVEFORMATEX: Guid = Guid {
    data1: 0x0558_9f81,
    data2: 0xc356,
    data3: 0x11ce,
    data4: [0xbf, 0x01, 0x00, 0xaa, 0x00, 0x55, 0x59, 0x5a],
};

/// `WAVE_FORMAT_PCM`, the only format tag the decoder outputs.
pub const WAVE_FORMAT_PCM: u16 = 1;

/// A `WAVEFORMATEX`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

/// The mpg123 encoding for an accepted output type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Unsigned8,
    Signed16,
    Float32,
}

/// `SetInputType`: accepts an MP3 audio input with at least one channel. The caller stores it.
pub fn set_input_type(
    stream: u32,
    major: Guid,
    subtype: Guid,
    formattype: Guid,
    format: &WaveFormat,
) -> Result<(), Hresult> {
    check_stream(stream)?;
    if major != WMMEDIATYPE_AUDIO
        || subtype != WMMEDIASUBTYPE_MP3
        || formattype != WMFORMAT_WAVEFORMATEX
        || format.channels == 0
    {
        return Err(hresult::DMO_E_TYPE_NOT_ACCEPTED);
    }
    Ok(())
}

/// `GetOutputType`: PCM output type `index` for the input `input`. Indices 0 and 1 keep the input's
/// channel count and later pairs are mono. Even indices are 16-bit and odd ones 8-bit.
pub fn output_type(
    stream: u32,
    input: Option<&WaveFormat>,
    index: u32,
) -> Result<WaveFormat, Hresult> {
    check_stream(stream)?;
    let input = input.ok_or(hresult::DMO_E_TYPE_NOT_SET)?;
    if index >= 2 * u32::from(input.channels) {
        return Err(hresult::DMO_E_NO_MORE_ITEMS);
    }
    let channels = if index >= 2 { 1 } else { input.channels };
    let bits_per_sample: u16 = if index % 2 == 1 { 8 } else { 16 };
    let block_align = (u32::from(channels) * u32::from(bits_per_sample) / 8) as u16;
    Ok(WaveFormat {
        format_tag: WAVE_FORMAT_PCM,
        channels,
        samples_per_sec: input.samples_per_sec,
        avg_bytes_per_sec: input.samples_per_sec.wrapping_mul(u32::from(block_align)),
        block_align,
        bits_per_sample,
        cb_size: 0,
    })
}

/// `SetOutputType`: checks `format` against the input type and returns the encoding the caller
/// passes to mpg123. The caller stores the type.
pub fn set_output_type(
    stream: u32,
    input: Option<&WaveFormat>,
    formattype: Guid,
    format: &WaveFormat,
) -> Result<Encoding, Hresult> {
    check_stream(stream)?;
    let input = input.ok_or(hresult::DMO_E_TYPE_NOT_SET)?;
    if formattype != WMFORMAT_WAVEFORMATEX {
        return Err(hresult::DMO_E_TYPE_NOT_ACCEPTED);
    }
    let encoding = match format.bits_per_sample {
        8 => Encoding::Unsigned8,
        16 => Encoding::Signed16,
        32 => Encoding::Float32,
        _ => return Err(hresult::DMO_E_TYPE_NOT_ACCEPTED),
    };
    let block_align = u32::from(format.channels) * u32::from(format.bits_per_sample) / 8;
    if block_align != u32::from(format.block_align)
        || format
            .samples_per_sec
            .wrapping_mul(u32::from(format.block_align))
            != format.avg_bytes_per_sec
    {
        return Err(hresult::E_INVALIDARG);
    }
    let rate = format.samples_per_sec;
    let input_rate = input.samples_per_sec;
    if rate != input_rate
        && rate.wrapping_mul(2) != input_rate
        && rate.wrapping_mul(4) != input_rate
    {
        return Err(hresult::DMO_E_TYPE_NOT_ACCEPTED);
    }
    Ok(encoding)
}
