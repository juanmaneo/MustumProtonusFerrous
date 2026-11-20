//! Replays of the oracle rows in `target/canon/d3dx9_36`. The oracle compiles the verbatim
//! `dlls/d3dx9_36/math.c` with `gcc -fsanitize=address,undefined` and prints each result. Its
//! run reported no sanitizer finding, so no vector is excluded. Each test names the row it
//! reproduces. Comparisons are on the bit pattern, so the sign of zero is checked as well.

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

pub(crate) fn assert_bits(actual: &[f32], expected: &[f32]) {
    let actual: Vec<u32> = actual.iter().map(|v| v.to_bits()).collect();
    let expected: Vec<u32> = expected.iter().map(|v| v.to_bits()).collect();
    assert_eq!(actual, expected);
}

pub(crate) fn xyz(v: Vec3) -> [f32; 3] {
    [v.x, v.y, v.z]
}

fn abcd(p: Plane) -> [f32; 4] {
    [p.a, p.b, p.c, p.d]
}

fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
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
