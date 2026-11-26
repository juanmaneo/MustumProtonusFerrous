//! Edge-case tests for the ported acledit logic.
//!
//! These cover the reason boundaries around `DLL_PROCESS_ATTACH`, extreme
//! handle and argument values, and the stub table. Expectations that are not
//! oracle-derived say so in a comment.

use super::*;

#[test]
fn dll_main_reason_boundaries() {
    // Not oracle-derived: the reasons next to DLL_PROCESS_ATTACH (0 and 2), and
    // u32::MAX, follow from the C condition `reason == DLL_PROCESS_ATTACH`.
    assert!(!dll_main(DLL_PROCESS_ATTACH - 1).disable_thread_library_calls);
    assert!(!dll_main(DLL_PROCESS_ATTACH + 1).disable_thread_library_calls);
    assert!(!dll_main(u32::MAX).disable_thread_library_calls);
    assert_eq!(DLL_PROCESS_ATTACH, 1);
}

#[test]
fn dll_main_always_returns_true() {
    // Not oracle-derived: the C body returns TRUE on every path. These reasons
    // are the extremes of the u32 range, which the oracle does not sample.
    for reason in [0, 1, 2, 3, 0x8000_0000, u32::MAX] {
        assert!(dll_main(reason).ret);
    }
}

#[test]
fn fm_extension_proc_w_ignores_extreme_arguments() {
    // Not oracle-derived: the C body never reads its arguments, so extreme
    // handle and message values follow from the same code path.
    assert_eq!(fm_extension_proc_w(usize::MAX, usize::MAX, isize::MAX), 0);
    assert_eq!(fm_extension_proc_w(usize::MAX, 0, isize::MIN), 0);
    assert_eq!(fm_extension_proc_w(0, usize::MAX, 0), 0);
}

#[test]
fn unimplemented_stub_flags_match_wine_headers() {
    // Not oracle-derived: the constants come from include/winternl.h and
    // include/winnt.h. The oracle only checks their values through the printed rows.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    let e = unimplemented_stub("m", "f");
    assert_eq!(e.code, EXCEPTION_WINE_STUB);
    assert_eq!(e.flags, EXCEPTION_NONCONTINUABLE);
}

#[test]
fn unimplemented_stub_passes_names_through_unchanged() {
    // Not oracle-derived: the C stub stores the two pointers as-is, including
    // the empty string.
    assert_eq!(
        unimplemented_stub("", ""),
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            module: "",
            function: ""
        }
    );
    assert_eq!(unimplemented_stub("mod.dll", "Fn").module, "mod.dll");
    assert_eq!(unimplemented_stub("mod.dll", "Fn").function, "Fn");
}

#[test]
fn stub_exports_match_the_spec_file() {
    // Not oracle-derived: the six names are the `stub` lines of acledit.spec
    // (ordinals 1, 2, 3, 6, 7, 8). Every one has a distinct name and raises the same exception.
    assert_eq!(STUB_EXPORTS.len(), 6);
    for (i, name) in STUB_EXPORTS.iter().enumerate() {
        assert!(!STUB_EXPORTS[..i].contains(name));
        let e = export_stub(name);
        assert_eq!(e.code, EXCEPTION_WINE_STUB);
        assert_eq!(e.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(e.module, MODULE_FILE_NAME);
        assert_eq!(e.function, *name);
    }
}

#[test]
fn module_name_follows_winebuild_rule() {
    // Not oracle-derived: winebuild strips ".spec" from `acledit.spec` (main.c
    // set_dll_file_name) and appends ".dll" when no dot is left (main.c, main()).
    // The oracle harness hard-codes the same string.
    assert_eq!(MODULE_FILE_NAME, "acledit.dll");
}
