/// `GUID` from `include/guiddef.h`. It has no padding, so `==` compares the same 16 bytes as
/// `IsEqualGUID`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID` from `include/guiddef.h`: a byte-wise comparison of two GUIDs.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `CLSID_WindowsMediaPlayer` from `include/wmp.idl`.
pub const CLSID_WINDOWS_MEDIA_PLAYER: Guid = Guid {
    data1: 0x6bf5_2a52,
    data2: 0x394a,
    data3: 0x11d3,
    data4: [0xb1, 0x53, 0x00, 0xc0, 0x4f, 0x79, 0xfa, 0xa6],
};

/// `IID_IUnknown` from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory` from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};
