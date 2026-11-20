//! HRESULT mapping for HTTP status responses in `file.c`.

use crate::{HResult, S_OK};

pub const BG_E_HTTP_ERROR_400: HResult = 0x8019_0190_u32 as HResult;
pub const BG_E_HTTP_ERROR_401: HResult = 0x8019_0191_u32 as HResult;
pub const BG_E_HTTP_ERROR_404: HResult = 0x8019_0194_u32 as HResult;
pub const BG_E_HTTP_ERROR_407: HResult = 0x8019_0197_u32 as HResult;
pub const BG_E_HTTP_ERROR_414: HResult = 0x8019_019e_u32 as HResult;
pub const BG_E_HTTP_ERROR_501: HResult = 0x8019_01f5_u32 as HResult;
pub const BG_E_HTTP_ERROR_503: HResult = 0x8019_01f7_u32 as HResult;
pub const BG_E_HTTP_ERROR_504: HResult = 0x8019_01f8_u32 as HResult;
pub const BG_E_HTTP_ERROR_505: HResult = 0x8019_01f9_u32 as HResult;

/// Return Wine's BITS HRESULT for selected statuses; unhandled statuses succeed.
pub fn hresult_from_http_response(code: u32) -> HResult {
    match code {
        200 => S_OK,
        400 => BG_E_HTTP_ERROR_400,
        401 => BG_E_HTTP_ERROR_401,
        404 => BG_E_HTTP_ERROR_404,
        407 => BG_E_HTTP_ERROR_407,
        414 => BG_E_HTTP_ERROR_414,
        501 => BG_E_HTTP_ERROR_501,
        503 => BG_E_HTTP_ERROR_503,
        504 => BG_E_HTTP_ERROR_504,
        505 => BG_E_HTTP_ERROR_505,
        _ => S_OK,
    }
}
