//! Oracle-derived test vectors for the ported wmp logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`, the
//! `ClassFactory_*` functions and `DllMain` (`dlls/wmp/wmp_main.c`), and of
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`), with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/wmp/`. The harness printed 20 rows.
//! ASan and UBSan flagged none of them, so none is excluded. The stub recorder leaves through
//! `longjmp` instead of returning, because the C stub loops on `RtlRaiseException`.

use super::*;

/// `CLSID_FilterGraph` from the harness: a CLSID that is not the WMP class.
const CLSID_FILTER_GRAPH: Guid = Guid {
    data1: 0xe436_ebb3,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

/// `IID_IDispatch` from the harness: an IID that the factory does not implement.
const IID_IDISPATCH: Guid = Guid {
    data1: 0x0002_0400,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
};

#[test]
fn oracle_stub_rows() {
    // row 01 export 3000 | code=0x80000100 flags=0x1 nparams=2 module=wmp.dll function=3000(ordinal)
    assert_eq!(
        ordinal_3000(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "wmp.dll",
            function: "3000",
        }
    );
    // row 02 export 3002 | code=0x80000100 flags=0x1 nparams=2 module=wmp.dll function=3002(ordinal)
    assert_eq!(
        ordinal_3002(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "wmp.dll",
            function: "3002",
        }
    );
    // row 03 export 3003 | code=0x80000100 flags=0x1 nparams=2 module=wmp.dll function=3003(ordinal)
    assert_eq!(
        ordinal_3003(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "wmp.dll",
            function: "3003",
        }
    );
    // row 04 export 3004 | code=0x80000100 flags=0x1 nparams=2 module=wmp.dll function=3004(ordinal)
    assert_eq!(
        ordinal_3004(),
        StubException {
            code: 0x8000_0100,
            flags: 0x1,
            number_parameters: 2,
            module: "wmp.dll",
            function: "3004",
        }
    );
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 05 WMP IUnknown | hr=0x00000000 ppv=WMPFactory
    assert_eq!(
        dll_get_class_object(&CLSID_WINDOWS_MEDIA_PLAYER, &IID_IUNKNOWN),
        ClassObject::Queried(S_OK)
    );
    // row 06 WMP IClassFactory | hr=0x00000000 ppv=WMPFactory
    assert_eq!(
        dll_get_class_object(&CLSID_WINDOWS_MEDIA_PLAYER, &IID_ICLASSFACTORY),
        ClassObject::Queried(S_OK)
    );
    // row 07 WMP IDispatch | hr=0x80004002 ppv=NULL
    let r = dll_get_class_object(&CLSID_WINDOWS_MEDIA_PLAYER, &IID_IDISPATCH);
    assert_eq!(r, ClassObject::Queried(0x8000_4002));
    assert_eq!(r.hresult(), 0x8000_4002);
    // row 08 FilterGraph IUnknown | hr=0x80040111 ppv=sentinel
    let r = dll_get_class_object(&CLSID_FILTER_GRAPH, &IID_IUNKNOWN);
    assert_eq!(r, ClassObject::NotAvailable);
    assert_eq!(r.hresult(), 0x8004_0111);
}

#[test]
fn oracle_class_factory_query_interface_rows() {
    // row 09 IUnknown | hr=0x00000000 ppv=WMPFactory
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN), 0x0000_0000);
    // row 10 IClassFactory | hr=0x00000000 ppv=WMPFactory
    assert_eq!(
        class_factory_query_interface(&IID_ICLASSFACTORY),
        0x0000_0000
    );
    // row 11 IDispatch | hr=0x80004002 ppv=NULL
    assert_eq!(class_factory_query_interface(&IID_IDISPATCH), 0x8000_4002);
}

#[test]
fn oracle_class_factory_count_and_lock_rows() {
    // row 12 AddRef | ret=2
    assert_eq!(class_factory_add_ref(), 2);
    // row 13 Release | ret=1
    assert_eq!(class_factory_release(), 1);
    // row 14 LockServer TRUE | hr=0x00000000
    assert_eq!(class_factory_lock_server(true), 0x0000_0000);
    // row 15 LockServer FALSE | hr=0x00000000
    assert_eq!(class_factory_lock_server(false), 0x0000_0000);
}

#[test]
fn oracle_dll_main_rows() {
    // row 16 ATTACH lpv=NULL | ret=1 stored=1 calls=[DisableThreadLibraryCalls]
    assert_eq!(
        dll_main(DLL_PROCESS_ATTACH, true),
        DllMainOutcome {
            ret: true,
            attached: true,
            cleanup: false,
        }
    );
    // row 17 DETACH lpv=NULL | ret=1 stored=0 calls=[unregister_wmp_class,unregister_player_msg_class,release_typelib]
    assert_eq!(
        dll_main(DLL_PROCESS_DETACH, true),
        DllMainOutcome {
            ret: true,
            attached: false,
            cleanup: true,
        }
    );
    // row 18 DETACH lpv=non-NULL | ret=1 stored=0 calls=[]
    assert_eq!(
        dll_main(DLL_PROCESS_DETACH, false),
        DllMainOutcome {
            ret: true,
            attached: false,
            cleanup: false,
        }
    );
    // row 19 THREAD_ATTACH(2) lpv=NULL | ret=1 stored=0 calls=[]
    assert_eq!(
        dll_main(2, true),
        DllMainOutcome {
            ret: true,
            attached: false,
            cleanup: false,
        }
    );
    // row 20 THREAD_DETACH(3) lpv=NULL | ret=1 stored=0 calls=[]
    assert_eq!(
        dll_main(3, true),
        DllMainOutcome {
            ret: true,
            attached: false,
            cleanup: false,
        }
    );
}
