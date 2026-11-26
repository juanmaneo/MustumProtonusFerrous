//! Replays of the oracle rows in `target/canon/d3dx9_36`. The oracle compiles the verbatim
//! `dlls/d3dx9_36/math.c` with `gcc -fsanitize=address,undefined` and prints each result. Each
//! run reported no sanitizer finding, so no vector is excluded. Each test names the row it
//! reproduces. Rows 25 to 47 are the quaternion slice; the quaternion harness prints the same
//! 23 rows as `row 01` to `row 23` in `target/canon/d3dx9_36/oracle_out.txt`. Rows 48 to 67 are
//! the vec2 and vec4 slice; the vector harness (`vec_oracle.c`) prints the same 20 rows as
//! `row 01` to `row 20` in `target/canon/d3dx9_36/vec_oracle_out.txt`. Comparisons are on the
//! bit pattern, so the sign of zero is checked as well.

use super::*;

const A: Matrix = Matrix {
    m: [
        [1.5, -2.0, 0.25, 3.0],
        [0.5, 4.0, -1.0, 2.5],
        [2.0, 0.75, 3.5, -0.5],
        [-1.25, 1.0, 2.0, 5.0],
    ],
};
const B: Matrix = Matrix {
    m: [
        [0.1, 2.0, -3.0, 1.0],
        [4.0, -0.2, 0.3, 0.0],
        [1.0, 1.0, 1.0, 1.0],
        [0.7, -1.0, 2.5, 3.0],
    ],
};
const S: Matrix = Matrix {
    m: [
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 6.0, 8.0],
        [0.0, 1.0, 0.0, 1.0],
        [1.0, 0.0, 1.0, 0.0],
    ],
};
const PL: Plane = Plane {
    a: 0.3,
    b: 0.9,
    c: -0.2,
    d: 1.5,
};
const FLAT: Plane = Plane {
    a: 0.0,
    b: 1.0,
    c: 0.0,
    d: -2.0,
};
const QA: Quaternion = Quaternion {
    x: 0.5,
    y: -1.25,
    z: 2.0,
    w: 0.75,
};
const QB: Quaternion = Quaternion {
    x: -0.3,
    y: 0.8,
    z: 0.1,
    w: -0.6,
};
const QV: Quaternion = Quaternion {
    x: 0.4,
    y: -0.7,
    z: 0.2,
    w: 0.9,
};
const ZQ: Quaternion = Quaternion {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 5.0,
};
const QU: Quaternion = Quaternion {
    x: 0.3,
    y: -0.4,
    z: 0.5,
    w: 0.6,
};
const QONE: Quaternion = Quaternion {
    x: 0.2,
    y: 0.1,
    z: -0.3,
    w: 1.0,
};
const QM1: Quaternion = Quaternion {
    x: 0.2,
    y: 0.1,
    z: -0.3,
    w: -1.0,
};
const U1: Quaternion = Quaternion {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 1.0,
};
const U2: Quaternion = Quaternion {
    x: 0.6,
    y: 0.0,
    z: 0.0,
    w: 0.8,
};
const U3: Quaternion = Quaternion {
    x: 0.0,
    y: 0.6,
    z: 0.0,
    w: -0.8,
};
const U4: Quaternion = Quaternion {
    x: 0.0,
    y: 0.0,
    z: 0.6,
    w: -0.8,
};
const U6: Quaternion = Quaternion {
    x: 0.0005,
    y: 0.0,
    z: 0.0,
    w: 1.0,
};
const AXIS: Vec3 = Vec3 {
    x: 1.0,
    y: 2.0,
    z: -2.0,
};
const ROT_TRACE: Matrix = Matrix {
    m: [
        [0.6, -0.8, 0.0, 0.0],
        [0.8, 0.6, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};
const ROT_MAX_X: Matrix = Matrix {
    m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, -0.8, 0.6, 0.0],
        [0.0, -0.6, -0.8, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};
const ROT_MAX_Y: Matrix = Matrix {
    m: [
        [-0.8, 0.0, -0.6, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.6, 0.0, -0.8, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};
const ROT_MAX_Z: Matrix = Matrix {
    m: [
        [-0.8, 0.6, 0.0, 0.0],
        [-0.6, -0.8, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

const P0: Vec2 = Vec2 { x: 1.5, y: -2.0 };
const P1: Vec2 = Vec2 { x: 0.5, y: 4.0 };
const P2: Vec2 = Vec2 { x: -1.25, y: 0.75 };
const P3: Vec2 = Vec2 { x: 2.0, y: -0.5 };
const V1: Vec4 = Vec4 {
    x: 1.5,
    y: -2.0,
    z: 0.25,
    w: 3.0,
};
const V2: Vec4 = Vec4 {
    x: 0.5,
    y: 4.0,
    z: -1.0,
    w: 2.5,
};
const V3: Vec4 = Vec4 {
    x: 2.0,
    y: 0.75,
    z: 3.5,
    w: -0.5,
};
const V4: Vec4 = Vec4 {
    x: -1.25,
    y: 1.0,
    z: 2.0,
    w: 5.0,
};

pub(crate) fn assert_bits(actual: &[f32], expected: &[f32]) {
    let actual: Vec<u32> = actual.iter().map(|v| v.to_bits()).collect();
    let expected: Vec<u32> = expected.iter().map(|v| v.to_bits()).collect();
    assert_eq!(actual, expected);
}

pub(crate) fn xyz(v: Vec3) -> [f32; 3] {
    [v.x, v.y, v.z]
}

pub(crate) fn xy(v: Vec2) -> [f32; 2] {
    [v.x, v.y]
}

pub(crate) fn xyzw(v: Vec4) -> [f32; 4] {
    [v.x, v.y, v.z, v.w]
}

pub(crate) fn quat(q: Quaternion) -> [f32; 4] {
    [q.x, q.y, q.z, q.w]
}

fn abcd(p: Plane) -> [f32; 4] {
    [p.a, p.b, p.c, p.d]
}

fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

#[test]
fn oracle_01_mul() {
    let out = A.multiply(B);
    assert_bits(
        out.m.as_flattened(),
        &[
            -5.5, 0.6500001, 2.65, 10.75, 16.8, -3.3, 4.95, 7.0, 6.35, 7.85, -3.525, 4.0, 9.375,
            -5.7, 18.55, 15.75,
        ],
    );
}

#[test]
fn oracle_02_mul_transpose() {
    let out = A.multiply_transpose(B);
    assert_bits(
        out.m.as_flattened(),
        &[
            -5.5, 16.8, 6.35, 9.375, 0.6500001, -3.3, 7.85, -5.7, 2.65, 4.95, -3.525, 18.55, 10.75,
            7.0, 4.0, 15.75,
        ],
    );
}

#[test]
fn oracle_03_transpose() {
    let out = A.transpose();
    assert_bits(
        out.m.as_flattened(),
        &[
            1.5, 0.5, 2.0, -1.25, -2.0, 4.0, 0.75, 1.0, 0.25, -1.0, 3.5, 2.0, 3.0, 2.5, -0.5, 5.0,
        ],
    );
}

#[test]
fn oracle_04_det() {
    assert_bits(&[A.determinant()], &[294.3047]);
}

#[test]
fn oracle_05_det_singular() {
    assert_bits(&[S.determinant()], &[0.0]);
}

#[test]
fn oracle_06_inverse() {
    let (inv, det) = A.inverse().expect("A is invertible");
    assert_bits(&[det], &[294.3047]);
    assert_bits(
        inv.m.as_flattened(),
        &[
            0.24889171,
            0.14971729,
            0.1448329,
            -0.20971039,
            -0.13442701,
            0.17169707,
            0.058294177,
            0.00063709484,
            -0.09524568,
            -0.11531416,
            0.18348332,
            0.13315283,
            0.12720661,
            0.049215578,
            -0.04884394,
            0.094183855,
        ],
    );
}

#[test]
fn oracle_07_inverse_singular() {
    assert!(S.inverse().is_none());
}

#[test]
fn oracle_08_translation() {
    let out = Matrix::translation(1.5, -2.25, 3.0);
    assert_bits(
        out.m.as_flattened(),
        &[
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.5, -2.25, 3.0, 1.0,
        ],
    );
}

#[test]
fn oracle_09_scaling() {
    let out = Matrix::scaling(2.0, -0.5, 4.0);
    assert_bits(
        out.m.as_flattened(),
        &[
            2.0, 0.0, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );
}

#[test]
fn oracle_10_rotation_x() {
    let out = Matrix::rotation_x(0.6);
    assert_bits(
        out.m.as_flattened(),
        &[
            1.0, 0.0, 0.0, 0.0, 0.0, 0.8253356, 0.5646425, 0.0, 0.0, -0.5646425, 0.8253356, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    );
}

#[test]
fn oracle_11_rotation_y() {
    let out = Matrix::rotation_y(-1.2);
    assert_bits(
        out.m.as_flattened(),
        &[
            0.3623577, 0.0, 0.9320391, 0.0, 0.0, 1.0, 0.0, 0.0, -0.9320391, 0.0, 0.3623577, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    );
}

#[test]
fn oracle_12_rotation_z() {
    let out = Matrix::rotation_z(2.0);
    assert_bits(
        out.m.as_flattened(),
        &[
            -0.41614684,
            0.9092974,
            0.0,
            0.0,
            -0.9092974,
            -0.41614684,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    );
}

#[test]
fn oracle_13_plane_from_point_normal() {
    let plane = Plane::from_point_normal(vec3(1.0, 2.0, 3.0), vec3(0.5, -1.5, 2.0));
    assert_bits(&abcd(plane), &[0.5, -1.5, 2.0, -3.5]);
}

#[test]
fn oracle_14_plane_from_points() {
    let plane = Plane::from_points(
        vec3(0.5, 1.0, -0.25),
        vec3(2.0, 0.5, 1.0),
        vec3(-1.0, 3.0, 0.75),
    );
    assert_bits(
        &abcd(plane),
        &[-0.59463537, -0.66896474, 0.4459765, 1.0777766],
    );
}

#[test]
fn oracle_15_plane_intersect_line() {
    let point = PL
        .intersect_line(vec3(0.5, -1.0, 2.0), vec3(2.0, 1.5, -3.0))
        .expect("the line crosses the plane");
    assert_bits(&xyz(point), &[0.3581081, -1.2364864, 2.4729729]);
}

#[test]
fn oracle_16_plane_intersect_line_parallel() {
    let point = FLAT.intersect_line(vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 1.0));
    assert!(point.is_none());
}

#[test]
fn oracle_17_plane_normalize() {
    assert_bits(
        &abcd(PL.normalize()),
        &[0.3094264, 0.9282791, -0.20628425, 1.5471319],
    );
}

#[test]
fn oracle_18_plane_normalize_zero() {
    let zero_normal = Plane {
        a: 0.0,
        b: 0.0,
        c: 0.0,
        d: 5.0,
    };
    assert_bits(&abcd(zero_normal.normalize()), &[0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn oracle_19_plane_transform() {
    assert_bits(&abcd(PL.transform(A)), &[-1.375, 4.35, 1.475, 10.75]);
}

#[test]
fn oracle_20_vec3_cross() {
    let cross = vec3(0.5, -2.0, 4.0).cross(vec3(3.0, 0.25, -1.5));
    assert_bits(&xyz(cross), &[2.0, 12.75, 6.125]);
}

#[test]
fn oracle_21_vec3_length() {
    assert_bits(&[vec3(1.5, -2.0, 0.25).length()], &[2.5124688]);
}

#[test]
fn oracle_22_vec3_normalize() {
    let unit = vec3(1.5, -2.0, 0.25).normalize();
    assert_bits(&xyz(unit), &[0.59702235, -0.7960298, 0.099503726]);
}

#[test]
fn oracle_23_vec3_normalize_zero() {
    let unit = vec3(0.0, 0.0, 0.0).normalize();
    assert_bits(&xyz(unit), &[0.0, 0.0, 0.0]);
}

#[test]
fn oracle_24_plane_from_points_collinear() {
    let plane = Plane::from_points(
        vec3(1.0, 2.0, 3.0),
        vec3(2.0, 2.0, 3.0),
        vec3(3.0, 2.0, 3.0),
    );
    assert_bits(&abcd(plane), &[0.0, 0.0, 0.0, -0.0]);
}

#[test]
fn oracle_25_quaternion_multiply() {
    assert_bits(&quat(QA.multiply(QB)), &[1.2, 2.0, -1.15, 0.5]);
}

#[test]
fn oracle_26_quaternion_inverse() {
    assert_bits(
        &quat(QA.inverse()),
        &[-0.078431375, 0.19607843, -0.3137255, 0.11764706],
    );
}

#[test]
fn oracle_27_quaternion_normalize() {
    assert_bits(
        &quat(QA.normalize()),
        &[0.19802952, -0.4950738, 0.7921181, 0.29704428],
    );
}

#[test]
fn oracle_28_quaternion_exp() {
    assert_bits(
        &quat(QV.exp()),
        &[0.3555612, -0.6222321, 0.1777806, 0.67438686],
    );
}

#[test]
fn oracle_29_quaternion_exp_zero_vector() {
    assert_bits(&quat(ZQ.exp()), &[0.0, 0.0, 0.0, 1.0]);
}

#[test]
fn oracle_30_quaternion_ln_acos() {
    assert_bits(&quat(QU.ln()), &[0.3477357, -0.4636476, 0.5795595, 0.0]);
}

#[test]
fn oracle_31_quaternion_ln_w_one() {
    assert_bits(&quat(QONE.ln()), &[0.2, 0.1, -0.3, 0.0]);
}

#[test]
fn oracle_32_quaternion_ln_w_minus_one() {
    assert_bits(&quat(QM1.ln()), &[0.2, 0.1, -0.3, 0.0]);
}

#[test]
fn oracle_33_quaternion_rotation_axis() {
    let out = Quaternion::rotation_axis(AXIS, 0.9);
    assert_bits(&quat(out), &[0.1449885, 0.289977, -0.289977, 0.90044713]);
}

#[test]
fn oracle_34_quaternion_rotation_yaw_pitch_roll() {
    let out = Quaternion::rotation_yaw_pitch_roll(0.7, -0.3, 1.1);
    assert_bits(&quat(out), &[0.057539985, 0.3624201, 0.52916986, 0.7650622]);
}

#[test]
fn oracle_35_quaternion_rotation_matrix_trace() {
    let out = Quaternion::rotation_matrix(ROT_TRACE);
    assert_bits(&quat(out), &[0.0, 0.0, -0.44721362, 0.8944272]);
}

#[test]
fn oracle_36_quaternion_rotation_matrix_max_x() {
    let out = Quaternion::rotation_matrix(ROT_MAX_X);
    assert_bits(&quat(out), &[0.94868326, 0.0, 0.0, 0.3162278]);
}

#[test]
fn oracle_37_quaternion_rotation_matrix_max_y() {
    let out = Quaternion::rotation_matrix(ROT_MAX_Y);
    assert_bits(&quat(out), &[0.0, 0.94868326, 0.0, 0.3162278]);
}

#[test]
fn oracle_38_quaternion_rotation_matrix_max_z() {
    let out = Quaternion::rotation_matrix(ROT_MAX_Z);
    assert_bits(&quat(out), &[0.0, 0.0, 0.94868326, 0.3162278]);
}

#[test]
fn oracle_39_quaternion_slerp_acos() {
    assert_bits(
        &quat(U1.slerp(U2, 0.5)),
        &[0.31622776, 0.0, 0.0, 0.94868326],
    );
}

#[test]
fn oracle_40_quaternion_slerp_negative_dot() {
    assert_bits(
        &quat(U1.slerp(U3, 0.3)),
        &[0.0, -0.19185346, 0.0, 0.9814236],
    );
}

#[test]
fn oracle_41_quaternion_slerp_linear() {
    assert_bits(&quat(U1.slerp(U6, 0.4)), &[0.00020000001, 0.0, 0.0, 1.0]);
}

#[test]
fn oracle_42_quaternion_squad() {
    assert_bits(
        &quat(U1.squad(U2, U3, U4, 0.3)),
        &[0.1968271, -0.0888545, -0.11507988, 0.96959823],
    );
}

#[test]
fn oracle_43_quaternion_squad_setup() {
    let (aout, bout, cout) = Quaternion::squad_setup(U1, U2, U3, U4);
    assert_bits(
        &quat(aout),
        &[0.7985049, 0.16772652, 7.450581e-09, 0.5781504],
    );
    assert_bits(
        &quat(bout),
        &[-0.16728905, -0.7745395, 0.16728903, 0.58661515],
    );
    assert_bits(&quat(cout), &[0.0, -0.6, 0.0, 0.8]);
}

#[test]
fn oracle_44_quaternion_squad_setup_flip_pq0() {
    let (aout, bout, cout) = Quaternion::squad_setup(U3, U1, U2, U4);
    assert_bits(&quat(aout), &[-0.15949102, 0.15949102, 0.0, 0.9742306]);
    assert_bits(
        &quat(bout),
        &[0.7985049, -7.450581e-09, 0.16772652, 0.5781504],
    );
    assert_bits(&quat(cout), &[0.6, 0.0, 0.0, 0.8]);
}

#[test]
fn oracle_45_quaternion_squad_setup_no_flip() {
    let (aout, bout, cout) = Quaternion::squad_setup(U1, U2, U2, U1);
    assert_bits(&quat(aout), &[0.7203983, 0.0, 0.0, 0.6935606]);
    assert_bits(&quat(bout), &[0.7203983, 0.0, 0.0, 0.6935606]);
    assert_bits(&quat(cout), &[0.6, 0.0, 0.0, 0.8]);
}

#[test]
fn oracle_46_quaternion_to_axis_angle() {
    let (axis, angle) = U2.to_axis_angle();
    assert_bits(&xyz(axis), &[0.6, 0.0, 0.0]);
    assert_bits(&[angle], &[1.2870022]);
}

#[test]
fn oracle_47_quaternion_bary_centric() {
    let out = U1.bary_centric(U2, U3, 0.3, 0.2);
    assert_bits(&quat(out), &[0.19393805, -0.13017485, 0.0, 0.9723387]);
}

#[test]
fn oracle_48_vec2_bary_centric() {
    let out = P0.bary_centric(P1, P2, 0.3, 0.2);
    assert_bits(&xy(out), &[0.65, 0.35000005]);
}

#[test]
fn oracle_49_vec2_bary_centric_vertex() {
    let out = P0.bary_centric(P1, P2, 1.0, 0.0);
    assert_bits(&xy(out), &[0.5, 4.0]);
}

#[test]
fn oracle_50_vec2_catmull_rom() {
    let out = P0.catmull_rom(P1, P2, P3, 0.4);
    assert_bits(&xy(out), &[-0.38600004, 3.2700005]);
}

#[test]
fn oracle_51_vec2_catmull_rom_s_zero() {
    let out = P0.catmull_rom(P1, P2, P3, 0.0);
    assert_bits(&xy(out), &[0.5, 4.0]);
}

#[test]
fn oracle_52_vec2_hermite() {
    let out = P0.hermite(P1, P2, P3, 0.25);
    assert_bits(&xy(out), &[1.046875, -0.984375]);
}

#[test]
fn oracle_53_vec2_hermite_s_one() {
    let out = P0.hermite(P1, P2, P3, 1.0);
    assert_bits(&xy(out), &[-1.25, 0.75]);
}

#[test]
fn oracle_54_vec2_normalize() {
    assert_bits(&xy(P0.normalize()), &[0.6, -0.8]);
}

#[test]
fn oracle_55_vec2_normalize_zero() {
    let unit = Vec2 { x: 0.0, y: 0.0 }.normalize();
    assert_bits(&xy(unit), &[0.0, 0.0]);
}

#[test]
fn oracle_56_vec2_transform() {
    let out = P0.transform(A);
    assert_bits(&xyzw(out), &[0.0, -10.0, 4.375, 4.5]);
}

#[test]
fn oracle_57_vec2_transform_coord() {
    assert_bits(&xy(P0.transform_coord(A)), &[0.0, -2.2222223]);
}

#[test]
fn oracle_58_vec2_transform_normal() {
    assert_bits(&xy(P0.transform_normal(A)), &[1.25, -11.0]);
}

#[test]
fn oracle_59_vec4_bary_centric() {
    let out = V1.bary_centric(V2, V3, 0.3, 0.2);
    assert_bits(&xyzw(out), &[1.3, 0.35000005, 0.525, 2.15]);
}

#[test]
fn oracle_60_vec4_catmull_rom() {
    let out = V1.catmull_rom(V2, V3, V4, 0.4);
    assert_bits(&xyzw(out), &[1.1480001, 3.198, 0.6739999, 1.0719999]);
}

#[test]
fn oracle_61_vec4_cross() {
    let out = V1.cross(V2, V3);
    assert_bits(&xyzw(out), &[61.71875, -0.1875, -39.1875, -27.71875]);
}

#[test]
fn oracle_62_vec4_cross_unit_axes() {
    let out = vec4(1.0, 0.0, 0.0, 0.0).cross(vec4(0.0, 1.0, 0.0, 0.0), vec4(0.0, 0.0, 1.0, 0.0));
    assert_bits(&xyzw(out), &[0.0, -0.0, 0.0, -1.0]);
}

#[test]
fn oracle_63_vec4_hermite() {
    let out = V1.hermite(V2, V3, V4, 0.25);
    assert_bits(&xyzw(out), &[1.7070312, -1.0546875, 0.5234375, 2.5703125]);
}

#[test]
fn oracle_64_vec4_normalize() {
    let unit = V1.normalize();
    assert_bits(
        &xyzw(unit),
        &[0.38332593, -0.51110125, 0.063887656, 0.76665187],
    );
}

#[test]
fn oracle_65_vec4_normalize_negative() {
    let unit = V4.normalize();
    assert_bits(
        &xyzw(unit),
        &[-0.22249709, 0.17799766, 0.35599533, 0.88998836],
    );
}

#[test]
fn oracle_66_vec4_transform() {
    assert_bits(&xyzw(V1.transform(A)), &[-2.0, -7.8125, 9.25, 14.375]);
}

#[test]
fn oracle_67_vec4_transform_second() {
    assert_bits(&xyzw(V4.transform(A)), &[-3.625, 13.0, 15.6875, 22.75]);
}
