//! Oracle-derived test vectors for the ported msident logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/msident/msident.c` (lines 30 to 286)
//! and the default `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/msident/`, then running the harness. It printed
//! 25 rows. ASan and UBSan flagged none of them, so none is excluded.
//!
//! The C rows also print the out-pointer state (`ppv=set`, `NULL` or `untouched`). `set` maps to `Ok(())`.
//! The `NULL` and `untouched` states are not modelled: the Rust methods return only the `Err` code.

use super::*;

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 DllGetClassObject(CLSID_UserIdentityManager,IID_IClassFactory) hr=00000000 ppv=set
    assert_eq!(
        dll_get_class_object(&CLSID_USER_IDENTITY_MANAGER, &IID_ICLASSFACTORY),
        Ok(())
    );
    // row 02 DllGetClassObject(IID_IUnknown,IID_IClassFactory) hr=80040111 ppv=untouched
    assert_eq!(
        dll_get_class_object(&IID_IUNKNOWN, &IID_ICLASSFACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_class_factory_rows() {
    let cf = ClassFactory;
    // row 03 ClassFactory.QueryInterface(IID_IUnknown) hr=00000000 ppv=set
    assert_eq!(cf.query_interface(&IID_IUNKNOWN), Ok(()));
    // row 04 ClassFactory.QueryInterface(IID_IClassFactory) hr=00000000 ppv=set
    assert_eq!(cf.query_interface(&IID_ICLASSFACTORY), Ok(()));
    // row 05 ClassFactory.QueryInterface(IID_IEnumUserIdentity) hr=80004002 ppv=NULL
    assert_eq!(
        cf.query_interface(&IID_IENUM_USER_IDENTITY),
        Err(E_NOINTERFACE)
    );
    // row 06 ClassFactory.AddRef=2 Release=1
    assert_eq!((cf.add_ref(), cf.release()), (2, 1));
    // row 07 ClassFactory.LockServer(TRUE) hr=00000000
    assert_eq!(cf.lock_server(), S_OK);
    // row 08 ClassFactory.CreateInstance(NULL,IID_IUserIdentityManager) hr=00000000 ppv=set
    assert_eq!(cf.create_instance(&IID_IUSER_IDENTITY_MANAGER), Ok(()));
    // row 09 ClassFactory.CreateInstance(NULL,IID_IEnumUserIdentity) hr=80004002 ppv=NULL
    assert_eq!(
        cf.create_instance(&IID_IENUM_USER_IDENTITY),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn oracle_user_identity_manager_rows() {
    let uim = UserIdentityManager;
    // row 10 UserIdentityManager.QueryInterface(IID_IUnknown) hr=00000000 ppv=set
    assert_eq!(uim.query_interface(&IID_IUNKNOWN), Ok(()));
    // row 11 UserIdentityManager.QueryInterface(IID_IUserIdentityManager) hr=00000000 ppv=set
    assert_eq!(uim.query_interface(&IID_IUSER_IDENTITY_MANAGER), Ok(()));
    // row 12 UserIdentityManager.QueryInterface(IID_IClassFactory) hr=80004002 ppv=NULL
    assert_eq!(uim.query_interface(&IID_ICLASSFACTORY), Err(E_NOINTERFACE));
    // row 13 UserIdentityManager.AddRef=2 Release=1
    assert_eq!((uim.add_ref(), uim.release()), (2, 1));
    // row 14 UserIdentityManager.EnumIdentities(alloc ok) hr=00000000 ppv=set ref=1
    let enumerator = uim.enum_identities(true).expect("alloc succeeds");
    assert_eq!(enumerator.ref_count(), 1);
    // row 15 UserIdentityManager.EnumIdentities(alloc fails) hr=8007000e ppv=untouched
    assert_eq!(uim.enum_identities(false).err(), Some(E_OUTOFMEMORY));
    // row 16 UserIdentityManager.ManageIdentities(NULL,0) hr=80004001
    assert_eq!(uim.manage_identities(), E_NOTIMPL);
    // row 17 UserIdentityManager.Logon(NULL,0) hr=80007112 ppIdentity=untouched
    assert_eq!(uim.logon(), E_USER_CANCELLED);
    // row 18 UserIdentityManager.Logoff(NULL) hr=80004001
    assert_eq!(uim.logoff(), E_NOTIMPL);
    // row 19 UserIdentityManager.GetIdentityByCookie(NULL,&ident) hr=80004001 ppIdentity=untouched
    assert_eq!(uim.get_identity_by_cookie(), E_NOTIMPL);
}

#[test]
fn oracle_enum_user_identity_rows() {
    // The enumerator from row 14, with ref = 1.
    let mut enumerator = UserIdentityManager
        .enum_identities(true)
        .expect("alloc succeeds");
    // row 20 Enum.QueryInterface(IID_IEnumUserIdentity) hr=00000000 ppv=set ref=2
    assert_eq!(enumerator.query_interface(&IID_IENUM_USER_IDENTITY), Ok(()));
    assert_eq!(enumerator.ref_count(), 2);
    // row 21 Enum.QueryInterface(IID_IClassFactory) hr=80004002 ppv=NULL ref=2
    assert_eq!(
        enumerator.query_interface(&IID_ICLASSFACTORY),
        Err(E_NOINTERFACE)
    );
    assert_eq!(enumerator.ref_count(), 2);
    // row 22 Enum.Release from 2 -> 1, then Release from 1 -> 0 (freed)
    assert_eq!(enumerator.release(), 1);
    assert_eq!(enumerator.release(), 0);

    // row 23 Enum.Next=80004001 Skip=80004001 Reset=80004001 Clone=80004001
    let fresh = UserIdentityManager
        .enum_identities(true)
        .expect("alloc succeeds");
    assert_eq!(fresh.next(), E_NOTIMPL);
    assert_eq!(fresh.skip(), E_NOTIMPL);
    assert_eq!(fresh.reset(), E_NOTIMPL);
    assert_eq!(fresh.clone_enum(), E_NOTIMPL);

    // row 24 Enum.GetCount(&count, count was 0xffffffff) hr=00000000 count=0
    let mut count = 0xffff_ffff_u32;
    assert_eq!(fresh.get_count(&mut count), S_OK);
    assert_eq!(count, 0);
}

#[test]
fn oracle_dll_can_unload_now_row() {
    // row 25 DllCanUnloadNow() hr=00000001
    assert_eq!(dll_can_unload_now(), S_FALSE);
}
