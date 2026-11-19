//! The WAVEFORMATEX fields that mciwave.c reads and writes, and the PCM fix-ups.

use crate::WAVE_FORMAT_PCM;

/// The WAVEFORMATEX of the open device.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

/// The format WAVE_mciDefaultFmt installs: 8-bit mono PCM at 11025 Hz.
pub const PCM_DEFAULT: WaveFormat = WaveFormat {
    format_tag: WAVE_FORMAT_PCM,
    channels: 1,
    samples_per_sec: 11025,
    avg_bytes_per_sec: 11025,
    block_align: 1,
    bits_per_sample: 8,
    cb_size: 0,
};

impl WaveFormat {
    pub fn is_pcm(&self) -> bool {
        self.format_tag == WAVE_FORMAT_PCM
    }

    /// The PCM fix-ups of WAVE_mciPlay and WAVE_mciCreateRIFFSkeleton: the block
    /// align follows from the channels and bits, and the average rate from the
    /// block align. Non-PCM formats are left as they are.
    pub fn normalize_pcm(&mut self) {
        if !self.is_pcm() {
            return;
        }
        self.block_align = (u32::from(self.channels) * u32::from(self.bits_per_sample) / 8) as u16;
        self.avg_bytes_per_sec = self
            .samples_per_sec
            .wrapping_mul(u32::from(self.block_align));
    }

    /// The size of the fmt chunk that WAVE_mciCreateRIFFSkeleton writes: a
    /// PCMWAVEFORMAT for PCM, otherwise WAVEFORMATEX plus its extra bytes.
    pub fn chunk_len(&self) -> u32 {
        if self.is_pcm() {
            16
        } else {
            18 + u32::from(self.cb_size)
        }
    }
}
