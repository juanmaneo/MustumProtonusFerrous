//! `D3DXMATRIX` functions of Wine's `dlls/d3dx9_36/math.c`. Matrices are row-major: `m[row][column]`.

/// A `D3DXMATRIX`, indexed as `m[row][column]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    pub m: [[f32; 4]; 4],
}

impl Matrix {
    /// `D3DXMatrixIdentity`.
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// `D3DXMatrixMultiply`: `self * other`.
    pub fn multiply(self, other: Self) -> Self {
        Self {
            m: std::array::from_fn(|i| {
                std::array::from_fn(|j| {
                    self.m[i][0] * other.m[0][j]
                        + self.m[i][1] * other.m[1][j]
                        + self.m[i][2] * other.m[2][j]
                        + self.m[i][3] * other.m[3][j]
                })
            }),
        }
    }

    /// `D3DXMatrixMultiplyTranspose`: the transpose of `self * other`.
    pub fn multiply_transpose(self, other: Self) -> Self {
        self.multiply(other).transpose()
    }

    /// `D3DXMatrixTranspose`.
    pub fn transpose(self) -> Self {
        Self {
            m: std::array::from_fn(|i| std::array::from_fn(|j| self.m[j][i])),
        }
    }

    /// `D3DXMatrixDeterminant`.
    pub fn determinant(self) -> f32 {
        let m = self.m;
        let mut t = [0.0f32; 3];
        let mut v = [0.0f32; 4];

        t[0] = m[2][2] * m[3][3] - m[2][3] * m[3][2];
        t[1] = m[1][2] * m[3][3] - m[1][3] * m[3][2];
        t[2] = m[1][2] * m[2][3] - m[1][3] * m[2][2];
        v[0] = m[1][1] * t[0] - m[2][1] * t[1] + m[3][1] * t[2];
        v[1] = -m[1][0] * t[0] + m[2][0] * t[1] - m[3][0] * t[2];

        t[0] = m[1][0] * m[2][1] - m[2][0] * m[1][1];
        t[1] = m[1][0] * m[3][1] - m[3][0] * m[1][1];
        t[2] = m[2][0] * m[3][1] - m[3][0] * m[2][1];
        v[2] = m[3][3] * t[0] - m[2][3] * t[1] + m[1][3] * t[2];
        v[3] = -m[3][2] * t[0] + m[2][2] * t[1] - m[1][2] * t[2];

        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2] + m[0][3] * v[3]
    }

    /// `D3DXMatrixInverse`. Returns the inverse and the determinant, or `None` when the
    /// determinant is zero.
    pub fn inverse(self) -> Option<(Self, f32)> {
        let m = self.m;
        let mut t = [0.0f32; 3];
        let mut v = [0.0f32; 16];

        t[0] = m[2][2] * m[3][3] - m[2][3] * m[3][2];
        t[1] = m[1][2] * m[3][3] - m[1][3] * m[3][2];
        t[2] = m[1][2] * m[2][3] - m[1][3] * m[2][2];
        v[0] = m[1][1] * t[0] - m[2][1] * t[1] + m[3][1] * t[2];
        v[4] = -m[1][0] * t[0] + m[2][0] * t[1] - m[3][0] * t[2];

        t[0] = m[1][0] * m[2][1] - m[2][0] * m[1][1];
        t[1] = m[1][0] * m[3][1] - m[3][0] * m[1][1];
        t[2] = m[2][0] * m[3][1] - m[3][0] * m[2][1];
        v[8] = m[3][3] * t[0] - m[2][3] * t[1] + m[1][3] * t[2];
        v[12] = -m[3][2] * t[0] + m[2][2] * t[1] - m[1][2] * t[2];

        let det = m[0][0] * v[0] + m[0][1] * v[4] + m[0][2] * v[8] + m[0][3] * v[12];
        if det == 0.0 {
            return None;
        }

        t[0] = m[2][2] * m[3][3] - m[2][3] * m[3][2];
        t[1] = m[0][2] * m[3][3] - m[0][3] * m[3][2];
        t[2] = m[0][2] * m[2][3] - m[0][3] * m[2][2];
        v[1] = -m[0][1] * t[0] + m[2][1] * t[1] - m[3][1] * t[2];
        v[5] = m[0][0] * t[0] - m[2][0] * t[1] + m[3][0] * t[2];

        t[0] = m[0][0] * m[2][1] - m[2][0] * m[0][1];
        t[1] = m[3][0] * m[0][1] - m[0][0] * m[3][1];
        t[2] = m[2][0] * m[3][1] - m[3][0] * m[2][1];
        v[9] = -m[3][3] * t[0] - m[2][3] * t[1] - m[0][3] * t[2];
        v[13] = m[3][2] * t[0] + m[2][2] * t[1] + m[0][2] * t[2];

        t[0] = m[1][2] * m[3][3] - m[1][3] * m[3][2];
        t[1] = m[0][2] * m[3][3] - m[0][3] * m[3][2];
        t[2] = m[0][2] * m[1][3] - m[0][3] * m[1][2];
        v[2] = m[0][1] * t[0] - m[1][1] * t[1] + m[3][1] * t[2];
        v[6] = -m[0][0] * t[0] + m[1][0] * t[1] - m[3][0] * t[2];

        t[0] = m[0][0] * m[1][1] - m[1][0] * m[0][1];
        t[1] = m[3][0] * m[0][1] - m[0][0] * m[3][1];
        t[2] = m[1][0] * m[3][1] - m[3][0] * m[1][1];
        v[10] = m[3][3] * t[0] + m[1][3] * t[1] + m[0][3] * t[2];
        v[14] = -m[3][2] * t[0] - m[1][2] * t[1] - m[0][2] * t[2];

        t[0] = m[1][2] * m[2][3] - m[1][3] * m[2][2];
        t[1] = m[0][2] * m[2][3] - m[0][3] * m[2][2];
        t[2] = m[0][2] * m[1][3] - m[0][3] * m[1][2];
        v[3] = -m[0][1] * t[0] + m[1][1] * t[1] - m[2][1] * t[2];
        v[7] = m[0][0] * t[0] - m[1][0] * t[1] + m[2][0] * t[2];

        v[11] = -m[0][0] * (m[1][1] * m[2][3] - m[1][3] * m[2][1])
            + m[1][0] * (m[0][1] * m[2][3] - m[0][3] * m[2][1])
            - m[2][0] * (m[0][1] * m[1][3] - m[0][3] * m[1][1]);

        v[15] = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[1][0] * (m[0][1] * m[2][2] - m[0][2] * m[2][1])
            + m[2][0] * (m[0][1] * m[1][2] - m[0][2] * m[1][1]);

        let inv_det = 1.0 / det;
        let out = std::array::from_fn(|i| std::array::from_fn(|j| v[4 * i + j] * inv_det));
        Some((Self { m: out }, det))
    }

    /// `D3DXMatrixTranslation`.
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut out = Self::identity();
        out.m[3][0] = x;
        out.m[3][1] = y;
        out.m[3][2] = z;
        out
    }

    /// `D3DXMatrixScaling`.
    pub fn scaling(sx: f32, sy: f32, sz: f32) -> Self {
        let mut out = Self::identity();
        out.m[0][0] = sx;
        out.m[1][1] = sy;
        out.m[2][2] = sz;
        out
    }

    /// `D3DXMatrixRotationX`.
    pub fn rotation_x(angle: f32) -> Self {
        let mut out = Self::identity();
        out.m[1][1] = angle.cos();
        out.m[2][2] = angle.cos();
        out.m[1][2] = angle.sin();
        out.m[2][1] = -angle.sin();
        out
    }

    /// `D3DXMatrixRotationY`.
    pub fn rotation_y(angle: f32) -> Self {
        let mut out = Self::identity();
        out.m[0][0] = angle.cos();
        out.m[2][2] = angle.cos();
        out.m[0][2] = -angle.sin();
        out.m[2][0] = angle.sin();
        out
    }

    /// `D3DXMatrixRotationZ`.
    pub fn rotation_z(angle: f32) -> Self {
        let mut out = Self::identity();
        out.m[0][0] = angle.cos();
        out.m[1][1] = angle.cos();
        out.m[0][1] = angle.sin();
        out.m[1][0] = -angle.sin();
        out
    }
}
