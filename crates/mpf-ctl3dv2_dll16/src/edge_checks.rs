//! Edge-case tests for the ported ctl3dv2.dll16 logic.
//!
//! The oracle covers each export once, in one order. These tests cover the other flag starting
//! values, argument boundaries, and the export table. Expectations not produced by the oracle
//! harness are marked "Not oracle-derived" in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn flag_defaults_to_false() {
    // Not oracle-derived: matches `static BOOL16 CTL3D16_is_auto_subclass = FALSE;`.
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
    // Not oracle-derived: the oracle calls each function once.
    let mut c = Ctl3d::default();
    assert!(c.auto_subclass(0));
    assert!(c.auto_subclass(0));
    assert!(c.is_auto_subclass());
}

#[test]
fn auto_subclass_ex_ignores_handle_and_type() {
    // Not oracle-derived: the oracle covers (0x1234, 2) only.
    for hinst in [0, 1, u16::MAX] {
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
    // Not oracle-derived: the oracle covers only the flag=1 case, once.
    let mut c = Ctl3d {
        auto_subclass: false,
    };
    assert!(!c.un_auto_subclass());
    assert_eq!(
        c,
        Ctl3d {
            auto_subclass: false
        }
    );

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
}

#[test]
fn unregister_returns_true_and_clears_flag_from_either_state() {
    // Not oracle-derived: the oracle covers the flag=0 case, once.
    let mut c = Ctl3d {
        auto_subclass: true,
    };
    assert!(c.unregister(u16::MAX));
    assert_eq!(
        c,
        Ctl3d {
            auto_subclass: false
        }
    );

    let mut c = Ctl3d {
        auto_subclass: false,
    };
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
    // Not oracle-derived: a longer interleaving than the oracle's single pass.
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
    // Not oracle-derived for the byte split; the oracle covers the 0x021f value.
    assert_eq!(get_ver(), 543);
    assert_eq!(get_ver(), 0x021f);
    assert_eq!(get_ver().to_le_bytes(), [31, 2]);
    assert_eq!(get_ver() & 0xff, 31);
    assert_eq!(get_ver() >> 8, 2);
}

#[test]
fn fixed_returns_ignore_their_arguments() {
    // Not oracle-derived: the oracle passes one set of arguments per export. These check that
    // the fixed results hold at the argument boundaries.
    for hwnd in [0, 1, u16::MAX] {
        assert!(!subclass_dlg(hwnd, u16::MAX));
        assert!(!subclass_dlg_ex(hwnd, u32::MAX));
        assert!(!subclass_ctl(hwnd));
        assert!(!subclass_ctl_ex(hwnd, i16::MIN));
        assert!(!subclass_ctl_ex(hwnd, i16::MAX));
        assert!(!unsubclass_ctl(hwnd));
        assert!(register(hwnd));
        assert_eq!(ctl_color(hwnd, i32::MIN), 0);
        assert_eq!(ctl_color_ex(hwnd, u16::MAX, i32::MAX), 0);
    }
    assert!(color_change());
    assert!(!enabled());
    win_ini_change();
}

#[test]
fn window_procedures_return_zero_for_any_message() {
    // Not oracle-derived: the oracle covers one message per procedure.
    for msg in [0, 0x0010, u16::MAX] {
        for lparam in [i32::MIN, 0, i32::MAX] {
            assert_eq!(dlg_frame_paint(1, msg, u16::MAX, lparam), 0);
            assert_eq!(dlg_proc(1, msg, u16::MAX, lparam), 0);
            assert_eq!(btn_wnd_proc_3d(1, msg, u16::MAX, lparam), 0);
            assert_eq!(edit_wnd_proc_3d(1, msg, u16::MAX, lparam), 0);
            assert_eq!(list_wnd_proc_3d(1, msg, u16::MAX, lparam), 0);
            assert_eq!(combo_wnd_proc_3d(1, msg, u16::MAX, lparam), 0);
            assert_eq!(static_wnd_proc_3d(1, msg, u16::MAX, lparam), 0);
        }
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness passed only the fixed names of ctl3dv2.dll16.spec.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 25 export lines of dlls/ctl3dv2.dll16/ctl3dv2.dll16.spec, in order.
    assert_eq!(EXPORTS.len(), 25);
    assert_eq!(EXPORTS[0], ("Ctl3dGetVer", ExportKind::Pascal));
    assert_eq!(EXPORTS[16], ("Ctl3dSetStyle", ExportKind::Stub));
    assert_eq!(EXPORTS[24], ("Ctl3dAutoSubclassEx", ExportKind::Pascal));
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|(_, k)| *k == ExportKind::Stub)
            .count(),
        1
    );
    assert_eq!(export_kind("Ctl3dGetVer"), Some(ExportKind::Pascal));
    assert_eq!(export_kind("Ctl3dSetStyle"), Some(ExportKind::Stub));
    assert_eq!(export_kind("ctl3dgetver"), None);
    assert_eq!(export_kind("Ctl3dGetVer16"), None);
    assert_eq!(export_kind(""), None);
}
