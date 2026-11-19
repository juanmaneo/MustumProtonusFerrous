//! Deterministic logic from Wine's `dlls/d2d1`: the adaptive orientation predicate and the stroke
//! hit tests of `geometry.c`, the arc radius and Bezier distance helpers of `geometry.c`, and the
//! bitmap format and option checks of `bitmap.c`.

mod arc;
mod bitmap;
mod expansion;
mod point;
mod stroke;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use arc::{bezier_distance, check_radius};
pub use bitmap::{
    check_bitmap_options, format_supported, PixelFormat, D2D1_ALPHA_MODE_IGNORE,
    D2D1_ALPHA_MODE_PREMULTIPLIED, D2D1_ALPHA_MODE_STRAIGHT, DXGI_FORMAT_A8_UNORM,
    DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM_SRGB, DXGI_FORMAT_B8G8R8X8_UNORM,
    DXGI_FORMAT_R16G16B16A16_FLOAT, DXGI_FORMAT_R16G16B16A16_UNORM, DXGI_FORMAT_R32G32B32A32_FLOAT,
    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
};
pub use point::{Bezier, Matrix3x2, Point};
pub use stroke::{point_on_bezier_segment, point_on_line_segment};

use crate::expansion::{
    estimate, fast_expansion_sum_zeroelim, two_diff_tail, two_product, two_two_diff,
};

/// `D2D_FP_EPS`: the `f32` unit roundoff `2^-24`.
const FP_EPS: f32 = 1.0 / 16_777_216.0;
const ERR_BOUND_RESULT: f32 = (3.0 + 8.0 * FP_EPS) * FP_EPS;
const ERR_BOUND_A: f32 = (3.0 + 16.0 * FP_EPS) * FP_EPS;
const ERR_BOUND_B: f32 = (2.0 + 12.0 * FP_EPS) * FP_EPS;
const ERR_BOUND_C: f32 = (9.0 + 64.0 * FP_EPS) * FP_EPS * FP_EPS;

/// Twice the signed area of the triangle `a`, `b`, `c`: positive when `c` is left of `a` to `b`.
///
/// The float filter returns early when the sign is certain. Otherwise the determinant is
/// refined exactly as an expansion, so the sign of a non-zero result is always correct.
pub fn ccw(a: Point, b: Point, c: Point) -> f32 {
    let ab_x = b.x - a.x;
    let ab_y = b.y - a.y;
    let ac_x = c.x - a.x;
    let ac_y = c.y - a.y;

    let abxacy = ab_x * ac_y;
    let abyacx = ab_y * ac_x;
    let mut det = abxacy - abyacx;

    let det_sum = if abxacy > 0.0 {
        if abyacx <= 0.0 {
            return det;
        }
        abxacy + abyacx
    } else if abxacy < 0.0 {
        if abyacx >= 0.0 {
            return det;
        }
        -abxacy - abyacx
    } else {
        return det;
    };

    let err_bound = ERR_BOUND_A * det_sum;
    if det >= err_bound || -det >= err_bound {
        return det;
    }

    let det_b = two_two_diff(two_product(ab_x, ac_y), two_product(ab_y, ac_x));
    det = estimate(&det_b);
    let err_bound = ERR_BOUND_B * det_sum;
    if det >= err_bound || -det >= err_bound {
        return det;
    }

    let ab_x_tail = two_diff_tail(b.x, a.x, ab_x);
    let ab_y_tail = two_diff_tail(b.y, a.y, ab_y);
    let ac_x_tail = two_diff_tail(c.x, a.x, ac_x);
    let ac_y_tail = two_diff_tail(c.y, a.y, ac_y);

    if ab_x_tail == 0.0 && ab_y_tail == 0.0 && ac_x_tail == 0.0 && ac_y_tail == 0.0 {
        return det;
    }

    let err_bound = ERR_BOUND_C * det_sum + ERR_BOUND_RESULT * det.abs();
    det += (ab_x * ac_y_tail + ac_y * ab_x_tail) - (ab_y * ac_x_tail + ac_x * ab_y_tail);
    if det >= err_bound || -det >= err_bound {
        return det;
    }

    let mut det_c1 = [0.0_f32; 8];
    let c1_len = fast_expansion_sum_zeroelim(
        &mut det_c1,
        &det_b,
        &two_two_diff(two_product(ab_x_tail, ac_y), two_product(ab_y_tail, ac_x)),
    );

    let mut det_c2 = [0.0_f32; 12];
    let c2_len = fast_expansion_sum_zeroelim(
        &mut det_c2,
        &det_c1[..c1_len],
        &two_two_diff(two_product(ab_x, ac_y_tail), two_product(ab_y, ac_x_tail)),
    );

    let mut det_d = [0.0_f32; 16];
    let d_len = fast_expansion_sum_zeroelim(
        &mut det_d,
        &det_c2[..c2_len],
        &two_two_diff(
            two_product(ab_x_tail, ac_y_tail),
            two_product(ab_y_tail, ac_x_tail),
        ),
    );

    det_d[d_len - 1]
}
