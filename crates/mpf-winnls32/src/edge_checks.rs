//! Edge-case tests for the ported winnls32 logic.
//!
//! These cover the `BOOL` negation boundary, the unread window argument, and the stub and export
//! tables. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/windef.h, include/winternl.h, include/winnt.h
    // and the MODULE line of dlls/winnls32/Makefile.in.
    assert_eq!(TRUE, 1);
    assert_eq!(FALSE, 0);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WINNLS32_DLL_NAME, "winnls32.dll");
}

#[test]
fn enable_ime_is_logical_not_for_every_value() {
    // Not oracle-derived: the oracle covers 0, 1, 2, -1, INT_MIN and INT_MAX. These extra values
    // follow from `!fEnable` in the C body: any non-zero value gives 0, and 0 gives 1.
    for enable in [-100, -3, -2, 3, 100, 0x4000_0000] {
        assert_eq!(winnls32_enable_ime(0, enable), FALSE, "enable={enable}");
    }
    assert_eq!(winnls32_enable_ime(0, FALSE), TRUE);
}

#[test]
fn enable_ime_result_does_not_depend_on_the_window() {
    // Not oracle-derived: the C body never reads hWnd. The oracle used only NULL and 0x1234.
    for hwnd in [0, 1, 0x1234, usize::MAX / 2, usize::MAX] {
        assert_eq!(winnls32_enable_ime(hwnd, FALSE), TRUE);
        assert_eq!(winnls32_enable_ime(hwnd, TRUE), FALSE);
    }
}

#[test]
fn get_enable_status_is_always_false() {
    // Not oracle-derived beyond the three oracle rows: FALSE for every window handle.
    for hwnd in [0, 1, 0x1234, usize::MAX / 2, usize::MAX] {
        assert_eq!(winnls32_get_enable_status(hwnd), FALSE);
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of winnls32.spec. The code and
    // flags are the same for any names.
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
fn every_stub_helper_raises_the_same_record_shape() {
    // Not oracle-derived: calls each helper in STUB_EXPORTS and checks the shared fields and the
    // export name it passes.
    assert_eq!(STUB_EXPORTS.len(), 5);
    for (name, helper) in STUB_EXPORTS {
        let record = helper();
        assert_eq!(record.code, EXCEPTION_WINE_STUB, "{name}");
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE, "{name}");
        assert_eq!(record.number_parameters, 2, "{name}");
        assert_eq!(record.module, WINNLS32_DLL_NAME, "{name}");
        assert_eq!(record.function, *name);
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the seven lines of dlls/winnls32/winnls32.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("WINNLS32EnableIME", ExportKind::Stdcall),
            ("WINNLS32GetEnableStatus", ExportKind::Stdcall),
            ("WINNLS32GetIMEHotKey", ExportKind::Stub),
            ("IMP32GetIME", ExportKind::Stub),
            ("IMP32QueryIME", ExportKind::Stub),
            ("IMP32SetIME", ExportKind::Stub),
            ("IME32SendIMEMessageEx", ExportKind::Stub),
        ]
    );
    assert_eq!(export_kind("WINNLS32EnableIME"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("IMP32SetIME"), Some(ExportKind::Stub));
    assert_eq!(export_kind("imp32setime"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_table_matches_the_stub_exports_of_the_spec() {
    // Not oracle-derived: every `stub` entry of EXPORTS has a helper in STUB_EXPORTS, and the
    // helper's export name is the one the spec lists.
    let stub_names: Vec<&str> = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .map(|(name, _)| *name)
        .collect();
    let helper_names: Vec<&str> = STUB_EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(stub_names, helper_names);
}
