//! Oracle-derived test vectors for the ported uiribbon logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`, the
//! `XFCF_*` class factory, and the `UIRibbonFrameworkImpl_*` object and `IUIFramework` stubs
//! (`dlls/uiribbon/main.c`, `dlls/uiribbon/uiribbon.c`), plus the winecrt0 `DllCanUnloadNow`
//! body, with `gcc -fsanitize=address,undefined -g` under `target/canon/uiribbon/`. The harness
//! printed 36 rows (R01 to R36). Neither sanitizer flagged a row, so no vector is excluded.
//!
//! The `Release` rows (R33 to R36) print `release=N` in the harness, not an HRESULT. The
//! `obj=untouched` rows are ported as "no interface returned": a `None` in the port.

use super::*;

/// `IUIImage`, from `include/uiribbon.idl`. The harness uses it as an IID that no class handles.
const IID_IUIIMAGE: Guid = Guid {
    data1: 0x23c8_c838,
    data2: 0x4de6,
    data3: 0x436b,
    data4: [0xab, 0x01, 0x55, 0x54, 0xbb, 0x7c, 0x30, 0xdd],
};

/// The harness's `near_clsid`: `CLSID_UIRibbonFramework` with `Data1` XOR 1.
const NEAR_CLSID: Guid = Guid {
    data1: 0x9267_49fb,
    data2: 0x2615,
    data3: 0x4987,
    data4: [0x88, 0x45, 0xc3, 0x3e, 0x65, 0xf2, 0xb9, 0x57],
};

/// The harness's `factory_refs` and `fw_refs`: `AddRef` then `Release`, which reports the count
/// before the pair.
fn factory_refs(f: &mut ClassFactory) -> u32 {
    let c = f.add_ref();
    f.release();
    c - 1
}

fn framework_refs(f: &mut UIRibbonFramework) -> u32 {
    let c = f.add_ref();
    f.release();
    c - 1
}

fn new_factory() -> ClassFactory {
    let (hr, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_ICLASSFACTORY);
    assert_eq!(hr, S_OK);
    factory.expect("R01 obj=set")
}

fn new_framework() -> UIRibbonFramework {
    let (hr, framework) = new_factory().create_instance(&IID_IUIFRAMEWORK);
    assert_eq!(hr, S_OK);
    framework.expect("R14 obj=set")
}

#[test]
fn oracle_dll_rows() {
    // R01 DllGetClassObject(UIRibbonFramework,IClassFactory) hr=0x00000000 obj=set refs=1
    let mut factory = new_factory();
    assert_eq!(factory_refs(&mut factory), 1);

    // R02 DllGetClassObject(UIRibbonFramework,IUnknown) hr=0x00000000 obj=set release=0
    let (hr, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_IUNKNOWN);
    assert_eq!(hr, S_OK);
    assert_eq!(factory.expect("R02 obj=set").release(), 0);

    // R03 DllGetClassObject(UIRibbonFramework,IUIFramework) hr=0x80004002 obj=untouched
    let (hr, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_IUIFRAMEWORK);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(factory.is_none());

    // R04 DllGetClassObject(UIRibbonFramework,IUIImage) hr=0x80004002 obj=untouched
    let (hr, factory) = dll_get_class_object(&CLSID_UIRIBBONFRAMEWORK, &IID_IUIIMAGE);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(factory.is_none());

    // R05 DllGetClassObject(near-miss CLSID,IClassFactory) hr=0x80040111 obj=untouched
    let (hr, factory) = dll_get_class_object(&NEAR_CLSID, &IID_ICLASSFACTORY);
    assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);
    assert!(factory.is_none());

    // R06 DllGetClassObject(IUnknown as CLSID,IUIFramework) hr=0x80004002 obj=untouched
    let (hr, factory) = dll_get_class_object(&IID_IUNKNOWN, &IID_IUIFRAMEWORK);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(factory.is_none());

    // R07 DllGetClassObject(IUnknown as CLSID,IClassFactory) hr=0x80040111 obj=untouched
    let (hr, factory) = dll_get_class_object(&IID_IUNKNOWN, &IID_ICLASSFACTORY);
    assert_eq!(hr, CLASS_E_CLASSNOTAVAILABLE);
    assert!(factory.is_none());

    // R08 DllCanUnloadNow() hr=0x00000001
    assert_eq!(dll_can_unload_now(), S_FALSE);
}

