//! Edge-case tests for the ported ctl3d32 logic.
//!
//! The ported code has no branches, so these cover argument boundaries (ignored
//! handle and type values), repeated and interleaved flag transitions, and the
//! version word bytes. Expectations not produced by the oracle harness are
//! marked "Not oracle-derived" in a comment.

use super::*;

#[test]
fn flag_defaults_to_false() {
    // Not oracle-derived: matches `static BOOL CTL3D_is_auto_subclass = FALSE;` in the C source.
    assert_eq!(
        Ctl3d::default(),
        Ctl3d {
            auto_subclass: false
        }
    );
    assert!(!Ctl3d::default().is_auto_subclass());
}

#[test]
fn auto_subclass_is_idempotent() {
    // Not oracle-derived: the oracle calls each function once per row.
    let mut c = Ctl3d::default();
    assert!(c.auto_subclass(0));
    assert!(c.auto_subclass(0));
    assert!(c.is_auto_subclass());
}

#[test]
fn auto_subclass_ex_ignores_handle_and_type() {
    // Not oracle-derived: the oracle covers (0x1234, 0) and (0, u32::MAX) only.
    for hinst in [0, 1, usize::MAX] {
        for ty in [0, 1, u32::MAX] {
            let mut c = Ctl3d::default();
            assert!(c.auto_subclass_ex(hinst, ty));
            assert_eq!(
                c,
                Ctl3d {
                    auto_subclass: true
                }
            );
        }
    }
}

#[test]
fn un_auto_subclass_returns_false_from_either_state() {
    // Not oracle-derived: the oracle covers init=0 and init=1 once each.
    let mut c = Ctl3d {
        auto_subclass: true,
    };
    assert!(!c.un_auto_subclass());
    assert_eq!(
        c,
        Ctl3d {
            auto_subclass: false
        }
    );
    assert!(!c.un_auto_subclass());
    assert_eq!(
        c,
        Ctl3d {
            auto_subclass: false
        }
    );
}

#[test]
fn unregister_returns_true_and_clears_flag_from_either_state() {
    // Not oracle-derived: the oracle covers init=0 and init=1 once each.
    let mut c = Ctl3d {
        auto_subclass: false,
    };
    assert!(c.unregister(usize::MAX));
    assert_eq!(
        c,
        Ctl3d {
            auto_subclass: false
        }
    );
    c.auto_subclass(0);
    assert!(c.unregister(0));
    assert_eq!(
        c,
        Ctl3d {
            auto_subclass: false
        }
    );
}

#[test]
fn is_auto_subclass_does_not_change_state() {
    // Not oracle-derived: checks that the getter leaves the flag untouched.
    let c = Ctl3d {
        auto_subclass: true,
    };
    assert!(c.is_auto_subclass());
    assert_eq!(
        c,
        Ctl3d {
            auto_subclass: true
        }
    );
    let c = Ctl3d::default();
    assert!(!c.is_auto_subclass());
    assert_eq!(c, Ctl3d::default());
}

#[test]
fn flag_transition_sequences_round_trip() {
    // Not oracle-derived: longer interleavings than the oracle's three sequences.
    let mut c = Ctl3d::default();
    assert!(!c.un_auto_subclass());
    assert!(c.auto_subclass_ex(0, 0));
    assert!(c.unregister(0));
    assert!(!c.is_auto_subclass());
    assert!(c.auto_subclass(1));
    assert!(c.is_auto_subclass());
    assert!(!c.un_auto_subclass());
    assert!(!c.is_auto_subclass());
    assert!(c.unregister(1));
    assert!(!c.is_auto_subclass());
}

#[test]
fn get_ver_is_makeword_31_2() {
    // Not oracle-derived for the byte split; the oracle covers the 0x21f value.
    assert_eq!(get_ver(), 543);
    assert_eq!(get_ver(), 0x021f);
    assert_eq!(get_ver().to_le_bytes(), [31, 2]);
    assert_eq!(get_ver() & 0xff, 31);
    assert_eq!(get_ver() >> 8, 2);
}
