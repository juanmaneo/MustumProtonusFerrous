//! `D3DXPLANE` functions of Wine's `dlls/d3dx9_36/math.c`.

use crate::{Matrix, Vec3};

/// A `D3DXPLANE`: `a*x + b*y + c*z + d = 0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Plane {
    /// `D3DXPlaneFromPointNormal`.
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        Self {
            a: normal.x,
            b: normal.y,
            c: normal.z,
            d: -point.dot(normal),
        }
    }

    /// `D3DXPlaneFromPoints`: the plane through three points, with a unit normal.
    pub fn from_points(pv1: Vec3, pv2: Vec3, pv3: Vec3) -> Self {
        let edge1 = pv2.subtract(pv1);
        let edge2 = pv3.subtract(pv1);
        let normal = edge1.cross(edge2).normalize();
        Self::from_point_normal(pv1, normal)
    }

    /// `D3DXPlaneIntersectLine`: where the line through `pv1` and `pv2` meets the plane, or
    /// `None` when the line is parallel to the plane.
    pub fn intersect_line(self, pv1: Vec3, pv2: Vec3) -> Option<Vec3> {
        let normal = Vec3 {
            x: self.a,
            y: self.b,
            z: self.c,
        };
        let direction = pv2.subtract(pv1);
        let dot = normal.dot(direction);
        if dot == 0.0 {
            return None;
        }
        let temp = (self.d + normal.dot(pv1)) / dot;
        Some(Vec3 {
            x: pv1.x - temp * direction.x,
            y: pv1.y - temp * direction.y,
            z: pv1.z - temp * direction.z,
        })
    }

    /// `D3DXPlaneNormalize`. A zero normal gives the zero plane, as in C.
    pub fn normalize(self) -> Self {
        let norm = (self.a * self.a + self.b * self.b + self.c * self.c).sqrt();
        if norm != 0.0 {
            Self {
                a: self.a / norm,
                b: self.b / norm,
                c: self.c / norm,
                d: self.d / norm,
            }
        } else {
            Self {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 0.0,
            }
        }
    }

    /// `D3DXPlaneTransform`: `self` transformed by `m`.
    pub fn transform(self, m: Matrix) -> Self {
        let m = m.m;
        Self {
            a: m[0][0] * self.a + m[1][0] * self.b + m[2][0] * self.c + m[3][0] * self.d,
            b: m[0][1] * self.a + m[1][1] * self.b + m[2][1] * self.c + m[3][1] * self.d,
            c: m[0][2] * self.a + m[1][2] * self.b + m[2][2] * self.c + m[3][2] * self.d,
            d: m[0][3] * self.a + m[1][3] * self.b + m[2][3] * self.c + m[3][3] * self.d,
        }
    }
}
