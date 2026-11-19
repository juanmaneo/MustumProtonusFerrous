//! Stroked line and cubic Bezier hit tests, ports of `d2d_point_on_line_segment` and
//! `d2d_point_on_bezier_segment` in `geometry.c`.

use crate::ccw;
use crate::point::{
    dot, length, lerp, normalise, scale, subtract, transform_point, Bezier, Matrix3x2, Point,
};

/// C `min` as Wine defines it: `a < b ? a : b`.
fn c_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// C `max` as Wine defines it: `a > b ? a : b`.
fn c_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Scales an extreme distance by 3/4 when both control distances lie on the same side of the
/// chord, and by 4/9 otherwise.
fn attenuate(d: f32, d1: f32, d2: f32) -> f32 {
    if d1 * d2 > 0.0 {
        d * 0.75
    } else {
        d * 4.0 / 9.0
    }
}

fn attenuate_pair(d1: f32, d2: f32) -> (f32, f32) {
    (attenuate(d1, d1, d2), attenuate(d2, d1, d2))
}

/// Whether `q` is within the stroke of the segment `p0`-`p1` after `transform`.
pub fn point_on_line_segment(
    q: Point,
    p0: Point,
    p1: Point,
    transform: &Matrix3x2,
    stroke_width: f32,
    tolerance: f32,
) -> bool {
    let v_p0 = subtract(p1, p0);
    let l0 = length(v_p0);
    if l0 == 0.0 {
        return false;
    }

    let l = stroke_width / l0;
    let origin = Point {
        x: transform.m31,
        y: transform.m32,
    };
    let mut v_n = subtract(transform_point(transform, -v_p0.y * l, v_p0.x * l), origin);
    let v_p = subtract(transform_point(transform, v_p0.x, v_p0.y), origin);
    let mut v_q = subtract(q, transform_point(transform, p0.x, p0.y));

    let mut l = v_p.x * v_n.y - v_p.y * v_n.x;
    let mut v_r = Point {
        x: v_q.x * v_n.y - v_q.y * v_n.x,
        y: v_q.x * v_p.y - v_q.y * v_p.x,
    };
    if l < 0.0 {
        l = -l;
        v_r.x = -v_r.x;
    }

    if v_r.x < 0.0 || v_r.x > l {
        return false;
    }
    if v_r.y.abs() < l {
        return true;
    }

    if v_r.y > 0.0 {
        v_n = scale(v_n, -1.0);
    }
    v_q = subtract(v_q, v_n);

    let proj = dot(v_q, v_p);
    if proj < 0.0 || proj > dot(v_p, v_p) {
        return false;
    }

    let v_n = normalise(Point {
        x: -v_p.y,
        y: v_p.x,
    });
    dot(v_q, v_n).abs() < tolerance
}

/// Whether `q` is within the stroke of the cubic Bezier `p0`, `b` after `transform`.
///
/// The curve is approximated by a segment whose error is bounded, recursively subdividing until
/// either the approximation is within tolerance or `q` is provably outside the stroke.
pub fn point_on_bezier_segment(
    q: Point,
    p0: Point,
    b: &Bezier,
    transform: &Matrix3x2,
    stroke_width: f32,
    tolerance: f32,
) -> bool {
    let mut m = 1.0_f32;
    let w = stroke_width * 0.5;

    let mut v_p = subtract(b.point3, p0);
    let mut l = dot(v_p, v_p);
    if l == 0.0 {
        v_p = subtract(b.point2, b.point1);
        l = dot(v_p, v_p);
        if l == 0.0 {
            v_p = subtract(b.point1, p0);
            let end = Point {
                x: p0.x + 0.75 * v_p.x,
                y: p0.y + 0.75 * v_p.y,
            };
            return point_on_line_segment(q, p0, end, transform, w, tolerance);
        }
        m = 0.0;
    }
    l = l.sqrt();
    v_p = scale(v_p, 1.0 / l);
    m *= l;

    let end = Point {
        x: p0.x + v_p.x,
        y: p0.y + v_p.y,
    };
    let (d1, d2) = attenuate_pair(ccw(p0, end, b.point1), ccw(p0, end, b.point2));
    let w2 = c_max(d1.abs(), d2.abs());

    let d1 = dot(v_p, subtract(b.point1, p0));
    let d2 = dot(v_p, subtract(b.point2, p0));
    let d = attenuate(c_min(c_min(d1, d2), 0.0), d1, d2);
    let d = d - c_min(w + tolerance, w2);
    let start = Point {
        x: p0.x + d * v_p.x,
        y: p0.y + d * v_p.y,
    };

    let d1 = d1 - m;
    let d2 = d2 - m;
    let d = m + attenuate(c_max(c_max(d1, d2), 0.0), d1, d2);
    let d = d + c_min(w2, w + tolerance);
    let end = Point {
        x: p0.x + d * v_p.x,
        y: p0.y + d * v_p.y,
    };

    let t0 = transform_point(transform, p0.x, p0.y);
    let t1 = transform_point(transform, b.point1.x, b.point1.y);
    let t2 = transform_point(transform, b.point2.x, b.point2.y);
    let t3 = transform_point(transform, b.point3.x, b.point3.y);
    let t4 = transform_point(transform, start.x, start.y);
    let t5 = transform_point(transform, end.x, end.y);

    let l = length(subtract(t5, t4));
    let (d1, d2) = attenuate_pair(ccw(t4, t5, t1) / l, ccw(t4, t5, t2) / l);
    let l = c_max(c_max(d1, d2), 0.0) - c_min(c_min(d1, d2), 0.0);
    let d3 = length(subtract(t4, t0));
    let d4 = length(subtract(t5, t3));
    let l = c_max(c_max(d3, d4), l);

    if l <= tolerance && point_on_line_segment(q, start, end, transform, w, tolerance - l) {
        return true;
    }
    if !point_on_line_segment(q, start, end, transform, w + w2, tolerance) {
        return false;
    }

    let mid = lerp(b.point1, b.point2, 0.5);
    let b1_point2 = lerp(b.point3, b.point2, 0.5);
    let b1 = Bezier {
        point1: lerp(mid, b1_point2, 0.5),
        point2: b1_point2,
        point3: b.point3,
    };
    let b0_point1 = lerp(p0, b.point1, 0.5);
    let b0_point2 = lerp(mid, b0_point1, 0.5);
    let b0 = Bezier {
        point1: b0_point1,
        point2: b0_point2,
        point3: lerp(b0_point2, b1.point1, 0.5),
    };

    point_on_bezier_segment(q, p0, &b0, transform, stroke_width, tolerance)
        || point_on_bezier_segment(q, b0.point3, &b1, transform, stroke_width, tolerance)
}
