//! Oracle-derived test vectors for the ported mmcndmgr logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/mmcndmgr/mmcndmgr.c`
//! (lines 36-155) and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`, lines 504-516)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/mmcndmgr/`, and running the
//! harness. The 25 printed rows came out clean under both sanitizers, so none is excluded. The
//! harness's `RtlRaiseException` shim records the record and longjmps out, because the verbatim
//! stub loops forever.
//!
//! The C prints `ppv` as `NULL`, `mmcversion`, `factory` or `sentinel` (untouched). The `Result`
//! carries the same information: `Ok` is the object, `Err(E_NOINTERFACE)` is `NULL`, and
//! `Err(CLASS_E_CLASSNOTAVAILABLE)` from `DllGetClassObject` is `sentinel`.

use super::*;

#[test]
fn oracle_mmcversion_rows() {
    // row 01 mmcversion.QueryInterface(IID_IMMCVersionInfo) -> hr=0x00000000 ppv=mmcversion
    assert_eq!(
        mmcversion_query_interface(&IID_IMMCVERSIONINFO),
        Ok(Object::MmcVersionInfo)
    );
    // row 02 mmcversion.QueryInterface(IID_IUnknown) -> hr=0x00000000 ppv=mmcversion
    assert_eq!(
        mmcversion_query_interface(&IID_IUNKNOWN),
        Ok(Object::MmcVersionInfo)
    );
    // row 03 mmcversion.QueryInterface(IID_IClassFactory) -> hr=0x80004002 ppv=NULL
    assert_eq!(
        mmcversion_query_interface(&IID_ICLASSFACTORY),
        Err(E_NOINTERFACE)
    );
    // row 04 mmcversion.QueryInterface(CLSID_MMCVersionInfo) -> hr=0x80004002 ppv=NULL
    assert_eq!(
        mmcversion_query_interface(&CLSID_MMCVERSIONINFO),
        Err(E_NOINTERFACE)
    );
    // row 05 mmcversion.AddRef -> refs=2
    assert_eq!(mmcversion_add_ref(), 2);
    // row 06 mmcversion.Release -> refs=1
    assert_eq!(mmcversion_release(), 1);

    // row 07 GetMMCVersion(&major, &minor) -> hr=0x00000000 major=3 minor=0
    let (mut major, mut minor) = (-1, -1);
    assert_eq!(
        mmcversion_get_mmc_version(Some(&mut major), Some(&mut minor)),
        S_OK
    );
    assert_eq!((major, minor), (3, 0));
    // row 08 GetMMCVersion(NULL, &minor) -> hr=0x00000000 major=-1 minor=0
    let (major, mut minor) = (-1, -1);
    assert_eq!(mmcversion_get_mmc_version(None, Some(&mut minor)), S_OK);
    assert_eq!((major, minor), (-1, 0));
    // row 09 GetMMCVersion(&major, NULL) -> hr=0x00000000 major=3 minor=-1
    let (mut major, minor) = (-1, -1);
    assert_eq!(mmcversion_get_mmc_version(Some(&mut major), None), S_OK);
    assert_eq!((major, minor), (3, -1));
}

#[test]
fn oracle_class_factory_rows() {
    // row 10 factory.QueryInterface(IID_IClassFactory) -> hr=0x00000000 ppv=factory
    assert_eq!(
        class_factory_query_interface(&IID_ICLASSFACTORY),
        Ok(Object::ClassFactory)
    );
    // row 11 factory.QueryInterface(IID_IUnknown) -> hr=0x00000000 ppv=factory
    assert_eq!(
        class_factory_query_interface(&IID_IUNKNOWN),
        Ok(Object::ClassFactory)
    );
    // row 12 factory.QueryInterface(IID_IMMCVersionInfo) -> hr=0x80004002 ppv=NULL
    assert_eq!(
        class_factory_query_interface(&IID_IMMCVERSIONINFO),
        Err(E_NOINTERFACE)
    );
    // row 13 factory.AddRef -> refs=2
    assert_eq!(class_factory_add_ref(), 2);
    // row 14 factory.Release -> refs=1
    assert_eq!(class_factory_release(), 1);
    // row 15 factory.CreateInstance(NULL, IID_IMMCVersionInfo) -> hr=0x00000000 ppv=mmcversion
    assert_eq!(
        class_factory_create_instance(&IID_IMMCVERSIONINFO),
        Ok(Object::MmcVersionInfo)
    );
    // row 16 factory.CreateInstance(outer, IID_IUnknown) -> hr=0x00000000 ppv=mmcversion
    assert_eq!(
        class_factory_create_instance(&IID_IUNKNOWN),
        Ok(Object::MmcVersionInfo)
    );
    // row 17 factory.CreateInstance(NULL, IID_IClassFactory) -> hr=0x80004002 ppv=NULL
    assert_eq!(
        class_factory_create_instance(&IID_ICLASSFACTORY),
        Err(E_NOINTERFACE)
    );
    // row 18 factory.LockServer(TRUE) -> hr=0x00000000
    assert_eq!(class_factory_lock_server(), S_OK);
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 19 DllGetClassObject(CLSID_MMCVersionInfo, IID_IClassFactory) -> hr=0x00000000 ppv=factory
    assert_eq!(
        dll_get_class_object(&CLSID_MMCVERSIONINFO, &IID_ICLASSFACTORY),
        Ok(Object::ClassFactory)
    );
    // row 20 DllGetClassObject(CLSID_MMCVersionInfo, IID_IUnknown) -> hr=0x00000000 ppv=factory
    assert_eq!(
        dll_get_class_object(&CLSID_MMCVERSIONINFO, &IID_IUNKNOWN),
        Ok(Object::ClassFactory)
    );
    // row 21 DllGetClassObject(CLSID_MMCVersionInfo, IID_IMMCVersionInfo) -> hr=0x80004002 ppv=NULL
    assert_eq!(
        dll_get_class_object(&CLSID_MMCVERSIONINFO, &IID_IMMCVERSIONINFO),
        Err(E_NOINTERFACE)
    );
    // row 22 DllGetClassObject(IID_IClassFactory, IID_IClassFactory) -> hr=0x80040111 ppv=sentinel
    assert_eq!(
        dll_get_class_object(&IID_ICLASSFACTORY, &IID_ICLASSFACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn oracle_stub_rows() {
    // row 23 stub CreateExecutivePlatform -> code=0x80000100 flags=0x1 nargs=2 p0=mmcndmgr.dll p1=CreateExecutivePlatform
    assert_eq!(
        create_executive_platform(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mmcndmgr.dll",
            function: "CreateExecutivePlatform",
        }
    );
    // row 24 stub _GetAllocCounters@0 -> code=0x80000100 flags=0x1 nargs=2 p0=mmcndmgr.dll p1=_GetAllocCounters@0
    assert_eq!(
        get_alloc_counters_win32(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mmcndmgr.dll",
            function: "_GetAllocCounters@0",
        }
    );
    // row 25 stub GetAllocCounters -> code=0x80000100 flags=0x1 nargs=2 p0=mmcndmgr.dll p1=GetAllocCounters
    assert_eq!(
        get_alloc_counters_win64(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "mmcndmgr.dll",
            function: "GetAllocCounters",
        }
    );
}
