//! Edge-case tests for the ported msdelta stubs.
//!
//! These cover the constants, the export table order, the lookup edges, and the stub record
//! builder. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h. The module name is
    // from MODULE = msdelta.dll in dlls/msdelta/Makefile.in.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
    assert_eq!(apply_delta_a().module, "msdelta.dll");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the fifteen `@ stub` lines of dlls/msdelta/msdelta.spec, in order.
    let names: Vec<&str> = EXPORTS.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        [
            "ApplyDeltaA",
            "ApplyDeltaB",
            "ApplyDeltaProvidedB",
            "ApplyDeltaW",
            "CreateDeltaA",
            "CreateDeltaB",
            "CreateDeltaW",
            "DeltaFree",
            "DeltaNormalizeProvidedB",
            "GetDeltaInfoA",
            "GetDeltaInfoB",
            "GetDeltaInfoW",
            "GetDeltaSignatureA",
            "GetDeltaSignatureB",
            "GetDeltaSignatureW",
        ]
    );
}

#[test]
fn every_export_resolves_to_its_own_name() {
    // Not oracle-derived: each table entry's helper must raise a record for the same export name,
    // and the lookup must return that same record.
    for (name, stub) in EXPORTS {
        let record = stub();
        assert_eq!(record.function, name);
        assert_eq!(record.code, EXCEPTION_WINE_STUB);
        assert_eq!(record.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(export_stub(name), Some(record));
    }
}

#[test]
fn export_stub_rejects_names_the_spec_does_not_export() {
    // Not oracle-derived: lookups are exact and case-sensitive, so none of these is an export.
    assert_eq!(export_stub("applydeltaa"), None);
    assert_eq!(export_stub("ApplyDelta"), None);
    assert_eq!(export_stub("ApplyDeltaA "), None);
    assert_eq!(export_stub(""), None);
    assert_eq!(export_stub("MergeFontPackage"), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of msdelta.spec. The code and
    // flags are the same for any names.
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
