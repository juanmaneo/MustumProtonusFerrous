//! Edge-case tests for the ported magnification logic.
//!
//! These cover the last-error write, the stub record's pass-through of its names, and the
//! export table. Not every expectation is oracle-derived; the ones that aren't say so in a
//! comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    // The oracle harness uses the same values.
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn mag_initialize_overwrites_any_previous_last_error() {
    // Not oracle-derived: the harness only starts from 0. SetLastError overwrites whatever was
    // there, so the boundary values 0 and u32::MAX both end at ERROR_CALL_NOT_IMPLEMENTED.
    for start in [0, 1, ERROR_CALL_NOT_IMPLEMENTED, u32::MAX] {
        let mut last_error = start;
        assert!(!mag_initialize(&mut last_error));
        assert_eq!(last_error, ERROR_CALL_NOT_IMPLEMENTED);
    }
}

#[test]
fn mag_uninitialize_is_repeatable() {
    // Not oracle-derived: the harness calls MagUninitialize once. The C body has no state, so
    // every call returns TRUE.
    assert!(mag_uninitialize());
    assert!(mag_uninitialize());
}

#[test]
fn wine_spec_unimplemented_stub_passes_names_through() {
    // Not oracle-derived: the harness only passed the fixed names of magnification.spec. The
    // code and flags do not depend on the names.
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
fn stub_helpers_match_the_stub_entries_of_the_export_table() {
    // Not oracle-derived: every helper must name an export that EXPORTS marks as a stub, and
    // EXPORTS must have no other stub entries.
    let helpers: [fn() -> StubException; 19] = [
        mag_set_fullscreen_use_bitmap_smoothing,
        mag_set_lens_use_bitmap_smoothing,
        mag_get_color_effect,
        mag_get_fullscreen_color_effect,
        mag_get_fullscreen_transform,
        mag_get_image_scaling_callback,
        mag_get_input_transform,
        mag_get_window_filter_list,
        mag_get_window_source,
        mag_get_window_transform,
        mag_set_color_effect,
        mag_set_fullscreen_color_effect,
        mag_set_fullscreen_transform,
        mag_set_image_scaling_callback,
        mag_set_input_transform,
        mag_set_window_filter_list,
        mag_set_window_source,
        mag_set_window_transform,
        mag_show_system_cursor,
    ];
    for helper in helpers {
        let name = helper().function;
        assert_eq!(export_kind(name), Some(ExportKind::Stub), "{name}");
    }
    let stubs = EXPORTS
        .iter()
        .filter(|(_, kind)| *kind == ExportKind::Stub)
        .count();
    assert_eq!(stubs, helpers.len());
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the order and kinds of magnification.spec, and case-sensitive lookup.
    assert_eq!(EXPORTS.len(), 21);
    assert_eq!(
        EXPORTS[0],
        ("MagSetFullscreenUseBitmapSmoothing", ExportKind::Stub)
    );
    assert_eq!(EXPORTS[10], ("MagInitialize", ExportKind::Stdcall));
    assert_eq!(EXPORTS[20], ("MagUninitialize", ExportKind::Stdcall));
    assert_eq!(export_kind("MagInitialize"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("MagUninitialize"), Some(ExportKind::Stdcall));
    assert_eq!(export_kind("MagGetColorEffect"), Some(ExportKind::Stub));
    assert_eq!(export_kind("maginitialize"), None);
    assert_eq!(export_kind(""), None);
}
