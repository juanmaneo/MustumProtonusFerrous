//! Oracle-derived test vectors for the ported dsuiext logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from `dlls/dsuiext/dsuiext.c`,
//! `libs/winecrt0/dll_canunload.c` and `libs/winecrt0/stub.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/dsuiext/`, then running the harness.
//! It printed 25 rows (E01-E10, O01-O15) and `rows=25 excluded=0`. ASan, UBSan and LSan reported
//! nothing, so no row is excluded.
//!
//! Each test quotes its harness row in a comment. HRESULTs are compared as `u32` hex literals.
//! Where the port creates the object internally, the test checks only `hr` and the out-pointer,
//! because the object is not reachable from outside. The row's `refs` value is observed through
//! the `release()` return value, which is the count after the release.
//!
//! The C harness starts each out-pointer at the sentinel `0x5e47`. `sentinel()` plays that role
//! with `Some(Interface::IClassFactory)`. No row that checks for an untouched `obj` writes that
//! value, so a call that wrongly writes `None` or another interface fails the check.

use super::*;
use crate::dll::CLASS_INFO;

/// The out-pointer value the C harness starts from (`SENTINEL`).
fn sentinel() -> Option<Interface> {
    Some(Interface::IClassFactory)
}

#[test]
fn oracle_e01_dll_get_class_object_classfactory() {
    // E01 DllGetClassObject(CLSID_DsDisplaySpecifier,IID_IClassFactory) -> hr=0x00000000 out=IClassFactory refs=1 freed=0
    let mut out = sentinel();
    let hr = dll_get_class_object(
        Some(&CLSID_DS_DISPLAY_SPECIFIER),
        Some(&IID_ICLASSFACTORY),
        Some(&mut out),
    );
    assert_eq!(hr as u32, 0x0000_0000);
    assert_eq!(out, Some(Interface::IClassFactory));
}

#[test]
fn oracle_e02_dll_get_class_object_unknown_clsid() {
    // E02 DllGetClassObject(unknown clsid=IID_IUnknown,IID_IClassFactory) -> hr=0x80040111 out=NULL refs=-1 freed=0
    let mut out = sentinel();
    let hr = dll_get_class_object(
        Some(&IID_IUNKNOWN),
        Some(&IID_ICLASSFACTORY),
        Some(&mut out),
    );
    assert_eq!(hr as u32, 0x8004_0111);
    assert_eq!(out, None);
}

#[test]
fn oracle_e03_dll_get_class_object_null_clsid() {
    // E03 DllGetClassObject(NULL clsid,IID_IClassFactory) -> hr=0x80070057 out=untouched refs=-1 freed=0
    let mut out = sentinel();
    let hr = dll_get_class_object(None, Some(&IID_ICLASSFACTORY), Some(&mut out));
    assert_eq!(hr as u32, 0x8007_0057);
    assert_eq!(out, sentinel());
}

#[test]
fn oracle_e04_dll_get_class_object_idsdisplayspecifier_iid() {
    // E04 DllGetClassObject(CLSID_DsDisplaySpecifier,IID_IDsDisplaySpecifier) -> hr=0x80004002 out=NULL refs=-1 freed=1
    let mut out = sentinel();
    let hr = dll_get_class_object(
        Some(&CLSID_DS_DISPLAY_SPECIFIER),
        Some(&IID_IDS_DISPLAY_SPECIFIER),
        Some(&mut out),
    );
    assert_eq!(hr as u32, 0x8000_4002);
    assert_eq!(out, None);
}

#[test]
fn oracle_e05_dll_can_unload_now() {
    // E05 DllCanUnloadNow() -> hr=0x00000001
    assert_eq!(dll_can_unload_now() as u32, 0x0000_0001);
}

#[test]
fn oracle_e06_dll_install_stub() {
    // E06 DllInstall -> raised=0x80000100 flags=0x01 nargs=2 module=dsuiext function=DllInstall
    assert_eq!(
        dll_install(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "dsuiext",
            function: "DllInstall",
        }
    );
}

