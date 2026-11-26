//! The `HRESULT` values, `GUID`s and `IsEqualGUID` that `inseng_main.c` uses.

/// `HRESULT`. The constants below are the unsigned bit patterns of the signed C values.
pub type HResult = u32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HResult = 0;
/// `S_FALSE` from `include/winerror.h`.
pub const S_FALSE: HResult = 1;
/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: HResult = 0x8000_4002;
/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HResult = 0x8000_4001;
/// `E_OUTOFMEMORY` from `include/winerror.h`.
pub const E_OUTOFMEMORY: HResult = 0x8007_000E;
/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: HResult = 0x8004_0111;

/// `GUID` from `include/guiddef.h`.
#[derive(PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `IID_IUnknown` from `include/unknwn.h`.
pub const IID_UNKNOWN: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// `IID_IClassFactory` from `include/unknwn.h`.
pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 1,
    data2: 0,
    data3: 0,
    data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
};

/// `IID_IInstallEngine` from `include/inseng.idl`.
pub const IID_INSTALL_ENGINE: Guid = Guid {
    data1: 0x6e44_9684,
    data2: 0xc509,
    data3: 0x11cf,
    data4: [0xaa, 0xfa, 0x00, 0xaa, 0x00, 0xb6, 0x01, 0x5c],
};

/// `IID_IInstallEngine2` from `include/inseng.idl`.
pub const IID_INSTALL_ENGINE2: Guid = Guid {
    data1: 0x6e44_9689,
    data2: 0xc509,
    data3: 0x11cf,
    data4: [0xaa, 0xfa, 0x00, 0xaa, 0x00, 0xb6, 0x01, 0x5c],
};

/// `CLSID_InstallEngine` from `include/inseng.idl`.
pub const CLSID_INSTALL_ENGINE: Guid = Guid {
    data1: 0x6e44_9686,
    data2: 0xc509,
    data3: 0x11cf,
    data4: [0xaa, 0xfa, 0x00, 0xaa, 0x00, 0xb6, 0x01, 0x5c],
};

/// `IsEqualGUID`: compares all 16 bytes of the two `GUID`s.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}
