//! Edge-case tests for the ported msdaps logic.
//!
//! These cover the constants and GUID values, the `db_type_size` case groups and default branch
//! that the oracle rows do not print, the near-miss CLSIDs and IIDs, and the field-by-field
//! comparison. Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/dbs.idl and include/winerror.h.
    assert_eq!(DBTYPE_I1, 16);
    assert_eq!(DBTYPE_UI1, 17);
    assert_eq!(DBTYPE_I2, 2);
    assert_eq!(DBTYPE_UI2, 18);
    assert_eq!(DBTYPE_I4, 3);
    assert_eq!(DBTYPE_UI4, 19);
    assert_eq!(DBTYPE_R4, 4);
    assert_eq!(DBTYPE_I8, 20);
    assert_eq!(DBTYPE_UI8, 21);
    assert_eq!(DBTYPE_R8, 5);
    assert_eq!(DBTYPE_CY, 6);
    assert_eq!(DBTYPE_FILETIME, 64);
    assert_eq!(DBTYPE_BSTR, 8);
    assert_eq!(DBTYPE_GUID, 72);
    assert_eq!(DBTYPE_WSTR, 130);
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOINTERFACE, 0x8000_4002_u32 as i32);
    assert_eq!(E_NOTIMPL, 0x8000_4001_u32 as i32);
}

#[test]
fn guids_match_the_idl() {
    // Not oracle-derived: the uuid() values in dlls/msdaps/row_server.idl and include/unknwn.idl.
    assert_eq!(
        CLSID_WINE_ROW_SERVER,
        Guid {
            data1: 0x3824_8178,
            data2: 0xcf6d,
            data3: 0x11de,
            data4: [0xab, 0xe5, 0x00, 0x0c, 0x29, 0x16, 0xd8, 0x65],
        }
    );
    assert_eq!(CLSID_WINE_ROW_PROXY.data1, 0x3824_8179);
    assert_eq!(CLSID_WINE_ROWSET_SERVER.data1, 0x3824_817a);
    assert_eq!(CLSID_WINE_ROWSET_PROXY.data1, 0x3824_817b);
    assert_eq!(IID_IUNKNOWN.data4[7], 0x46);
    assert_eq!(IID_ICLASSFACTORY.data1, 1);
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: the memcmp semantics of IsEqualGUID, checked one field at a time.
    let base = CLSID_WINE_ROW_SERVER;
    assert!(is_equal_guid(&base, &base));

    let mut changed = base;
    changed.data1 ^= 1;
    assert!(!is_equal_guid(&base, &changed));

    let mut changed = base;
    changed.data2 ^= 1;
    assert!(!is_equal_guid(&base, &changed));

    let mut changed = base;
    changed.data3 ^= 1;
    assert!(!is_equal_guid(&base, &changed));

    let mut changed = base;
    changed.data4[7] ^= 1;
    assert!(!is_equal_guid(&base, &changed));
}

#[test]
fn class_object_near_misses_fall_through() {
    // Not oracle-derived: a GUID that differs from a coclass in one byte is not one of the four
    // classes, so the C code falls through to msdaps_DllGetClassObject, which the port reports as
    // None. The IIDs are not coclasses, so they fall through too.
    let mut near = CLSID_WINE_ROW_PROXY;
    near.data4[0] ^= 1;
    assert_eq!(class_object_for_clsid(&near), None);
    assert_eq!(class_object_for_clsid(&IID_IUNKNOWN), None);
    assert_eq!(
        class_object_for_clsid(&Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8]
        }),
        None
    );
}

#[test]
fn query_interface_rejects_near_miss_iids() {
    // Not oracle-derived: near-miss IIDs of IID_IUnknown and IID_IClassFactory are rejected, and
    // the second operand of the `||` is evaluated when the first one is false.
    let mut near = IID_IUNKNOWN;
    near.data4[7] ^= 1;
    assert_eq!(class_factory_query_interface(&near), E_NOINTERFACE);

    let mut near = IID_ICLASSFACTORY;
    near.data3 ^= 1;
    assert_eq!(class_factory_query_interface(&near), E_NOINTERFACE);
}

#[test]
fn lock_server_ignores_dolock() {
    // Not oracle-derived: the oracle row uses dolock=1. The C body ignores the flag, so dolock=0
    // returns the same S_OK.
    assert_eq!(class_factory_lock_server(false), S_OK);
}

#[test]
fn wstr_passes_var_len_through() {
    // Not oracle-derived: boundary values of var_len for DBTYPE_WSTR, the only type that reads it.
    assert_eq!(db_type_size(DBTYPE_WSTR, 0), 0);
    assert_eq!(db_type_size(DBTYPE_WSTR, usize::MAX), usize::MAX);
}

#[test]
fn fixed_size_types_ignore_var_len() {
    // Not oracle-derived: fixed-size types ignore var_len. The oracle rows use var_len=99, so this
    // checks a different value.
    assert_eq!(db_type_size(DBTYPE_I4, 12345), 4);
    assert_eq!(db_type_size(DBTYPE_GUID, 0), 16);
}

#[test]
fn r4_shares_the_i4_case_group() {
    // Not oracle-derived: R4 is the third label of the `I4 / UI4 / R4` case in row_server.c. No
    // oracle row prints it.
    assert_eq!(db_type_size(DBTYPE_R4, 0), 4);
}

#[test]
fn pointer_and_union_sizes() {
    // Not oracle-derived: the expected values follow from the C types. BSTR is a pointer, so the
    // 64-bit oracle host gives 8 and a 32-bit target gives 4. CY is an 8-byte union and FILETIME is
    // two DWORDs.
    assert_eq!(
        db_type_size(DBTYPE_BSTR, 0),
        std::mem::size_of::<*const u16>()
    );
    assert_eq!(db_type_size(DBTYPE_CY, 0), 8);
    assert_eq!(db_type_size(DBTYPE_FILETIME, 0), 8);
}

#[test]
fn unlisted_types_have_size_zero() {
    // Not oracle-derived: the default branch for DBTYPE values the switch does not list, such as
    // DBTYPE_EMPTY (0), DBTYPE_NULL (1), DBTYPE_STR (129) and DBTYPE_NUMERIC (131). The C code
    // logs a FIXME and returns 0.
    for db_type in [
        0, 1, 7, 9, 10, 11, 12, 13, 14, 128, 129, 131, 132, 133, 134, 135, 138, 0xffff,
    ] {
        assert_eq!(db_type_size(db_type, 5), 0, "db_type {db_type}");
    }
}
