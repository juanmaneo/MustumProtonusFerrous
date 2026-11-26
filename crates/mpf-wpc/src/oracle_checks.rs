//! Oracle-derived test vectors for the ported wpc logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/wpc/wpc.c` and
//! `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`) with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/wpc/`, and running the harness.
//! The 24 printed rows all ran clean: ASan and UBSan reported nothing, so no vector is excluded.

use super::*;
use crate::guid::*;

/// The `CLSID` from the oracle harness that no DLL in this port serves.
fn other_clsid() -> Guid {
    Guid {
        data1: 0x1234_5678,
        data2: 0x1234,
        data3: 0x1234,
        data4: [1, 2, 3, 4, 5, 6, 7, 8],
    }
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 01 DllGetClassObject(CLSID_WindowsParentalControls, IID_IUnknown) -> hr=0x00000000 ppv=factory
    let mut ppv = Some(Object::ParentalControls);
    assert_eq!(
        dll_get_class_object(&CLSID_WINDOWS_PARENTAL_CONTROLS, &IID_UNKNOWN, &mut ppv),
        S_OK
    );
    assert_eq!(ppv, Some(Object::ClassFactory));

    // row 02 DllGetClassObject(CLSID_WindowsParentalControls, IID_IClassFactory) -> hr=0x00000000 ppv=factory
    let mut ppv = Some(Object::ParentalControls);
    assert_eq!(
        dll_get_class_object(
            &CLSID_WINDOWS_PARENTAL_CONTROLS,
            &IID_CLASS_FACTORY,
            &mut ppv
        ),
        S_OK
    );
    assert_eq!(ppv, Some(Object::ClassFactory));

    // row 03 DllGetClassObject(CLSID_WindowsParentalControls, IID_IWindowsParentalControls) -> hr=0x80004002 ppv=NULL
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        dll_get_class_object(
            &CLSID_WINDOWS_PARENTAL_CONTROLS,
            &IID_WINDOWS_PARENTAL_CONTROLS,
            &mut ppv
        ),
        E_NOINTERFACE
    );
    assert_eq!(ppv, None);

    // row 04 DllGetClassObject(CLSID_WindowsParentalControls, IID_IWindowsParentalControlsCore) -> hr=0x80004002 ppv=NULL
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        dll_get_class_object(
            &CLSID_WINDOWS_PARENTAL_CONTROLS,
            &IID_WINDOWS_PARENTAL_CONTROLS_CORE,
            &mut ppv
        ),
        E_NOINTERFACE
    );
    assert_eq!(ppv, None);

    // row 05 DllGetClassObject(other CLSID, IID_IUnknown) -> hr=0x80040111 ppv=sentinel
    let mut ppv = Some(Object::ParentalControls);
    assert_eq!(
        dll_get_class_object(&other_clsid(), &IID_UNKNOWN, &mut ppv),
        CLASS_E_CLASSNOTAVAILABLE
    );
    assert_eq!(ppv, Some(Object::ParentalControls));
}

#[test]
fn oracle_class_factory_rows() {
    // row 06 ClassFactory.QueryInterface(IID_IUnknown) -> hr=0x00000000 ppv=factory
    let mut ppv = Some(Object::ParentalControls);
    assert_eq!(class_factory::query_interface(&IID_UNKNOWN, &mut ppv), S_OK);
    assert_eq!(ppv, Some(Object::ClassFactory));

    // row 07 ClassFactory.QueryInterface(IID_IClassFactory) -> hr=0x00000000 ppv=factory
    let mut ppv = Some(Object::ParentalControls);
    assert_eq!(
        class_factory::query_interface(&IID_CLASS_FACTORY, &mut ppv),
        S_OK
    );
    assert_eq!(ppv, Some(Object::ClassFactory));

    // row 08 ClassFactory.QueryInterface(IID_IWindowsParentalControls) -> hr=0x80004002 ppv=NULL
    let mut ppv = Some(Object::ParentalControls);
    assert_eq!(
        class_factory::query_interface(&IID_WINDOWS_PARENTAL_CONTROLS, &mut ppv),
        E_NOINTERFACE
    );
    assert_eq!(ppv, None);

    // row 09 ClassFactory.AddRef -> ret=2
    assert_eq!(class_factory::add_ref(), 2);

    // row 10 ClassFactory.Release -> ret=1
    assert_eq!(class_factory::release(), 1);
}

