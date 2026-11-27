//! The 3D vector helpers from dlls/dsound/sound3d.c. The arithmetic is single precision, as in
//! the C code. The values are widened to double only where the C code widens them: RadToDeg
//! and the acos call.

/// D3DVECTOR: three single precision components.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// The zero vector when any component is NaN, as VectorProduct and VectorBetweenTwoPoints do.
fn zero_if_nan(c: Vector) -> Vector {
    if c.x.is_nan() || c.y.is_nan() || c.z.is_nan() {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    } else {
        c
    }
}

/// ScalarProduct. A NaN result is replaced by 0.
pub fn scalar_product(a: &Vector, b: &Vector) -> f32 {
    let c = (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
    if c.is_nan() {
        0.0
    } else {
        c
    }
}

/// VectorProduct: the cross product a x b.
pub fn vector_product(a: &Vector, b: &Vector) -> Vector {
    zero_if_nan(Vector {
        x: (a.y * b.z) - (a.z * b.y),
        y: (a.z * b.x) - (a.x * b.z),
        z: (a.x * b.y) - (a.y * b.x),
    })
}

/// VectorMagnitude: the length of a.
pub fn vector_magnitude(a: &Vector) -> f32 {
    scalar_product(a, a).sqrt()
}

/// RadToDeg. The product is computed in double precision and then rounded to single.
pub fn rad_to_deg(angle: f32) -> f32 {
    (f64::from(angle) * (360.0 / (2.0 * std::f64::consts::PI))) as f32
}

/// AngleBetweenVectorsRad. A zero vector gives 0. A cosine above 1 gives 0, and one below -1
/// gives pi, so the acos call only sees values in range.
pub fn angle_between_vectors_rad(a: &Vector, b: &Vector) -> f32 {
    let product = scalar_product(a, b);
    let la = vector_magnitude(a);
    let lb = vector_magnitude(b);
    if la == 0.0 || lb == 0.0 {
        return 0.0;
    }
    let cos = product / (la * lb);
    if cos > 1.0 {
        0.0
    } else if cos < -1.0 {
        std::f64::consts::PI as f32
    } else {
        f64::from(cos).acos() as f32
    }
}

/// AngleBetweenVectorsDeg: the angle of AngleBetweenVectorsRad, converted to degrees.
pub fn angle_between_vectors_deg(a: &Vector, b: &Vector) -> f32 {
    rad_to_deg(angle_between_vectors_rad(a, b))
}

/// VectorBetweenTwoPoints: the vector from a to b.
pub fn vector_between_two_points(a: &Vector, b: &Vector) -> Vector {
    zero_if_nan(Vector {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    })
}

/// ProjectVector: the length of the projection of a onto p, or 0 when p is zero.
pub fn project_vector(a: &Vector, p: &Vector) -> f32 {
    let prod = scalar_product(a, p);
    let mag = vector_magnitude(p);
    if mag != 0.0 {
        prod / mag
    } else {
        0.0
    }
}
