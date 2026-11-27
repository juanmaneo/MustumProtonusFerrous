//! Edge-case tests for the ported profapi logic.
//!
//! These cover the export table boundaries, the stub helper names and the stub constants. The
//! expectations that are not oracle-derived say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the ordinals of profapi.spec. The code and
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

#[test]
fn every_helper_passes_its_ordinal_and_the_dll_name() {
    // Not oracle-derived: each helper is checked against the matching entry of EXPORTS.
    let stubs = [
        ordinal_101(),
        ordinal_102(),
        ordinal_103(),
        ordinal_104(),
        ordinal_105(),
        ordinal_106(),
        ordinal_107(),
        ordinal_108(),
        ordinal_109(),
        ordinal_110(),
        ordinal_111(),
        ordinal_112(),
        ordinal_113(),
        ordinal_114(),
        ordinal_115(),
        ordinal_116(),
        ordinal_117(),
    ];
    assert_eq!(stubs.len(), EXPORTS.len());
    for (stub, name) in stubs.iter().zip(EXPORTS) {
        assert_eq!(stub.module, "profapi.dll");
        assert_eq!(stub.function, name);
        assert_eq!(*stub, wine_spec_unimplemented_stub("profapi.dll", name));
    }
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 17 lines of dlls/profapi/profapi.spec, in order. Each line is
    // `stub -noname @` at ordinals 101 to 117.
    assert_eq!(EXPORTS.len(), 17);
    for (index, name) in EXPORTS.iter().enumerate() {
        assert_eq!(*name, (101 + index).to_string());
    }
}

#[test]
fn is_export_accepts_only_the_listed_ordinals() {
    // Not oracle-derived: the boundaries of the 101 to 117 range and some near misses.
    assert!(is_export("101"));
    assert!(is_export("117"));
    assert!(!is_export("100"));
    assert!(!is_export("118"));
    assert!(!is_export("0101"));
    assert!(!is_export(""));
    assert!(!is_export("ordinal_101"));
}
