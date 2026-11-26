/// `HRESULT`, kept as the unsigned bit pattern that the C code prints.
pub type Hresult = u32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0x0000_0000;
/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: Hresult = 0x8000_4001;
/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: Hresult = 0x8000_4002;
/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111;

/// The `HRESULT` a `QueryInterface` returns: `S_OK` when the IID matched, else `E_NOINTERFACE`.
pub fn query_hresult(matched: bool) -> Hresult {
    if matched {
        S_OK
    } else {
        E_NOINTERFACE
    }
}
