//! Edge-case tests for the ported d3d8thk export table and stub records.
//!
//! Not every expectation is oracle-derived. The ones that aren't say so in a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Oracle-derived: the harness raised code 0x80000100 with flags 0x1.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the 56 export names. The code, flags and count
    // are the same for any names.
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
fn export_table_has_56_distinct_names() {
    // Not oracle-derived: the spec has 56 `@ stub` lines, and the harness read the same list.
    // A duplicate name would make `export_stub` return only the first match.
    assert_eq!(EXPORTS.len(), 56);
    let names: HashSet<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(names.len(), 56);
}

#[test]
fn unknown_names_are_not_exports() {
    // Not oracle-derived: lookups are exact and case-sensitive. `MergeFontPackage` is fontsub's
    // export, not d3d8thk's.
    assert_eq!(export_stub("osthunkddblt"), None);
    assert_eq!(export_stub("OsThunkDdBlt "), None);
    assert_eq!(export_stub("MergeFontPackage"), None);
    assert_eq!(export_stub(""), None);
}

#[test]
fn helpers_are_called_directly() {
    // Not oracle-derived: checks two helpers by hand, including one whose C name has a `D3D` run.
    assert_eq!(
        os_thunk_dd_blt(),
        wine_spec_unimplemented_stub("d3d8thk.dll", "OsThunkDdBlt")
    );
    assert_eq!(
        os_thunk_dd_lock_d3d(),
        wine_spec_unimplemented_stub("d3d8thk.dll", "OsThunkDdLockD3D")
    );
}
