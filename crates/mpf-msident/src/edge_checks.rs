//! Edge-case tests for the ported msident logic.
//!
//! These cover the constants, the GUID comparison, the `ULONG` view of the reference count, the `SUCCEEDED`
//! boundary, and the branches that the oracle rows do not reach. Each expectation that is not oracle-derived
//! says so in a comment.

use super::*;
use crate::refcount::RefCount;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h and include/msident.idl.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(E_NOTIMPL as u32, 0x8000_4001);
    assert_eq!(E_OUTOFMEMORY as u32, 0x8007_000e);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    assert_eq!(E_USER_CANCELLED as u32, 0x8000_7112);
}

#[test]
fn guid_constants_match_the_idl() {
    // Not oracle-derived: the bytes come from the uuid() attributes in include/unknwn.idl and include/msident.idl.
    assert_eq!(
        IID_IUNKNOWN,
        Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
    assert_eq!(
        IID_ICLASSFACTORY,
        Guid {
            data1: 1,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
    assert_eq!(IID_IENUM_USER_IDENTITY.data1, 0xa9ae_6c8f);
    assert_eq!(IID_IUSER_IDENTITY_MANAGER.data1, 0xa9ae_6c90);
    assert_eq!(
        CLSID_USER_IDENTITY_MANAGER,
        Guid {
            data1: 0xa9ae_6c91,
            data2: 0x1d1b,
            data3: 0x11d2,
            data4: [0xb2, 0x1a, 0x00, 0xc0, 0x4f, 0xa3, 0x57, 0xfa],
        }
    );
}

#[test]
fn is_equal_guid_compares_every_field() {
    // Not oracle-derived: IsEqualGUID is a 16-byte memcmp, so a difference in any field must be seen.
    let base = CLSID_USER_IDENTITY_MANAGER;
    assert!(is_equal_guid(&base, &base));

    let mut first = base;
    first.data1 ^= 1;
    assert!(!is_equal_guid(&base, &first));

    let mut middle = base;
    middle.data3 ^= 0x8000;
    assert!(!is_equal_guid(&base, &middle));

    let mut last = base;
    last.data4[7] ^= 1;
    assert!(!is_equal_guid(&base, &last));
}

#[test]
fn succeeded_uses_the_sign_bit() {
    // Not oracle-derived: SUCCEEDED is `hr >= 0`, and the boundaries are i32::MAX and i32::MIN.
    assert!(succeeded(S_OK));
    assert!(succeeded(S_FALSE));
    assert!(succeeded(i32::MAX));
    assert!(!succeeded(-1));
    assert!(!succeeded(i32::MIN));
    assert!(!succeeded(E_NOTIMPL));
}

#[test]
fn ref_count_below_zero_reads_back_as_ulong() {
    // Not oracle-derived: the LONG count wraps, and Release returns the bits as ULONG. The oracle never reaches
    // a count below zero, because the C code frees the object at zero.
    let mut count = RefCount::initial();
    assert_eq!(count.release(), 0);
    assert_eq!(count.release(), u32::MAX);
    assert_eq!(count.get(), -1);
    assert_eq!(count.add_ref(), 0);
}

#[test]
fn enumerator_query_interface_failure_keeps_the_count() {
    // Not oracle-derived: the C body adds a reference only on success. Oracle row 21 covers IID_IClassFactory.
    let mut enumerator = UserIdentityManager
        .enum_identities(true)
        .expect("alloc succeeds");
    assert_eq!(enumerator.query_interface(&IID_IUNKNOWN), Ok(()));
    assert_eq!(enumerator.ref_count(), 2);
    assert_eq!(
        enumerator.query_interface(&IID_IUSER_IDENTITY_MANAGER),
        Err(E_NOINTERFACE)
    );
    assert_eq!(enumerator.ref_count(), 2);
    assert_eq!(enumerator.add_ref(), 3);
    assert_eq!(enumerator.release(), 2);
}

#[test]
fn dll_get_class_object_checks_the_clsid_first() {
    // Not oracle-derived: the CLSID check runs first, so any CLSID other than CLSID_UserIdentityManager gets
    // the class-not-available code, even for a riid that the factory accepts.
    assert_eq!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_IUNKNOWN),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    // Not oracle-derived: with the right CLSID, IUserIdentityManager is not a class factory interface.
    assert_eq!(
        dll_get_class_object(&CLSID_USER_IDENTITY_MANAGER, &IID_IUSER_IDENTITY_MANAGER),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn class_factory_create_instance_accepts_iunknown() {
    // Not oracle-derived: row 08 covers IID_IUserIdentityManager and row 09 covers IID_IEnumUserIdentity.
    assert_eq!(ClassFactory.create_instance(&IID_IUNKNOWN), Ok(()));
    assert_eq!(
        ClassFactory.create_instance(&IID_ICLASSFACTORY),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn static_objects_keep_no_state() {
    // Not oracle-derived: the static objects return constant counts, so repeated calls do not change them.
    let cf = ClassFactory;
    assert_eq!(cf.lock_server(), S_OK);
    assert_eq!((cf.add_ref(), cf.add_ref(), cf.release()), (2, 2, 1));
    let uim = UserIdentityManager;
    assert_eq!(uim.query_interface(&IID_IUNKNOWN), Ok(()));
    assert_eq!((uim.add_ref(), uim.release()), (2, 1));
}
