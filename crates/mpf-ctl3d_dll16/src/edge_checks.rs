//! Edge-case tests for the ported ctl3d.dll16 logic.
//!
//! These cover the constants, the flag's defaults and transitions, the fixed-value exports, the
//! export table and the stub record. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn get_ver_is_makeword_31_2() {
    // Not oracle-derived: MAKEWORD(31, 2) from include/minwindef.h. The oracle row checks 543.
    assert_eq!(ctl3d_get_ver() & 0xff, 31);
    assert_eq!(ctl3d_get_ver() >> 8, 2);
}

#[test]
fn flag_starts_false_and_each_instance_is_separate() {
    // Not oracle-derived: the C static starts FALSE (oracle row 01 shows flag=0). Keeping one
    // flag per instance is a modelling choice; the C static is process-wide.
    let mut first = Ctl3d::default();
    let second = Ctl3d::default();
    assert!(!first.is_auto_subclass());
    assert!(first.auto_subclass(1));
    assert!(first.is_auto_subclass());
    assert!(!second.is_auto_subclass());
}

#[test]
fn auto_subclass_is_idempotent_and_ignores_its_arguments() {
    // Not oracle-derived: the oracle calls each setter once. The C bodies ignore the arguments
    // and set the same flag, so repeated and extreme calls keep the flag TRUE.
    let mut ctl = Ctl3d::default();
    assert!(ctl.auto_subclass(0));
    assert!(ctl.auto_subclass(u16::MAX));
    assert!(ctl.is_auto_subclass());
    assert!(ctl.auto_subclass_ex(0, 0));
    assert!(ctl.auto_subclass_ex(u16::MAX, u32::MAX));
    assert!(ctl.is_auto_subclass());
}

#[test]
fn un_auto_subclass_returns_false_with_or_without_the_flag() {
    // Not oracle-derived for the fresh-instance case. Row 22 covers the set-then-clear case; both
    // calls return FALSE, as the C body does.
    let mut ctl = Ctl3d::default();
    assert!(!ctl.un_auto_subclass());
    assert!(!ctl.is_auto_subclass());
    ctl.auto_subclass(1);
    assert!(!ctl.un_auto_subclass());
    assert!(!ctl.is_auto_subclass());
}

#[test]
fn unregister_clears_the_flag_and_returns_true_each_time() {
    // Not oracle-derived for the repeated call. Row 13 covers the fresh-instance call.
    let mut ctl = Ctl3d::default();
    ctl.auto_subclass(1);
    assert!(ctl.unregister(1));
    assert!(!ctl.is_auto_subclass());
    assert!(ctl.unregister(0));
    assert!(!ctl.is_auto_subclass());
}

#[test]
fn fixed_value_exports_ignore_their_arguments() {
    // Not oracle-derived: the oracle passes one set of arguments per export. The C bodies never
    // read their arguments, so extreme values give the same fixed result.
    assert!(!ctl3d_subclass_dlg(u16::MAX, u16::MAX));
    assert!(!ctl3d_subclass_ctl(u16::MAX));
    assert_eq!(ctl3d_ctl_color(u16::MAX, i32::MIN), 0);
    assert!(!ctl3d_enabled());
    assert!(ctl3d_color_change());
    assert!(!ctl3d_register(u16::MAX));
    assert_eq!(ctl3d_ctl_color_ex(u16::MAX, u16::MAX, isize::MIN), 0);
    assert_eq!(
        ctl3d_dlg_frame_paint(u16::MAX, u16::MAX, u16::MAX, isize::MAX),
        0
    );
    assert!(!ctl3d_subclass_dlg_ex(u16::MAX, u32::MAX));
    assert!(!ctl3d_subclass_ctl_ex(u16::MAX, i16::MIN));
    assert!(!ctl3d_unsubclass_ctl(u16::MAX));
    ctl3d_win_ini_change();
}

#[test]
fn window_procs_return_zero_for_any_message() {
    // Not oracle-derived: the oracle passes one message per proc. Extreme values also give 0.
    for proc in [
        btn_wnd_proc3d,
        edit_wnd_proc3d,
        list_wnd_proc3d,
        combo_wnd_proc3d,
        static_wnd_proc3d,
        ctl3d_dlg_proc,
    ] {
        assert_eq!(proc(u16::MAX, u16::MAX, u16::MAX, isize::MIN), 0);
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of ctl3d.dll16.spec. The code
    // and flags are the same for any names.
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
    // Not oracle-derived: the 25 export lines of dlls/ctl3d.dll16/ctl3d.dll16.spec, in order.
    // Ordinals 14 and 15 are not exported.
    let ordinals: Vec<u16> = EXPORTS.iter().map(|export| export.ordinal).collect();
    assert_eq!(ordinals, (1..=13).chain(16..=27).collect::<Vec<u16>>());
    assert!(EXPORTS
        .windows(2)
        .all(|pair| pair[0].ordinal < pair[1].ordinal));
    assert_eq!(EXPORTS[0].name, "Ctl3dGetVer");
    assert_eq!(EXPORTS[24].name, "Ctl3dAutoSubclassEx");
}

#[test]
fn only_ctl3d_set_style_is_a_stub() {
    // Not oracle-derived: every other line of the spec is `pascal`.
    let stubs: Vec<&str> = EXPORTS
        .iter()
        .filter(|export| export.kind == ExportKind::Stub)
        .map(|export| export.name)
        .collect();
    assert_eq!(stubs, ["Ctl3dSetStyle"]);
    assert_eq!(
        EXPORTS
            .iter()
            .filter(|export| export.kind == ExportKind::Pascal)
            .count(),
        24
    );
}

#[test]
fn export_kind_looks_up_names_case_sensitively() {
    // Not oracle-derived: the spec's export names. `Ctl3dGetVer16` is the C link name, which the
    // spec does not export.
    assert_eq!(export_kind("Ctl3dGetVer"), Some(ExportKind::Pascal));
    assert_eq!(export_kind("Ctl3dSetStyle"), Some(ExportKind::Stub));
    assert_eq!(export_kind("ctl3dgetver"), None);
    assert_eq!(export_kind("Ctl3dGetVer16"), None);
    assert_eq!(export_kind(""), None);
}
