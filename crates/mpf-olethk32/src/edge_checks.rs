//! Edge-case tests for the ported olethk32 logic.
//!
//! These cover the constants, the export table, the name lookup and the stub helpers. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the 14 `@ stub` lines of dlls/olethk32/olethk32.spec, in order.
    let rows: Vec<(u16, &str)> = EXPORTS.iter().map(|e| (e.ordinal, e.name)).collect();
    assert_eq!(
        rows,
        [
            (3, "InvokeOn32"),
            (4, "IntOpInitialize"),
            (5, "CallbackProcessing_3216"),
            (6, "IUnknownObj32"),
            (8, "CSm16ReleaseHandler_Release32"),
            (9, "ThkMgrInitialize"),
            (10, "ThkMgrUninitialize"),
            (11, "TransformHRESULT_1632"),
            (12, "TransformHRESULT_3216"),
            (13, "ConvertObjDescriptor"),
            (14, "ConvertHr1632Thunk"),
            (15, "ConvertHr3216Thunk"),
            (16, "IntOpUninitialize"),
            (18, "ThkAddAppCompatFlag"),
        ]
    );
}

#[test]
fn ordinals_increase_and_skip_the_unexported_gaps() {
    // Not oracle-derived: the spec lists the ordinals in increasing order. Ordinals 1, 2, 7 and 17
    // have no line in olethk32.spec, so they are not exports.
    assert!(EXPORTS
        .windows(2)
        .all(|pair| pair[0].ordinal < pair[1].ordinal));
    for gap in [1, 2, 7, 17] {
        assert!(EXPORTS.iter().all(|export| export.ordinal != gap));
    }
}

#[test]
fn every_export_helper_raises_its_own_name() {
    // Not oracle-derived: each table entry calls its helper, and the helper must raise the
    // entry's name, with the olethk32.dll module and the fixed stub code, flags and count.
    for export in EXPORTS {
        let stub = (export.stub)();
        assert_eq!(stub.code, EXCEPTION_WINE_STUB);
        assert_eq!(stub.flags, EXCEPTION_NONCONTINUABLE);
        assert_eq!(stub.number_parameters, 2);
        assert_eq!(stub.module, "olethk32.dll");
        assert_eq!(stub.function, export.name);
        assert_eq!(export_stub(export.name), Some(stub));
    }
}

#[test]
fn export_stub_rejects_names_the_spec_does_not_export() {
    // Not oracle-derived: names are case-sensitive, and these names are not in the spec.
    assert_eq!(export_stub("invokeon32"), None);
    assert_eq!(export_stub("InvokeOn33"), None);
    assert_eq!(export_stub("ThkMgrInit"), None);
    assert_eq!(export_stub(""), None);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the C body copies whatever module and function it is given. The
    // harness only passed the olethk32.spec names and the empty name.
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
