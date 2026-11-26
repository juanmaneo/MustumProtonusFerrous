//! The `GUID`, `HRESULT` and interface values from the Windows headers that `dlls/msttsengine` includes, and
//! the `ComPtr` contract for the `ISpObjectToken` pointer that the engine holds.

/// A COM `GUID`, laid out as in the Windows header.
#[derive(PartialEq, Eq)]
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

/// A COM object pointer that the engine holds, such as the `ISpObjectToken *` that `SetObjectToken` stores.
/// `add_ref` and `release` stand in for the `IUnknown` calls that the C makes on it. The pointer is copied
/// freely, as a raw C pointer is, so the type is `Copy`.
pub trait ComPtr: Copy {
    /// `IUnknown::AddRef` on the pointee.
    fn add_ref(self);
    /// `IUnknown::Release` on the pointee.
    fn release(self);
}

/// `HRESULT`, a signed 32-bit value, so that `FAILED` is a sign test.
pub type Hresult = i32;

/// `S_OK` and `S_FALSE`, from `include/winerror.h`.
pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;

/// `E_*` and `CLASS_E_*` values from `include/winerror.h`, and `SPERR_ALREADY_INITIALIZED` from `include/sperror.h`.
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_POINTER: Hresult = 0x8000_4003u32 as i32;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as i32;
pub const E_INVALIDARG: Hresult = 0x8007_0057u32 as i32;
pub const CLASS_E_NOAGGREGATION: Hresult = 0x8004_0110u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;
pub const SPERR_ALREADY_INITIALIZED: Hresult = 0x8004_5002u32 as i32;

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

/// `IID_ISpTTSEngine`, from `include/sapiddk.idl`.
pub const IID_SP_TTS_ENGINE: Guid = Guid {
    data1: 0xa74d_7c8e,
    data2: 0x4cc5,
    data3: 0x4f2f,
    data4: [0xa6, 0xeb, 0x80, 0x4d, 0xee, 0x18, 0x50, 0x0e],
};

/// `IID_ISpObjectWithToken`, from `include/sapi.idl`.
pub const IID_SP_OBJECT_WITH_TOKEN: Guid = Guid {
    data1: 0x5b55_9f40,
    data2: 0xe952,
    data3: 0x11d2,
    data4: [0xbb, 0x91, 0x00, 0xc0, 0x4f, 0x8e, 0xe6, 0xc0],
};

/// `CLSID_TTSEngineCom`, from `dlls/msttsengine/ttseng_classes.idl`.
pub const CLSID_TTS_ENGINE_COM: Guid = Guid {
    data1: 0xc645_01f6,
    data2: 0xe6e6,
    data3: 0x451f,
    data4: [0xa1, 0x50, 0x25, 0xd0, 0x83, 0x9b, 0xc5, 0x10],
};
