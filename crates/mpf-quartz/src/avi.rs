//! Helpers of `dlls/quartz/avidec.c`: the bitmap info size, the late-sample drop test and the
//! rectangle test that selects `ICDecompressEx`.

use crate::{BitmapInfoHeader, Rect};

/// `BI_BITFIELDS`.
pub const BI_BITFIELDS: u32 = 3;

const DWORD_SIZE: u64 = 4;
const RGBQUAD_SIZE: u64 = 4;

/// `get_bitmap_info_size`: the header, then either three bit masks or the colour table.
pub fn bitmap_info_size(header: &BitmapInfoHeader) -> u64 {
    if header.compression == BI_BITFIELDS {
        u64::from(header.size) + 3 * DWORD_SIZE
    } else {
        u64::from(header.size) + u64::from(header.clr_used) * RGBQUAD_SIZE
    }
}

/// `AVIDec_DropSample`. While `late` is set, a sample that starts before `late` is dropped. The
/// first sample that is not dropped clears `late`.
pub fn drop_sample(late: &mut i64, start: i64) -> bool {
    if *late < 0 {
        return false;
    }
    if start < *late {
        return true;
    }
    *late = -1;
    false
}

/// `is_nontrivial_rect`. `left` and `top` must be zero. A zero `right` or `bottom` means the
/// image edge, so only a non-zero value that differs from the image size counts.
pub fn is_nontrivial_rect(header: &BitmapInfoHeader, rect: &Rect) -> bool {
    rect.left != 0
        || rect.top != 0
        || (rect.right != 0 && rect.right != header.width)
        || (rect.bottom != 0 && rect.bottom != header.height.wrapping_abs())
}

/// `needs_decompressex`. The `VIDEOINFOHEADER` and `VIDEOINFOHEADER2` branches of the C code read
/// the same three fields, so one function covers both.
pub fn needs_decompressex(header: &BitmapInfoHeader, source: &Rect, target: &Rect) -> bool {
    is_nontrivial_rect(header, source) || is_nontrivial_rect(header, target)
}
