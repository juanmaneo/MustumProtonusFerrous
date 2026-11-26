//! Oracle-derived test vectors for the ported rtscom logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `rtscom_main.c` and the
//! winecrt0 `DllCanUnloadNow` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/rtscom/`, and running the harness. It printed 25 rows (R01 to R25). Neither
//! sanitizer flagged a row, so no vector is excluded. R24 is one aggregated row: it runs the 31
//! `FIXME` bodies and counts the ones that return `E_NOTIMPL`.

use super::*;

/// The unknown CLSID that the harness uses (`other_clsid`).
pub(crate) const OTHER_CLSID: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x1234,
    data3: 0x5678,
    data4: [1, 2, 3, 4, 5, 6, 7, 8],
};

/// The unknown IID that the harness uses (`other_iid`).
pub(crate) const OTHER_IID: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x1234,
    data3: 0x5678,
    data4: [9; 8],
};

/// The harness's `sentinel_obj`. It is an `obj` value that no successful call returns, so a
/// NULL or untouched result is visible.
fn sentinel() -> Option<Interface> {
    Some(Interface::RealTimeStylus)
}

/// The harness's `make_stylus`: `ClassFactory.CreateInstance(NULL, IRealTimeStylus)`, with one
/// reference.
pub(crate) fn make_stylus() -> RealTimeStylus {
    let (hr, stylus) = class_factory_create_instance(&IID_IREALTIMESTYLUS);
    assert_eq!(hr, S_OK);
    stylus.expect("CreateInstance returns an object")
}

