//! Edge checks for the port. The rows marked generated come from
//! `target/canon/glu32/gen_rows.py`, run against the verbatim Wine C code under ASan and
//! UBSan. They reach branches the oracle rows do not. The hand-written tests cover the C
//! `assert` preconditions, the `Interpolate` weights and the IEEE corner cases.

use crate::geom::{
    edge_eval, edge_intersect, edge_sign, interpolate, trans_eval, trans_leq, trans_sign, vert_ccw,
    vert_eq, vert_leq, Vertex,
};
use crate::project::unproject;

fn v(s: f64, t: f64) -> Vertex {
    Vertex { s, t }
}

fn assert_bits(got: f64, want: f64) {
    assert_eq!(got.to_bits(), want.to_bits());
}

/// Wine C: `__gl_edgeSign`. Covers the vertical-edge zero of edge_sign.
#[test]
fn edge_sign_vertical() {
    let x0 = v(2.0, 0.0);
    let x1 = v(2.0, 1.0);
    let x2 = v(2.0, 5.0);
    assert_bits(edge_sign(&x0, &x1, &x2), 0.0);
}

/// Wine C: `__gl_transEval`. Covers the horizontal-edge zero of trans_eval.
#[test]
fn edge_trans_eval_horizontal() {
    let x0 = v(0.0, 2.0);
    let x1 = v(1.0, 2.0);
    let x2 = v(5.0, 2.0);
    assert_bits(trans_eval(&x0, &x1, &x2), 0.0);
}

/// Wine C: `__gl_transSign`. Covers the horizontal-edge zero of trans_sign.
#[test]
fn edge_trans_sign_horizontal() {
    let x0 = v(0.0, 2.0);
    let x1 = v(1.0, 2.0);
    let x2 = v(5.0, 2.0);
    assert_bits(trans_sign(&x0, &x1, &x2), 0.0);
}

/// Wine C: `gluUnProject`. Covers the w-zero return of unproject after the inverse succeeds.
#[test]
fn edge_unproject_w_zero_after_inverse() {
    let x0 = [640.0, 0.0, 0.5];
    let x1 = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    let x2 = [
        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0,
    ];
    let x3 = [0, 0, 640, 480];
    assert!(unproject(x0, &x1, &x2, x3).is_none());
}

