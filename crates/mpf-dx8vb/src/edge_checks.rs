//! Edge-case tests for the ported dx8vb logic.
//!
//! These cover the branch of every check, near-miss GUIDs, and refcount wraparound. Expectations
//! not taken from an oracle row say so in a comment.

use super::*;

/// Not oracle-derived: a GUID that differs from `IID_UNKNOWN` only in its last `data4` byte.
const NEAR_IID_UNKNOWN: Guid = Guid {
    data4: [0xC0, 0, 0, 0, 0, 0, 0, 0x47],
    ..IID_UNKNOWN
};

#[test]
fn query_interface_accepts_each_riid_and_rejects_near_miss() {
    let mut f = ClassFactory::created();
    assert_eq!(f.query_interface(&IID_UNKNOWN), Ok(()));
    assert_eq!(f.query_interface(&IID_CLASS_FACTORY), Ok(()));
    assert_eq!(f.ref_count(), 3);
    // Not oracle-derived: near-miss riid is rejected and leaves the count alone.
    assert_eq!(f.query_interface(&NEAR_IID_UNKNOWN), Err(E_NOINTERFACE));
    assert_eq!(f.ref_count(), 3);
}

#[test]
fn release_decrements_without_freeing_above_zero() {
    let mut f = ClassFactory::created();
    f.add_ref();
    f.add_ref();
    assert_eq!(f.release(), 2);
    assert_eq!(f.release(), 1);
    assert_eq!(f.release(), 0);
}

/// Not oracle-derived: `InterlockedDecrement` on a `ULONG` of 0 wraps to `u32::MAX`. The C
/// code never reaches this with a live factory. Bug-compatible wrap is kept anyway.
#[test]
fn release_wraps_like_unsigned_decrement() {
    let mut f = ClassFactory::created();
    f.release();
    assert_eq!(f.release(), u32::MAX);
}

/// Not oracle-derived: `InterlockedIncrement` wraps past `u32::MAX` back to 0.
#[test]
fn add_ref_wraps_like_unsigned_increment() {
    let mut f = ClassFactory::created();
    f.release();
    assert_eq!(f.release(), u32::MAX);
    assert_eq!(f.add_ref(), 0);
}

/// Not oracle-derived: a non-null CLSID with a valid riid is rejected at the class lookup,
/// which is the R04 path with a different CLSID.
#[test]
fn class_lookup_rejects_clsid_that_is_not_null() {
    assert_eq!(
        dll_get_class_object(&NEAR_IID_UNKNOWN, &IID_CLASS_FACTORY, true),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

/// Not oracle-derived: the riid check runs before the class lookup, so a bad riid wins over a bad
/// CLSID (R06). An alloc failure with a good riid and CLSID reports E_OUTOFMEMORY (R07).
#[test]
fn dll_get_class_object_check_order() {
    assert_eq!(
        dll_get_class_object(&CLSID_NON_NULL, &NEAR_IID_UNKNOWN, false),
        Err(E_NOINTERFACE)
    );
    assert_eq!(
        dll_get_class_object(&CLSID_NON_NULL, &IID_UNKNOWN, false),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(
        dll_get_class_object(&GUID_NULL, &IID_UNKNOWN, false).unwrap_err(),
        E_OUTOFMEMORY
    );
}

const CLSID_NON_NULL: Guid = Guid {
    data1: 1,
    ..GUID_NULL
};

/// Not oracle-derived: every `@ stub` name in `dx8vb.spec` raises the same exception. The spec
/// count (135 `@ stub` lines) was checked against the table length.
#[test]
fn every_stub_export_raises_the_same_exception() {
    assert_eq!(STUB_EXPORTS.len(), 135);
    for name in STUB_EXPORTS {
        let raised = unimplemented_stub(name).expect("stub export");
        assert_eq!(raised.code, EXCEPTION_WINE_STUB);
        assert_eq!(raised.flags, 1);
        assert_eq!(raised.number_parameters, 2);
        assert_eq!(raised.module, WINE_SPEC_FILE_NAME);
        assert_eq!(raised.function, *name);
    }
}

/// Not oracle-derived: names outside the `@ stub` list are not stubs. `DllGetClassObject` is a
/// real export, and `DllCanUnloadNow` has no body in `main.c`.
#[test]
fn non_stub_names_are_not_stubs() {
    assert_eq!(unimplemented_stub("DllGetClassObject"), None);
    assert_eq!(unimplemented_stub("DllCanUnloadNow"), None);
    assert_eq!(unimplemented_stub(""), None);
}
