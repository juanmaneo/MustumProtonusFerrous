/// The `WAVEFORMATEX` fields that `wodOpenHelper` / `widOpenHelper` derive for a PCM candidate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PcmFormat {
    pub samples_per_sec: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub avg_bytes_per_sec: u32,
}

impl PcmFormat {
    /// Derives `nBlockAlign` and `nAvgBytesPerSec` as the helpers do. The block align is
    /// computed in C `int` and stored in a `WORD`. The average is a `DWORD` product of the
    /// stored block align. Both wrap on overflow, which is what gcc produces on x86-64.
    pub fn new(samples_per_sec: u32, channels: u16, bits_per_sample: u16) -> Self {
        let block_align = (i32::from(channels).wrapping_mul(i32::from(bits_per_sample)) / 8) as u16;
        Self {
            samples_per_sec,
            channels,
            bits_per_sample,
            block_align,
            avg_bytes_per_sec: samples_per_sec.wrapping_mul(u32::from(block_align)),
        }
    }
}
