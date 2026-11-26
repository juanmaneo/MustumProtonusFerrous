//! The COM types and the GUID and HRESULT values the port compares and returns.

/// `GUID` from `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `HRESULT`, a 32-bit signed value.
pub type HResult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HResult = 0;

/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HResult = 0x8000_4001_u32 as i32;

/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: HResult = 0x8000_4002_u32 as i32;

/// `E_OUTOFMEMORY` from `include/winerror.h`.
pub const E_OUTOFMEMORY: HResult = 0x8007_000e_u32 as i32;

/// `CLASS_E_NOAGGREGATION` from `include/winerror.h`.
pub const CLASS_E_NOAGGREGATION: HResult = 0x8004_0110_u32 as i32;

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: HResult = 0x8004_0111_u32 as i32;

/// `IID_IUnknown`, the standard COM interface ID.
pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IClassFactory`, the standard COM interface ID.
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_INetCfg` from `include/netcfgx.idl`.
pub const IID_INETCFG: Guid = Guid {
    data1: 0xc0e8_ae93,
    data2: 0x306e,
    data3: 0x11d1,
    data4: [0xaa, 0xcf, 0x00, 0x80, 0x5f, 0xc1, 0x27, 0x0e],
};

/// `IID_INetCfgLock` from `include/netcfgx.idl`.
pub const IID_INETCFGLOCK: Guid = Guid {
    data1: 0xc0e8_ae9f,
    data2: 0x306e,
    data3: 0x11d1,
    data4: [0xaa, 0xcf, 0x00, 0x80, 0x5f, 0xc1, 0x27, 0x0e],
};

/// `CLSID_CNetCfg` from `include/netcfgx.idl`.
pub const CLSID_CNETCFG: Guid = Guid {
    data1: 0x5b03_5261,
    data2: 0x40f9,
    data3: 0x11d1,
    data4: [0xaa, 0xec, 0x00, 0x80, 0x5f, 0xc1, 0x27, 0x0e],
};

/// `IsEqualGUID`: whether two GUIDs have the same value.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}