#[test]
fn oracle_e07_ds_browse_for_container_a_stub() {
    // E07 DsBrowseForContainerA -> raised=0x80000100 flags=0x01 nargs=2 module=dsuiext function=DsBrowseForContainerA
    assert_eq!(
        ds_browse_for_container_a(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "dsuiext",
            function: "DsBrowseForContainerA",
        }
    );
}

#[test]
fn oracle_e08_ds_browse_for_container_w_stub() {
    // E08 DsBrowseForContainerW -> raised=0x80000100 flags=0x01 nargs=2 module=dsuiext function=DsBrowseForContainerW
    assert_eq!(
        ds_browse_for_container_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "dsuiext",
            function: "DsBrowseForContainerW",
        }
    );
}

#[test]
fn oracle_e09_ds_get_friendly_class_name_stub() {
    // E09 DsGetFriendlyClassName -> raised=0x80000100 flags=0x01 nargs=2 module=dsuiext function=DsGetFriendlyClassName
    assert_eq!(
        ds_get_friendly_class_name(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "dsuiext",
            function: "DsGetFriendlyClassName",
        }
    );
}

#[test]
fn oracle_e10_ds_get_icon_stub() {
    // E10 DsGetIcon -> raised=0x80000100 flags=0x01 nargs=2 module=dsuiext function=DsGetIcon
    assert_eq!(
        ds_get_icon(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            module: "dsuiext",
            function: "DsGetIcon",
        }
    );
}

#[test]
fn oracle_o01_dispspec_query_iunknown_on_new_object() {
    // O01 dispspec.QueryInterface(IID_IUnknown) on ref=1 -> hr=0x00000000 out=IDsDisplaySpecifier refs=2 freed=0
    let mut spec = DisplaySpec::new();
    let mut out = sentinel();
    let hr = spec.query_interface(Some(&IID_IUNKNOWN), Some(&mut out));
    assert_eq!(hr as u32, 0x0000_0000);
    assert_eq!(out, Some(Interface::IDsDisplaySpecifier));
    assert_eq!(spec.release(), 1);
    assert_eq!(spec.release(), 0);
}

#[test]
fn oracle_o02_dispspec_query_classfactory_on_new_object() {
    // O02 dispspec.QueryInterface(IID_IClassFactory) on ref=1 -> hr=0x80004002 out=untouched refs=1 freed=0
    let mut spec = DisplaySpec::new();
    let mut out = sentinel();
    let hr = spec.query_interface(Some(&IID_ICLASSFACTORY), Some(&mut out));
    assert_eq!(hr as u32, 0x8000_4002);
    assert_eq!(out, sentinel());
    assert_eq!(spec.release(), 0);
}

#[test]
fn oracle_o03_dispspec_query_null_iid() {
    // O03 dispspec.QueryInterface(NULL iid) on ref=1 -> hr=0x80070057 out=untouched refs=1 freed=0
    let mut spec = DisplaySpec::new();
    let mut out = sentinel();
    let hr = spec.query_interface(None, Some(&mut out));
    assert_eq!(hr as u32, 0x8007_0057);
    assert_eq!(out, sentinel());
    assert_eq!(spec.release(), 0);
}

#[test]
fn oracle_o04_dispspec_add_ref_on_new_object() {
    // O04 dispspec.AddRef() on ref=1 -> ret=2 refs=2 freed=0
    let mut spec = DisplaySpec::new();
    assert_eq!(spec.add_ref(), 2);
    assert_eq!(spec.release(), 1);
    assert_eq!(spec.release(), 0);
}

#[test]
fn oracle_o05_dispspec_release_on_ref_two() {
    // O05 dispspec.Release() on ref=2 -> ret=1 refs=1 freed=0
    let mut spec = DisplaySpec::new();
    spec.add_ref();
    assert_eq!(spec.release(), 1);
    assert_eq!(spec.release(), 0);
}

#[test]
fn oracle_o06_dispspec_release_on_ref_one() {
    // O06 dispspec.Release() on ref=1 -> ret=0 refs=-1 freed=1
    let mut spec = DisplaySpec::new();
    assert_eq!(spec.release(), 0);
}

