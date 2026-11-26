/// `HRESULT` from `include/winerror.h`. As a signed value, a negative code is a failure.
pub type HResult = u32;

pub const S_OK: HResult = 0x0000_0000;
pub const S_FALSE: HResult = 0x0000_0001;
pub const E_NOTIMPL: HResult = 0x8000_4001;
pub const E_NOINTERFACE: HResult = 0x8000_4002;
pub const E_FAIL: HResult = 0x8000_4005;
pub const E_UNEXPECTED: HResult = 0x8000_FFFF;
pub const E_ACCESSDENIED: HResult = 0x8007_0005;
pub const CLASS_E_CLASSNOTAVAILABLE: HResult = 0x8004_0111;

/// `FAILED(hr)` from `include/winerror.h`: `hr` is negative as a signed 32-bit value.
pub fn failed(hr: HResult) -> bool {
    (hr as i32) < 0
}
