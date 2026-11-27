//! Edge-case tests for the ported msvcp120_app logic.
//!
//! These cover the constants, the stub record's fields, the uniqueness of the export names, the
//! agreement between the stub rows and the table, and the lookup and forward boundaries. Not every
//! expectation is oracle-derived; the ones that are not say so in a comment.

use super::oracle_checks::STUB_ROWS;
use super::stub::MSVCP120_APP_DLL_NAME;
use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h, the MODULE line in
    // dlls/msvcp120_app/Makefile.in and the forward prefix in msvcp120_app.spec.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(MSVCP120_APP_DLL_NAME, "msvcp120_app.dll");
    assert_eq!(FORWARD_MODULE, "msvcp120");
}

#[test]
fn stub_record_copies_module_and_function() {
    // Not oracle-derived: the C body copies both pointers into ExceptionInformation unchanged, so
    // an empty function name and a module other than msvcp120_app.dll pass through as they are.
    assert_eq!(
        wine_spec_unimplemented_stub("other.dll", ""),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "other.dll",
            function: "",
        }
    );
}

#[test]
fn export_names_are_distinct() {
    // Not oracle-derived: no two lines of the table describe the same export name.
    let names: HashSet<&str> = EXPORTS.iter().map(|export| export.name).collect();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn stub_rows_cover_every_stub_export_once() {
    // Not oracle-derived: the helper rows and the table's `@ stub` entries name the same exports,
    // and no export has two rows.
    let stub_names: HashSet<&str> = EXPORTS
        .iter()
        .filter(|export| export.kind == ExportKind::Stub)
        .map(|export| export.name)
        .collect();
    let row_names: HashSet<&str> = STUB_ROWS.iter().map(|&(_, function, _)| function).collect();
    assert_eq!(row_names.len(), STUB_ROWS.len());
    assert_eq!(row_names, stub_names);
}

#[test]
fn lookup_is_exact_and_case_sensitive() {
    // Not oracle-derived: an empty name, a real name in another case, and a real name with one
    // character added or removed at the end.
    assert_eq!(find_export(""), None);
    assert_eq!(
        find_export("_Dint").map(|export| export.name),
        Some("_Dint")
    );
    assert_eq!(find_export("_dint"), None);
    assert_eq!(find_export("_Dinte"), None);
    assert_eq!(find_export("_Din"), None);
}

#[test]
fn forward_is_none_for_stubs_and_unknown_names() {
    // Not oracle-derived: an `@ stub` line is a generated stub, not a forward, and an unknown name
    // has no entry at all.
    assert_eq!(forward("_Dint"), None);
    assert_eq!(forward("not_exported"), None);
}

#[test]
fn forward_names_the_same_function_in_msvcp120() {
    // Not oracle-derived: one spot check of a `@ cdecl -ret64 -arch=win32` line, whose target is
    // `msvcp120.<name>` in the spec.
    let name = "?_File_size@sys@tr2@std@@YA_KPBD@Z";
    assert_eq!(
        forward(name),
        Some(Forward {
            module: "msvcp120",
            function: name,
        })
    );
}