#[test]
fn oracle_create_instance_rows() {
    // row 11 ClassFactory.CreateInstance(NULL outer, IID_IUnknown) -> hr=0x00000000 ppv=obj
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(class_factory::create_instance(&IID_UNKNOWN, &mut ppv), S_OK);
    assert_eq!(ppv, Some(Object::ParentalControls));

    // row 12 ClassFactory.CreateInstance(outer, IID_IWindowsParentalControls) -> hr=0x00000000 ppv=obj
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        class_factory::create_instance(&IID_WINDOWS_PARENTAL_CONTROLS, &mut ppv),
        S_OK
    );
    assert_eq!(ppv, Some(Object::ParentalControls));

    // row 13 ClassFactory.CreateInstance(NULL outer, IID_IWindowsParentalControlsCore) -> hr=0x00000000 ppv=obj
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        class_factory::create_instance(&IID_WINDOWS_PARENTAL_CONTROLS_CORE, &mut ppv),
        S_OK
    );
    assert_eq!(ppv, Some(Object::ParentalControls));

    // row 14 ClassFactory.CreateInstance(NULL outer, IID_IClassFactory) -> hr=0x80004002 ppv=NULL
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        class_factory::create_instance(&IID_CLASS_FACTORY, &mut ppv),
        E_NOINTERFACE
    );
    assert_eq!(ppv, None);
}

#[test]
fn oracle_windows_parental_controls_rows() {
    // row 15 WindowsParentalControls.QueryInterface(IID_IUnknown) -> hr=0x00000000 ppv=obj
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(object::query_interface(&IID_UNKNOWN, &mut ppv), S_OK);
    assert_eq!(ppv, Some(Object::ParentalControls));

    // row 16 WindowsParentalControls.QueryInterface(IID_IClassFactory) -> hr=0x80004002 ppv=NULL
    let mut ppv = Some(Object::ClassFactory);
    assert_eq!(
        object::query_interface(&IID_CLASS_FACTORY, &mut ppv),
        E_NOINTERFACE
    );
    assert_eq!(ppv, None);

    // row 17 WindowsParentalControls.AddRef -> ret=2
    assert_eq!(object::add_ref(), 2);

    // row 18 WindowsParentalControls.Release -> ret=1
    assert_eq!(object::release(), 1);

    // row 19 WindowsParentalControls.GetVisibility -> hr=0x80004001 ppv=NULL
    assert_eq!(object::get_visibility(), E_NOTIMPL);

    // row 20 WindowsParentalControls.GetUserSettings(S-1) -> hr=0x80004001 ppv=NULL
    assert_eq!(object::get_user_settings(), E_NOTIMPL);

    // row 21 WindowsParentalControls.GetWebSettings(S-1) -> hr=0x80004001 ppv=NULL
    assert_eq!(object::get_web_settings(), E_NOTIMPL);

    // row 22 WindowsParentalControls.GetWebFilterInfo -> hr=0x80004001 ppv=NULL
    assert_eq!(object::get_web_filter_info(), E_NOTIMPL);

    // row 23 WindowsParentalControls.GetGamesSettings(S-1) -> hr=0x80004001 ppv=NULL
    assert_eq!(object::get_games_settings(), E_NOTIMPL);
}

#[test]
fn oracle_apply_parental_controls_w_row() {
    // row 24 ApplyParentalControlsW (@ stub) -> raised=1 code=0x80000100 flags=0x1 nargs=2 p0=wpc.dll p1=ApplyParentalControlsW
    assert_eq!(
        apply_parental_controls_w(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "wpc.dll",
            function: "ApplyParentalControlsW",
        }
    );
}
