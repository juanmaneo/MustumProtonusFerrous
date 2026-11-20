//! Matrix and projection arithmetic of Wine's `dlls/glu32/project.c`: the helpers behind
//! `gluProject` and `gluUnProject`, and the two functions themselves.
//!
//! Matrices are column-major `[f64; 16]`, as the C `GLdouble[16]` arguments are. The
//! C `GL_TRUE` and `GL_FALSE` results become `Option`: `None` where C returns `GL_FALSE`
//! and leaves the output unwritten.
//!
//! Not ported: `gluOrtho2D`, `gluPerspective`, `gluLookAt` and `gluPickMatrix`. Each one
//! only issues GL state calls (`glOrtho`, `glMultMatrix*`, `glTranslate*`, `glScale*`).

use crate::mult_matrix_vec;

/// `__gluMultMatricesd`: `a * b`.
pub fn mult_matrices(a: &[f64; 16], b: &[f64; 16]) -> [f64; 16] {
    std::array::from_fn(|k| {
        let (i, j) = (k / 4, k % 4);
        a[i * 4] * b[j]
            + a[i * 4 + 1] * b[4 + j]
            + a[i * 4 + 2] * b[8 + j]
            + a[i * 4 + 3] * b[12 + j]
    })
}

/// `__gluInvertMatrixd`: the inverse of a 4x4 matrix, or `None` when the determinant is
/// zero. The cofactor expansion is kept term for term, so rounding matches the C code.
pub fn invert_matrix(m: &[f64; 16]) -> Option<[f64; 16]> {
    let mut inv = [0.0_f64; 16];
    inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
        + m[9] * m[7] * m[14]
        + m[13] * m[6] * m[11]
        - m[13] * m[7] * m[10];
    inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
        - m[8] * m[7] * m[14]
        - m[12] * m[6] * m[11]
        + m[12] * m[7] * m[10];
    inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
        + m[8] * m[7] * m[13]
        + m[12] * m[5] * m[11]
        - m[12] * m[7] * m[9];
    inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
        - m[8] * m[6] * m[13]
        - m[12] * m[5] * m[10]
        + m[12] * m[6] * m[9];
    inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
        - m[9] * m[3] * m[14]
        - m[13] * m[2] * m[11]
        + m[13] * m[3] * m[10];
    inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
        + m[8] * m[3] * m[14]
        + m[12] * m[2] * m[11]
        - m[12] * m[3] * m[10];
    inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
        - m[8] * m[3] * m[13]
        - m[12] * m[1] * m[11]
        + m[12] * m[3] * m[9];
    inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
        + m[8] * m[2] * m[13]
        + m[12] * m[1] * m[10]
        - m[12] * m[2] * m[9];
    inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
        + m[5] * m[3] * m[14]
        + m[13] * m[2] * m[7]
        - m[13] * m[3] * m[6];
    inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
        - m[4] * m[3] * m[14]
        - m[12] * m[2] * m[7]
        + m[12] * m[3] * m[6];
    inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
        + m[4] * m[3] * m[13]
        + m[12] * m[1] * m[7]
        - m[12] * m[3] * m[5];
    inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
        - m[4] * m[2] * m[13]
        - m[12] * m[1] * m[6]
        + m[12] * m[2] * m[5];
    inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
        - m[5] * m[3] * m[10]
        - m[9] * m[2] * m[7]
        + m[9] * m[3] * m[6];
    inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
        + m[4] * m[3] * m[10]
        + m[8] * m[2] * m[7]
        - m[8] * m[3] * m[6];
    inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
        - m[4] * m[3] * m[9]
        - m[8] * m[1] * m[7]
        + m[8] * m[3] * m[5];
    inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
        + m[4] * m[2] * m[9]
        + m[8] * m[1] * m[6]
        - m[8] * m[2] * m[5];

    let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
    if det == 0.0 {
        return None;
    }
    let det = 1.0 / det;
    Some(std::array::from_fn(|i| inv[i] * det))
}

/// `gluProject`: maps an object-space point to window coordinates `(x, y, z)`.
/// Returns `None` when the clip-space `w` is zero.
pub fn project(
    obj: [f64; 3],
    model: &[f64; 16],
    proj: &[f64; 16],
    viewport: [i32; 4],
) -> Option<[f64; 3]> {
    let eye = mult_matrix_vec(model, &[obj[0], obj[1], obj[2], 1.0]);
    let mut clip = mult_matrix_vec(proj, &eye);
    if clip[3] == 0.0 {
        return None;
    }
    clip[0] /= clip[3];
    clip[1] /= clip[3];
    clip[2] /= clip[3];

    let x = (clip[0] * 0.5 + 0.5) * f64::from(viewport[2]) + f64::from(viewport[0]);
    let y = (clip[1] * 0.5 + 0.5) * f64::from(viewport[3]) + f64::from(viewport[1]);
    let z = clip[2] * 0.5 + 0.5;
    Some([x, y, z])
}

/// `gluUnProject`: maps window coordinates back to object space. Returns `None` when
/// `model * proj` is singular or the homogeneous `w` is zero.
pub fn unproject(
    win: [f64; 3],
    model: &[f64; 16],
    proj: &[f64; 16],
    viewport: [i32; 4],
) -> Option<[f64; 3]> {
    let inverse = invert_matrix(&mult_matrices(model, proj))?;
    let x = (win[0] - f64::from(viewport[0])) / f64::from(viewport[2]) * 2.0 - 1.0;
    let y = (win[1] - f64::from(viewport[1])) / f64::from(viewport[3]) * 2.0 - 1.0;
    let z = win[2] * 2.0 - 1.0;
    let out = mult_matrix_vec(&inverse, &[x, y, z, 1.0]);
    if out[3] == 0.0 {
        return None;
    }
    Some([out[0] / out[3], out[1] / out[3], out[2] / out[3]])
}
