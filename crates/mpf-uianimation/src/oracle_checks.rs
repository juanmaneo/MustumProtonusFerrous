//! Oracle-derived test vectors for the ported uianimation logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`, the class
//! factory methods and the `QueryInterface`, `AddRef`, `Release` and creator functions of the four
//! classes (`dlls/uianimation/main.c`), and of `DllCanUnloadNow`, `DllRegisterServer` and
//! `DllUnregisterServer` (`libs/winecrt0`), with `gcc -fsanitize=address,undefined -g`, under
//! `target/canon/uianimation/`. The harness printed 25 rows. None was flagged by ASan or UBSan, so
//! no vector was excluded.
//!
//! The rows show the `ret` value and the `obj` out-parameter. `obj=cf:<class>` is the class factory,
//! `obj=inst:<class>` is an object the factory created, and `release=0` is the harness's `Release`
//! of that object, which the port does not model.

use super::*;
use std::cell::Cell;

const IID_IUIANIMATION_STORYBOARD: Guid = 0xa8ff128f9bf94af19e67e5e410defb84;

#[test]
fn oracle_dll_entry_point_rows() {
    // row 01 DllCanUnloadNow | ret=0x00000001
    assert_eq!(dll_can_unload_now(), 0x0000_0001);

    // row 02 DllRegisterServer ok | ret=0x00000000 calls=1
    let calls = Cell::new(0);
    let hr = dll_register_server(|| {
        calls.set(calls.get() + 1);
        S_OK
    });
    assert_eq!((hr, calls.get()), (0x0000_0000, 1));

    // row 03 DllRegisterServer fail | ret=0x8007000e calls=1
    let calls = Cell::new(0);
    let hr = dll_register_server(|| {
        calls.set(calls.get() + 1);
        0x8007_000e
    });
    assert_eq!((hr, calls.get()), (0x8007_000e, 1));

    // row 04 DllUnregisterServer ok | ret=0x00000000 calls=1
    let calls = Cell::new(0);
    let hr = dll_unregister_server(|| {
        calls.set(calls.get() + 1);
        S_OK
    });
    assert_eq!((hr, calls.get()), (0x0000_0000, 1));

    // row 05 DllUnregisterServer fail | ret=0x8007000e calls=1
    let calls = Cell::new(0);
    let hr = dll_unregister_server(|| {
        calls.set(calls.get() + 1);
        0x8007_000e
    });
    assert_eq!((hr, calls.get()), (0x8007_000e, 1));
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // row 06 DllGetClassObject manager IUnknown | ret=0x00000000 obj=cf:manager
    assert_eq!(
        dll_get_class_object(CLSID_UIANIMATION_MANAGER, IID_IUNKNOWN),
        (S_OK, Out::Set(ClassKind::Manager))
    );

    // row 07 DllGetClassObject manager IUIAnimationManager | ret=0x80004002 obj=null
    assert_eq!(
        dll_get_class_object(CLSID_UIANIMATION_MANAGER, IID_IUIANIMATION_MANAGER),
        (E_NOINTERFACE, Out::Null)
    );

    // row 08 DllGetClassObject timer IClassFactory | ret=0x00000000 obj=cf:timer
    assert_eq!(
        dll_get_class_object(CLSID_UIANIMATION_TIMER, IID_ICLASSFACTORY),
        (S_OK, Out::Set(ClassKind::Timer))
    );

    // row 09 DllGetClassObject timer IUIAnimationTimer | ret=0x80004002 obj=null
    assert_eq!(
        dll_get_class_object(CLSID_UIANIMATION_TIMER, IID_IUIANIMATION_TIMER),
        (E_NOINTERFACE, Out::Null)
    );

    // row 10 DllGetClassObject transition IUnknown | ret=0x00000000 obj=cf:transition
    assert_eq!(
        dll_get_class_object(CLSID_UIANIMATION_TRANSITION_FACTORY, IID_IUNKNOWN),
        (S_OK, Out::Set(ClassKind::TransitionFactory))
    );

    // row 11 DllGetClassObject library IClassFactory | ret=0x00000000 obj=cf:library
    assert_eq!(
        dll_get_class_object(CLSID_UIANIMATION_TRANSITION_LIBRARY, IID_ICLASSFACTORY),
        (S_OK, Out::Set(ClassKind::TransitionLibrary))
    );

    // row 12 DllGetClassObject library IUIAnimationTransitionLibrary | ret=0x80004002 obj=null
    assert_eq!(
        dll_get_class_object(
            CLSID_UIANIMATION_TRANSITION_LIBRARY,
            IID_IUIANIMATION_TRANSITION_LIBRARY
        ),
        (E_NOINTERFACE, Out::Null)
    );

    // row 13 DllGetClassObject unknown clsid | ret=0x80040111 obj=untouched
    assert_eq!(
        dll_get_class_object(IID_IUIANIMATION_STORYBOARD, IID_IUNKNOWN),
        (CLASS_E_CLASSNOTAVAILABLE, Out::Untouched)
    );
}

