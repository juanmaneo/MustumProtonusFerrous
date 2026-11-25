use crate::{paper_to_media, DMORIENT_PORTRAIT};

/// `MulDiv` from `dlls/kernelbase/main.c`: `a * b / c`, rounded half away from zero.
/// Returns `-1` when `c` is 0 or the rounded result is outside `-2147483647..=2147483647`.
///
/// The C code negates `a` and `c` as `int`, which is undefined for `INT_MIN`. This port
/// widens to `i64` first, so those inputs give the exact quotient instead.
pub fn mul_div(a: i32, b: i32, c: i32) -> i32 {
    if c == 0 {
        return -1;
    }
    let (mut a, b, mut c) = (i64::from(a), i64::from(b), i64::from(c));
    if c < 0 {
        a = -a;
        c = -c;
    }
    let product = a * b;
    // A non-negative result (zero counts as non-negative) rounds up, otherwise down.
    let ret = if (a < 0) == (b < 0) {
        (product + c / 2) / c
    } else {
        (product - c / 2) / c
    };
    if ret.abs() > i64::from(i32::MAX) {
        -1
    } else {
        ret as i32
    }
}

/// `pixels_to_mm_x1000` from `ticket.c`: `pixels` at `dpi`, in thousandths of a millimetre.
pub fn pixels_to_mm_x1000(pixels: i32, dpi: i32) -> i32 {
    mul_div(pixels, 25400, dpi)
}

/// The `GetDeviceCaps` values that `write_PageImageableSize_caps` reads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DeviceMetrics {
    pub logpixels_x: i32,
    pub logpixels_y: i32,
    pub physical_width: i32,
    pub physical_height: i32,
    pub horz_res: i32,
    pub vert_res: i32,
    pub physical_offset_x: i32,
    pub physical_offset_y: i32,
}

/// The `psk:PageImageableSize` values, in thousandths of a millimetre.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ImageableArea {
    pub imageable_size_width: i32,
    pub imageable_size_height: i32,
    pub origin_width: i32,
    pub origin_height: i32,
    pub extent_width: i32,
    pub extent_height: i32,
}

/// The imageable size, origin and extent for a device and orientation, as
/// `write_PageImageableSize_caps` computes them. Portrait keeps the device axes;
/// any other orientation swaps them.
pub fn imageable_area(metrics: &DeviceMetrics, orientation: i32) -> ImageableArea {
    let res_x = metrics.logpixels_x;
    let res_y = metrics.logpixels_y;
    let phys_width = pixels_to_mm_x1000(metrics.physical_width, res_x);
    let phys_height = pixels_to_mm_x1000(metrics.physical_height, res_y);
    let width = pixels_to_mm_x1000(metrics.horz_res, res_x);
    let height = pixels_to_mm_x1000(metrics.vert_res, res_y);
    let offset_x = pixels_to_mm_x1000(metrics.physical_offset_x, res_x);
    let offset_y = pixels_to_mm_x1000(metrics.physical_offset_y, res_y);

    if orientation == DMORIENT_PORTRAIT {
        ImageableArea {
            imageable_size_width: phys_width,
            imageable_size_height: phys_height,
            origin_width: offset_x,
            origin_height: offset_y,
            extent_width: width,
            extent_height: height,
        }
    } else {
        ImageableArea {
            imageable_size_width: phys_height,
            imageable_size_height: phys_width,
            origin_width: offset_y,
            origin_height: offset_x,
            extent_width: height,
            extent_height: width,
        }
    }
}

/// One `psk:PageMediaSize` option: its media name, display name and size in micrometres.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaperOption<'a> {
    pub media: &'static str,
    pub display_name: &'a str,
    pub width: i32,
    pub height: i32,
}

/// The option list of `write_PageMediaSize_caps`. `papers` (`DC_PAPERS`), `sizes`
/// (`DC_PAPERSIZE`, in tenths of a millimetre) and `names` (`DC_PAPERNAMES`) are read
/// in parallel. Entries whose paper code has no media name are skipped.
///
/// Wine checks that the three lists have equal length. This port stops at the shortest.
/// Sizes are scaled by 100 to micrometres with `wrapping_mul`. The C multiplication
/// overflows as signed `int` (undefined behaviour), and gcc wraps it. Oracle row 24
/// shows that wrap, but the row is excluded from the vectors.
pub fn paper_size_options<'a>(
    papers: &[u16],
    sizes: &[(i32, i32)],
    names: &[&'a str],
) -> Vec<PaperOption<'a>> {
    papers
        .iter()
        .zip(sizes)
        .zip(names)
        .filter_map(|((&paper, &(x, y)), &display_name)| {
            Some(PaperOption {
                media: paper_to_media(i32::from(paper))?,
                display_name,
                width: x.wrapping_mul(100),
                height: y.wrapping_mul(100),
            })
        })
        .collect()
}
