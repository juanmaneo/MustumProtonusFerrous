//! GUIDs, and the `DEFINE_MEDIATYPE_GUID` pattern that builds the MF media type identifiers.

/// `GUID`, with the fields in their in-memory order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    /// The 16-byte little-endian layout of the first three fields, then `data4` as stored.
    pub fn to_bytes(self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[0..4].copy_from_slice(&self.data1.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.data2.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.data3.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.data4);
        bytes
    }
}

/// `DEFINE_MEDIATYPE_GUID(name, format)`: `data1` is the format value, the rest is the MF suffix
/// `0000-0010-8000-00aa00389b71`.
pub const fn media_guid(data1: u32) -> Guid {
    Guid {
        data1,
        data2: 0x0000,
        data3: 0x0010,
        data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
    }
}
