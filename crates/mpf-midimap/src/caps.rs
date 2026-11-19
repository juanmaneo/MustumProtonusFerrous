//! The `MIDIOUTCAPSW` that the mapper reports for itself.

/// `sizeof(MIDIOUTCAPSW)` on x86-64: the largest prefix `MODM_GETDEVCAPS` writes.
pub const MIDIOUTCAPSW_SIZE: usize = 84;

const MOD_MAPPER: u16 = 5;
const MIDICAPS_VOLUME: u32 = 0x0001;
const MIDICAPS_LRVOLUME: u32 = 0x0002;
const PRODUCT_NAME: &str = "Wine midi mapper";

/// The 84-byte `MIDIOUTCAPSW` image, laid out as the C compiler lays it out.
pub fn mapper_caps() -> [u8; MIDIOUTCAPSW_SIZE] {
    let mut image = [0u8; MIDIOUTCAPSW_SIZE];
    image[0..2].copy_from_slice(&0x00FFu16.to_le_bytes());
    image[2..4].copy_from_slice(&0x0001u16.to_le_bytes());
    image[4..8].copy_from_slice(&0x0100u32.to_le_bytes());
    let (name_units, _) = image[8..72].as_chunks_mut::<2>();
    for (unit_bytes, unit) in name_units.iter_mut().zip(PRODUCT_NAME.encode_utf16()) {
        *unit_bytes = unit.to_le_bytes();
    }
    image[72..74].copy_from_slice(&MOD_MAPPER.to_le_bytes());
    image[78..80].copy_from_slice(&0xFFFFu16.to_le_bytes());
    image[80..84].copy_from_slice(&(MIDICAPS_VOLUME | MIDICAPS_LRVOLUME).to_le_bytes());
    image
}