#[test]
fn oracle_dll_rows() {
    // R01 DllGetClassObject(rts,IClassFactory) hr=0x00000000 obj=ClassFactory
    let mut obj = None;
    assert_eq!(
        dll_get_class_object(&CLSID_REALTIMESTYLUS, &IID_ICLASSFACTORY, &mut obj),
        S_OK
    );
    assert!(matches!(obj, Some(Interface::ClassFactory)));

    // R02 DllGetClassObject(rts,IUnknown) hr=0x00000000 obj=ClassFactory
    let mut obj = None;
    assert_eq!(
        dll_get_class_object(&CLSID_REALTIMESTYLUS, &IID_IUNKNOWN, &mut obj),
        S_OK
    );
    assert!(matches!(obj, Some(Interface::ClassFactory)));

    // R03 DllGetClassObject(rts,IRealTimeStylus) hr=0x80004002 obj=NULL
    let mut obj = sentinel();
    assert_eq!(
        dll_get_class_object(&CLSID_REALTIMESTYLUS, &IID_IREALTIMESTYLUS, &mut obj),
        E_NOINTERFACE
    );
    assert!(obj.is_none());

    // R04 DllGetClassObject(other,IClassFactory) hr=0x80040111 obj=untouched
    let mut obj = sentinel();
    assert_eq!(
        dll_get_class_object(&OTHER_CLSID, &IID_ICLASSFACTORY, &mut obj),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert!(matches!(obj, Some(Interface::RealTimeStylus)));

    // R05 DllGetClassObject(rts,other) hr=0x80004002 obj=NULL
    let mut obj = sentinel();
    assert_eq!(
        dll_get_class_object(&CLSID_REALTIMESTYLUS, &OTHER_IID, &mut obj),
        E_NOINTERFACE
    );
    assert!(obj.is_none());

    // R06 DllCanUnloadNow() hr=0x00000001
    assert_eq!(dll_can_unload_now(), S_FALSE);

    // R25 DllGetClassObject(other,IUnknown) hr=0x80040111 obj=untouched
    let mut obj = sentinel();
    assert_eq!(
        dll_get_class_object(&OTHER_CLSID, &IID_IUNKNOWN, &mut obj),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert!(matches!(obj, Some(Interface::RealTimeStylus)));
}

#[test]
fn oracle_class_factory_rows() {
    // R07 ClassFactory.QueryInterface(IUnknown) hr=0x00000000 obj=ClassFactory
    let mut obj = None;
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert!(matches!(obj, Some(Interface::ClassFactory)));

    // R08 ClassFactory.QueryInterface(IClassFactory) hr=0x00000000 obj=ClassFactory
    let mut obj = None;
    assert_eq!(
        class_factory_query_interface(&IID_ICLASSFACTORY, &mut obj),
        S_OK
    );
    assert!(matches!(obj, Some(Interface::ClassFactory)));

    // R09 ClassFactory.QueryInterface(IRealTimeStylus) hr=0x80004002 obj=NULL
    let mut obj = sentinel();
    assert_eq!(
        class_factory_query_interface(&IID_IREALTIMESTYLUS, &mut obj),
        E_NOINTERFACE
    );
    assert!(obj.is_none());

    // R10 ClassFactory.AddRef() ref=2
    assert_eq!(class_factory_add_ref(), 2);

    // R11 ClassFactory.Release() ref=1
    assert_eq!(class_factory_release(), 1);

    // R12 ClassFactory.LockServer(TRUE) hr=0x00000000
    assert_eq!(class_factory_lock_server(true), S_OK);

    // R13 ClassFactory.LockServer(FALSE) hr=0x00000000
    assert_eq!(class_factory_lock_server(false), S_OK);
}

#[test]
fn oracle_create_instance_rows() {
    // R14 ClassFactory.CreateInstance(NULL,IRealTimeStylus) hr=0x00000000 obj=RealTimeStylus refs=1
    let (hr, stylus) = class_factory_create_instance(&IID_IREALTIMESTYLUS);
    assert_eq!(hr, S_OK);
    let mut stylus = stylus.expect("obj=RealTimeStylus");
    assert_eq!(stylus.release(), 0);

    // R15 ClassFactory.CreateInstance(outer,IUnknown) hr=0x00000000 obj=RealTimeStylus refs=1
    // The harness passes a non-NULL outer. The port has no outer argument.
    let (hr, stylus) = class_factory_create_instance(&IID_IUNKNOWN);
    assert_eq!(hr, S_OK);
    let mut stylus = stylus.expect("obj=RealTimeStylus");
    assert_eq!(stylus.release(), 0);

    // R16 ClassFactory.CreateInstance(NULL,IClassFactory) hr=0x80004002 obj=NULL
    let (hr, stylus) = class_factory_create_instance(&IID_ICLASSFACTORY);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(stylus.is_none());
}

#[test]
fn oracle_realtime_stylus_rows() {
    // R17 RealTimeStylus.QueryInterface(IRealTimeStylus) on fresh hr=0x00000000 obj=RealTimeStylus refs=2
    let mut stylus = make_stylus();
    let mut obj = None;
    assert_eq!(stylus.query_interface(&IID_IREALTIMESTYLUS, &mut obj), S_OK);
    assert!(matches!(obj, Some(Interface::RealTimeStylus)));
    assert_eq!(stylus.release(), 1);
    assert_eq!(stylus.release(), 0);

    // R18 RealTimeStylus.QueryInterface(IUnknown) on fresh hr=0x00000000 obj=RealTimeStylus refs=2
    let mut stylus = make_stylus();
    let mut obj = None;
    assert_eq!(stylus.query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert!(matches!(obj, Some(Interface::RealTimeStylus)));
    assert_eq!(stylus.release(), 1);
    assert_eq!(stylus.release(), 0);

    // R19 RealTimeStylus.QueryInterface(IClassFactory) on fresh hr=0x80004002 obj=NULL refs=1
    let mut stylus = make_stylus();
    let mut obj = sentinel();
    assert_eq!(
        stylus.query_interface(&IID_ICLASSFACTORY, &mut obj),
        E_NOINTERFACE
    );
    assert!(obj.is_none());
    assert_eq!(stylus.release(), 0);

    // R20 RealTimeStylus.AddRef() on fresh ref=2
    let mut stylus = make_stylus();
    assert_eq!(stylus.add_ref(), 2);
    assert_eq!(stylus.release(), 1);
    assert_eq!(stylus.release(), 0);

    // R21 RealTimeStylus.Release() on fresh ref=0 freed=yes
    let mut stylus = make_stylus();
    assert_eq!(stylus.release(), 0);

    // R22 RealTimeStylus.AddRef then Release ref=1
    let mut stylus = make_stylus();
    stylus.add_ref();
    assert_eq!(stylus.release(), 1);
    assert_eq!(stylus.release(), 0);

    // R23 realtime_stylus_create(NULL,other) hr=0x80004002 obj=NULL
    let (hr, stylus) = realtime_stylus_create(&OTHER_IID);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(stylus.is_none());
}

#[test]
fn oracle_e_notimpl_bodies_row() {
    // R24 FIXME bodies returning E_NOTIMPL: 31 of 31 (0x80004001)
    let count = STUB_METHODS
        .into_iter()
        .map(stub_method)
        .filter(|&hr| hr == E_NOTIMPL)
        .count();
    assert_eq!(count, 31);
}
