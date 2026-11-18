//! The `HRESULT` values the decoder returns, and the DMO-to-MF error mapping.

use crate::Hresult;

pub const E_FAIL: Hresult = 0x8000_4005;
pub const E_INVALIDARG: Hresult = 0x8007_0057;

pub const DMO_E_INVALIDSTREAMINDEX: Hresult = 0x8004_0201;
pub const DMO_E_INVALIDTYPE: Hresult = 0x8004_0202;
pub const DMO_E_TYPE_NOT_SET: Hresult = 0x8004_0203;
pub const DMO_E_NOTACCEPTING: Hresult = 0x8004_0204;
pub const DMO_E_TYPE_NOT_ACCEPTED: Hresult = 0x8004_0205;
pub const DMO_E_NO_MORE_ITEMS: Hresult = 0x8004_0206;

pub const MF_E_INVALIDSTREAMNUMBER: Hresult = 0xC00D_36B3;
pub const MF_E_INVALIDMEDIATYPE: Hresult = 0xC00D_36B4;
pub const MF_E_NOTACCEPTING: Hresult = 0xC00D_36B5;
pub const MF_E_NO_MORE_TYPES: Hresult = 0xC00D_36B9;
pub const MF_E_TRANSFORM_TYPE_NOT_SET: Hresult = 0xC00D_6D60;

/// Maps a DMO error to the MF error an MFTransform reports for it. Other codes pass through.
pub fn convert_dmo_to_mf_error(dmo_error: Hresult) -> Hresult {
    match dmo_error {
        DMO_E_INVALIDSTREAMINDEX => MF_E_INVALIDSTREAMNUMBER,
        DMO_E_INVALIDTYPE | DMO_E_TYPE_NOT_ACCEPTED => MF_E_INVALIDMEDIATYPE,
        DMO_E_TYPE_NOT_SET => MF_E_TRANSFORM_TYPE_NOT_SET,
        DMO_E_NOTACCEPTING => MF_E_NOTACCEPTING,
        DMO_E_NO_MORE_ITEMS => MF_E_NO_MORE_TYPES,
        _ => dmo_error,
    }
}
