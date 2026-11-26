//! COM values from `include/winerror.h`, `include/guiddef.h` and `include/peninputpanel.idl`.

/// `HRESULT`, unsigned so the constants below read the way Wine prints them.
pub type HRESULT = u32;

pub const S_OK: HRESULT = 0x0000_0000;
pub const E_NOINTERFACE: HRESULT = 0x8000_4002;
pub const E_NOTIMPL: HRESULT = 0x8000_4001;
pub const E_OUTOFMEMORY: HRESULT = 0x8007_000E;
pub const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111;

/// `GUID` from `include/guiddef.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const IID_UNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_CLASS_FACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

/// `ITextInputPanel`, from `include/peninputpanel.idl`.
pub const IID_TEXT_INPUT_PANEL: Guid = Guid {
    data1: 0x6b6a_65a5,
    data2: 0x6af3,
    data3: 0x46c2,
    data4: [0xb6, 0xea, 0x56, 0xcd, 0x1f, 0x80, 0xdf, 0x71],
};

/// `CLSID_TextInputPanel`, from `include/peninputpanel.idl`.
pub const CLSID_TEXT_INPUT_PANEL: Guid = Guid {
    data1: 0xf9b1_89d7,
    data2: 0x228b,
    data3: 0x4f2b,
    data4: [0x86, 0x50, 0xb9, 0x7f, 0x59, 0xe0, 0x2c, 0x8c],
};

/// `IsEqualGUID`, which `IsEqualIID` and `IsEqualCLSID` reduce to.
pub fn is_equal_guid(a: &Guid, b: &Guid) -> bool {
    a == b
}
