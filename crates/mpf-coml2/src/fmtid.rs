//! The `FMTID` (GUID) value and its well-known property-set identifiers.

/// A `FMTID`, with the same fields as the Win32 `GUID`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fmtid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const FMTID_SUMMARY_INFORMATION: Fmtid = Fmtid {
    data1: 0xF29F_85E0,
    data2: 0x4FF9,
    data3: 0x1068,
    data4: [0xAB, 0x91, 0x08, 0x00, 0x2B, 0x27, 0xB3, 0xD9],
};

pub const FMTID_DOC_SUMMARY_INFORMATION: Fmtid = Fmtid {
    data1: 0xD5CD_D502,
    data2: 0x2E9C,
    data3: 0x101B,
    data4: [0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE],
};

pub const FMTID_USER_DEFINED_PROPERTIES: Fmtid = Fmtid {
    data1: 0xD5CD_D505,
    data2: 0x2E9C,
    data3: 0x101B,
    data4: [0x93, 0x97, 0x08, 0x00, 0x2B, 0x2C, 0xF9, 0xAE],
};

impl Fmtid {
    /// Reads the 16-byte in-memory representation of a FMTID on little-endian targets.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self {
            data1: u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            data2: u16::from_le_bytes([bytes[4], bytes[5]]),
            data3: u16::from_le_bytes([bytes[6], bytes[7]]),
            data4: [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
        }
    }

    /// Returns the 16-byte in-memory representation of a FMTID on little-endian targets.
    pub fn to_bytes(self) -> [u8; 16] {
        let data1 = self.data1.to_le_bytes();
        let data2 = self.data2.to_le_bytes();
        let data3 = self.data3.to_le_bytes();
        [
            data1[0],
            data1[1],
            data1[2],
            data1[3],
            data2[0],
            data2[1],
            data3[0],
            data3[1],
            self.data4[0],
            self.data4[1],
            self.data4[2],
            self.data4[3],
            self.data4[4],
            self.data4[5],
            self.data4[6],
            self.data4[7],
        ]
    }
}
