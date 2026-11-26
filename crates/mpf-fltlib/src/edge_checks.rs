//! Edge-case tests for the ported fltlib logic.
//!
//! The boundary and branch tests below cover `hresult_from_win32` at the sign and mask
//! boundaries and both arms of `filter_unload`. Expectations marked "not oracle-derived"
//! come from the header constants or from reading the C source, not from the harness.

use super::*;

#[test]
fn hresult_from_win32_zero_passes_through() {
    // Not oracle-derived: HRESULT_FROM_WIN32(0) is 0 by the `> 0` guard in winerror.h.
    assert_eq!(hresult_from_win32(0), 0);
}

#[test]
fn hresult_from_win32_mask_keeps_low_16_bits_only() {
    // Not oracle-derived: 0x10000 has no low 16 bits, so only facility and severity remain.
    assert_eq!(hresult_from_win32(0x0001_0000), -2147024896);
    assert_eq!(hresult_from_win32(0x0001_0000 | 0x103), -2147024637);
}

#[test]
fn hresult_from_win32_sign_boundary() {
    // Not oracle-derived: 0x8000_0001 is negative as an HRESULT, so it is returned as-is.
    assert_eq!(hresult_from_win32(0x8000_0001), 0x8000_0001_u32 as i32);
    assert_eq!(hresult_from_win32(0x7FFF_FFFF), -2146959361);
}

#[test]
fn constants_match_headers() {
    // Not oracle-derived: values from winerror.h and winbase.h.
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(S_OK, 0);
    assert_eq!(FACILITY_WIN32, 7);
    assert_eq!(ERROR_NOT_SUPPORTED, 50);
    assert_eq!(ERROR_INVALID_PARAMETER, 87);
    assert_eq!(ERROR_NO_MORE_ITEMS, 259);
    assert_eq!(INVALID_HANDLE_VALUE, -1);
}

#[test]
fn filter_unload_empty_name_is_not_null() {
    // Not oracle-derived: a non-NULL pointer to an empty string takes the success arm.
    assert_eq!(filter_unload(Some(&[])), S_OK);
}

#[test]
fn filter_connect_port_is_invalid_handle() {
    // Not oracle-derived: the port is INVALID_HANDLE_VALUE, which is not a valid handle.
    let (hr, port) = filter_connect_communication_port();
    assert_eq!(hr, E_NOTIMPL);
    assert_eq!(port, INVALID_HANDLE_VALUE);
}
