//! COM and Win32 primitives shared by the sti port: `HRESULT` values, GUIDs and
//! `HRESULT_FROM_WIN32`.

/// `HRESULT`, as a signed 32-bit value so that `FAILED` is a sign test.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000Eu32 as i32;
pub const CLASS_E_NOAGGREGATION: Hresult = 0x8004_0110u32 as i32;
pub const STG_E_UNIMPLEMENTEDFUNCTION: Hresult = 0x8003_00FEu32 as i32;

const FACILITY_WIN32: u32 = 7;

/// `GUID`. Compared by value, as `IsEqualGUID` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const IID_IUNKNOWN: Guid = Guid {
    data1: 0x0000_0000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_ICLASSFACTORY: Guid = Guid {
    data1: 0x0000_0001,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

pub const IID_ISTILLIMAGEW: Guid = Guid {
    data1: 0x641B_D880,
    data2: 0x2DC8,
    data3: 0x11D0,
    data4: [0x90, 0xEA, 0x00, 0xAA, 0x00, 0x60, 0xF8, 0x6C],
};

pub const IID_ISTILLIMAGEA: Guid = Guid {
    data1: 0xA7B1_F740,
    data2: 0x1D7F,
    data3: 0x11D1,
    data4: [0xAC, 0xA9, 0x00, 0xA0, 0x24, 0x38, 0xAD, 0x48],
};

pub const CLSID_STI: Guid = Guid {
    data1: 0xB323_F8E0,
    data2: 0x2E68,
    data3: 0x11D0,
    data4: [0x90, 0xEA, 0x00, 0xAA, 0x00, 0x60, 0xF8, 0x6C],
};

/// `HRESULT_FROM_WIN32` from `include/winerror.h`. A code that is not positive as an
/// `HRESULT` (so `ERROR_SUCCESS` as well) is returned unchanged.
pub fn hresult_from_win32(code: u32) -> Hresult {
    let signed = code as Hresult;
    if signed > 0 {
        ((code & 0x0000_FFFF) | (FACILITY_WIN32 << 16) | 0x8000_0000) as Hresult
    } else {
        signed
    }
}
