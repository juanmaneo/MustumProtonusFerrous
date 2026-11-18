//! Frame geometry: the padded row stride and the grid of 4x4 blocks that a Video 1 frame holds.

use crate::{IcError, IcResult};

/// `get_stride`: bytes per row of a `width`-pixel frame at `depth` bits per pixel, padded to a
/// multiple of four. Wraps on overflow, as C's `int` arithmetic does in practice.
pub fn get_stride(width: i32, depth: i32) -> i32 {
    (depth.wrapping_mul(width).wrapping_add(31) >> 3) & !3
}

/// Checks that `rows` rows of `row_len` bytes, `stride` bytes apart, fit in `available` items,
/// and returns the stride. `rows` must be at least one.
pub(crate) fn check_rows(
    rows: usize,
    stride: i32,
    row_len: usize,
    available: usize,
) -> IcResult<usize> {
    let Ok(stride) = usize::try_from(stride) else {
        return Err(IcError::BadSize);
    };
    let needed = (rows as u64 - 1) * stride as u64 + row_len as u64;
    if needed > available as u64 {
        return Err(IcError::BadSize);
    }
    Ok(stride)
}

/// The 4x4 blocks of a frame: `wide` blocks per block row, `high` block rows, and the row
/// `stride` in pixels. A frame with no whole block has no blocks and no stride.
pub(crate) struct Grid {
    pub(crate) wide: usize,
    pub(crate) high: usize,
    pub(crate) stride: usize,
}

impl Grid {
    /// Checks that `available` pixels hold every block of the frame, so that decoding cannot
    /// write outside them.
    pub(crate) fn new(width: i32, height: i32, stride: i32, available: usize) -> IcResult<Self> {
        let wide = (width.max(0) / 4) as usize;
        let high = (height.max(0) / 4) as usize;
        if wide.min(high) == 0 {
            return Ok(Self {
                wide,
                high,
                stride: 0,
            });
        }
        let stride = check_rows(4 * high, stride, 4 * wide, available)?;
        Ok(Self { wide, high, stride })
    }
}
