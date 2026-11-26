/// `GUID` from `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID` from `include/guiddef.h`.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `CLSID_WindowsParentalControls`, the `coclass` uuid in `include/wpcapi.idl`.
pub const CLSID_WINDOWS_PARENTAL_CONTROLS: Guid = Guid {
    data1: 0xe77c_c89b,
    data2: 0x7401,
    data3: 0x4c04,
    data4: [0x8c, 0xed, 0x14, 0x9d, 0xb3, 0x5a, 0xdd, 0x04],
};

/// `IID_IUnknown` from `include/unknwn.idl`.
pub const IID_UNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory` from `include/unknwn.idl`.
pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IWindowsParentalControlsCore` from `include/wpcapi.idl`.
pub const IID_WINDOWS_PARENTAL_CONTROLS_CORE: Guid = Guid {
    data1: 0x4ff4_0a0f,
    data2: 0x3f3b,
    data3: 0x4d7c,
    data4: [0xa4, 0x1b, 0x4f, 0x39, 0xd7, 0xb4, 0x4d, 0x05],
};

/// `IID_IWindowsParentalControls` from `include/wpcapi.idl`.
pub const IID_WINDOWS_PARENTAL_CONTROLS: Guid = Guid {
    data1: 0x28b4_d88b,
    data2: 0xe072,
    data3: 0x49e6,
    data4: [0x80, 0x4d, 0x26, 0xed, 0xbe, 0x21, 0xa7, 0xb9],
};
