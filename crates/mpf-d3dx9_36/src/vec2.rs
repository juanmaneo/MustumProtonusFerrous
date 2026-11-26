//! `D3DXVECTOR2` functions of Wine's `dlls/d3dx9_36/math.c`. `self` is the first operand of
//! each C function, and the result is returned by value.

use crate::{Matrix, Vec4};

/// A `D3DXVECTOR2`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// One component of the barycentric blend, shared with `D3DXVec4BaryCentric`.
pub(crate) fn bary_centric_term(v1: f32, v2: f32, v3: f32, f: f32, g: f32) -> f32 {
    (1.0 - f - g) * v1 + f * v2 + g * v3
}

/// One component of the Catmull-Rom spline, shared with `D3DXVec4CatmullRom`.
pub(crate) fn catmull_rom_term(v0: f32, v1: f32, v2: f32, v3: f32, s: f32) -> f32 {
    0.5 * (2.0 * v1
        + (v2 - v0) * s
        + (2.0 * v0 - 5.0 * v1 + 4.0 * v2 - v3) * s * s
        + (v3 - 3.0 * v2 + 3.0 * v1 - v0) * s * s * s)
}

/// The four Hermite basis weights for `s`, shared with `D3DXVec4Hermite`.
pub(crate) fn hermite_weights(s: f32) -> [f32; 4] {
    [
        2.0 * s * s * s - 3.0 * s * s + 1.0,
        s * s * s - 2.0 * s * s + s,
        -2.0 * s * s * s + 3.0 * s * s,
        s * s * s - s * s,
    ]
}

impl Vec2 {
    /// `D3DXVec2Length`, an inline helper of `d3dx9math.inl`.
    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// `D3DXVec2BaryCentric`: `self` is `pv1`.
    pub fn bary_centric(self, pv2: Self, pv3: Self, f: f32, g: f32) -> Self {
        Self {
            x: bary_centric_term(self.x, pv2.x, pv3.x, f, g),
            y: bary_centric_term(self.y, pv2.y, pv3.y, f, g),
        }
    }

    /// `D3DXVec2CatmullRom`: `self` is `pv0`.
    pub fn catmull_rom(self, pv1: Self, pv2: Self, pv3: Self, s: f32) -> Self {
        Self {
            x: catmull_rom_term(self.x, pv1.x, pv2.x, pv3.x, s),
            y: catmull_rom_term(self.y, pv1.y, pv2.y, pv3.y, s),
        }
    }

    /// `D3DXVec2Hermite`: `self` is `pv1`.
    pub fn hermite(self, pt1: Self, pv2: Self, pt2: Self, s: f32) -> Self {
        let [h1, h2, h3, h4] = hermite_weights(s);
        Self {
            x: h1 * self.x + h2 * pt1.x + h3 * pv2.x + h4 * pt2.x,
            y: h1 * self.y + h2 * pt1.y + h3 * pv2.y + h4 * pt2.y,
        }
    }

    /// `D3DXVec2Normalize`. A zero length gives the zero vector, as in C.
    pub fn normalize(self) -> Self {
        let norm = self.length();
        if norm == 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self {
                x: self.x / norm,
                y: self.y / norm,
            }
        }
    }

    /// `D3DXVec2Transform`: the row vector `(x, y, 0, 1)` times `pm`.
    pub fn transform(self, pm: Matrix) -> Vec4 {
        let m = pm.m;
        Vec4 {
            x: m[0][0] * self.x + m[1][0] * self.y + m[3][0],
            y: m[0][1] * self.x + m[1][1] * self.y + m[3][1],
            z: m[0][2] * self.x + m[1][2] * self.y + m[3][2],
            w: m[0][3] * self.x + m[1][3] * self.y + m[3][3],
        }
    }

    /// `D3DXVec2TransformCoord`: the transformed point divided by its `w`. A zero `w` gives
    /// infinities or NaN, as in C.
    pub fn transform_coord(self, pm: Matrix) -> Self {
        let m = pm.m;
        let norm = m[0][3] * self.x + m[1][3] * self.y + m[3][3];
        Self {
            x: (m[0][0] * self.x + m[1][0] * self.y + m[3][0]) / norm,
            y: (m[0][1] * self.x + m[1][1] * self.y + m[3][1]) / norm,
        }
    }

    /// `D3DXVec2TransformNormal`: the linear part of `pm` only, with no translation.
    pub fn transform_normal(self, pm: Matrix) -> Self {
        let m = pm.m;
        Self {
            x: m[0][0] * self.x + m[1][0] * self.y,
            y: m[0][1] * self.x + m[1][1] * self.y,
        }
    }
}
