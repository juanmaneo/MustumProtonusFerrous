//! Vertex predicates and edge arithmetic of Wine's `dlls/glu32/geom.c`, with the
//! `VertEq`, `VertLeq` and `TransLeq` macros of `tess.h`.
//!
//! A vertex is reduced to the two sweep-plane coordinates the predicates read, `s` and
//! `t`. The C `GLUvertex` also carries mesh links, which these functions never touch.
//! The `EdgeEval`, `EdgeSign`, `TransEval`, `TransSign` and `VertCCW` macros call the
//! `__gl_*` functions below, so they map to the same Rust functions.
//!
//! The C `assert`s in `__gl_edgeEval`, `__gl_edgeSign`, `__gl_transEval` and
//! `__gl_transSign` are kept as `assert!`s: a caller that breaks the ordering precondition
//! panics, as the C build would abort.

/// A vertex as the sweep sees it: the `s` and `t` coordinates of `GLUvertex`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub s: f64,
    pub t: f64,
}

/// `VertEq(u, v)`: both coordinates are equal.
pub fn vert_eq(u: &Vertex, v: &Vertex) -> bool {
    u.s == v.s && u.t == v.t
}

/// `__gl_vertLeq` and `VertLeq(u, v)`: `u` is lexicographically at or before `v` in `(s, t)`.
pub fn vert_leq(u: &Vertex, v: &Vertex) -> bool {
    u.s < v.s || (u.s == v.s && u.t <= v.t)
}

/// `TransLeq(u, v)`: `u` is at or before `v` in `(t, s)` order.
pub fn trans_leq(u: &Vertex, v: &Vertex) -> bool {
    u.t < v.t || (u.t == v.t && u.s <= v.s)
}

/// `__gl_edgeEval`: the signed distance from edge `uw` to `v`, measured in `t` at `v.s`.
/// Requires `vert_leq(u, v)` and `vert_leq(v, w)`.
pub fn edge_eval(u: &Vertex, v: &Vertex, w: &Vertex) -> f64 {
    assert!(vert_leq(u, v) && vert_leq(v, w));
    let gap_l = v.s - u.s;
    let gap_r = w.s - v.s;
    if gap_l + gap_r > 0.0 {
        if gap_l < gap_r {
            (v.t - u.t) + (u.t - w.t) * (gap_l / (gap_l + gap_r))
        } else {
            (v.t - w.t) + (w.t - u.t) * (gap_r / (gap_l + gap_r))
        }
    } else {
        0.0
    }
}

/// `__gl_edgeSign`: a value with the same sign as `edge_eval`, cheaper to compute.
/// Requires `vert_leq(u, v)` and `vert_leq(v, w)`.
pub fn edge_sign(u: &Vertex, v: &Vertex, w: &Vertex) -> f64 {
    assert!(vert_leq(u, v) && vert_leq(v, w));
    let gap_l = v.s - u.s;
    let gap_r = w.s - v.s;
    if gap_l + gap_r > 0.0 {
        (v.t - w.t) * gap_l + (v.t - u.t) * gap_r
    } else {
        0.0
    }
}

/// `__gl_transEval`: `edge_eval` with `s` and `t` swapped. Requires `trans_leq(u, v)`
/// and `trans_leq(v, w)`.
pub fn trans_eval(u: &Vertex, v: &Vertex, w: &Vertex) -> f64 {
    assert!(trans_leq(u, v) && trans_leq(v, w));
    let gap_l = v.t - u.t;
    let gap_r = w.t - v.t;
    if gap_l + gap_r > 0.0 {
        if gap_l < gap_r {
            (v.s - u.s) + (u.s - w.s) * (gap_l / (gap_l + gap_r))
        } else {
            (v.s - w.s) + (w.s - u.s) * (gap_r / (gap_l + gap_r))
        }
    } else {
        0.0
    }
}