#[test]
fn oracle_o07_dispspec_create_idsdisplayspecifier() {
    // O07 DsDisplaySpecifier_create(IID_IDsDisplaySpecifier) -> hr=0x00000000 out=IDsDisplaySpecifier refs=1 freed=0
    let mut out = sentinel();
    let hr = DisplaySpec::create(&IID_IDS_DISPLAY_SPECIFIER, &mut out);
    assert_eq!(hr as u32, 0x0000_0000);
    assert_eq!(out, Some(Interface::IDsDisplaySpecifier));
}

#[test]
fn oracle_o08_dispspec_create_classfactory_iid() {
    // O08 DsDisplaySpecifier_create(IID_IClassFactory) -> hr=0x80004002 out=untouched refs=-1 freed=1
    let mut out = sentinel();
    let hr = DisplaySpec::create(&IID_ICLASSFACTORY, &mut out);
    assert_eq!(hr as u32, 0x8000_4002);
    assert_eq!(out, sentinel());
}

#[test]
fn oracle_o09_factory_query_iunknown_on_new_object() {
    // O09 factory.QueryInterface(IID_IUnknown) on ref=1 -> hr=0x00000000 out=IClassFactory refs=2 freed=0
    let mut factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    let hr = factory.query_interface(Some(&IID_IUNKNOWN), Some(&mut out));
    assert_eq!(hr as u32, 0x0000_0000);
    assert_eq!(out, Some(Interface::IClassFactory));
    assert_eq!(factory.release(), 1);
    assert_eq!(factory.release(), 0);
}

#[test]
fn oracle_o10_factory_query_idsdisplayspecifier_on_new_object() {
    // O10 factory.QueryInterface(IID_IDsDisplaySpecifier) on ref=1 -> hr=0x80004002 out=NULL refs=1 freed=0
    let mut factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    let hr = factory.query_interface(Some(&IID_IDS_DISPLAY_SPECIFIER), Some(&mut out));
    assert_eq!(hr as u32, 0x8000_4002);
    assert_eq!(out, None);
    assert_eq!(factory.release(), 0);
}

#[test]
fn oracle_o11_factory_query_null_iid() {
    // O11 factory.QueryInterface(NULL iid) on ref=1 -> hr=0x80070057 out=untouched refs=1 freed=0
    let mut factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    let hr = factory.query_interface(None, Some(&mut out));
    assert_eq!(hr as u32, 0x8007_0057);
    assert_eq!(out, sentinel());
    assert_eq!(factory.release(), 0);
}

#[test]
fn oracle_o12_factory_create_instance_with_outer() {
    // O12 factory.CreateInstance(outer!=NULL, IID_IDsDisplaySpecifier) -> hr=0x80040110 out=NULL refs=-1 freed=0
    let factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    let hr = factory.create_instance(true, Some(&IID_IDS_DISPLAY_SPECIFIER), Some(&mut out));
    assert_eq!(hr as u32, 0x8004_0110);
    assert_eq!(out, None);
}

#[test]
fn oracle_o13_factory_create_instance_without_outer() {
    // O13 factory.CreateInstance(outer=NULL, IID_IDsDisplaySpecifier) -> hr=0x00000000 out=IDsDisplaySpecifier refs=1 freed=0
    let factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    let hr = factory.create_instance(false, Some(&IID_IDS_DISPLAY_SPECIFIER), Some(&mut out));
    assert_eq!(hr as u32, 0x0000_0000);
    assert_eq!(out, Some(Interface::IDsDisplaySpecifier));
}

#[test]
fn oracle_o14_factory_create_instance_classfactory_iid() {
    // O14 factory.CreateInstance(outer=NULL, IID_IClassFactory) -> hr=0x80004002 out=NULL refs=-1 freed=1
    let factory = ClassFactory::new(&CLASS_INFO[0]);
    let mut out = sentinel();
    let hr = factory.create_instance(false, Some(&IID_ICLASSFACTORY), Some(&mut out));
    assert_eq!(hr as u32, 0x8000_4002);
    assert_eq!(out, None);
}

#[test]
fn oracle_o15_factory_release_on_ref_one() {
    // O15 factory.Release() on ref=1 -> ret=0 refs=-1 freed=1
    let mut factory = ClassFactory::new(&CLASS_INFO[0]);
    assert_eq!(factory.release(), 0);
}
