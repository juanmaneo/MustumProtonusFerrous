//! Edge-case tests for the ported dxtrans logic.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn class_not_available_constant_matches_winerror() {
    // Oracle-derived: CLASS_E_CLASSNOTAVAILABLE is 0x80040111 in the harness.
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    // Not oracle-derived: an HRESULT failure has the severity bit set.
    assert_ne!(CLASS_E_CLASSNOTAVAILABLE & 0x8000_0000, 0);
}

#[test]
fn dll_get_class_object_ignores_every_argument_combination() {
    // Not oracle-derived: the Wine body never inspects its inputs, so every
    // combination of NULL/non-NULL pointers returns the same HRESULT.
    let guid = [0xffu8; 16];
    for clsid in [None, Some(&guid)] {
        for riid in [None, Some(&guid)] {
            for out in [false, true] {
                assert_eq!(
                    dll_get_class_object(clsid, riid, out),
                    CLASS_E_CLASSNOTAVAILABLE
                );
            }
        }
    }
}

#[test]
fn stub_exception_flags_are_noncontinuable() {
    // Not oracle-derived: the flags are the noncontinuable bit the stub passes.
    assert_eq!(EXCEPTION_NONCONTINUABLE, 1);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
}

#[test]
fn stub_exception_is_independent_of_function_name() {
    // Not oracle-derived: an unknown name still yields the same exception,
    // because the Wine stub only forwards its arguments.
    let known = unimplemented_stub("dxtrans.dll", STUB_EXPORTS[0]);
    let unknown = unimplemented_stub("dxtrans.dll", "NoSuchExport");
    assert_eq!(known.code, unknown.code);
    assert_eq!(known.flags, unknown.flags);
    assert_eq!(unknown.function, "NoSuchExport");
}

#[test]
fn stub_exports_table_is_unique_and_nonempty() {
    // Not oracle-derived: a sanity check on the spec-derived name table.
    assert_eq!(STUB_EXPORTS.len(), 7);
    for (i, a) in STUB_EXPORTS.iter().enumerate() {
        for b in &STUB_EXPORTS[i + 1..] {
            assert_ne!(a, b);
        }
    }
}
