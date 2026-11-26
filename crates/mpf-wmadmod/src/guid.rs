/// `GUID`, with `Data1` to `Data3` and `Data4` in their declared order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID`: all 16 bytes are equal.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `IID_IUnknown` from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory` from `include/unknwn.idl`.
pub const IID_ICLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `CLSID_WMADecMediaObject` from `include/mfidl.idl`.
pub const CLSID_WMA_DEC_MEDIA_OBJECT: Guid = Guid {
    data1: 0x2eeb_4adf,
    data2: 0x4578,
    data3: 0x4d10,
    data4: [0xbc, 0xa7, 0xbb, 0x95, 0x5f, 0x56, 0x32, 0x0a],
};

/// `MFT_CATEGORY_AUDIO_DECODER` from `include/mfapi.h`.
pub const MFT_CATEGORY_AUDIO_DECODER: Guid = Guid {
    data1: 0x9ea7_3fb4,
    data2: 0xef7a,
    data3: 0x4559,
    data4: [0x8d, 0x5d, 0x71, 0x9d, 0x8f, 0x04, 0x26, 0xc7],
};

/// `DMOCATEGORY_AUDIO_DECODER` from `include/dmoreg.h`.
pub const DMO_CATEGORY_AUDIO_DECODER: Guid = Guid {
    data1: 0x57f2_db8b,
    data2: 0xe6bb,
    data3: 0x4513,
    data4: [0x9d, 0x43, 0xdc, 0xd2, 0xa6, 0x59, 0x31, 0x25],
};