/// `__gl_transSign`: `edge_sign` with `s` and `t` swapped. Requires `trans_leq(u, v)`
/// and `trans_leq(v, w)`.
pub fn trans_sign(u: &Vertex, v: &Vertex, w: &Vertex) -> f64 {
    assert!(trans_leq(u, v) && trans_leq(v, w));
    let gap_l = v.t - u.t;
    let gap_r = w.t - v.t;
    if gap_l + gap_r > 0.0 {
        (v.s - w.s) * gap_l + (v.s - u.s) * gap_r
    } else {
        0.0
    }
}

/// `__gl_vertCCW` and `VertCCW(u, v, w)`: the signed area of `uvw` is not negative.
pub fn vert_ccw(u: &Vertex, v: &Vertex, w: &Vertex) -> bool {
    u.s * (v.t - w.t) + v.s * (w.t - u.t) + w.s * (u.t - v.t) >= 0.0
}

/// `Interpolate(a, x, b, y)` from `geom.c`: `(b*x + a*y) / (a + b)`, or the midpoint
/// when `a == b == 0`. Negative weights are clamped to zero.
pub(crate) fn interpolate(a: f64, x: f64, b: f64, y: f64) -> f64 {
    let a = if a < 0.0 { 0.0 } else { a };
    let b = if b < 0.0 { 0.0 } else { b };
    if a <= b {
        if b == 0.0 {
            (x + y) / 2.0
        } else {
            x + (y - x) * (a / (a + b))
        }
    } else {
        y + (x - y) * (b / (a + b))
    }
}

/// `__gl_edgeIntersect`: the point where edges `o1d1` and `o2d2` cross. The C out
/// parameter `v` becomes the return value. Each coordinate is interpolated in the
/// same way as the C code, and the result stays inside both edges' bounding boxes.
pub fn edge_intersect(o1: &Vertex, d1: &Vertex, o2: &Vertex, d2: &Vertex) -> Vertex {
    let (mut o1, mut d1, mut o2, mut d2) = (o1, d1, o2, d2);

    if !vert_leq(o1, d1) {
        std::mem::swap(&mut o1, &mut d1);
    }
    if !vert_leq(o2, d2) {
        std::mem::swap(&mut o2, &mut d2);
    }
    if !vert_leq(o1, o2) {
        std::mem::swap(&mut o1, &mut o2);
        std::mem::swap(&mut d1, &mut d2);
    }

    let s = if !vert_leq(o2, d1) {
        (o2.s + d1.s) / 2.0
    } else if vert_leq(d1, d2) {
        let mut z1 = edge_eval(o1, o2, d1);
        let mut z2 = edge_eval(o2, d1, d2);
        if z1 + z2 < 0.0 {
            z1 = -z1;
            z2 = -z2;
        }
        interpolate(z1, o2.s, z2, d1.s)
    } else {
        let mut z1 = edge_sign(o1, o2, d1);
        let mut z2 = -edge_sign(o1, d2, d1);
        if z1 + z2 < 0.0 {
            z1 = -z1;
            z2 = -z2;
        }
        interpolate(z1, o2.s, z2, d2.s)
    };

    if !trans_leq(o1, d1) {
        std::mem::swap(&mut o1, &mut d1);
    }
    if !trans_leq(o2, d2) {
        std::mem::swap(&mut o2, &mut d2);
    }
    if !trans_leq(o1, o2) {
        std::mem::swap(&mut o1, &mut o2);
        std::mem::swap(&mut d1, &mut d2);
    }

    let t = if !trans_leq(o2, d1) {
        (o2.t + d1.t) / 2.0
    } else if trans_leq(d1, d2) {
        let mut z1 = trans_eval(o1, o2, d1);
        let mut z2 = trans_eval(o2, d1, d2);
        if z1 + z2 < 0.0 {
            z1 = -z1;
            z2 = -z2;
        }
        interpolate(z1, o2.t, z2, d1.t)
    } else {
        let mut z1 = trans_sign(o1, o2, d1);
        let mut z2 = -trans_sign(o1, d2, d1);
        if z1 + z2 < 0.0 {
            z1 = -z1;
            z2 = -z2;
        }
        interpolate(z1, o2.t, z2, d2.t)
    };

    Vertex { s, t }
}
