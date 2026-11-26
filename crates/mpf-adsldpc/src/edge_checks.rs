//! Edge-case tests for the ported adsldpc logic.
//!
//! These cover the stub record, the export table and the name lookup. Expectations that are not
//! oracle-derived say so in a comment.

use super::*;
use std::collections::HashSet;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_record_carries_the_names_it_is_given() {
    // Not oracle-derived: the oracle only covers the adsldpc.dll module, so this checks that the
    // two arguments are stored unchanged for any other module and function.
    let record = wine_spec_unimplemented_stub("other.dll", "SomeExport");
    assert_eq!(
        record,
        StubException {
            code: EXCEPTION_WINE_STUB,
            flags: EXCEPTION_NONCONTINUABLE,
            number_parameters: 2,
            module: "other.dll",
            function: "SomeExport",
        }
    );
}

#[test]
fn export_table_matches_the_spec_line_count() {
    // Not oracle-derived: counts taken from `grep -c '^@ stub' dlls/adsldpc/adsldpc.spec` (189
    // lines) and `grep -c -- '-arch=' dlls/adsldpc/adsldpc.spec` (21 lines, seven per architecture).
    assert_eq!(EXPORTS.len(), 189);
    assert_eq!(EXPORTS.iter().filter(|e| e.arch.is_some()).count(), 21);
    for arch in [Arch::Arm, Arch::I386, Arch::Win64] {
        assert_eq!(EXPORTS.iter().filter(|e| e.arch == Some(arch)).count(), 7);
    }
}

#[test]
fn export_names_are_unique() {
    // Not oracle-derived: each spec export name appears once in the table.
    let names: HashSet<&str> = EXPORTS.iter().map(|e| e.name).collect();
    assert_eq!(names.len(), EXPORTS.len());
}

#[test]
fn find_export_matches_exact_case_sensitive_names() {
    // Not oracle-derived: the found, found-with-arch and miss branches of the lookup.
    assert_eq!(
        find_export("ADSIPrint"),
        Some(StubExport {
            name: "ADSIPrint",
            arch: None,
        })
    );
    assert_eq!(
        find_export("?SetFSlashDisabler@CLexer@@QEAAXH@Z"),
        Some(StubExport {
            name: "?SetFSlashDisabler@CLexer@@QEAAXH@Z",
            arch: Some(Arch::Win64),
        })
    );
    assert_eq!(find_export("adsiprint"), None);
    assert_eq!(find_export("ADSIPrin"), None);
    assert_eq!(find_export("ADSIPrint "), None);
    assert_eq!(find_export(""), None);
}

#[test]
fn every_export_resolves_to_itself() {
    // Not oracle-derived: each table entry is found by its own name.
    for export in EXPORTS {
        assert_eq!(find_export(export.name), Some(export));
    }
}
