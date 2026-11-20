//! Colour, quaternion and vector helpers of Wine's `dlls/d3drm/math.c` and the colour code of
//! `d3drm_private.h`.
//!
//! Arithmetic on `float` operands stays `f32`. Where the C multiplies by a `double` constant or
//! calls `sin`, `cos`, `acos` or `sqrt`, the port evaluates in `f64` and rounds to `f32` where the
//! C stores the result.

use crate::{Color, Matrix, Quaternion, Vector};

/// `D3DRMCreateColorRGB`: a colour with alpha 1.
pub fn color_rgb(red: f32, green: f32, blue: f32) -> Color {
    color_rgba(red, green, blue, 1.0)
}

/// `D3DRMCreateColorRGBA`. Each channel is clamped to `0..=255`, and a NaN channel gives 0.
///
/// Wine's `RGBA_MAKE` shifts the alpha byte into a signed `int`, which is undefined when the
/// alpha byte is 0x80 or above. The port packs the bits without that undefined shift, and the
/// oracle rows cover alpha below 0.5 only.
pub fn color_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
    (color_component(alpha) << 24)
        | (color_component(red) << 16)
        | (color_component(green) << 8)
        | color_component(blue)
}

/// `D3DRMColorGetAlpha`.
pub fn color_alpha(color: Color) -> f32 {
    byte_fraction(color >> 24)
}

/// `D3DRMColorGetRed`.
pub fn color_red(color: Color) -> f32 {
    byte_fraction((color >> 16) & 0xff)
}

/// `D3DRMColorGetGreen`.
pub fn color_green(color: Color) -> f32 {
    byte_fraction((color >> 8) & 0xff)
}

/// `D3DRMColorGetBlue`.
pub fn color_blue(color: Color) -> f32 {
    byte_fraction(color & 0xff)
}

/// `d3drm_color_component`. A NaN falls through and converts to 0.
fn color_component(c: f32) -> u32 {
    if c <= 0.0 {
        0
    } else if c >= 1.0 {
        255
    } else {
        (c * 255.0).floor() as u32
    }
}

fn byte_fraction(byte: u32) -> f32 {
    (f64::from(byte) / 255.0) as f32
}

/// `D3DRMQuaternionMultiply`: `a * b`.
pub fn quaternion_multiply(a: Quaternion, b: Quaternion) -> Quaternion {
    let cross = vector_cross(a.v, b.v);
    Quaternion {
        s: a.s * b.s - vector_dot(a.v, b.v),
        v: Vector {
            x: a.s * b.v.x + b.s * a.v.x + cross.x,
            y: a.s * b.v.y + b.s * a.v.y + cross.y,
            z: a.s * b.v.z + b.s * a.v.z + cross.z,
        },
    }
}

