//! The wave format of an open synth (`synth_GetFormat`): 16-bit PCM with the synth's channel count
//! and sample rate.

use crate::port::PortParams;
use crate::Error;

pub const WAVE_FORMAT_PCM: u16 = 1;
/// `sizeof(WAVEFORMATEX)`, the size `synth_GetFormat` reports.
pub const WAVEFORMATEX_SIZE: u32 = 18;

/// The `WAVEFORMATEX` fields `synth_GetFormat` sets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveFormat {
    pub tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

/// The 16-bit PCM format for `channels` and `rate`. The `WORD` fields truncate, and the byte rate
/// wraps, as the unsigned C arithmetic does.
pub fn pcm16(channels: u32, rate: u32) -> WaveFormat {
    let block_align = (channels.wrapping_mul(16) / 8) as u16;
    WaveFormat {
        tag: WAVE_FORMAT_PCM,
        channels: channels as u16,
        samples_per_sec: rate,
        avg_bytes_per_sec: rate.wrapping_mul(u32::from(block_align)),
        block_align,
        bits_per_sample: 16,
        cb_size: 0,
    }
}

/// The format of an open synth, or `NotConfigured` when `current` is `None` (the synth is closed).
pub fn synth_format(current: Option<&PortParams>) -> Result<WaveFormat, Error> {
    let params = current.ok_or(Error::NotConfigured)?;
    Ok(pcm16(params.audio_channels, params.sample_rate))
}
