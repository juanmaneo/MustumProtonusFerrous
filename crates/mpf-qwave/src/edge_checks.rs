//! Edge-case tests for `qos_create_handle`.
//!
//! These cover the full `MajorVersion == 1 && MinorVersion == 0` boundary and the
//! NULL-handle branch. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

fn v(major: u16, minor: u16) -> Option<QosVersion> {
    Some(QosVersion { major, minor })
}

#[test]
fn error_constants_match_winerror() {
    // Not oracle-derived: values from Wine's include/winerror.h.
    assert_eq!(ERROR_INVALID_PARAMETER, 87);
    assert_eq!(ERROR_SERVICE_ALREADY_RUNNING, 1056);
}

#[test]
fn full_major_sweep_only_major_1_takes_placeholder() {
    // Not oracle-derived: every u16 major with minor 0 and a handle present. The
    // expectation comes from the C predicate `MajorVersion == 1`.
    for major in 0..=u16::MAX {
        let expected = if major == 1 {
            ERROR_SERVICE_ALREADY_RUNNING
        } else {
            ERROR_INVALID_PARAMETER
        };
        assert_eq!(
            qos_create_handle(v(major, 0), true),
            expected,
            "major={major}"
        );
    }
}

#[test]
fn full_minor_sweep_only_minor_0_takes_placeholder() {
    // Not oracle-derived: every u16 minor with major 1 and a handle present. The
    // expectation comes from the C predicate `MinorVersion == 0`.
    for minor in 0..=u16::MAX {
        let expected = if minor == 0 {
            ERROR_SERVICE_ALREADY_RUNNING
        } else {
            ERROR_INVALID_PARAMETER
        };
        assert_eq!(
            qos_create_handle(v(1, minor), true),
            expected,
            "minor={minor}"
        );
    }
}

#[test]
fn absent_handle_is_invalid_for_every_sampled_version() {
    // Not oracle-derived: the oracle covers the absent handle only for (1,0), (0,0),
    // and (1,256). This samples other boundary versions too, using the C `!handle` test.
    for (major, minor) in [(0, 0), (1, 0), (1, 1), (0, 1), (2, 0), (u16::MAX, u16::MAX)] {
        assert_eq!(
            qos_create_handle(v(major, minor), false),
            ERROR_INVALID_PARAMETER
        );
    }
}
