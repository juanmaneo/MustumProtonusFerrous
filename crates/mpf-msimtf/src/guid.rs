//! `GUID` values and `IsEqualGUID` from the headers that `dlls/msimtf` includes.

/// A COM `GUID`, laid out as in the Windows header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID`: compares all sixteen bytes, field by field.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}

/// `IID_IUnknown`, from `include/unknwn.idl`.
pub const IID_UNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory`, from `include/unknwn.idl`.
pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IActiveIMMApp`, from `include/dimm.idl`.
pub const IID_ACTIVE_IMM_APP: Guid = Guid {
    data1: 0x08C0_E040,
    data2: 0x62D1,
    data3: 0x11D1,
    data4: [0x93, 0x26, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E],
};

/// `IID_IActiveIMMMessagePumpOwner`, from `include/dimm.idl`.
pub const IID_ACTIVE_IMM_MESSAGE_PUMP_OWNER: Guid = Guid {
    data1: 0xB5CF_2CFA,
    data2: 0x8AEB,
    data3: 0x11D1,
    data4: [0x93, 0x64, 0x00, 0x60, 0xB0, 0x67, 0xB8, 0x6E],
};

/// `CLSID_CActiveIMM`, from `include/dimm.idl`.
pub const CLSID_CACTIVE_IMM: Guid = Guid {
    data1: 0x4955_DD33,
    data2: 0xB159,
    data3: 0x11D0,
    data4: [0x8F, 0xCF, 0x00, 0xAA, 0x00, 0x6B, 0xCC, 0x59],
};
