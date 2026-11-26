//! Edge cases: NaN propagation, the zero and point-on-plane cases of the line intersection, the
//! signed zeros that the C arithmetic leaves in the trigonometric and inverse matrices, and the
//! quaternion boundaries. The quaternion expectations are the `edge outputs` section of
//! `target/canon/d3dx9_36/oracle_out.txt`, which runs the verbatim `math.c` on the same inputs.
//! The matrix and plane expectations come from an earlier edge driver whose output is not kept
//! in that directory. Expectations marked "not oracle-derived" follow from the C branches alone.

use super::*;
use crate::oracle_checks::{assert_bits, quat, xyz};

fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

fn quaternion(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
    Quaternion { x, y, z, w }
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

#[test]
fn quaternion_normalize_zero_divides_by_zero() {
    let out = quaternion(0.0, 0.0, 0.0, 0.0).normalize();
    assert!(quat(out).iter().all(|v| v.is_nan()));
}

#[test]
fn quaternion_inverse_zero_divides_by_zero() {
    let out = quaternion(0.0, 0.0, 0.0, 0.0).inverse();
    assert!(quat(out).iter().all(|v| v.is_nan()));
}

#[test]
fn quaternion_inverse_identity_keeps_negative_zero() {
    let out = quaternion(0.0, 0.0, 0.0, 1.0).inverse();
    assert_bits(&quat(out), &[-0.0, -0.0, -0.0, 1.0]);
}

#[test]
fn quaternion_exp_nan_vector_is_nan() {
    let out = quaternion(f32::NAN, 0.0, 0.0, 0.0).exp();
    assert!(quat(out).iter().all(|v| v.is_nan()));
}

#[test]
fn quaternion_ln_nan_w_gives_nan_vector_and_zero_w() {
    let out = quaternion(0.1, 0.2, 0.3, f32::NAN).ln();
    assert!(out.x.is_nan() && out.y.is_nan() && out.z.is_nan());
    assert_bits(&[out.w], &[0.0]);
}

#[test]
fn quaternion_ln_w_above_one_takes_limit_branch() {
    // Not oracle-derived: the oracle covers w = 1 and w = -1 only. The C test is `w >= 1.0f`,
    // so a larger w also takes t = 1 and keeps the vector part unchanged.
    let out = quaternion(0.2, 0.1, -0.3, 1.5).ln();
    assert_bits(&quat(out), &[0.2, 0.1, -0.3, 0.0]);
}

#[test]
fn quaternion_rotation_axis_zero_axis_keeps_only_w() {
    let out = Quaternion::rotation_axis(vec3(0.0, 0.0, 0.0), 1.2);
    assert_bits(&quat(out), &[0.0, 0.0, 0.0, 0.8253356]);
}

#[test]
fn quaternion_rotation_matrix_zero_trace_takes_else_branch() {
    let out = Quaternion::rotation_matrix(Matrix { m: [[0.0; 4]; 4] });
    assert_bits(&quat(out), &[0.5, 0.0, 0.0, 0.0]);
}

#[test]
fn quaternion_to_axis_angle_identity_has_zero_angle() {
    let (axis, angle) = quaternion(0.0, 0.0, 0.0, 1.0).to_axis_angle();
    assert_bits(&xyz(axis), &[0.0, 0.0, 0.0]);
    assert_bits(&[angle], &[0.0]);
}

#[test]
fn quaternion_to_axis_angle_w_minus_one_is_two_pi() {
    let (_, angle) = quaternion(0.2, 0.1, -0.3, -1.0).to_axis_angle();
    assert_bits(&[angle], &[6.2831855]);
}

#[test]
fn quaternion_slerp_t_zero_returns_first_operand() {
    let out = quaternion(0.0, 0.0, 0.0, 1.0).slerp(quaternion(0.6, 0.0, 0.0, 0.8), 0.0);
    assert_bits(&quat(out), &[0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn quaternion_squad_t_zero_returns_first_operand() {
    let out = quaternion(0.0, 0.0, 0.0, 1.0).squad(
        quaternion(0.6, 0.0, 0.0, 0.8),
        quaternion(0.0, 0.6, 0.0, -0.8),
        quaternion(0.0, 0.0, 0.6, -0.8),
        0.0,
    );
    assert_bits(&quat(out), &[0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn quaternion_bary_centric_zero_g_matches_slerp() {
    let out = quaternion(0.0, 0.0, 0.0, 1.0).bary_centric(
        quaternion(0.6, 0.0, 0.0, 0.8),
        quaternion(0.0, 0.6, 0.0, -0.8),
        0.3,
        0.0,
    );
    assert_bits(&quat(out), &[0.19185346, 0.0, 0.0, 0.9814236]);
}

#[test]
fn quaternion_multiply_identity_left_returns_right_operand() {
    let out = quaternion(0.0, 0.0, 0.0, 1.0).multiply(quaternion(0.5, -1.25, 2.0, 0.75));
    assert_bits(&quat(out), &[0.5, -1.25, 2.0, 0.75]);
}
