//! Point, affine matrix and vector helpers shared by the geometry code, following the operation
//! order of `d2d1_private.h` and `geometry.c`.

/// `D2D1_POINT_2F`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// `D2D_MATRIX_3X2_F`: `m11`/`m12`/`m21`/`m22` are the linear part and `m31`/`m32` the translation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x2 {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}

/// `D2D1_BEZIER_SEGMENT`: two control points followed by the end point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bezier {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
}

pub(crate) fn subtract(a: Point, b: Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

pub(crate) fn scale(p: Point, s: f32) -> Point {
    Point {
        x: p.x * s,
        y: p.y * s,
    }
}

pub(crate) fn lerp(a: Point, b: Point, t: f32) -> Point {
    Point {
        x: a.x * (1.0 - t) + b.x * t,
        y: a.y * (1.0 - t) + b.y * t,
    }
}

pub(crate) fn dot(a: Point, b: Point) -> f32 {
    a.x * b.x + a.y * b.y
}

pub(crate) fn length(p: Point) -> f32 {
    dot(p, p).sqrt()
}

pub(crate) fn normalise(p: Point) -> Point {
    let l = length(p);
    if l != 0.0 {
        scale(p, 1.0 / l)
    } else {
        p
    }
}

pub(crate) fn transform_point(m: &Matrix3x2, x: f32, y: f32) -> Point {
    Point {
        x: x * m.m11 + y * m.m21 + m.m31,
        y: x * m.m12 + y * m.m22 + m.m32,
    }
}
