use std::f64::consts::{FRAC_PI_2, PI};

/// `gdip_round` from `dlls/gdiplus/gdiplus_private.h`: `(INT) floorf(x + 0.5)`.
///
/// The C conversion of an out-of-range or NaN value to `INT` is undefined. The port returns
/// `i32::MIN`, the `cvttss2si` result on x86-64, for those inputs.
pub fn gdip_round(x: f32) -> i32 {
    let rounded = ((f64::from(x) + 0.5) as f32).floor();
    if (-2_147_483_648.0_f32..2_147_483_648.0).contains(&rounded) {
        rounded as i32
    } else {
        i32::MIN
    }
}

/// `deg2rad` from `dlls/gdiplus/gdiplus_private.h`. The product is computed in double precision
/// and then rounded to `REAL`.
pub fn deg2rad(degrees: f32) -> f32 {
    (PI * f64::from(degrees) / 180.0) as f32
}

/// `gdiplus_atan2` from `dlls/gdiplus/gdiplus.c`: `atan2` that puts the angle in the right
/// quadrant when `dx` is zero.
pub fn gdiplus_atan2(dy: f32, dx: f32) -> f32 {
    if dx == 0.0 && dy != 0.0 {
        return if dy > 0.0 {
            FRAC_PI_2 as f32
        } else {
            (-FRAC_PI_2) as f32
        };
    }
    f64::from(dy).atan2(f64::from(dx)) as f32
}
