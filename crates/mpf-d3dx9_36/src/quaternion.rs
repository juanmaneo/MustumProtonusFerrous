//! `D3DXQUATERNION` functions of Wine's `dlls/d3dx9_36/math.c`. `self` is the first operand of
//! each C function, and the result is returned by value.

use crate::{Matrix, Vec3};

/// A `D3DXQUATERNION`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    /// `D3DXQuaternionDot`, an inline helper of `d3dx9math.inl`.
    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// `D3DXQuaternionLengthSq`, an inline helper of `d3dx9math.inl`.
    fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// `D3DXQuaternionLength`, an inline helper of `d3dx9math.inl`.
    fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Each component times `s`, as the `*=` statements of `D3DXQuaternionSquadSetup`.
    fn scale(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }

    /// `D3DXQuaternionMultiply`: `self` is `pq1` and `other` is `pq2`.
    pub fn multiply(self, other: Self) -> Self {
        Self {
            x: other.w * self.x + other.x * self.w + other.y * self.z - other.z * self.y,
            y: other.w * self.y - other.x * self.z + other.y * self.w + other.z * self.x,
            z: other.w * self.z + other.x * self.y - other.y * self.x + other.z * self.w,
            w: other.w * self.w - other.x * self.x - other.y * self.y - other.z * self.z,
        }
    }

    /// `D3DXQuaternionInverse`: the conjugate divided by the squared length.
    pub fn inverse(self) -> Self {
        let norm = self.length_sq();
        Self {
            x: -self.x / norm,
            y: -self.y / norm,
            z: -self.z / norm,
            w: self.w / norm,
        }
    }

    /// `D3DXQuaternionNormalize`. A zero length divides by zero, which gives NaN, as in C.
    pub fn normalize(self) -> Self {
        let norm = self.length();
        Self {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
            w: self.w / norm,
        }
    }

    /// `D3DXQuaternionExp`: only the vector part is used. A zero vector gives the identity.
    pub fn exp(self) -> Self {
        let norm = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if norm != 0.0 {
            let sin = norm.sin();
            Self {
                x: sin * self.x / norm,
                y: sin * self.y / norm,
                z: sin * self.z / norm,
                w: norm.cos(),
            }
        } else {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }
        }
    }

    /// `D3DXQuaternionLn`. A `w` of 1 or -1 (or more) takes the limit branch. The result
    /// always has `w = 0`.
    pub fn ln(self) -> Self {
        let t = if self.w >= 1.0 || self.w == -1.0 {
            1.0
        } else {
            self.w.acos() / (1.0 - self.w * self.w).sqrt()
        };
        Self {
            x: t * self.x,
            y: t * self.y,
            z: t * self.z,
            w: 0.0,
        }
    }

    /// `D3DXQuaternionRotationAxis`: `v` is normalized first, so a zero axis keeps only `w`.
    pub fn rotation_axis(v: Vec3, angle: f32) -> Self {
        let axis = v.normalize();
        let sin = (angle / 2.0).sin();
        Self {
            x: sin * axis.x,
            y: sin * axis.y,
            z: sin * axis.z,
            w: (angle / 2.0).cos(),
        }
    }

    /// `D3DXQuaternionRotationMatrix`. The largest diagonal entry picks the branch when the
    /// trace is at most 1, as in C.
    pub fn rotation_matrix(m: Matrix) -> Self {
        let m = m.m;
        let trace = m[0][0] + m[1][1] + m[2][2] + 1.0;
        if trace > 1.0 {
            let s = 2.0 * trace.sqrt();
            Self {
                x: (m[1][2] - m[2][1]) / s,
                y: (m[2][0] - m[0][2]) / s,
                z: (m[0][1] - m[1][0]) / s,
                w: 0.25 * s,
            }
        } else {
            let mut maxi = 0;
            for i in 1..3 {
                if m[i][i] > m[maxi][maxi] {
                    maxi = i;
                }
            }
            match maxi {
                0 => {
                    let s = 2.0 * (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt();
                    Self {
                        x: 0.25 * s,
                        y: (m[0][1] + m[1][0]) / s,
                        z: (m[0][2] + m[2][0]) / s,
                        w: (m[1][2] - m[2][1]) / s,
                    }
                }
                1 => {
                    let s = 2.0 * (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt();
                    Self {
                        x: (m[0][1] + m[1][0]) / s,
                        y: 0.25 * s,
                        z: (m[1][2] + m[2][1]) / s,
                        w: (m[2][0] - m[0][2]) / s,
                    }
                }
                _ => {
                    let s = 2.0 * (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt();
                    Self {
                        x: (m[0][2] + m[2][0]) / s,
                        y: (m[1][2] + m[2][1]) / s,
                        z: 0.25 * s,
                        w: (m[0][1] - m[1][0]) / s,
                    }
                }
            }
        }
    }

    /// `D3DXQuaternionRotationYawPitchRoll`.
    pub fn rotation_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Self {
        let (syaw, cyaw) = ((yaw / 2.0).sin(), (yaw / 2.0).cos());
        let (spitch, cpitch) = ((pitch / 2.0).sin(), (pitch / 2.0).cos());
        let (sroll, croll) = ((roll / 2.0).sin(), (roll / 2.0).cos());
        Self {
            x: syaw * cpitch * sroll + cyaw * spitch * croll,
            y: syaw * cpitch * croll - cyaw * spitch * sroll,
            z: cyaw * cpitch * sroll - syaw * spitch * croll,
            w: cyaw * cpitch * croll + syaw * spitch * sroll,
        }
    }

    /// `D3DXQuaternionSlerp`: `self` at `t = 0` and `other` at `t = 1`. A negative dot product
    /// takes the shorter arc, and a dot product within 0.001 of 1 is blended linearly.
    pub fn slerp(self, other: Self, t: f32) -> Self {
        let mut temp = 1.0 - t;
        let mut t = t;
        let mut dot = self.dot(other);
        if dot < 0.0 {
            t = -t;
            dot = -dot;
        }
        if 1.0 - dot > 0.001 {
            let theta = dot.acos();
            temp = (theta * temp).sin() / theta.sin();
            t = (theta * t).sin() / theta.sin();
        }
        Self {
            x: temp * self.x + t * other.x,
            y: temp * self.y + t * other.y,
            z: temp * self.z + t * other.z,
            w: temp * self.w + t * other.w,
        }
    }

    /// `D3DXQuaternionSquad`: `self`, `q2`, `q3` and `q4` are the control points, `t` in `[0, 1]`.
    pub fn squad(self, q2: Self, q3: Self, q4: Self, t: f32) -> Self {
        let temp1 = self.slerp(q4, t);
        let temp2 = q2.slerp(q3, t);
        temp1.slerp(temp2, 2.0 * t * (1.0 - t))
    }

    /// `D3DXQuaternionSquadSetup`: returns `(aout, bout, cout)`. A control point is negated when
    /// its dot product with the point before it is negative.
    pub fn squad_setup(pq0: Self, pq1: Self, pq2: Self, pq3: Self) -> (Self, Self, Self) {
        let zero = Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        let temp2 = if pq0.dot(pq1) < 0.0 {
            add_diff(zero, pq0, -1.0)
        } else {
            pq0
        };
        let cout = if pq1.dot(pq2) < 0.0 {
            add_diff(zero, pq2, -1.0)
        } else {
            pq2
        };
        let temp3 = if cout.dot(pq3) < 0.0 {
            add_diff(zero, pq3, -1.0)
        } else {
            pq3
        };

        let inv1 = pq1.inverse();
        let temp2 = inv1.multiply(temp2).ln();
        let q = inv1.multiply(cout).ln();
        let temp1 = add_diff(temp2, q, 1.0).scale(-0.25).exp();
        let aout = pq1.multiply(temp1);

        let inv_c = cout.inverse();
        let temp2 = inv_c.multiply(pq1).ln();
        let q = inv_c.multiply(temp3).ln();
        let temp1 = add_diff(temp2, q, 1.0).scale(-0.25).exp();
        let bout = cout.multiply(temp1);

        (aout, bout, cout)
    }

    /// `D3DXQuaternionToAxisAngle`: the vector part is the axis, and `2 * acos(w)` the angle.
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        (
            Vec3 {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            2.0 * self.w.acos(),
        )
    }

    /// `D3DXQuaternionBaryCentric`: `self` is `pq1`, blended towards `pq2` by `f` and `pq3` by `g`.
    pub fn bary_centric(self, pq2: Self, pq3: Self, f: f32, g: f32) -> Self {
        let temp1 = self.slerp(pq2, f + g);
        let temp2 = self.slerp(pq3, f + g);
        temp1.slerp(temp2, g / (f + g))
    }
}

/// `add_diff` of `math.c`: `q1 + add * q2`, component-wise.
fn add_diff(q1: Quaternion, q2: Quaternion, add: f32) -> Quaternion {
    Quaternion {
        x: q1.x + add * q2.x,
        y: q1.y + add * q2.y,
        z: q1.z + add * q2.z,
        w: q1.w + add * q2.w,
    }
}
