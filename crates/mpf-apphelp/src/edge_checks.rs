//! Edge-case tests for the ported apphelp logic.
//!
//! These cover the buffer and pointer boundaries, the derived trait impls, and the export
//! table. Expectations that no printed oracle row covers say so in a comment; they follow
//! from the reference C source or from the spec.

use super::*;

#[test]
fn shell_object_clears_a_zero_flags_value_too() {
    // Not oracle-derived: no harness row used a zero pre-value. The C writes `*flags = 0`
    // unconditionally whenever the pointer is non-NULL.
    let mut flags = 0;
    assert!(apphelp_check_shell_object(Some(&mut flags)));
    assert_eq!(flags, 0);
}

#[test]
fn app_patch_dir_size_one_writes_only_the_first_element() {
    // Not oracle-derived: the harness used size 2 and size 0 only. The C writes `*path = 0`
    // once, so the second element is untouched.
    let mut buf = [0x41_u16, 0x42, 0x43];
    sdb_get_app_patch_dir(Some(&mut buf[0]), 1);
    assert_eq!(buf, [0x0, 0x42, 0x43]);
}

#[test]
fn app_patch_dir_max_size_still_writes_the_first_element() {
    // Not oracle-derived: the harness did not run size 0xffffffff. The test only checks
    // that a large size takes the same path as a small non-zero one.
    let mut buf = [0x41_u16, 0x42];
    sdb_get_app_patch_dir(Some(&mut buf[0]), u32::MAX);
    assert_eq!(buf, [0x0, 0x42]);
}

#[test]
fn app_patch_dir_null_path_with_zero_size_is_a_noop() {
    // Not oracle-derived: the harness never passed NULL with size 0. Neither condition holds,
    // so nothing is written and nothing faults.
    sdb_get_app_patch_dir(None, 0);
}

#[test]
fn stub_result_not_implemented_keeps_the_value() {
    // Not oracle-derived: the constructor is also used with a non-bool value.
    let r = StubResult::not_implemented(7usize);
    assert_eq!(r.value, 7);
    assert_eq!(r.last_error, ERROR_CALL_NOT_IMPLEMENTED);
}

#[test]
fn stub_result_derived_impls_for_bool() {
    let a = shim_flush_cache();
    let b = a;
    assert_eq!(a.clone(), b);
    assert!(a != StubResult::not_implemented(false));
    assert_eq!(
        format!("{a:?}"),
        "StubResult { value: true, last_error: 120 }"
    );
}

#[test]
fn stub_result_derived_impls_for_handle() {
    let a = sdb_create_database();
    let b = a;
    assert_eq!(a.clone(), b);
    assert!(a != StubResult::not_implemented(1));
    assert_eq!(format!("{a:?}"), "StubResult { value: 0, last_error: 120 }");
}

#[test]
fn stub_exception_derived_impls() {
    let e = unimplemented_stub("apphelp", "SdbGetNextChild");
    assert_eq!(e.clone(), e);
    assert!(e != unimplemented_stub("apphelp", "SetPermLayers"));
    assert!(format!("{e:?}").contains("function: \"SdbGetNextChild\""));
}

#[test]
fn unimplemented_stub_forwards_names_unchanged() {
    // Not oracle-derived: the harness only used non-empty names. The C passes the module and
    // function pointers straight through to the exception arguments.
    let e = unimplemented_stub("", "");
    assert_eq!(e.module, "");
    assert_eq!(e.function, "");
    assert_eq!(e.code, EXCEPTION_WINE_STUB);
    assert_eq!(e.flags, EXCEPTION_NONCONTINUABLE);
}

#[test]
fn exception_and_error_constants_match_the_headers() {
    // Not oracle-derived: the values come from `winternl.h`, `winnt.h` and `winerror.h`, the
    // same headers the harness includes through its shim.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(ERROR_CALL_NOT_IMPLEMENTED, 120);
}

#[test]
fn every_stub_export_raises_the_same_exception() {
    // Repeats harness check S1 on the Rust table. The harness is the oracle; this guards
    // the table against edits.
    for name in STUB_EXPORTS {
        let e = unimplemented_stub("apphelp", name);
        assert_eq!(e.code, EXCEPTION_WINE_STUB);
        assert_eq!(e.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(e.module, "apphelp");
        assert_eq!(e.function, name);
    }
}

#[test]
fn stub_exports_are_spec_ordered_unique_and_exclude_defined_exports() {
    // Not oracle-derived: the names and order come from the `@ stub` lines of `apphelp.spec`.
    assert_eq!(STUB_EXPORTS[0], "AllowPermLayer");
    assert_eq!(STUB_EXPORTS[154], "ShimDumpCache");

    let mut names = STUB_EXPORTS.to_vec();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), 155);

    // The 13 `@ stdcall` exports in `apphelp.spec` have bodies in `apphelp.c`, so they are not stubs.
    let defined = [
        "ApphelpCheckInstallShieldPackage",
        "ApphelpCheckMsiPackage",
        "ApphelpCheckShellObject",
        "SdbCloseDatabase",
        "SdbCreateDatabase",
        "SdbGetAppPatchDir",
        "SdbGetFirstChild",
        "SdbGetPermLayerKeys",
        "SdbInitDatabase",
        "SdbOpenDatabase",
        "SdbSetPermLayerKeys",
        "SetPermLayerState",
        "ShimFlushCache",
    ];
    for name in defined {
        assert!(
            !STUB_EXPORTS.contains(&name),
            "{name} is defined in apphelp.c"
        );
    }
}
