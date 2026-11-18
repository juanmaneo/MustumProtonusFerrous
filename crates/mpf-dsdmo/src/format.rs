//! The `WAVEFORMATEX` header of `dsdmo`: the fields `effect_SetInputType` reads, and the packed
//! 18-byte layout that `effect_SetOutputType` compares with `memcmp`.

/// `WAVE_FORMAT_PCM`.
pub const WAVE_FORMAT_PCM: u16 = 0x0001;
/// `WAVE_FORMAT_IEEE_FLOAT`.
pub const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;
/// `sizeof(WAVEFORMATEX)`: `mmreg.h` packs the struct to 18 bytes.
pub const WAVE_FORMAT_EX_SIZE: usize = 18;

/// The fields of a `WAVEFORMATEX`. Every field is an integer with no padding, so two values are
/// equal exactly when their 18-byte layouts are equal, which is what `memcmp` tests.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaveFormat {
    pub tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

impl WaveFormat {
    /// Reads the 18-byte header at the start of a format buffer. Bytes past it are ignored, as
    /// the C struct copy ignores them. `None` when the buffer is shorter than the header.
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
            cb_size: le16(bytes, 16),
        })
    }

    /// The packed little-endian layout of the header.
    pub fn to_bytes(self) -> [u8; WAVE_FORMAT_EX_SIZE] {
        let mut bytes = [0; WAVE_FORMAT_EX_SIZE];
        bytes[0..2].copy_from_slice(&self.tag.to_le_bytes());
        bytes[2..4].copy_from_slice(&self.channels.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.samples_per_sec.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.avg_bytes_per_sec.to_le_bytes());
        bytes[12..14].copy_from_slice(&self.block_align.to_le_bytes());
        bytes[14..16].copy_from_slice(&self.bits_per_sample.to_le_bytes());
        bytes[16..18].copy_from_slice(&self.cb_size.to_le_bytes());
        bytes
    }
}

fn le16(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn le32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}
