//! d3dx9_36 subset for MPF: the deterministic vector, matrix, plane and quaternion math of
//! Wine's `dlls/d3dx9_36/math.c`.
//!
//! Ported (names in `dlls/d3dx9_36/math.c`; oracle rows in the `oracle_checks` tests):
//! - `D3DXVec3Dot`, `D3DXVec3Length`, `D3DXVec3Cross`, `D3DXVec3Subtract` and
//!   `D3DXVec3Normalize`: [`Vec3`] methods.
//! - `D3DXVec2BaryCentric`, `D3DXVec2CatmullRom`, `D3DXVec2Hermite`, `D3DXVec2Normalize`,
//!   `D3DXVec2Transform`, `D3DXVec2TransformCoord` and `D3DXVec2TransformNormal`: [`Vec2`]
//!   methods. `D3DXVec2Transform` returns a [`Vec4`].
//! - `D3DXVec4BaryCentric`, `D3DXVec4CatmullRom`, `D3DXVec4Cross`, `D3DXVec4Hermite`,
//!   `D3DXVec4Normalize` and `D3DXVec4Transform`: [`Vec4`] methods.
//! - `D3DXMatrixIdentity`, `D3DXMatrixMultiply`, `D3DXMatrixMultiplyTranspose`,
//!   `D3DXMatrixTranspose`, `D3DXMatrixDeterminant`, `D3DXMatrixInverse`,
//!   `D3DXMatrixTranslation`, `D3DXMatrixScaling` and `D3DXMatrixRotationX/Y/Z`: [`Matrix`].
//! - `D3DXPlaneFromPointNormal`, `D3DXPlaneFromPoints`, `D3DXPlaneIntersectLine`,
//!   `D3DXPlaneNormalize` and `D3DXPlaneTransform`: [`Plane`].
//! - `D3DXQuaternionBaryCentric`, `D3DXQuaternionExp`, `D3DXQuaternionInverse`,
//!   `D3DXQuaternionLn`, `D3DXQuaternionMultiply`, `D3DXQuaternionNormalize`,
//!   `D3DXQuaternionRotationAxis`, `D3DXQuaternionRotationMatrix`,
//!   `D3DXQuaternionRotationYawPitchRoll`, `D3DXQuaternionSlerp`, `D3DXQuaternionSquad`,
//!   `D3DXQuaternionSquadSetup` and `D3DXQuaternionToAxisAngle`: [`Quaternion`].
//!
//! Divergences from the C bodies:
//! - The C functions write through an output pointer and return it. The port returns the value,
//!   or a tuple where C writes several outputs. The NULL checks on the pointers cannot be
//!   expressed with references, so they are not modelled. The `d3dx9math.inl` helpers used by
//!   the quaternion and vector functions (`D3DXQuaternionDot`, `Length` and `LengthSq`,
//!   `D3DXVec2Length` and `D3DXVec4Length`) have the same omission.
//! - C returns NULL where the port returns `None`: `D3DXMatrixInverse` for a zero determinant,
//!   and `D3DXPlaneIntersectLine` for a zero dot product.
//! - `D3DXMatrixInverse` writes `*pdeterminant` only for an invertible matrix. The port returns
//!   the determinant next to the inverse in that case.
//!
//! Skipped, outside this subset:
//! - The stride-based `*Array` functions of `D3DXVec2` and `D3DXVec4`, which walk caller memory
//!   by byte stride, colour, spherical harmonic, projection and view matrix,
//!   `D3DXMatrixDecompose`, the bounding-volume probes, and the `ID3DXMatrixStack` COM object.
//! - `D3DXFloat32To16Array` and `D3DXFloat16To32Array`, which depend on the float16 helpers of
//!   `d3dx_helpers.c`.

mod matrix;
mod plane;
mod quaternion;
mod vec2;
mod vec4;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use matrix::Matrix;
pub use plane::Plane;
pub use quaternion::Quaternion;
pub use vec2::Vec2;
pub use vec4::Vec4;

/// A `D3DXVECTOR3`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// `D3DXVec3Dot`.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// `D3DXVec3Length`.
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// `D3DXVec3Cross`: `self x other`.
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// `D3DXVec3Subtract`: `self - other`.
    pub fn subtract(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    /// `D3DXVec3Normalize`. A zero length gives the zero vector, as in C.
    pub fn normalize(self) -> Self {
        let norm = self.length();
        if norm == 0.0 {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Self {
                x: self.x / norm,
                y: self.y / norm,
                z: self.z / norm,
            }
        }
    }
}
