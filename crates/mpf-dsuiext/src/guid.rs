//! The GUIDs the port compares against, from `unknwn.idl`, `dsclient.h` and `dsuiext.idl`.

/// `GUID`. `IsEqualGUID` is a byte comparison, which derived equality matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    /// `IsEqualGUID`, also used for `IsEqualIID` and `IsEqualCLSID`.
    pub fn is_equal(&self, other: &Guid) -> bool {
        self == other
    }
}

/// `IID_IUnknown` from `unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory` from `unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `CLSID_DsDisplaySpecifier` from `dsuiext.idl`.
pub const CLSID_DS_DISPLAY_SPECIFIER: Guid = Guid {
    data1: 0x1ab4_a8c0,
    data2: 0x6a0b,
    data3: 0x11d2,
    data4: [0xad, 0x49, 0x00, 0xc0, 0x4f, 0xa3, 0x1a, 0x86],
};

/// `IID_IDsDisplaySpecifier` is `#define`d to the CLSID in `dsclient.h`.
pub const IID_IDS_DISPLAY_SPECIFIER: Guid = CLSID_DS_DISPLAY_SPECIFIER;
