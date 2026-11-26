//! Edge-case tests for the ported cng.sys logic.
//!
//! These cover the shape of the export table, both branches of the lookup and stub functions,
//! and the names around each entry. Expectations that are not oracle-derived say so in a comment.

use super::*;

/// Not oracle-derived: counts of `@ stub` and `@ stdcall` lines in `cng.sys.spec`
/// (`grep -c`), 63 in all.
#[test]
fn spec_entry_counts() {
    assert_eq!(EXPORTS.len(), 63);
    let stubs = EXPORTS
        .iter()
        .filter(|e| e.kind == ExportKind::Stub)
        .count();
    let forwards = EXPORTS
        .iter()
        .filter(|e| matches!(e.kind, ExportKind::Forward(_)))
        .count();
    assert_eq!(stubs, 28);
    assert_eq!(forwards, 35);
}

/// Not oracle-derived: boundary entries, the first and last lines of `cng.sys.spec`.
#[test]
fn first_and_last_spec_lines() {
    assert_eq!(
        EXPORTS.first().map(|e| e.name),
        Some("BCryptAddContextFunctionProvider")
    );
    assert_eq!(EXPORTS.last().map(|e| e.name), Some("SystemPrng"));
}

/// Not oracle-derived: every forward target is `bcrypt.` plus the export name, as written in
/// `cng.sys.spec`.
#[test]
fn forward_targets_name_bcrypt() {
    for export in EXPORTS {
        if let ExportKind::Forward(target) = export.kind {
            assert_eq!(target, format!("bcrypt.{}", export.name));
        }
    }
}

/// Not oracle-derived: names are unique, so `find_export` is unambiguous.
#[test]
fn export_names_are_unique() {
    let mut names: Vec<&str> = EXPORTS.iter().map(|e| e.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), EXPORTS.len());
}

/// Not oracle-derived: each entry is found by its own name, and the lookup returns that entry.
#[test]
fn find_export_matches_each_entry() {
    for export in EXPORTS {
        assert_eq!(find_export(export.name), Some(export));
    }
}

/// Not oracle-derived: case, prefix, suffix, empty and embedded-NUL names are not exports, and
/// neither is `DriverEntry`, which the spec does not list.
#[test]
fn find_export_rejects_near_misses() {
    for name in [
        "",
        "bcryptcreatehash",
        "BCryptCreateHas",
        "BCryptCreateHash ",
        "BCryptCreateHash\0",
        "systemprng",
        "DriverEntry",
    ] {
        assert_eq!(find_export(name), None, "{name:?}");
    }
}

/// Not oracle-derived: the same stub record as R03-R16, applied to all 28 stub exports. The
/// record fields are constant in the C body, so the expectation comes from the source.
#[test]
fn unimplemented_stub_covers_every_entry_kind() {
    for export in EXPORTS {
        match export.kind {
            ExportKind::Stub => assert_eq!(
                unimplemented_stub(export.name),
                Some(StubException {
                    code: EXCEPTION_WINE_STUB,
                    flags: EXCEPTION_NONCONTINUABLE,
                    number_parameters: 2,
                    module: "cng.sys",
                    function: export.name,
                })
            ),
            ExportKind::Forward(_) => assert_eq!(unimplemented_stub(export.name), None),
        }
    }
}

/// Not oracle-derived: names that are not exports give no stub record.
#[test]
fn unimplemented_stub_returns_none_for_unknown_names() {
    assert_eq!(unimplemented_stub("DriverEntry"), None);
    assert_eq!(unimplemented_stub(""), None);
}

/// Not oracle-derived: `STATUS_SUCCESS` is `0` in `ntstatus.h`, and `DriverEntry` returns it
/// unconditionally.
#[test]
fn driver_entry_returns_status_success() {
    assert_eq!(STATUS_SUCCESS, 0);
    assert_eq!(driver_entry(), STATUS_SUCCESS);
}

/// Not oracle-derived: the header constants, and the module name winebuild infers from the spec
/// file name.
#[test]
fn stub_constants_match_headers() {
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 1);
    assert_eq!(WINE_SPEC_FILE_NAME, "cng.sys");
}
