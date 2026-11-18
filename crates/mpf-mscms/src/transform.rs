//! Port of the pure parts of `dlls/mscms/transform.c`: the `BMFORMAT` and `COLORTYPE` mapping
//! to lcms pixel formats, the rendering intent and proofing flag of `CreateColorTransformW`,
//! and the argument checks of `CreateMultiProfileTransform`.

use crate::error::McsError;

pub const BM_RGBTRIPLETS: u32 = 2;
pub const BM_BGRTRIPLETS: u32 = 4;
pub const BM_XRGBQUADS: u32 = 8;
pub const BM_XBGRQUADS: u32 = 16;
pub const BM_GRAY: u32 = 521;
pub const BM_KYMCQUADS: u32 = 773;

pub const COLOR_GRAY: u32 = 1;
pub const COLOR_RGB: u32 = 2;
pub const COLOR_XYZ: u32 = 3;
pub const COLOR_YXY: u32 = 4;
pub const COLOR_LAB: u32 = 5;
pub const COLOR_CMYK: u32 = 7;

pub const INTENT_PERCEPTUAL: u32 = 0;
/// The intent lcms receives for the proofing leg of a proofing transform.
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3;
/// `cmsFLAGS_SOFTPROOFING`.
pub const CMS_FLAGS_SOFTPROOFING: u32 = 0x4000;

pub const TYPE_GRAY_8: u32 = 0x30009;
pub const TYPE_RGB_8: u32 = 0x40019;
pub const TYPE_BGR_8: u32 = 0x40419;
pub const TYPE_BGRA_8: u32 = 0x44499;
pub const TYPE_RGBA_8: u32 = 0x40099;
pub const TYPE_CMYK_8: u32 = 0x60021;
pub const TYPE_GRAY_16: u32 = 0x3000a;
pub const TYPE_RGB_16: u32 = 0x4001a;
pub const TYPE_XYZ_16: u32 = 0x9001a;
pub const TYPE_YXY_16: u32 = 0xe001a;
pub const TYPE_LAB_16: u32 = 0xa001a;
pub const TYPE_CMYK_16: u32 = 0x60022;

/// The lcms pixel format for a `BMFORMAT`. Unknown formats map to `TYPE_RGB_8`.
pub fn from_bmformat(format: u32) -> u32 {
    match format {
        BM_RGBTRIPLETS => TYPE_BGR_8,
        BM_BGRTRIPLETS => TYPE_RGB_8,
        BM_GRAY => TYPE_GRAY_8,
        BM_XRGBQUADS => TYPE_BGRA_8,
        BM_XBGRQUADS => TYPE_RGBA_8,
        BM_KYMCQUADS => TYPE_CMYK_8,
        _ => TYPE_RGB_8,
    }
}

/// The lcms pixel format for a `COLORTYPE`. Unknown colour types map to `TYPE_RGB_16`.
pub fn from_type(color_type: u32) -> u32 {
    match color_type {
        COLOR_GRAY => TYPE_GRAY_16,
        COLOR_RGB => TYPE_RGB_16,
        COLOR_XYZ => TYPE_XYZ_16,
        COLOR_YXY => TYPE_YXY_16,
        COLOR_LAB => TYPE_LAB_16,
        COLOR_CMYK => TYPE_CMYK_16,
        _ => TYPE_RGB_16,
    }
}

/// The rendering intent `CreateColorTransformW` passes on: intents above 3 fall back to perceptual.
pub fn rendering_intent(lcs_intent: u32) -> u32 {
    if lcs_intent > 3 {
        INTENT_PERCEPTUAL
    } else {
        lcs_intent
    }
}

/// The lcms flags for `CreateColorTransformW`: soft proofing when a target profile is given.
pub fn proofing_flags(has_target: bool) -> u32 {
    if has_target {
        CMS_FLAGS_SOFTPROOFING
    } else {
        0
    }
}

/// The argument checks of `CreateMultiProfileTransform`, in Wine's order. `nintents` must be 1
/// or equal `nprofiles`; fewer than two profiles is an error; more than two is refused silently.
pub fn check_multi_profile(
    has_profiles: bool,
    has_intents: bool,
    nprofiles: u32,
    nintents: u32,
) -> Result<(), McsError> {
    if !has_profiles || !has_intents || (nintents != 1 && nintents != nprofiles) {
        return Err(McsError::InvalidParameter);
    }
    if nprofiles < 2 {
        return Err(McsError::InvalidTransform);
    }
    if nprofiles > 2 {
        return Err(McsError::MoreThanTwoProfiles);
    }
    Ok(())
}
