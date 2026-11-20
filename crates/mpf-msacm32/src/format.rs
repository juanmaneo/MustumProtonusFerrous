//! The PCM format table (`PCM_Formats`) and its index lookup (`PCM_GetFormatIndex`).

/// `sizeof(PCMWAVEFORMAT)`, the size of the format structure the PCM converter reports.
pub const PCM_WAVEFORMAT_SIZE: u32 = 16;

/// The fields of a `PCMWAVEFORMAT`, which is all the PCM converter reads or writes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaveFormat {
    pub format_tag: u16,
    pub channels: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

const RATES: [u32; 6] = [8000, 11025, 22050, 44100, 48000, 96000];
const LAYOUTS: [(u16, u16); 6] = [(1, 8), (2, 8), (1, 16), (2, 16), (1, 24), (2, 24)];

/// `PCM_Formats`: the 36 standard layouts as (channels, bits per sample, samples per
/// second), ordered by rate and then by layout.
pub(crate) const PCM_FORMATS: [(u16, u16, u32); 36] = {
    let mut table = [(0, 0, 0); 36];
    let mut i = 0;
    while i < table.len() {
        let (channels, bits) = LAYOUTS[i % LAYOUTS.len()];
        table[i] = (channels, bits, RATES[i / LAYOUTS.len()]);
        i += 1;
    }
    table
};

/// `PCM_GetFormatIndex`: the index of the standard layout with the same channels,
/// rate and bits per sample. The format tag and the block fields are not compared.
pub fn format_index(fmt: &WaveFormat) -> Option<u32> {
    PCM_FORMATS
        .iter()
        .position(|&(channels, bits, rate)| {
            channels == fmt.channels && bits == fmt.bits_per_sample && rate == fmt.samples_per_sec
        })
        .map(|i| i as u32)
}

/// Sets the block alignment and the average bytes per second from the channels,
/// the bits per sample and the rate, as Wine does for every format it reports.
pub(crate) fn fill_block_fields(fmt: &mut WaveFormat) {
    fmt.block_align = (u32::from(fmt.channels) * u32::from(fmt.bits_per_sample) / 8) as u16;
    fmt.avg_bytes_per_sec = fmt.samples_per_sec.wrapping_mul(u32::from(fmt.block_align));
}
