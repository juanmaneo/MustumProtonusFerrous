/// `GpStatus` from `include/gdiplus.h`, limited to the values the ported code returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Status {
    Ok = 0,
    GenericError = 1,
    InvalidParameter = 2,
    OutOfMemory = 3,
    NotImplemented = 6,
}

/// `S_OK` from `include/winerror.h`.
const S_OK: i32 = 0;
/// `E_OUTOFMEMORY` from `include/winerror.h`, as the signed `HRESULT` value.
const E_OUTOFMEMORY: i32 = 0x8007_000e_u32 as i32;
/// `E_INVALIDARG` from `include/winerror.h`, as the signed `HRESULT` value.
const E_INVALIDARG: i32 = 0x8007_0057_u32 as i32;

/// `hresult_to_status` from `dlls/gdiplus/gdiplus.c`.
pub fn hresult_to_status(res: i32) -> Status {
    match res {
        S_OK => Status::Ok,
        E_OUTOFMEMORY => Status::OutOfMemory,
        E_INVALIDARG => Status::InvalidParameter,
        _ => Status::GenericError,
    }
}