#[test]
fn oracle_class_factory_rows() {
    let mut factory = new_factory();

    // R09 factory QueryInterface(IUnknown) hr=0x00000000 obj=set refs_after=2
    let mut obj = None;
    assert_eq!(factory.query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert_eq!(obj, Some(Interface::ClassFactory));
    assert_eq!(factory_refs(&mut factory), 2);
    factory.release();

    // R10 factory QueryInterface(IClassFactory) hr=0x00000000 obj=set refs_after=2
    let mut obj = None;
    assert_eq!(factory.query_interface(&IID_ICLASSFACTORY, &mut obj), S_OK);
    assert_eq!(obj, Some(Interface::ClassFactory));
    assert_eq!(factory_refs(&mut factory), 2);
    factory.release();

    // R11 factory QueryInterface(IUIFramework) hr=0x80004002 obj=untouched refs_after=1
    let mut obj = Some(Interface::UIRibbonFramework);
    assert_eq!(
        factory.query_interface(&IID_IUIFRAMEWORK, &mut obj),
        E_NOINTERFACE
    );
    assert_eq!(obj, Some(Interface::UIRibbonFramework));
    assert_eq!(factory_refs(&mut factory), 1);

    // R12 factory LockServer(TRUE) hr=0x00000000
    assert_eq!(factory.lock_server(true), S_OK);

    // R13 factory LockServer(FALSE) hr=0x00000000
    assert_eq!(factory.lock_server(false), S_OK);
}

#[test]
fn oracle_create_instance_rows() {
    let factory = new_factory();

    // R14 factory CreateInstance(NULL,IUIFramework) hr=0x00000000 obj=set refs=1
    let (hr, framework) = factory.create_instance(&IID_IUIFRAMEWORK);
    assert_eq!(hr, S_OK);
    assert_eq!(framework_refs(&mut framework.expect("R14 obj=set")), 1);

    // R15 factory CreateInstance(NULL,IUnknown) hr=0x00000000 obj=set refs=1
    let (hr, framework) = factory.create_instance(&IID_IUNKNOWN);
    assert_eq!(hr, S_OK);
    assert_eq!(framework_refs(&mut framework.expect("R15 obj=set")), 1);

    // R16 factory CreateInstance(NULL,IClassFactory) hr=0x80004002 obj=null
    let (hr, framework) = factory.create_instance(&IID_ICLASSFACTORY);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(framework.is_none());

    // R17 factory CreateInstance(outer set,IUIFramework) hr=0x00000000 obj=set refs=1
    // The C harness passes a non-NULL outer. The C code never reads it, so the port has no
    // argument for it and the call is the same as R14.
    let (hr, framework) = factory.create_instance(&IID_IUIFRAMEWORK);
    assert_eq!(hr, S_OK);
    assert_eq!(framework_refs(&mut framework.expect("R17 obj=set")), 1);

    // R18 factory CreateInstance(NULL,IUIImage) hr=0x80004002 obj=null
    let (hr, framework) = factory.create_instance(&IID_IUIIMAGE);
    assert_eq!(hr, E_NOINTERFACE);
    assert!(framework.is_none());
}

#[test]
fn oracle_framework_rows() {
    let mut framework = new_framework();

    // R19 framework QueryInterface(IUnknown) hr=0x00000000 obj=set refs_after=2
    let mut obj = None;
    assert_eq!(framework.query_interface(&IID_IUNKNOWN, &mut obj), S_OK);
    assert_eq!(obj, Some(Interface::UIRibbonFramework));
    assert_eq!(framework_refs(&mut framework), 2);
    framework.release();

    // R20 framework QueryInterface(IUIFramework) hr=0x00000000 obj=set refs_after=2
    let mut obj = None;
    assert_eq!(framework.query_interface(&IID_IUIFRAMEWORK, &mut obj), S_OK);
    assert_eq!(obj, Some(Interface::UIRibbonFramework));
    assert_eq!(framework_refs(&mut framework), 2);
    framework.release();

    // R21 framework QueryInterface(IClassFactory) hr=0x80004002 obj=untouched refs_after=1
    let mut obj = Some(Interface::ClassFactory);
    assert_eq!(
        framework.query_interface(&IID_ICLASSFACTORY, &mut obj),
        E_NOINTERFACE
    );
    assert_eq!(obj, Some(Interface::ClassFactory));
    assert_eq!(framework_refs(&mut framework), 1);

    // R22 framework AddRef addref=2
    assert_eq!(framework.add_ref(), 2);

    // R23 framework Release release=1
    assert_eq!(framework.release(), 1);

    // R33 framework Release (last ref) release=0
    assert_eq!(framework.release(), 0);
}

#[test]
fn oracle_release_rows() {
    // R34 framework Release (CreateInstance IUnknown) release=0
    let mut framework = new_framework();
    assert_eq!(framework.release(), 0);

    // R35 framework Release (outer set) release=0
    let mut framework = new_framework();
    assert_eq!(framework.release(), 0);

    // R36 factory Release (last ref) release=0
    let mut factory = new_factory();
    assert_eq!(factory.release(), 0);
}

#[test]
fn oracle_ui_framework_method_rows() {
    // R24 framework Initialize(NULL,NULL) hr=0x80004001
    assert_eq!(
        ui_framework_method(UIFrameworkMethod::Initialize),
        E_NOTIMPL
    );

    // R25 framework Destroy() hr=0x80004001
    assert_eq!(ui_framework_method(UIFrameworkMethod::Destroy), E_NOTIMPL);

    // R26 framework LoadUI(NULL,NULL) hr=0x80004001
    assert_eq!(ui_framework_method(UIFrameworkMethod::LoadUI), E_NOTIMPL);

    // R27 framework GetView(0,IUnknown) hr=0x80004001 obj=untouched
    assert_eq!(ui_framework_method(UIFrameworkMethod::GetView), E_NOTIMPL);

    // R28 framework GetUICommandProperty(0,NULL,NULL) hr=0x80004001
    assert_eq!(
        ui_framework_method(UIFrameworkMethod::GetUICommandProperty),
        E_NOTIMPL
    );

    // R29 framework SetUICommandProperty(0,NULL,NULL) hr=0x80004001
    assert_eq!(
        ui_framework_method(UIFrameworkMethod::SetUICommandProperty),
        E_NOTIMPL
    );

    // R30 framework InvalidateUICommand(0,0,NULL) hr=0x80004001
    assert_eq!(
        ui_framework_method(UIFrameworkMethod::InvalidateUICommand),
        E_NOTIMPL
    );

    // R31 framework FlushPendingInvalidations() hr=0x80004001
    assert_eq!(
        ui_framework_method(UIFrameworkMethod::FlushPendingInvalidations),
        E_NOTIMPL
    );

    // R32 framework SetModes(3) hr=0x80004001
    assert_eq!(ui_framework_method(UIFrameworkMethod::SetModes), E_NOTIMPL);
}
