//! Arc helpers of `geometry.c`: radius acceptance and the signed Bezier approximation distance.

/// `d2d_arc_check_radius`: returns the radius to use, or `None` when the arc is degenerate.
///
/// A radius is rejected when `radius * radius <= halfchord2 * fuzz2`. An accepted negative radius
/// is returned with its sign flipped.
pub fn check_radius(halfchord2: f32, fuzz2: f32, radius: f32) -> Option<f32> {
    if radius * radius <= halfchord2 * fuzz2 {
        None
    } else if radius < 0.0 {
        Some(-radius)
    } else {
        Some(radius)
    }
}

/// `d2d_arc_get_bezier_distance`: the signed distance of the Bezier approximation of an arc from
/// its chord, for the normalised dot product `r_dot` and the sweep direction.
///
/// The C original returns zero when `numerator <= denominator * 1e-6`. That test cannot succeed:
/// for `rA` in `[0, 1)`, `1 - rA` is at least `2^-24`, the numerator is at least `(2/3)(1 - rA)`,
/// and `(2/3)(1 - rA)` exceeds `sqrt(1 - rA) * 1e-6` once `1 - rA` exceeds `2.25e-12`. Larger and
/// negative `rA` return earlier, so the test is omitted.
pub fn bezier_distance(r_dot: f32, sweep_up: bool) -> f32 {
    let r_a = 0.5 * (1.0 + r_dot);
    if r_a < 0.0 {
        return 0.0;
    }

    let denom_squared = 1.0 - r_a;
    if denom_squared <= 0.0 {
        return 0.0;
    }

    let denominator = denom_squared.sqrt();
    let numerator = ((4.0_f32 / 3.0_f32) as f64 * (1.0 - (r_a as f64).sqrt())) as f32;

    let dist = numerator / denominator;
    if sweep_up {
        dist
    } else {
        -dist
    }
}
