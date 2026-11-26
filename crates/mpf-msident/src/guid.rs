//! `GUID` and `IsEqualGUID` from Wine's `include/guiddef.h`, and the GUIDs that `dlls/msident/msident.c` compares,
//! from `include/unknwn.idl` and `include/msident.idl`.

/// `GUID`: the 16 bytes that `IsEqualGUID` compares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IsEqualGUID`. The C macro compares the 16 bytes with `memcmp`, which the derived `PartialEq` matches.
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
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IEnumUserIdentity` from `include/msident.idl`.
pub const IID_IENUM_USER_IDENTITY: Guid = Guid {
    data1: 0xa9ae_6c8f,
    data2: 0x1d1b,
    data3: 0x11d2,
    data4: [0xb2, 0x1a, 0x00, 0xc0, 0x4f, 0xa3, 0x57, 0xfa],
};

/// `IID_IUserIdentityManager` from `include/msident.idl`.
pub const IID_IUSER_IDENTITY_MANAGER: Guid = Guid {
    data1: 0xa9ae_6c90,
    data2: 0x1d1b,
    data3: 0x11d2,
    data4: [0xb2, 0x1a, 0x00, 0xc0, 0x4f, 0xa3, 0x57, 0xfa],
};

/// `CLSID_UserIdentityManager` from `include/msident.idl`.
pub const CLSID_USER_IDENTITY_MANAGER: Guid = Guid {
    data1: 0xa9ae_6c91,
    data2: 0x1d1b,
    data3: 0x11d2,
    data4: [0xb2, 0x1a, 0x00, 0xc0, 0x4f, 0xa3, 0x57, 0xfa],
};
