//! `MFTransform::GetOutputAvailableType`: the audio output types the decoder offers.

use crate::types::WaveFormat;
use crate::{hresult, Hresult};

/// The sample format of an MF audio type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfSubtype {
    Pcm,
    Float,
}

/// An MF audio output type. Wine also sets `MF_MT_ALL_SAMPLES_INDEPENDENT` on every type, which
/// `all_samples_independent` records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MfAudioType {
    pub subtype: MfSubtype,
    pub bits_per_sample: u32,
    pub channels: u32,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_alignment: u32,
    pub all_samples_independent: bool,
}

/// The output type at `index` for stream `id`. Each triple of indices is float 32-bit, PCM 16-bit
/// and PCM 8-bit. Indices 0 to 2 keep the input's channel count and 3 to 5 are mono. An input that
/// is not stereo is offered only indices 0 to 2.
pub fn output_available_type(
    id: u32,
    input: Option<&WaveFormat>,
    index: u32,
) -> Result<MfAudioType, Hresult> {
    if id != 0 {
        return Err(hresult::MF_E_INVALIDSTREAMNUMBER);
    }
    let input = input.ok_or(hresult::MF_E_TRANSFORM_TYPE_NOT_SET)?;
    if index >= 6 || (input.channels != 2 && index >= 3) {
        return Err(hresult::MF_E_NO_MORE_TYPES);
    }
    let (subtype, bits_per_sample) = match index % 3 {
        0 => (MfSubtype::Float, 32),
        1 => (MfSubtype::Pcm, 16),
        _ => (MfSubtype::Pcm, 8),
    };
    let channels = if index >= 3 {
        1
    } else {
        u32::from(input.channels)
    };
    let block_alignment = bits_per_sample / 8 * channels;
    Ok(MfAudioType {
        subtype,
        bits_per_sample,
        channels,
        samples_per_sec: input.samples_per_sec,
        avg_bytes_per_sec: input.samples_per_sec.wrapping_mul(block_alignment),
        block_alignment,
        all_samples_independent: true,
    })
}
