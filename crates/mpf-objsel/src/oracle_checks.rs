//! Oracle-derived test vectors for the ported objsel logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`, the class
//! factory methods, `object_picker_create` and the `IDsObjectPicker` methods (`dlls/objsel/objsel.c`),
//! and of `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer` (`libs/winecrt0/`), with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/objsel/`, and running the harness. All 25
//! printed rows ran clean under ASan and UBSan, so none is excluded.
//!
//! Each `// row NN` comment quotes the harness line that produced the expectation below it. Where the
//! harness started an `out` pointer at a non-NULL sentinel, the test starts with `Some(...)`.

use super::*;

const OTHER_CLSID: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0, 0, 0, 0, 0, 0, 0, 1],
};

/// An allocator that succeeds or returns NULL, as `HeapAlloc` does.
fn heap(ok: bool) -> impl FnOnce() -> bool {
    move || ok
}

/// A picker with the given count and `destroyed` clear.
fn picker(refs: i32) -> ObjectPicker {
    ObjectPicker {
        refs,
        destroyed: false,
    }
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 DllGetClassObject(DsObjectPicker,IUnknown) -> hr=0x00000000 out=factory refs=1
    let mut factory = ClassFactory::default();
    let mut out = Some(Interface::DsObjectPicker);
    assert_eq!(
        dll_get_class_object(
            &mut factory,
            &CLSID_DS_OBJECT_PICKER,
            &IID_UNKNOWN,
            &mut out
        ),
        S_OK
    );
    assert_eq!(out, Some(Interface::ClassFactory));
    assert_eq!(factory.refs, 1);

    // row 02 DllGetClassObject(DsObjectPicker,IClassFactory) -> hr=0x00000000 out=factory refs=1
    let mut factory = ClassFactory::default();
    let mut out = Some(Interface::DsObjectPicker);
    assert_eq!(
        dll_get_class_object(
            &mut factory,
            &CLSID_DS_OBJECT_PICKER,
            &IID_CLASS_FACTORY,
            &mut out
        ),
        S_OK
    );
    assert_eq!(out, Some(Interface::ClassFactory));
    assert_eq!(factory.refs, 1);

    // row 03 DllGetClassObject(DsObjectPicker,IDsObjectPicker) -> hr=0x80004002 out=NULL refs=0
    let mut factory = ClassFactory::default();
    let mut out = Some(Interface::DsObjectPicker);
    assert_eq!(
        dll_get_class_object(
            &mut factory,
            &CLSID_DS_OBJECT_PICKER,
            &IID_DS_OBJECT_PICKER,
            &mut out
        ),
        E_NOINTERFACE
    );
    assert_eq!(out, None);
    assert_eq!(factory.refs, 0);

    // row 04 DllGetClassObject(other clsid,IUnknown) -> hr=0x80040111 out=NULL refs=0
    let mut factory = ClassFactory::default();
    let mut out = Some(Interface::DsObjectPicker);
    assert_eq!(
        dll_get_class_object(&mut factory, &OTHER_CLSID, &IID_UNKNOWN, &mut out),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(out, None);
    assert_eq!(factory.refs, 0);
}

#[test]
fn oracle_class_factory_rows() {
    // row 05 factory.QueryInterface(IUnknown,NULL) -> hr=0x80004003 refs=0
    let mut factory = ClassFactory::default();
    assert_eq!(factory.query_interface(&IID_UNKNOWN, None), E_POINTER);
    assert_eq!(factory.refs, 0);

    // row 06 factory.QueryInterface(IDsObjectPicker,&out) -> hr=0x80004002 out=sentinel refs=0
    let mut factory = ClassFactory::default();
    let mut out = Some(Interface::DsObjectPicker);
    assert_eq!(
        factory.query_interface(&IID_DS_OBJECT_PICKER, Some(&mut out)),
        E_NOINTERFACE
    );
    assert_eq!(out, Some(Interface::DsObjectPicker));
    assert_eq!(factory.refs, 0);

    // row 07 factory.Release() on refs 0 -> ret=4294967295 refs=-1
    let mut factory = ClassFactory::default();
    assert_eq!(factory.release(), u32::MAX);
    assert_eq!(factory.refs, -1);

    // row 08 factory.LockServer(TRUE) -> hr=0x00000000 refs=1
    let mut factory = ClassFactory::default();
    assert_eq!(factory.lock_server(true), S_OK);
    assert_eq!(factory.refs, 1);
}

#[test]
fn oracle_create_instance_rows() {
    let factory = ClassFactory::default();

    // row 09 factory.CreateInstance(no outer,IDsObjectPicker) -> hr=0x00000000 out=picker refs=1
    let mut out = None;
    assert_eq!(
        factory.create_instance(false, &IID_DS_OBJECT_PICKER, Some(&mut out), heap(true)),
        S_OK
    );
    assert_eq!(out, Some(picker(1)));

    // row 10 factory.CreateInstance(outer,IDsObjectPicker) -> hr=0x80040110 out=NULL
    let mut out = None;
    assert_eq!(
        factory.create_instance(true, &IID_DS_OBJECT_PICKER, Some(&mut out), heap(true)),
        CLASS_E_NOAGGREGATION
    );
    assert_eq!(out, None);

    // row 11 factory.CreateInstance(no outer,IUnknown) -> hr=0x80040111 out=NULL
    let mut out = None;
    assert_eq!(
        factory.create_instance(false, &IID_UNKNOWN, Some(&mut out), heap(true)),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(out, None);

    // row 12 factory.CreateInstance(outer,IDsObjectPicker,NULL) -> hr=0x80004003
    assert_eq!(
        factory.create_instance(true, &IID_DS_OBJECT_PICKER, None, heap(true)),
        E_POINTER
    );

    // row 13 factory.CreateInstance(no outer,IDsObjectPicker,heap fails) -> hr=0x8007000e out=NULL
    let mut out = None;
    assert_eq!(
        factory.create_instance(false, &IID_DS_OBJECT_PICKER, Some(&mut out), heap(false)),
        E_OUTOFMEMORY
    );
    assert_eq!(out, None);
}

#[test]
fn oracle_picker_query_interface_rows() {
    // row 14 picker(refs 1).QueryInterface(IUnknown) -> hr=0x00000000 out=picker refs=2
    let mut p = picker(1);
    let mut out = None;
    assert_eq!(p.query_interface(&IID_UNKNOWN, Some(&mut out)), S_OK);
    assert_eq!(out, Some(Interface::DsObjectPicker));
    assert_eq!(p.refs, 2);

    // row 15 picker(refs 1).QueryInterface(IDsObjectPicker) -> hr=0x00000000 out=picker refs=2
    let mut p = picker(1);
    let mut out = None;
    assert_eq!(
        p.query_interface(&IID_DS_OBJECT_PICKER, Some(&mut out)),
        S_OK
    );
    assert_eq!(out, Some(Interface::DsObjectPicker));
    assert_eq!(p.refs, 2);

    // row 16 picker(refs 1).QueryInterface(IClassFactory) -> hr=0x80004002 out=sentinel refs=1
    let mut p = picker(1);
    let mut out = Some(Interface::ClassFactory);
    assert_eq!(
        p.query_interface(&IID_CLASS_FACTORY, Some(&mut out)),
        E_NOINTERFACE
    );
    assert_eq!(out, Some(Interface::ClassFactory));
    assert_eq!(p.refs, 1);

    // row 17 picker(refs 1).QueryInterface(IUnknown,NULL) -> hr=0x80004003 refs=1
    let mut p = picker(1);
    assert_eq!(p.query_interface(&IID_UNKNOWN, None), E_POINTER);
    assert_eq!(p.refs, 1);
}

#[test]
fn oracle_picker_refcount_rows() {
    // row 18 picker(refs 2).Release() -> ret=1 freed=0 refs=1
    let mut p = picker(2);
    assert_eq!(p.release(), 1);
    assert!(!p.destroyed);
    assert_eq!(p.refs, 1);

    // row 19 picker(refs 1).Release() -> ret=0 freed=1
    let mut p = picker(1);
    assert_eq!(p.release(), 0);
    assert!(p.destroyed);
    assert_eq!(p.refs, 0);

    // row 20 picker(refs INT32_MAX).AddRef() -> ret=2147483648 refs=-2147483648
    let mut p = picker(i32::MAX);
    assert_eq!(p.add_ref(), 2_147_483_648);
    assert_eq!(p.refs, i32::MIN);
}

#[test]
fn oracle_picker_stub_rows() {
    // row 21 picker.Initialize(NULL) -> hr=0x00000000
    assert_eq!(picker(1).initialize(), S_OK);

    // row 22 picker.InvokeDialog(NULL,NULL) -> hr=0x00000001
    assert_eq!(picker(1).invoke_dialog(), S_FALSE);
}

#[test]
fn oracle_dll_export_rows() {
    // row 23 DllCanUnloadNow() -> hr=0x00000001
    assert_eq!(dll_can_unload_now(), S_FALSE);

    // row 24 DllRegisterServer(__wine_register_resources=E_FAIL) -> hr=0x80004005
    assert_eq!(dll_register_server(|| E_FAIL), E_FAIL);

    // row 25 DllUnregisterServer(__wine_unregister_resources=S_OK) -> hr=0x00000000
    assert_eq!(dll_unregister_server(|| S_OK), S_OK);
}
