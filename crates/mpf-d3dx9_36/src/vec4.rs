//! `D3DXVECTOR4` functions of Wine's `dlls/d3dx9_36/math.c`. `self` is the first operand of
//! each C function, and the result is returned by value.

use crate::vec2::{bary_centric_term, catmull_rom_term, hermite_weights};
use crate::Matrix;

/// A `D3DXVECTOR4`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    /// `D3DXVec4Length`, an inline helper of `d3dx9math.inl`.
    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// `D3DXVec4BaryCentric`: `self` is `pv1`.
    pub fn bary_centric(self, pv2: Self, pv3: Self, f: f32, g: f32) -> Self {
        Self {
            x: bary_centric_term(self.x, pv2.x, pv3.x, f, g),
            y: bary_centric_term(self.y, pv2.y, pv3.y, f, g),
            z: bary_centric_term(self.z, pv2.z, pv3.z, f, g),
            w: bary_centric_term(self.w, pv2.w, pv3.w, f, g),
        }
    }

    /// `D3DXVec4CatmullRom`: `self` is `pv0`.
    pub fn catmull_rom(self, pv1: Self, pv2: Self, pv3: Self, s: f32) -> Self {
        Self {
            x: catmull_rom_term(self.x, pv1.x, pv2.x, pv3.x, s),
            y: catmull_rom_term(self.y, pv1.y, pv2.y, pv3.y, s),
            z: catmull_rom_term(self.z, pv1.z, pv2.z, pv3.z, s),
            w: catmull_rom_term(self.w, pv1.w, pv2.w, pv3.w, s),
        }
    }

    /// `D3DXVec4Cross`: the four-dimensional cross product of `self`, `pv2` and `pv3`.
    pub fn cross(self, pv2: Self, pv3: Self) -> Self {
        let (v1, v2, v3) = (self, pv2, pv3);
        Self {
            x: v1.y * (v2.z * v3.w - v3.z * v2.w) - v1.z * (v2.y * v3.w - v3.y * v2.w)
                + v1.w * (v2.y * v3.z - v2.z * v3.y),
            y: -(v1.x * (v2.z * v3.w - v3.z * v2.w) - v1.z * (v2.x * v3.w - v3.x * v2.w)
                + v1.w * (v2.x * v3.z - v3.x * v2.z)),
            z: v1.x * (v2.y * v3.w - v3.y * v2.w) - v1.y * (v2.x * v3.w - v3.x * v2.w)
                + v1.w * (v2.x * v3.y - v3.x * v2.y),
            w: -(v1.x * (v2.y * v3.z - v3.y * v2.z) - v1.y * (v2.x * v3.z - v3.x * v2.z)
                + v1.z * (v2.x * v3.y - v3.x * v2.y)),
        }
    }

    /// `D3DXVec4Hermite`: `self` is `pv1`.
    pub fn hermite(self, pt1: Self, pv2: Self, pt2: Self, s: f32) -> Self {
        let [h1, h2, h3, h4] = hermite_weights(s);
        Self {
            x: h1 * self.x + h2 * pt1.x + h3 * pv2.x + h4 * pt2.x,
            y: h1 * self.y + h2 * pt1.y + h3 * pv2.y + h4 * pt2.y,
            z: h1 * self.z + h2 * pt1.z + h3 * pv2.z + h4 * pt2.z,
            w: h1 * self.w + h2 * pt1.w + h3 * pv2.w + h4 * pt2.w,
        }
    }

    /// `D3DXVec4Normalize`. The division is not guarded, so a zero vector gives NaN, as in C.
    pub fn normalize(self) -> Self {
        let norm = self.length();
        Self {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
            w: self.w / norm,
        }
    }

    /// `D3DXVec4Transform`: the row vector `self` times `pm`.
    pub fn transform(self, pm: Matrix) -> Self {
        let m = pm.m;
        Self {
            x: m[0][0] * self.x + m[1][0] * self.y + m[2][0] * self.z + m[3][0] * self.w,
            y: m[0][1] * self.x + m[1][1] * self.y + m[2][1] * self.z + m[3][1] * self.w,
            z: m[0][2] * self.x + m[1][2] * self.y + m[2][2] * self.z + m[3][2] * self.w,
            w: m[0][3] * self.x + m[1][3] * self.y + m[2][3] * self.z + m[3][3] * self.w,
        }
    }
}
