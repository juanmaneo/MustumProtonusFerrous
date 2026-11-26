use crate::guid::Guid;

/// `HRESULT`, a 32-bit signed value.
pub type Hresult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;
/// `E_NOTIMPL` from `include/winerror.h`.
pub const E_NOTIMPL: Hresult = 0x8000_4001_u32 as i32;
/// `E_NOINTERFACE` from `include/winerror.h`.
pub const E_NOINTERFACE: Hresult = 0x8000_4002_u32 as i32;
/// `E_OUTOFMEMORY` from `include/winerror.h`.
pub const E_OUTOFMEMORY: Hresult = 0x8007_000e_u32 as i32;
/// `CLASS_E_CLASSNOTAVAILABLE` from `include/winerror.h`.
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111_u32 as i32;

/// The `QueryInterface` check: `S_OK` when `iid` is one of `accepted`, else `E_NOINTERFACE`.
pub fn query_interface_hr(iid: &Guid, accepted: &[Guid]) -> Hresult {
    if accepted.contains(iid) {
        S_OK
    } else {
        E_NOINTERFACE
    }
}
