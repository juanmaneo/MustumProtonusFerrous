//! `HRESULT` values returned by the geolocation exports.

/// `HRESULT`, as an unsigned bit pattern.
pub type Hresult = u32;

pub const S_OK: Hresult = 0x0000_0000;
pub const S_FALSE: Hresult = 0x0000_0001;
pub const E_NOINTERFACE: Hresult = 0x8000_4002;
pub const E_NOTIMPL: Hresult = 0x8000_4001;
pub const E_OUTOFMEMORY: Hresult = 0x8007_000E;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111;

/// `FAILED(hr)`: the severity bit is set.
pub fn failed(hr: Hresult) -> bool {
    (hr as i32) < 0
}