/// Wine C: `__gl_edgeIntersect`. Covers I_GT, I_MID, S_EVAL, T_EVAL, T_SWAP2.
#[test]
fn edge_intersect_path_1() {
    let x0 = v(-2.0, -2.0);
    let x1 = v(-2.0, -1.0);
    let x2 = v(-2.0, -1.0);
    let x3 = v(-1.0, -2.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -1.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_CLAMP_A, I_LE, S_MID, T_SIGN, T_SIGN_NEG.
#[test]
fn edge_intersect_path_2() {
    let x0 = v(-2.0, -2.0);
    let x1 = v(-2.0, -1.0);
    let x2 = v(-1.0, -2.0);
    let x3 = v(0.0, -2.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -1.5);
    assert_bits(got.t, -2.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_LE, S_SIGN, S_SIGN_NEG, T_EVAL.
#[test]
fn edge_intersect_path_3() {
    let x0 = v(-2.0, -2.0);
    let x1 = v(-1.0, -2.0);
    let x2 = v(-2.0, -2.0);
    let x3 = v(-2.0, -1.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -2.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers S_MID, S_SWAP3, T_MID.
#[test]
fn edge_intersect_path_4() {
    let x0 = v(-2.0, -1.0);
    let x1 = v(-2.0, -1.0);
    let x2 = v(-2.0, -2.0);
    let x3 = v(-2.0, -2.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -1.5);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_MID, S_EVAL, S_SWAP2, T_EVAL.
#[test]
fn edge_intersect_path_5() {
    let x0 = v(-2.0, -2.0);
    let x1 = v(-2.0, -2.0);
    let x2 = v(-2.0, -1.0);
    let x3 = v(-2.0, -2.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -2.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_LE, S_EVAL, T_EVAL, T_EVAL_NEG.
#[test]
fn edge_intersect_path_6() {
    let x0 = v(-2.0, -2.0);
    let x1 = v(-2.0, -1.0);
    let x2 = v(-2.0, -2.0);
    let x3 = v(-1.0, -1.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -2.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_CLAMP_B, I_GT, S_MID, T_EVAL, T_SWAP2.
#[test]
fn edge_intersect_path_7() {
    let x0 = v(-2.0, -2.0);
    let x1 = v(-2.0, -1.0);
    let x2 = v(-2.0, 0.0);
    let x3 = v(-1.0, -2.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -1.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_LE, S_EVAL, S_EVAL_NEG, T_EVAL.
#[test]
fn edge_intersect_path_8() {
    let x0 = v(-2.0, -2.0);
    let x1 = v(-1.0, -2.0);
    let x2 = v(-2.0, -2.0);
    let x3 = v(-1.0, -1.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -2.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_MID, S_SIGN, S_SWAP1, T_SIGN.
#[test]
fn edge_intersect_path_9() {
    let x0 = v(-2.0, -1.0);
    let x1 = v(-2.0, -2.0);
    let x2 = v(-2.0, -2.0);
    let x3 = v(-2.0, -2.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -2.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_MID, S_EVAL, T_EVAL, T_SWAP2, T_SWAP3.
#[test]
fn edge_intersect_path_10() {
    let x0 = v(-2.0, -1.0);
    let x1 = v(-2.0, -1.0);
    let x2 = v(-2.0, -1.0);
    let x3 = v(-1.0, -2.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -1.0);
}

/// Wine C: `__gl_edgeIntersect`. Covers I_MID, S_SIGN, T_EVAL, T_SWAP1.
#[test]
fn edge_intersect_path_11() {
    let x0 = v(-2.0, -1.0);
    let x1 = v(-1.0, -2.0);
    let x2 = v(-2.0, -1.0);
    let x3 = v(-2.0, -1.0);
    let got = edge_intersect(&x0, &x1, &x2, &x3);
    assert_bits(got.s, -2.0);
    assert_bits(got.t, -1.0);
}

#[test]
fn vert_eq_needs_both_coordinates() {
    assert!(vert_eq(&v(1.0, 2.0), &v(1.0, 2.0)));
    assert!(!vert_eq(&v(1.0, 2.0), &v(3.0, 2.0)));
    assert!(!vert_eq(&v(1.0, 2.0), &v(1.0, 3.0)));
}

#[test]
fn vert_leq_rejects_larger_s() {
    assert!(!vert_leq(&v(3.0, 0.0), &v(1.0, 0.0)));
}

#[test]
fn trans_leq_orders_by_t_then_s() {
    assert!(trans_leq(&v(0.0, 2.0), &v(1.0, 2.0)));
    assert!(!trans_leq(&v(3.0, 2.0), &v(1.0, 2.0)));
    assert!(!trans_leq(&v(0.0, 3.0), &v(0.0, 1.0)));
}

#[test]
fn interpolate_weights() {
    assert_bits(interpolate(0.0, 1.0, 0.0, 3.0), 2.0);
    assert_bits(interpolate(-1.0, 1.0, 1.0, 3.0), 1.0);
    assert_bits(interpolate(3.0, 1.0, -1.0, 3.0), 3.0);
    assert_bits(interpolate(3.0, 0.0, 1.0, 4.0), 3.0);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn edge_eval_panics_when_u_is_after_v() {
    edge_eval(&v(1.0, 0.0), &v(0.0, 0.0), &v(2.0, 0.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn edge_eval_panics_when_v_is_after_w() {
    edge_eval(&v(0.0, 0.0), &v(2.0, 0.0), &v(1.0, 0.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn edge_sign_panics_when_u_is_after_v() {
    edge_sign(&v(1.0, 0.0), &v(0.0, 0.0), &v(2.0, 0.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn edge_sign_panics_when_v_is_after_w() {
    edge_sign(&v(0.0, 0.0), &v(2.0, 0.0), &v(1.0, 0.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn trans_eval_panics_when_u_is_after_v() {
    trans_eval(&v(0.0, 1.0), &v(0.0, 0.0), &v(0.0, 2.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn trans_eval_panics_when_v_is_after_w() {
    trans_eval(&v(0.0, 0.0), &v(0.0, 2.0), &v(0.0, 1.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn trans_sign_panics_when_u_is_after_v() {
    trans_sign(&v(0.0, 1.0), &v(0.0, 0.0), &v(0.0, 2.0));
}

#[test]
#[should_panic(expected = "assertion failed")]
fn trans_sign_panics_when_v_is_after_w() {
    trans_sign(&v(0.0, 0.0), &v(0.0, 2.0), &v(0.0, 1.0));
}

#[test]
fn vert_ccw_is_false_for_nan() {
    assert!(!vert_ccw(&v(f64::NAN, 0.0), &v(1.0, 0.0), &v(0.0, 1.0)));
}

#[test]
fn unproject_zero_viewport_width_gives_nan() {
    let identity = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    let got = unproject([400.0, 300.0, 0.25], &identity, &identity, [0, 0, 0, 480])
        .expect("identity is invertible and w is not zero");
    assert!(got.iter().all(|c| c.is_nan()));
}
