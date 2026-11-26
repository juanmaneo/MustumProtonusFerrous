//! Edge-case tests for the ported w32skrnl logic.
//!
//! These cover the export table, the stub record, and the `GetWin32sDirectory`
//! concatenation boundary. Not every expectation is oracle-derived; the ones that
//! aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(WIN32S_SUFFIX, "\\win32s");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the first and last lines of dlls/w32skrnl/w32skrnl.spec, in order.
    assert_eq!(
        EXPORTS.first().copied(),
        Some(("_kSetEnvironmentVariable", ExportKind::Stub))
    );
    assert_eq!(
        EXPORTS.last().copied(),
        Some(("_FWorkingSetSize", ExportKind::Stub))
    );
    assert_eq!(
        export_kind("_kSetEnvironmentVariable"),
        Some(ExportKind::Stub)
    );
    assert_eq!(export_kind("GetCurrentTask32"), Some(ExportKind::Cdecl));
    assert_eq!(export_kind("_kGetWin32sDirectory"), Some(ExportKind::Cdecl));
    assert_eq!(export_kind("W32S_BackTo32"), Some(ExportKind::Forward));
    assert_eq!(
        export_kind("_RtlNtStatusToDosError"),
        Some(ExportKind::Forward)
    );
    assert_eq!(export_kind("not_exported"), None);
    assert_eq!(export_kind(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of w32skrnl.spec. The code
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
fn get_win32s_directory_concatenates_the_suffix() {
    // Not oracle-derived: the C strcat always appends the suffix, even when the system
    // directory already ends with a backslash.
    assert_eq!(get_win32s_directory(""), "\\win32s");
    assert_eq!(get_win32s_directory("C:\\"), "C:\\\\win32s");
    assert_eq!(get_win32s_directory("C:\\Windows"), "C:\\Windows\\win32s");
}
