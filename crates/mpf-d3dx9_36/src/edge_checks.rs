//! Edge cases: NaN propagation, the zero and point-on-plane cases of the line intersection, the
//! signed zeros that the C arithmetic leaves in the trigonometric and inverse matrices. The
//! expected bit patterns are the output of `target/canon/d3dx9_36/edge_oracle`, which runs the
//! verbatim `math.c` on the same inputs.

use super::*;
use crate::oracle_checks::{assert_bits, xyz};

fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[test]
fn vec3_normalize_nan_length_divides() {
    let unit = vec3(f32::NAN, 0.0, 0.0).normalize();
    assert!(unit.x.is_nan() && unit.y.is_nan() && unit.z.is_nan());
}

#[test]
fn plane_normalize_nan_norm_divides() {
    let plane = Plane {
        a: f32::NAN,
        b: 0.0,
        c: 0.0,
        d: 2.0,
    }
    .normalize();
    assert!(plane.a.is_nan() && plane.b.is_nan() && plane.c.is_nan() && plane.d.is_nan());
}

#[test]
fn intersect_line_point_on_plane_returns_first_point() {
    let plane = Plane {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: -2.0,
    };
    let point = plane
        .intersect_line(vec3(2.0, 0.0, 0.0), vec3(3.0, 0.0, 0.0))
        .expect("the line is not parallel");
    assert_bits(&xyz(point), &[2.0, 0.0, 0.0]);
}

#[test]
fn intersect_line_zero_normal_is_parallel() {
    let plane = Plane {
        a: 0.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
    };
    let point = plane.intersect_line(vec3(2.0, 0.0, 0.0), vec3(3.0, 0.0, 0.0));
    assert!(point.is_none());
}

#[test]
fn cross_of_parallel_vectors_is_positive_zero() {
    let cross = vec3(1.0, 2.0, 3.0).cross(vec3(2.0, 4.0, 6.0));
    assert_bits(&xyz(cross), &[0.0, 0.0, 0.0]);
}

#[test]
fn rotation_x_at_zero_keeps_negative_zero() {
    let out = Matrix::rotation_x(0.0);
    assert_bits(
        out.m.as_flattened(),
        &[
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );
}

#[test]
fn rotation_y_at_zero_keeps_negative_zero() {
    let out = Matrix::rotation_y(0.0);
    assert_bits(
        out.m.as_flattened(),
        &[
            1.0, 0.0, -0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );
}

#[test]
fn rotation_z_at_zero_keeps_negative_zero() {
    let out = Matrix::rotation_z(0.0);
    assert_bits(
        out.m.as_flattened(),
        &[
            1.0, 0.0, 0.0, 0.0, -0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );
}

#[test]
fn inverse_with_nan_entry_is_nan_not_singular() {
    let mut m = Matrix { m: [[0.0; 4]; 4] };
    m.m[0][0] = f32::NAN;
    let (inv, det) = m
        .inverse()
        .expect("NaN is not zero, so the matrix is not singular");
    assert!(det.is_nan());
    assert!(inv.m.as_flattened().iter().all(|v| v.is_nan()));
}

#[test]
fn inverse_of_identity_keeps_negative_zero() {
    let (inv, det) = Matrix::identity()
        .inverse()
        .expect("identity is invertible");
    assert_bits(&[det], &[1.0]);
    assert_bits(
        inv.m.as_flattened(),
        &[
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -0.0, 1.0, 0.0, 0.0, 0.0, -0.0, 1.0,
        ],
    );
}

#[test]
fn multiply_identity_by_identity_is_identity() {
    let identity = Matrix::identity();
    let out = identity.multiply(identity);
    assert_bits(out.m.as_flattened(), identity.m.as_flattened());
}
