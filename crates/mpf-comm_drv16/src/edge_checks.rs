//! Edge-case tests for the modelled comm.drv16 stubs.
//!
//! These cover the export table, the name lookup, and the record passed through by the stub.
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h, the same values
    // the harness uses from the C headers.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the export lines of dlls/comm.drv16/comm.drv16.spec, in order. Ordinal
    // 16 is commented out in the spec (`#16 stub WEP`), so it is absent here.
    assert_eq!(EXPORTS.len(), 21);
    assert_eq!(EXPORTS[0], (1, "INICOM"));
    assert_eq!(EXPORTS[14], (15, "GETDCB"));
    assert_eq!(EXPORTS[15], (17, "SUSPENDOPENCOMMPORTS"));
    assert_eq!(EXPORTS[19], (100, "ENABLENOTIFICATION"));
    assert_eq!(EXPORTS[20], (101, "ordinal_only__win31"));
    assert!(EXPORTS.windows(2).all(|pair| pair[0].0 < pair[1].0));
    assert!(EXPORTS.iter().all(|&(_, name)| name != "WEP"));
}

#[test]
fn every_export_resolves_to_its_own_stub() {
    // Not oracle-derived: each name in the export table resolves to a record whose function is
    // that name and whose module is the comm.drv16 file name.
    for &(_, name) in EXPORTS.iter() {
        let stub = export_stub(name).expect("exported name has a stub record");
        assert_eq!(stub.function, name);
        assert_eq!(stub.module, "comm.drv16");
        assert_eq!(stub.code, EXCEPTION_WINE_STUB);
        assert_eq!(stub.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(stub.number_parameters, 2);
    }
}

#[test]
fn names_outside_the_spec_have_no_stub() {
    // Not oracle-derived: WEP is commented out in the spec. The other names are not exports:
    // the lookup is case-sensitive and does not trim, and an empty name matches nothing.
    for name in ["WEP", "inicom", "ORDINAL_ONLY__WIN31", "INICOM ", ""] {
        assert_eq!(export_stub(name), None, "{name:?}");
    }
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of comm.drv16.spec. The code
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
