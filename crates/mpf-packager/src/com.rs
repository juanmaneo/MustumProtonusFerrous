/// An `HRESULT`.
pub type HRESULT = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HRESULT = 0;

/// `S_FALSE` from `include/winerror.h`.
pub const S_FALSE: HRESULT = 1;

/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HRESULT = 0x8000_4001_u32 as i32;

/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: HRESULT = 0x8000_4002_u32 as i32;

/// `E_FAIL` from `include/winerror.h`.
pub const E_FAIL: HRESULT = 0x8000_4005_u32 as i32;

/// `E_OUTOFMEMORY` from `include/winerror.h`.
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000e_u32 as i32;

/// `E_INVALIDARG` from `include/winerror.h`.
pub const E_INVALIDARG: HRESULT = 0x8007_0057_u32 as i32;

/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111_u32 as i32;

/// `FAILED(hr)` from `include/winerror.h`: a negative `HRESULT` is a failure.
pub fn failed(hr: HRESULT) -> bool {
    hr < 0
}

/// A `GUID`. Equality compares every field, as `IsEqualGUID` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
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

/// `IID_IOleObject` from `include/oleidl.idl`.
pub const IID_IOLE_OBJECT: Guid = Guid {
    data1: 0x0000_0112,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IPersistStorage` from `include/objidl.idl`.
pub const IID_IPERSIST_STORAGE: Guid = Guid {
    data1: 0x0000_010a,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `CLSID_Package` from `dlls/packager/packager_classes.idl`.
pub const CLSID_PACKAGE: Guid = Guid {
    data1: 0xf20d_a720,
    data2: 0xc02f,
    data3: 0x11ce,
    data4: [0x92, 0x7b, 0x08, 0x00, 0x09, 0x5a, 0xe3, 0x40],
};
