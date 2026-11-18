//! Display aperture geometry of `MFVideoInfo`, as Wine's video processor reads and updates it:
//! `is_mf_video_area_empty()`, `get_mf_video_content_rect()`, `update_video_aperture()` and the
//! frame sizing of `video_frame_init_aperture()`.

use crate::{Rect, VideoInfo};

/// The content rectangle: the minimum display aperture when it is set, otherwise the full frame.
pub fn content_rect(info: &VideoInfo) -> Rect {
    let area = &info.minimum_display_aperture;
    if area.is_empty() {
        Rect {
            left: 0,
            top: 0,
            right: info.width as i32,
            bottom: info.height as i32,
        }
    } else {
        let left = i32::from(area.offset_x.value);
        let top = i32::from(area.offset_y.value);
        Rect {
            left,
            top,
            right: left + area.cx,
            bottom: top + area.cy,
        }
    }
}

/// The frame size `video_frame_init_aperture()` assigns: the aperture's far edges when it is set,
/// otherwise the full frame size.
pub fn frame_dimensions(info: &VideoInfo) -> (i32, i32) {
    let area = &info.minimum_display_aperture;
    if area.is_empty() {
        (info.width as i32, info.height as i32)
    } else {
        (
            i32::from(area.offset_x.value) + area.cx,
            i32::from(area.offset_y.value) + area.cy,
        )
    }
}

/// Makes the input's aperture describe its content rectangle and copies it to the output. Returns
/// true when the two content rectangles differ, the case Wine reports as a mismatch.
pub fn update_aperture(input: &mut VideoInfo, output: &mut VideoInfo) -> bool {
    let input_rect = content_rect(input);
    let mismatch = content_rect(output) != input_rect;

    let area = &mut input.minimum_display_aperture;
    area.offset_x.value = input_rect.left as i16;
    area.offset_y.value = input_rect.top as i16;
    area.cx = input_rect.right - input_rect.left;
    area.cy = input_rect.bottom - input_rect.top;
    output.minimum_display_aperture = input.minimum_display_aperture;

    mismatch
}
