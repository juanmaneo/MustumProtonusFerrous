use crate::{HResult, E_INVALIDARG};

/// `RECT`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// `MFVideoNormalizedRect`: a rectangle in the frame, with edges from 0 to 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NormalizedRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl NormalizedRect {
    /// The whole frame, which is the output rectangle of every input stream.
    pub const FULL: Self = Self {
        left: 0.0,
        top: 0.0,
        right: 1.0,
        bottom: 1.0,
    };
}

/// `SIZE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub cx: i32,
    pub cy: i32,
}

/// `video_mixer_scale_rect`: maps a normalized rectangle onto a frame of `width` by `height`.
///
/// The whole frame is returned without the float arithmetic, as in C. A NaN edge differs from
/// the whole frame, so it takes the scaling path, and the NaN converts to `0`.
pub fn scale_rect(width: u32, height: u32, scale: &NormalizedRect) -> Rect {
    if !rect_needs_scaling(scale) {
        return Rect {
            left: 0,
            top: 0,
            right: width as i32,
            bottom: height as i32,
        };
    }

    let w = width as f32;
    let h = height as f32;
    Rect {
        left: (w * scale.left) as i32,
        top: (h * scale.top) as i32,
        right: (w * scale.right) as i32,
        bottom: (h * scale.bottom) as i32,
    }
}

fn rect_needs_scaling(scale: &NormalizedRect) -> bool {
    scale.left != 0.0 || scale.top != 0.0 || scale.right != 1.0 || scale.bottom != 1.0
}

/// `video_mixer_correct_aspect_ratio`: shrinks `dst` along one axis so that `src` keeps its
/// shape inside it, and centres the shrunk rectangle on that axis.
///
/// The C body mixes signed and unsigned widths, and the port keeps each step at the same width.
/// The products and the sums wrap, as the unsigned C arithmetic does. Each division has a
/// non-zero divisor, because the branch that divides only runs when that size is non-zero.
pub fn correct_aspect_ratio(src: &Rect, dst: &mut Rect) {
    let src_w = src.right.wrapping_sub(src.left) as u32;
    let src_h = src.bottom.wrapping_sub(src.top) as u32;
    let dst_w = dst.right.wrapping_sub(dst.left) as u32;
    let dst_h = dst.bottom.wrapping_sub(dst.top) as u32;

    if src_w.wrapping_mul(dst_h) > dst_w.wrapping_mul(src_h) {
        let center = (dst.top.wrapping_add(dst.bottom) / 2) as u32;
        let scaled = src_h.wrapping_mul(dst_w) / src_w;
        dst.top = center.wrapping_sub(scaled / 2) as i32;
        dst.bottom = dst.top.wrapping_add(scaled as i32);
    } else if src_w.wrapping_mul(dst_h) < dst_w.wrapping_mul(src_h) {
        let center = (dst.left.wrapping_add(dst.right) / 2) as u32;
        let scaled = src_w.wrapping_mul(dst_h) / src_h;
        dst.left = center.wrapping_sub(scaled / 2) as i32;
        dst.right = dst.left.wrapping_add(scaled as i32);
    }
}

/// The rectangle check of `video_mixer_control_SetStreamOutputRect` and of
/// `video_presenter_control_SetVideoPosition`. A NaN edge fails each comparison, so it passes,
/// as it does in C.
pub fn check_normalized_rect(rect: &NormalizedRect) -> Result<(), HResult> {
    if rect.left > rect.right
        || rect.top > rect.bottom
        || rect.left < 0.0
        || rect.top < 0.0
        || rect.right > 1.0
        || rect.bottom > 1.0
    {
        return Err(E_INVALIDARG);
    }
    Ok(())
}
