//! Boundary tests for the ported hypot wrappers.
//!
//! The functions have no branches of their own; these tests exercise the libm
//! behaviour they delegate to (zeros, infinities, NaN, intermediate overflow,
//! sign symmetry). Expectations that are not oracle-derived say so in a comment.

use crate::{std_smf_hypot3, std_smf_hypot3f};

#[test]
fn hypot3_exact_pythagorean_results() {
    // Not oracle-derived: exact integer triples, in any argument order.
    assert_eq!(std_smf_hypot3(12.0, 3.0, 4.0), 13.0);
    assert_eq!(std_smf_hypot3(0.0, 0.0, -5.0), 5.0);
    assert_eq!(std_smf_hypot3(0.0, 5.0, 0.0), 5.0);
    assert_eq!(std_smf_hypot3f(12.0, 3.0, 4.0), 13.0);
}

#[test]
fn hypot3_is_sign_symmetric() {
    // Not oracle-derived: hypot ignores the sign of each argument.
    let samples = [(3.0, -4.0, 12.0), (0.1, 0.2, 0.3), (1e-310, 2e-310, 3e-310)];
    for (x, y, z) in samples {
        assert_eq!(
            std_smf_hypot3(-x, -y, -z).to_bits(),
            std_smf_hypot3(x, y, z).to_bits()
        );
    }
}

#[test]
fn hypot3_zero_sign_gives_positive_zero() {
    // Not oracle-derived: mixed-sign zeros; hypot(+-0, +-0) is +0.
    assert_eq!(std_smf_hypot3(-0.0, -0.0, 0.0).to_bits(), 0);
    assert_eq!(std_smf_hypot3f(-0.0, 0.0, -0.0).to_bits(), 0);
}

#[test]
fn hypot3_nan_and_infinity_rules() {
    // Not oracle-derived: C99 Annex F. NaN propagates unless an input is infinite.
    assert!(std_smf_hypot3(f64::NAN, 0.0, 0.0).is_nan());
    assert!(std_smf_hypot3f(0.0, f32::NAN, 0.0).is_nan());
    assert_eq!(
        std_smf_hypot3(f64::NAN, f64::NAN, f64::INFINITY),
        f64::INFINITY
    );
    assert_eq!(
        std_smf_hypot3f(f32::NAN, f32::NAN, f32::NEG_INFINITY),
        f32::INFINITY
    );
}

#[test]
fn hypot3_intermediate_overflow_boundaries() {
    // Not oracle-derived: the exact result (2.5e308) exceeds DBL_MAX, so the
    // first hypot overflows and the wrapper returns +inf.
    assert_eq!(std_smf_hypot3(f64::MAX, f64::MAX, 0.0), f64::INFINITY);
    // Not oracle-derived: a single DBL_MAX argument is returned unchanged.
    assert_eq!(std_smf_hypot3(0.0, f64::MAX, 0.0), f64::MAX);
    assert_eq!(std_smf_hypot3f(0.0, 0.0, f32::MAX), f32::MAX);
}

#[test]
fn hypot3f_overflows_where_hypot3_does_not() {
    // Not oracle-derived: hypotf(FLT_MAX, FLT_MAX) overflows float, so the
    // single-precision wrapper returns +inf for any z, while the double wrapper
    // on the same values stays finite (about 4.8e38 > f32::MAX).
    let m = f32::MAX;
    assert_eq!(std_smf_hypot3f(m, m, 0.0), f32::INFINITY);
    let wide = std_smf_hypot3(f64::from(m), f64::from(m), 0.0);
    assert!(wide.is_finite());
    assert!(wide > f64::from(f32::MAX));
}
