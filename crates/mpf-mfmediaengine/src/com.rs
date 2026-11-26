/// An `HRESULT`, kept as its unsigned bit pattern.
pub type HRESULT = u32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: HRESULT = 0;
/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: HRESULT = 0x8000_4001;
/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: HRESULT = 0x8000_4002;
/// `E_INVALIDARG` from `include/winerror.h`.
pub const E_INVALIDARG: HRESULT = 0x8007_0057;
/// `CLASS_E_NOAGGREGATION` from `include/winerror.h`.
pub const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110;
/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111;

/// A `GUID`. `IsEqualGUID` is `==`.
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
pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `IID_IMFMediaEngineClassFactory` from `include/mfmediaengine.idl`.
pub const IID_IMF_MEDIA_ENGINE_CLASS_FACTORY: Guid = Guid {
    data1: 0x4d64_5ace,
    data2: 0x26aa,
    data3: 0x4688,
    data4: [0x9b, 0xe1, 0xdf, 0x35, 0x16, 0x99, 0x0b, 0x93],
};

/// `IID_IMFMediaEngineClassFactoryEx` from `include/mfmediaengine.idl`.
pub const IID_IMF_MEDIA_ENGINE_CLASS_FACTORY_EX: Guid = Guid {
    data1: 0xc561_56c6,
    data2: 0xea5b,
    data3: 0x48a5,
    data4: [0x9d, 0xf8, 0xfb, 0xe0, 0x35, 0xd0, 0x92, 0x9e],
};

/// `CLSID_MFMediaEngineClassFactory` from `include/mfmediaengine.idl`.
pub const CLSID_MF_MEDIA_ENGINE_CLASS_FACTORY: Guid = Guid {
    data1: 0xb443_92da,
    data2: 0x499b,
    data3: 0x446b,
    data4: [0xa4, 0xcb, 0x00, 0x5f, 0xea, 0xd0, 0xe6, 0xd5],
};

/// The `IsEqualIID` chain that a `QueryInterface` runs over the IIDs it supports. `Ok` is `S_OK`
/// with the object handed out, and `Err` is `E_NOINTERFACE` with `*obj` set to NULL.
pub fn query_interface(riid: &Guid, supported: &[Guid]) -> Result<(), HRESULT> {
    if supported.contains(riid) {
        Ok(())
    } else {
        Err(E_NOINTERFACE)
    }
}
