//! Edge-case tests for the ported scsiport.sys logic.
//!
//! These cover the export table, the unknown-name path and the constants. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h, include/winnt.h and include/ntstatus.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(STATUS_SUCCESS, 0);
}

#[test]
fn export_table_has_48_unique_names() {
    // Not oracle-derived: the spec lists 48 `@ stub` lines, and each name must appear once.
    assert_eq!(EXPORTS.len(), 48);
    for (i, (name, _)) in EXPORTS.iter().enumerate() {
        assert!(EXPORTS[i + 1..].iter().all(|(other, _)| other != name));
    }
}

#[test]
fn unknown_names_are_not_exports() {
    // Not oracle-derived: none of these names is in the spec. Names are case-sensitive, and
    // DriverEntry is a C function, not a spec export.
    assert_eq!(export_stub(""), None);
    assert_eq!(export_stub("dllinitialize"), None);
    assert_eq!(export_stub("DriverEntry"), None);
    assert_eq!(export_stub("ScsiPortFlushDma "), None);
}
