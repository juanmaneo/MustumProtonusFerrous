//! Deterministic value logic from Wine's `dlls/ddraw/utils.c`.
//!
//! The port is intentionally limited to `multiply_matrix`; DirectDraw's COM, device, surface,
//! window and handle state remains outside this crate.

/// Wine's `wined3d_matrix`, stored in the same field order as the C struct.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix4x4 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}

/// Wine's `multiply_matrix` from `dlls/ddraw/utils.c`.
///
/// Computes the product with the operands in Wine's field convention, which is `src2 * src1` in
/// row-major algebra. The sums use the same association and order as the C code, so the `f32`
/// results match bit for bit. The result is built in full before it is written to `dst`, as the C
/// code does.
pub fn multiply_matrix(dst: &mut Matrix4x4, src1: &Matrix4x4, src2: &Matrix4x4) {
    let temp = Matrix4x4 {
        _11: (src1._11 * src2._11)
            + (src1._21 * src2._12)
            + (src1._31 * src2._13)
            + (src1._41 * src2._14),
        _21: (src1._11 * src2._21)
            + (src1._21 * src2._22)
            + (src1._31 * src2._23)
            + (src1._41 * src2._24),
        _31: (src1._11 * src2._31)
            + (src1._21 * src2._32)
            + (src1._31 * src2._33)
            + (src1._41 * src2._34),
        _41: (src1._11 * src2._41)
            + (src1._21 * src2._42)
            + (src1._31 * src2._43)
            + (src1._41 * src2._44),

        _12: (src1._12 * src2._11)
            + (src1._22 * src2._12)
            + (src1._32 * src2._13)
            + (src1._42 * src2._14),
        _22: (src1._12 * src2._21)
            + (src1._22 * src2._22)
            + (src1._32 * src2._23)
            + (src1._42 * src2._24),
        _32: (src1._12 * src2._31)
            + (src1._22 * src2._32)
            + (src1._32 * src2._33)
            + (src1._42 * src2._34),
        _42: (src1._12 * src2._41)
            + (src1._22 * src2._42)
            + (src1._32 * src2._43)
            + (src1._42 * src2._44),

        _13: (src1._13 * src2._11)
            + (src1._23 * src2._12)
            + (src1._33 * src2._13)
            + (src1._43 * src2._14),
        _23: (src1._13 * src2._21)
            + (src1._23 * src2._22)
            + (src1._33 * src2._23)
            + (src1._43 * src2._24),
        _33: (src1._13 * src2._31)
            + (src1._23 * src2._32)
            + (src1._33 * src2._33)
            + (src1._43 * src2._34),
        _43: (src1._13 * src2._41)
            + (src1._23 * src2._42)
            + (src1._33 * src2._43)
            + (src1._43 * src2._44),

        _14: (src1._14 * src2._11)
            + (src1._24 * src2._12)
            + (src1._34 * src2._13)
            + (src1._44 * src2._14),
        _24: (src1._14 * src2._21)
            + (src1._24 * src2._22)
            + (src1._34 * src2._23)
            + (src1._44 * src2._24),
        _34: (src1._14 * src2._31)
            + (src1._24 * src2._32)
            + (src1._34 * src2._33)
            + (src1._44 * src2._34),
        _44: (src1._14 * src2._41)
            + (src1._24 * src2._42)
            + (src1._34 * src2._43)
            + (src1._44 * src2._44),
    };
    *dst = temp;
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
