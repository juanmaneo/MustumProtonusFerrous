pub const S_OK: u32 = 0;
pub const S_FALSE: u32 = 1;
pub const E_NOTIMPL: u32 = 0x8000_4001;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const E_INVALIDARG: u32 = 0x8007_0057;
pub const CLASS_E_NOAGGREGATION: u32 = 0x8004_0110;
pub const CLASS_E_CLASSNOTAVAILABLE: u32 = 0x8004_0111;
pub const XAUDIO2_E_INVALID_CALL: u32 = 0x8896_0001;

/// Port of the `FAILED` macro: a negative HRESULT.
pub fn failed(hr: u32) -> bool {
    (hr as i32) < 0
}
