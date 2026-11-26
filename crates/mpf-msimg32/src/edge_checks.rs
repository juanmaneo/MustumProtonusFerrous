//! Edge-case tests for the ported msimg32 logic.
//!
//! These cover the constants, the last-error slot, the `DllMain` reason boundary and the export
//! table. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h and include/winnt.h.
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
    assert_eq!(DLL_PROCESS_ATTACH, 1);
}

#[test]
fn v_set_ddrawflag_is_idempotent() {
    // Not oracle-derived: the oracle calls vSetDdrawflag once per prior value. This checks that a
    // second call leaves the slot at the same value.
    let mut last_error = 0;
    v_set_ddrawflag(&mut last_error);
    v_set_ddrawflag(&mut last_error);
    assert_eq!(last_error, ERROR_CALL_NOT_IMPLEMENTED);
}

#[test]
fn dll_main_disables_thread_library_calls_only_at_the_attach_boundary() {
    // Not oracle-derived: the values either side of DLL_PROCESS_ATTACH are 0 and 2, which the
    // oracle covers, and u32::MAX, which it also covers. The expectations below follow from the C
    // `if` in libs/winecrt0/dll_main.c and were not run.
    assert_eq!(dll_main(7, 0).disable_thread_library_calls, None);
    assert_eq!(dll_main(7, 1).disable_thread_library_calls, Some(7));
    assert_eq!(dll_main(7, 2).disable_thread_library_calls, None);
    assert_eq!(dll_main(7, u32::MAX).disable_thread_library_calls, None);
}

#[test]
fn dll_main_passes_the_inst_handle_through_unchanged() {
    // Not oracle-derived: the oracle uses inst values 0 and 4096. This uses the largest handle
    // value, which the C body passes through without reading.
    assert_eq!(
        dll_main(usize::MAX, DLL_PROCESS_ATTACH).disable_thread_library_calls,
        Some(usize::MAX)
    );
}

#[test]
fn export_table_matches_msimg32_spec() {
    // Not oracle-derived: names, kinds and forwarder targets are read from
    // dlls/msimg32/msimg32.spec in spec order.
    assert_eq!(
        EXPORTS,
        [
            ("vSetDdrawflag", ExportKind::CFunction),
            (
                "AlphaBlend",
                ExportKind::Forward {
                    module: "gdi32",
                    function: "GdiAlphaBlend",
                },
            ),
            (
                "DllInitialize",
                ExportKind::Private {
                    implementation: "DllMain",
                },
            ),
            (
                "GradientFill",
                ExportKind::Forward {
                    module: "gdi32",
                    function: "GdiGradientFill",
                },
            ),
            (
                "TransparentBlt",
                ExportKind::Forward {
                    module: "gdi32",
                    function: "GdiTransparentBlt",
                },
            ),
        ]
    );
}

#[test]
fn export_kind_is_case_sensitive_and_rejects_unknown_names() {
    // Not oracle-derived: lookup behaviour of the table. Names are matched exactly.
    assert_eq!(export_kind("vSetDdrawflag"), Some(ExportKind::CFunction));
    assert_eq!(export_kind("vsetddrawflag"), None);
    assert_eq!(export_kind("AlphaBlend "), None);
    assert_eq!(export_kind("MergeFontPackage"), None);
    assert_eq!(export_kind(""), None);
}
