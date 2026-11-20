use crate::geometry::{check_normalized_rect, NormalizedRect, Rect, Size};
use crate::{HResult, E_INVALIDARG, E_POINTER};

/// `MFVideoARMode_Mask`: the bits of an aspect ratio mode that are defined.
pub const AR_MODE_MASK: u32 = 0x7;

/// The native size of a frame, and its shape reduced to the smallest integer ratio.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeVideoSize {
    pub size: Size,
    pub ratio: Size,
}

/// `get_gcd`: the greatest common divisor by the Euclidean algorithm. `gcd(0, 0)` is `0`.
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    a
}

/// `video_presenter_get_native_video_size`: decodes the `MF_MT_FRAME_SIZE` attribute, packed as
/// width in the high half and height in the low half. A missing frame size gives zeros, as the
/// C body does when the mixer or its input type is missing. The ratio is zero when the size is.
pub fn native_video_size(frame: Option<u64>) -> NativeVideoSize {
    let (cx, cy) = frame.map_or((0, 0), |frame| ((frame >> 32) as u32, frame as u32));
    let divisor = gcd(cx, cy);
    let (ratio_cx, ratio_cy) = match (cx.checked_div(divisor), cy.checked_div(divisor)) {
        (Some(ratio_cx), Some(ratio_cy)) => (ratio_cx, ratio_cy),
        _ => (0, 0),
    };

    NativeVideoSize {
        size: Size {
            cx: cx as i32,
            cy: cy as i32,
        },
        ratio: Size {
            cx: ratio_cx as i32,
            cy: ratio_cy as i32,
        },
    }
}

/// `video_presenter_control_SetAspectRatioMode`: rejects a mode with a bit outside the mask.
pub fn check_aspect_ratio_mode(mode: u32) -> Result<(), HResult> {
    if (mode & !AR_MODE_MASK) != 0 {
        return Err(E_INVALIDARG);
    }
    Ok(())
}

/// The argument checks of `video_presenter_control_SetVideoPosition`. Both rectangles absent is
/// `E_POINTER`. The source is checked as in [`check_normalized_rect`], and the destination must
/// not be inverted.
pub fn check_video_position(
    src: Option<&NormalizedRect>,
    dst: Option<&Rect>,
) -> Result<(), HResult> {
    if src.is_none() && dst.is_none() {
        return Err(E_POINTER);
    }
    if let Some(src) = src {
        check_normalized_rect(src)?;
    }
    if dst.is_some_and(|dst| dst.left > dst.right || dst.top > dst.bottom) {
        return Err(E_INVALIDARG);
    }
    Ok(())
}
