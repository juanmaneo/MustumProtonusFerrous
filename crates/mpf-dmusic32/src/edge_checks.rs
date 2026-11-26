//! Edge-case tests for the dmusic32 stub exports.
//!
//! The success return is the only observable behaviour, so these tests check
//! that it holds at the extremes of the argument type. The expectations are
//! not oracle-derived; they follow from the verbatim C bodies, which ignore
//! their arguments and return `S_OK` unconditionally.

use super::*;

#[test]
fn s_ok_is_zero() {
    // Not oracle-derived: `S_OK` is defined as 0 in winerror.h, matching the C shim.
    assert_eq!(S_OK, 0);
}

#[test]
fn create_port_ignores_every_argument_class() {
    // Not oracle-derived: boundary values of usize, beyond the oracle's sampled classes.
    for v in [0, 1, usize::MAX / 2, usize::MAX - 1, usize::MAX] {
        assert_eq!(create_cdirectmusic_emulate_port(v, 0, 0), S_OK);
        assert_eq!(create_cdirectmusic_emulate_port(0, v, usize::MAX), S_OK);
        assert_eq!(create_cdirectmusic_emulate_port(usize::MAX, v, v), S_OK);
    }
}

#[test]
fn enum_legacy_devices_ignores_every_argument_class() {
    // Not oracle-derived: boundary values of usize, beyond the oracle's sampled classes.
    for v in [0, 1, usize::MAX / 2, usize::MAX - 1, usize::MAX] {
        assert_eq!(enum_legacy_devices(v, 0), S_OK);
        assert_eq!(enum_legacy_devices(0, v), S_OK);
        assert_eq!(enum_legacy_devices(v, v), S_OK);
    }
}
