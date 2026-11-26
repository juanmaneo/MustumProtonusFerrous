//! The COM types the dispatch needs: `GUID`, `HRESULT`, the out-pointer targets and `IsEqualGUID`.

/// `GUID`, from `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IID_IUnknown`, from `include/unknwn.idl`.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory`, from `include/unknwn.idl`.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IMMCVersionInfo`, from `include/mmc.idl`.
pub const IID_IMMCVERSIONINFO: Guid = Guid {
    data1: 0xa8d2_c5fe,
    data2: 0xcdcb,
    data3: 0x4b9d,
    data4: [0xbd, 0xe5, 0xa2, 0x73, 0x43, 0xff, 0x54, 0xbc],
};

/// `CLSID_MMCVersionInfo`, from `include/mmc.idl`.
pub const CLSID_MMCVERSIONINFO: Guid = Guid {
    data1: 0xd6fe_db1d,
    data2: 0xcf21,
    data3: 0x4bd9,
    data4: [0xaf, 0x3b, 0xc5, 0x46, 0x8e, 0x9c, 0x66, 0x84],
};

/// `HRESULT`, as a signed 32-bit value (the convention in `mpf-coremessaging`).
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

/// The object an out-pointer (`ppv`) receives on success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Object {
    /// The static `IMMCVersionInfo` object, `mmcVersionInfo` in the C source.
    MmcVersionInfo,
    /// The static class factory, `MMCVersionInfoFactory` in the C source.
    ClassFactory,
}

/// `IsEqualGUID`: every byte of both GUIDs must match.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}