/// `D3DRMMatrixFromQuaternion`: the rotation matrix of a unit quaternion.
pub fn matrix_from_quaternion(q: Quaternion) -> Matrix {
    let (w, x, y, z) = (q.s, q.v.x, q.v.y, q.v.z);
    [
        [
            diagonal(y * y + z * z),
            off_diagonal(x * y - z * w),
            off_diagonal(x * z + y * w),
            0.0,
        ],
        [
            off_diagonal(x * y + z * w),
            diagonal(x * x + z * z),
            off_diagonal(y * z - x * w),
            0.0,
        ],
        [
            off_diagonal(x * z - y * w),
            off_diagonal(y * z + x * w),
            diagonal(x * x + y * y),
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// `1 - 2 * sum`, evaluated in `f64` as the C is.
fn diagonal(sum: f32) -> f32 {
    (1.0 - 2.0 * f64::from(sum)) as f32
}

/// `2 * sum`, evaluated in `f64` as the C is.
fn off_diagonal(sum: f32) -> f32 {
    (2.0 * f64::from(sum)) as f32
}

/// `D3DRMQuaternionFromRotation`: the unit quaternion for a rotation of `theta` about `axis`. The
/// axis is normalised in place, as in the C.
pub fn quaternion_from_rotation(axis: &mut Vector, theta: f32) -> Quaternion {
    let half = f64::from(theta) / 2.0;
    let s = half.cos() as f32;
    vector_normalize(axis);
    Quaternion {
        s,
        v: vector_scale(*axis, half.sin() as f32),
    }
}

/// `D3DRMQuaternionSlerp`: the interpolation from `a` to `b` at `alpha`. The spherical weights
/// apply when the quaternions are more than 0.001 apart, and linear weights otherwise.
pub fn quaternion_slerp(a: Quaternion, b: Quaternion, alpha: f32) -> Quaternion {
    let mut dot = a.s * b.s + vector_dot(a.v, b.v);
    let mut epsilon = 1.0_f32;
    let mut temp = 1.0 - alpha;
    let mut u = alpha;
    if dot < 0.0 {
        epsilon = -1.0;
        dot = -dot;
    }
    if 1.0 - dot > 0.001 {
        let theta = f64::from(dot).acos() as f32;
        temp = (f64::from(theta * temp).sin() / f64::from(theta).sin()) as f32;
        u = (f64::from(theta * alpha).sin() / f64::from(theta).sin()) as f32;
    }
    Quaternion {
        s: temp * a.s + epsilon * u * b.s,
        v: vector_add(vector_scale(a.v, temp), vector_scale(b.v, epsilon * u)),
    }
}

/// `D3DRMVectorAdd`.
pub fn vector_add(a: Vector, b: Vector) -> Vector {
    Vector {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

/// `D3DRMVectorSubtract`: `a - b`.
pub fn vector_subtract(a: Vector, b: Vector) -> Vector {
    Vector {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

/// `D3DRMVectorCrossProduct`.
pub fn vector_cross(a: Vector, b: Vector) -> Vector {
    Vector {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

/// `D3DRMVectorDotProduct`.
pub fn vector_dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// `D3DRMVectorModulus`. The square root is taken in `f64`.
pub fn vector_modulus(v: Vector) -> f32 {
    f64::from(vector_dot(v, v)).sqrt() as f32
}

/// `D3DRMVectorNormalize`: `u` scaled to unit length in place. A zero length gives `(1, 0, 0)`.
pub fn vector_normalize(u: &mut Vector) {
    let modulus = vector_modulus(*u);
    *u = if modulus != 0.0 {
        vector_scale(*u, (1.0 / f64::from(modulus)) as f32)
    } else {
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    };
}

/// `D3DRMVectorScale`: `v * factor`.
pub fn vector_scale(v: Vector, factor: f32) -> Vector {
    Vector {
        x: factor * v.x,
        y: factor * v.y,
        z: factor * v.z,
    }
}

/// `D3DRMVectorReflect`: `ray` reflected about the normal `norm`.
pub fn vector_reflect(ray: Vector, norm: Vector) -> Vector {
    let factor = (2.0 * f64::from(vector_dot(ray, norm))) as f32;
    vector_subtract(vector_scale(norm, factor), ray)
}

/// `D3DRMVectorRotate`: `v` rotated by `theta` about `axis`. The axis is normalised in place, and
/// the result is normalised, as in the C.
pub fn vector_rotate(v: Vector, axis: &mut Vector, theta: f32) -> Vector {
    let half = theta * 0.5;
    let c = f64::from(half).cos() as f32;
    let s = f64::from(half).sin() as f32;
    vector_normalize(axis);
    let q1 = Quaternion {
        s: c,
        v: vector_scale(*axis, s),
    };
    let q2 = Quaternion {
        s: c,
        v: vector_scale(*axis, -s),
    };
    let q3 = Quaternion { s: 0.0, v };
    let mut r = quaternion_multiply(quaternion_multiply(q1, q3), q2).v;
    vector_normalize(&mut r);
    r
}
