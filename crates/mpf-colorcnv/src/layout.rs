//! Plane layout of a frame buffer, as `fill_arrays_with_format` computes it.

use crate::{pixel_format_from_video_subtype, PixelFormat, VideoFormat, BOTTOM_UP_LINEAR_REP};

/// Plane offsets (`None` for unused planes), strides and the unscaled buffer size.
pub struct Filled {
    pub size: usize,
    pub planes: [Option<usize>; 4],
    pub strides: [i32; 4],
}

/// `(line length in bytes, row count)` for each plane of a pixel layout.
struct Geometry {
    planes: [(u128, u128); 4],
    count: usize,
}

fn geometry(pixel: PixelFormat, w: u128, h: u128) -> Option<Geometry> {
    let cw = w.div_ceil(2);
    let ch = h.div_ceil(2);
    let (planes, count) = match pixel {
        PixelFormat::Unsupported => return None,
        PixelFormat::Ayuv => ([(4 * w, h), (0, 0), (0, 0), (0, 0)], 1),
        PixelFormat::Yuv420p => ([(w, h), (cw, ch), (cw, ch), (0, 0)], 3),
        PixelFormat::Yuv411p => {
            let q = w.div_ceil(4);
            ([(w, h), (q, h), (q, h), (0, 0)], 3)
        }
        PixelFormat::Nv12 => ([(w, h), (2 * cw, ch), (0, 0), (0, 0)], 2),
        PixelFormat::Uyvy422 | PixelFormat::Yuyv422 | PixelFormat::Yvyu422 => {
            ([(4 * cw, h), (0, 0), (0, 0), (0, 0)], 1)
        }
        PixelFormat::Bgr24 => ([(3 * w, h), (0, 0), (0, 0), (0, 0)], 1),
        PixelFormat::Bgr0 => ([(4 * w, h), (0, 0), (0, 0), (0, 0)], 1),
        PixelFormat::Rgb555le | PixelFormat::Rgb565le => ([(2 * w, h), (0, 0), (0, 0), (0, 0)], 1),
        PixelFormat::Rgb8 => ([(w, h), (0, 0), (0, 0), (0, 0)], 1),
    };
    Some(Geometry { planes, count })
}

/// Computes plane offsets and strides for a frame of `format` stored in `buffer_len` bytes.
/// A non-zero `pitch` that differs from the packed line length rescales the strides of all
/// planes in proportion to the first one; the offsets then follow from the scaled strides.
pub fn fill(format: &VideoFormat, pitch: i32, buffer_len: usize) -> Option<Filled> {
    if format.width.min(format.height) == 0 || format.width.max(format.height) >= i32::MAX as u32 {
        return None;
    }
    let geo = geometry(
        pixel_format_from_video_subtype(&format.subtype),
        u128::from(format.width),
        u128::from(format.height),
    )?;
    let planes = &geo.planes[..geo.count];
    let size: u128 = planes.iter().map(|&(line, row)| line * row).sum();
    if size > i32::MAX as u128 || size > buffer_len as u128 {
        return None;
    }

    let mut strides = geo.planes.map(|(line, _)| line as i64);
    let line_pitch = i64::from(pitch).abs();
    if pitch != 0 && line_pitch != strides[0] {
        let first = strides[0];
        strides = strides.map(|stride| line_pitch * stride / first);
    }
    if !strides.iter().all(|&s| i32::try_from(s).is_ok()) {
        return None;
    }

    let mut offsets = [0i64; 4];
    let mut offset = 0;
    for (slot, (&(_, row), &stride)) in offsets.iter_mut().zip(planes.iter().zip(&strides)) {
        *slot = offset;
        offset += row as i64 * stride;
    }
    if (format.video_flags & BOTTOM_UP_LINEAR_REP) != 0 {
        offsets[0] = (i64::from(format.height) - 1) * strides[0];
        strides[0] = -strides[0];
    }

    Some(Filled {
        size: size as usize,
        planes: std::array::from_fn(|i| (i < geo.count).then(|| offsets[i] as usize)),
        strides: strides.map(|stride| stride as i32),
    })
}

/// Checks that the buffer size, every plane pointer and every stride are multiples of `align`.
pub fn check_arrays_alignment(
    planes: &[Option<usize>; 4],
    strides: &[i32; 4],
    size: usize,
    align: usize,
) -> bool {
    if align == 0 {
        return false;
    }
    size.is_multiple_of(align)
        && planes
            .iter()
            .flatten()
            .all(|plane| plane.is_multiple_of(align))
        && strides
            .iter()
            .all(|stride| (stride.unsigned_abs() as usize).is_multiple_of(align))
}