#[test]
fn oracle_class_factory_rows() {
    // row 14 class factory AddRef | count=2
    assert_eq!(class_factory_add_ref(), 2);

    // row 15 class factory Release | count=1
    assert_eq!(class_factory_release(), 1);

    // row 16 class factory LockServer TRUE | ret=0x00000000
    assert_eq!(class_factory_lock_server(true), S_OK);

    // row 17 class factory QI IUnknown | ret=0x00000000 obj=cf:manager
    assert_eq!(
        class_factory_query_interface(ClassKind::Manager, IID_IUNKNOWN),
        (S_OK, Out::Set(ClassKind::Manager))
    );

    // row 18 class factory QI IUIAnimationManager | ret=0x80004002 obj=null
    assert_eq!(
        class_factory_query_interface(ClassKind::Manager, IID_IUIANIMATION_MANAGER),
        (E_NOINTERFACE, Out::Null)
    );
}

#[test]
fn oracle_create_instance_rows() {
    // row 19 CreateInstance manager IUnknown | ret=0x00000000 obj=inst:manager release=0
    assert_eq!(
        class_factory_create_instance(ClassKind::Manager, IID_IUNKNOWN),
        (S_OK, Out::Set(ClassKind::Manager))
    );

    // row 20 CreateInstance timer IUIAnimationTimer | ret=0x00000000 obj=inst:timer release=0
    assert_eq!(
        class_factory_create_instance(ClassKind::Timer, IID_IUIANIMATION_TIMER),
        (S_OK, Out::Set(ClassKind::Timer))
    );

    // row 21 CreateInstance transition IClassFactory | ret=0x80004002 obj=null
    assert_eq!(
        class_factory_create_instance(ClassKind::TransitionFactory, IID_ICLASSFACTORY),
        (E_NOINTERFACE, Out::Null)
    );

    // row 22 CreateInstance library IUIAnimationTransitionLibrary | ret=0x00000000 obj=inst:library release=0
    assert_eq!(
        class_factory_create_instance(
            ClassKind::TransitionLibrary,
            IID_IUIANIMATION_TRANSITION_LIBRARY
        ),
        (S_OK, Out::Set(ClassKind::TransitionLibrary))
    );

    // row 23 CreateInstance manager outer set | ret=0x00000000 obj=inst:manager release=0
    // The harness passed a non-NULL outer; the port has no outer argument because the C code ignores it.
    assert_eq!(
        class_factory_create_instance(ClassKind::Manager, IID_IUIANIMATION_MANAGER),
        (S_OK, Out::Set(ClassKind::Manager))
    );

    // row 24 CreateInstance transition IUIAnimationTransitionFactory | ret=0x00000000 obj=inst:transition release=0
    assert_eq!(
        class_factory_create_instance(
            ClassKind::TransitionFactory,
            IID_IUIANIMATION_TRANSITION_FACTORY
        ),
        (S_OK, Out::Set(ClassKind::TransitionFactory))
    );

    // row 25 CreateInstance library IUIAnimationStoryboard | ret=0x80004002 obj=null
    assert_eq!(
        class_factory_create_instance(ClassKind::TransitionLibrary, IID_IUIANIMATION_STORYBOARD),
        (E_NOINTERFACE, Out::Null)
    );
}
