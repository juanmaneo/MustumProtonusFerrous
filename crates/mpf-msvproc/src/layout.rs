//! Plane layout of the video processor's frame buffers: the pitch and bottom-up adjustment that
//! Wine's `fill_arrays_with_format()` applies after `av_image_fill_arrays()`, and the alignment
//! check of `check_arrays_alignment()`.
//!
//! Plane addresses are `usize` values and 0 stands for a NULL plane. Like Wine, the pitch logic
//! stops at the first NULL plane.

/// Rescales the plane layout to the buffer `pitch` and, for a bottom-up buffer whose pitch is not
/// positive, flips the first plane so the image is read from its last row with a negative stride.
///
/// The `height` argument is the frame height, and `bottom_up` is the `MFVideoFlag_BottomUpLinearRep`
/// video flag. A zero `pitch` leaves the strides alone.
///
/// The row count of each plane is taken from the address of the previous plane after that plane
/// was already rescaled, and from the original address of the current plane. Wine does this, so
/// planes after the second one can land off their rescaled position.
pub fn apply_pitch(
    mut planes: [usize; 4],
    mut strides: [i32; 4],
    pitch: i32,
    height: u32,
    bottom_up: bool,
) -> ([usize; 4], [i32; 4]) {
    if pitch != 0 && pitch.abs() != strides[0] {
        let mut new_strides = [0i32; 4];
        for (new_stride, stride) in new_strides.iter_mut().zip(strides) {
            *new_stride = (pitch.abs() * stride) / strides[0];
        }
        for i in 1..4 {
            if planes[i] == 0 {
                break;
            }
            let rows = (planes[i] as isize - planes[i - 1] as isize) / strides[i - 1] as isize;
            planes[i] = planes[i - 1]
                .wrapping_add((rows as usize).wrapping_mul(new_strides[i - 1] as usize));
        }
        strides = new_strides;
    }

    if bottom_up && pitch <= 0 {
        planes[0] = planes[0]
            .wrapping_add((strides[0] as u32).wrapping_mul(height.wrapping_sub(1)) as usize);
        strides[0] = -strides[0];
    }

    (planes, strides)
}

/// True when `size`, every plane address and every stride are multiples of `align`, which must be
/// positive.
pub fn arrays_aligned(planes: [usize; 4], strides: [i32; 4], size: i32, align: i32) -> bool {
    size % align == 0
        && planes.iter().all(|&plane| plane % align as usize == 0)
        && strides.iter().all(|&stride| stride % align == 0)
}
