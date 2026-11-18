//! The wave format reads of Wine's `resampler.c`: `sample_format_from_wave_format_tag`,
//! `init_audio_format` and the frame layout `audio_frame_init_from_format` derives from them.

/// `WAVE_FORMAT_PCM`.
pub const WAVE_FORMAT_PCM: u32 = 0x0001;
/// `WAVE_FORMAT_IEEE_FLOAT`.
pub const WAVE_FORMAT_IEEE_FLOAT: u32 = 0x0003;
/// `WAVE_FORMAT_EXTENSIBLE`.
pub const WAVE_FORMAT_EXTENSIBLE: u16 = 0xfffe;
/// `sizeof(WAVEFORMATEX)`.
pub const WAVE_FORMAT_EX_SIZE: usize = 18;
/// `sizeof(WAVEFORMATEXTENSIBLE)`.
pub const WAVEFORMATEXTENSIBLE_SIZE: usize = 40;

/// The sample formats the resampler carries, the packed `AVSampleFormat` values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    U8,
    S16,
    S32,
    Flt,
    Dbl,
}

impl SampleFormat {
    /// `av_get_bytes_per_sample`.
    pub fn bytes(self) -> usize {
        match self {
            SampleFormat::U8 => 1,
            SampleFormat::S16 => 2,
            SampleFormat::S32 | SampleFormat::Flt => 4,
            SampleFormat::Dbl => 8,
        }
    }
}

/// `sample_format_from_wave_format_tag`: the format of a tag and bit depth, or `None` where Wine
/// returns `AV_SAMPLE_FMT_NONE`.
pub fn sample_format(tag: u32, depth: u32) -> Option<SampleFormat> {
    match (tag, depth) {
        (WAVE_FORMAT_PCM, 8) => Some(SampleFormat::U8),
        (WAVE_FORMAT_PCM, 16) => Some(SampleFormat::S16),
        (WAVE_FORMAT_PCM, 32) => Some(SampleFormat::S32),
        (WAVE_FORMAT_IEEE_FLOAT, 32) => Some(SampleFormat::Flt),
        (WAVE_FORMAT_IEEE_FLOAT, 64) => Some(SampleFormat::Dbl),
        _ => None,
    }
}

/// The leading `WAVEFORMATEX` fields of a format buffer, in little-endian layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveFormat {
    pub tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

impl WaveFormat {
    /// Reads the 18-byte `WAVEFORMATEX` header; `None` when the buffer is shorter.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < WAVE_FORMAT_EX_SIZE {
            return None;
        }
        Some(Self {
            tag: le16(bytes, 0),
            channels: le16(bytes, 2),
            samples_per_sec: le32(bytes, 4),
            avg_bytes_per_sec: le32(bytes, 8),
            block_align: le16(bytes, 12),
            bits_per_sample: le16(bytes, 14),
        })
    }
}

/// The audio layout of a stream: the frame `init_audio_format` and `audio_frame_init_from_format`
/// build from a format buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AudioSpec {
    pub format: SampleFormat,
    pub channels: u32,
    pub sample_rate: u32,
    pub block_align: u32,
}

impl AudioSpec {
    /// Derives the layout of a `WAVEFORMATEX` or `WAVEFORMATEXTENSIBLE` buffer. An extensible
    /// format takes its sample format from the `SubFormat` GUID's `Data1` and its channel count
    /// from the popcount of `dwChannelMask`. `None` when Wine would fail to build a frame: no
    /// sample format, no channels, or a zero block align.
    pub fn from_format(bytes: &[u8]) -> Option<Self> {
        let wfx = WaveFormat::parse(bytes)?;
        let (tag, channels) = if wfx.tag == WAVE_FORMAT_EXTENSIBLE {
            if bytes.len() < WAVEFORMATEXTENSIBLE_SIZE {
                return None;
            }
            (le32(bytes, 24), le32(bytes, 20).count_ones())
        } else {
            (u32::from(wfx.tag), u32::from(wfx.channels))
        };
        let format = sample_format(tag, u32::from(wfx.bits_per_sample))?;
        if channels == 0 || wfx.block_align == 0 {
            return None;
        }
        Some(Self {
            format,
            channels,
            sample_rate: wfx.samples_per_sec,
            block_align: u32::from(wfx.block_align),
        })
    }

    /// Bytes of `samples` packed frames (`av_samples_get_buffer_size` with no alignment).
    pub fn frame_bytes(&self, samples: usize) -> usize {
        samples * self.channels as usize * self.format.bytes()
    }

    /// `audio_frame_wrap_buffer`: the sample count and byte size of a `len`-byte buffer viewed as
    /// frames of `block_align`, or `None` when those frames need more bytes than the buffer has.
    pub fn wrap(&self, len: usize) -> Option<(usize, usize)> {
        let samples = len / self.block_align as usize;
        let size = self.frame_bytes(samples);
        (size <= len).then_some((samples, size))
    }
}

pub(crate) fn le16(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

pub(crate) fn le32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}
