//! Affine matrix helpers of Wine's `dlls/d3drm/frame.c`. A point is a row vector, so a transform
//! is `v * m`, and the translation is row 3 of the matrix.

use crate::math::vector_normalize;
use crate::{Matrix, Vector};

/// `d3drm_matrix_multiply_affine`: `a * b`. The fourth column stays `(0, 0, 0, 1)`, and the
/// translation row of `b` is added to the translation row of the product.
pub fn matrix_multiply_affine(a: &Matrix, b: &Matrix) -> Matrix {
    let mut dst = [[0.0; 4]; 4];
    for (dst_row, a_row) in dst.iter_mut().zip(a).take(3) {
        for (j, cell) in dst_row.iter_mut().enumerate().take(3) {
            *cell = a_row[0] * b[0][j] + a_row[1] * b[1][j] + a_row[2] * b[2][j];
        }
    }
    for (j, cell) in dst[3].iter_mut().enumerate().take(3) {
        *cell = a[3][0] * b[0][j] + a[3][1] * b[1][j] + a[3][2] * b[2][j] + b[3][j];
    }
    dst[3][3] = 1.0;
    dst
}

/// `d3drm_matrix_set_rotation`: the rotation matrix of `theta` about `axis`. The axis is
/// normalised in place, as in the C.
pub fn matrix_set_rotation(axis: &mut Vector, theta: f32) -> Matrix {
    vector_normalize(axis);
    let sin = theta.sin();
    let cos = theta.cos();
    let vers = 1.0 - cos;
    let Vector { x, y, z } = *axis;
    [
        [
            vers * x * x + cos,
            vers * y * x + sin * z,
            vers * z * x - sin * y,
            0.0,
        ],
        [
            vers * x * y - sin * z,
            vers * y * y + cos,
            vers * z * y + sin * x,
            0.0,
        ],
        [
            vers * x * z + sin * y,
            vers * y * z - sin * x,
            vers * z * z + cos,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// `d3drm_vector_transform_affine`: `v * m`, with the translation row added.
pub fn vector_transform_affine(v: Vector, m: &Matrix) -> Vector {
    Vector {
        x: v.x * m[0][0] + v.y * m[1][0] + v.z * m[2][0] + m[3][0],
        y: v.x * m[0][1] + v.y * m[1][1] + v.z * m[2][1] + m[3][1],
        z: v.x * m[0][2] + v.y * m[1][2] + v.z * m[2][2] + m[3][2],
    }
}
