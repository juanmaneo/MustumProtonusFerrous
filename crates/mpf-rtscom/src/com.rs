/// A `GUID`. Equality compares all 16 bytes, as `IsEqualGUID` does.
#[derive(PartialEq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID`.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `IUnknown`, from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IClassFactory`, from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IRealTimeStylus`, from `include/rtscom.idl`.
pub const IID_IREALTIMESTYLUS: Guid = Guid {
    data1: 0xa8bb_5d22,
    data2: 0x3144,
    data3: 0x4a7b,
    data4: [0x93, 0xcd, 0xf3, 0x4a, 0x16, 0xbe, 0x51, 0x3a],
};

/// `CLSID_RealTimeStylus`, from `dlls/rtscom/rtscom_reg.idl`.
pub const CLSID_REALTIMESTYLUS: Guid = Guid {
    data1: 0xe26b_366d,
    data2: 0xf998,
    data3: 0x43ce,
    data4: [0x83, 0x6f, 0xcb, 0x6d, 0x90, 0x44, 0x32, 0xb0],
};

/// The interface pointers that the C code hands out through `obj`.
pub enum Interface {
    RealTimeStylus,
    ClassFactory,
}
