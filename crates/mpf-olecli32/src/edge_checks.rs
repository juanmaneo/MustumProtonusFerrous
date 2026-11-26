//! Edge-case tests for the ported olecli32 logic.
//!
//! These cover the counter wrap, the object dispatch, the `OleIsDcMeta` boundary, and the stub
//! record. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Oracle-derived: OLE_OK (the return of rows 01-12), OLE_NULL_OBJECT_STATUS (the harness
    // printed OLE_ERROR_ADDRESS=16, and row 16 returned it), OBJ_METADC (rows 13-15).
    // Not oracle-derived: the EXCEPTION_* values, which the harness shimmed from
    // include/winternl.h and include/winnt.h.
    assert_eq!(OLE_OK, 0);
    assert_eq!(OLE_NULL_OBJECT_STATUS, 16);
    assert_eq!(OBJ_METADC, 4);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn register_wraps_past_long_max() {
    // Not oracle-derived: row 05 (counter at LONG_MAX) is excluded because UBSan flags the
    // signed overflow. This pins the wrapped value that the port returns.
    let mut registry = ClientDocRegistry {
        last_handle: i32::MAX,
    };
    let mut hret = 0;
    assert_eq!(ole_register_client_doc(&mut registry, &mut hret), OLE_OK);
    assert_eq!(hret, i32::MIN);
    assert_eq!(registry.last_handle, i32::MIN);
}

#[test]
fn register_hands_out_consecutive_handles() {
    // Not oracle-derived: each oracle row resets the counter before the call. This checks that
    // the counter carries over between calls, as the C static does.
    let mut registry = ClientDocRegistry::default();
    let mut hret = 0;
    assert_eq!(ole_register_client_doc(&mut registry, &mut hret), OLE_OK);
    assert_eq!(hret, 1);
    assert_eq!(ole_register_client_doc(&mut registry, &mut hret), OLE_OK);
    assert_eq!(hret, 2);
    assert_eq!(registry.last_handle, 2);
}

#[test]
fn query_type_passes_object_status_through() {
    // Not oracle-derived: the oracle objects returned only OLE_OK and 16. This checks that an
    // arbitrary status and an arbitrary written value come back unchanged.
    struct Odd;
    impl OleObject for Odd {
        fn query_type(&self, xlong: &mut i32) -> OleStatus {
            *xlong = i32::MIN;
            0xdead_beef
        }
    }

    let mut xlong = 0;
    assert_eq!(ole_query_type(Some(&Odd), &mut xlong), 0xdead_beef);
    assert_eq!(xlong, i32::MIN);
}

#[test]
fn is_dc_meta_only_matches_metadc() {
    // Not oracle-derived beyond rows 13-15: the neighbouring values of OBJ_METADC and the
    // largest DWORD.
    assert!(!ole_is_dc_meta(OBJ_METADC - 1));
    assert!(!ole_is_dc_meta(OBJ_METADC + 1));
    assert!(!ole_is_dc_meta(u32::MAX));
    assert!(ole_is_dc_meta(OBJ_METADC));
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the oracle only passed the 41 fixed export names. The code and flags
    // do not depend on the names, and the names are copied unchanged.
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
