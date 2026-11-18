//! The DMO media types of Wine's `resampler.c`: `check_dmo_media_type`,
//! `get_available_dmo_media_type` and the stream limits of `GetInputType` and `GetOutputType`.
//!
//! GUIDs are reduced to the comparisons the C code makes: the major type is audio or not, the
//! format type is `WAVEFORMATEX` or not, and the subtype is PCM, float or neither.

use crate::format::WAVE_FORMAT_EXTENSIBLE;
use crate::format::{
    WaveFormat, WAVEFORMATEXTENSIBLE_SIZE, WAVE_FORMAT_EX_SIZE, WAVE_FORMAT_IEEE_FLOAT,
    WAVE_FORMAT_PCM,
};
use crate::Error;

/// `ARRAY_SIZE(audio_formats)`: float and PCM.
const AUDIO_FORMAT_COUNT: u32 = 2;
/// The channel count and rate of the types `get_available_dmo_media_type` offers past the first two.
const AVAILABLE_CHANNELS: u16 = 2;
const AVAILABLE_RATE: u32 = 48000;

/// The major type: `MEDIATYPE_Audio` or another GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Major {
    Audio,
    Other,
}

/// The format type: `FORMAT_WaveFormatEx` or another GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatKind {
    WaveFormatEx,
    Other,
}

/// The subtype: `MFAudioFormat_PCM`, `MFAudioFormat_Float` or another GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subtype {
    Pcm,
    Float,
    Other,
}

/// A `DMO_MEDIA_TYPE` whose `pbFormat` buffer is `format` (`cbFormat` is its length).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaType {
    pub major: Major,
    pub format_kind: FormatKind,
    pub subtype: Subtype,
    pub format: Vec<u8>,
}

/// `check_dmo_media_type`: accepts an audio `WAVEFORMATEX` (or `WAVEFORMATEXTENSIBLE`) of PCM or
/// float with nonzero header fields, and returns its block align.
///
/// Deviation: an extensible buffer shorter than 40 bytes is rejected here. Wine accepts it and
/// then reads the channel mask and `SubFormat` past its end.
pub fn check_media_type(media: &MediaType) -> Result<u32, Error> {
    if media.major != Major::Audio || media.format_kind != FormatKind::WaveFormatEx {
        return Err(Error::InvalidType);
    }
    let wfx = WaveFormat::parse(&media.format).ok_or(Error::InvalidType)?;
    let short_extensible =
        wfx.tag == WAVE_FORMAT_EXTENSIBLE && media.format.len() < WAVEFORMATEXTENSIBLE_SIZE;
    if media.subtype == Subtype::Other
        || short_extensible
        || wfx.bits_per_sample == 0
        || wfx.avg_bytes_per_sec == 0
        || wfx.channels == 0
        || wfx.samples_per_sec == 0
        || wfx.block_align == 0
    {
        return Err(Error::InvalidType);
    }
    Ok(u32::from(wfx.block_align))
}

/// `get_available_dmo_media_type`: the type at `index`. Even indexes are float and odd ones PCM.
/// The first two indexes carry only the tag; later ones are 2 channels at 48000 Hz, 32-bit float
/// or 16-bit PCM.
pub fn available_type(index: u32) -> MediaType {
    let (subtype, tag, depth) = if index.is_multiple_of(2) {
        (Subtype::Float, WAVE_FORMAT_IEEE_FLOAT, 32)
    } else {
        (Subtype::Pcm, WAVE_FORMAT_PCM, 16)
    };
    let (channels, rate, bits) = if index < AUDIO_FORMAT_COUNT {
        (0, 0, 0)
    } else {
        (AVAILABLE_CHANNELS, AVAILABLE_RATE, depth as u16)
    };
    let block_align = bits * channels / 8;
    let avg_bytes = rate * u32::from(block_align);
    let mut format = Vec::with_capacity(WAVE_FORMAT_EX_SIZE);
    format.extend_from_slice(&(tag as u16).to_le_bytes());
    format.extend_from_slice(&channels.to_le_bytes());
    format.extend_from_slice(&rate.to_le_bytes());
    format.extend_from_slice(&avg_bytes.to_le_bytes());
    format.extend_from_slice(&block_align.to_le_bytes());
    format.extend_from_slice(&bits.to_le_bytes());
    format.extend_from_slice(&0u16.to_le_bytes());
    MediaType {
        major: Major::Audio,
        format_kind: FormatKind::WaveFormatEx,
        subtype,
        format,
    }
}

/// `media_object_GetInputType`: only stream 0 has types, and there are two of them.
pub fn get_input_type(stream: u32, type_index: u32) -> Result<MediaType, Error> {
    if stream > 0 {
        return Err(Error::InvalidStreamIndex);
    }
    if type_index >= AUDIO_FORMAT_COUNT {
        return Err(Error::NoMoreItems);
    }
    Ok(available_type(type_index))
}

/// `media_object_GetOutputType`: only stream 0 has types, and there are four of them.
pub fn get_output_type(stream: u32, type_index: u32) -> Result<MediaType, Error> {
    if stream > 0 {
        return Err(Error::InvalidStreamIndex);
    }
    if type_index >= 2 * AUDIO_FORMAT_COUNT {
        return Err(Error::NoMoreItems);
    }
    Ok(available_type(type_index))
}
