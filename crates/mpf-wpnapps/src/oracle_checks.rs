//! Oracle rows from `target/canon/wpnapps/harness.c`, compiled with `gcc -fsanitize=address,undefined` and run
//! against the verbatim `main.c`, `toast_notification.c` and `dll_canunload.c` bodies. 25 rows, 0 excluded,
//! ASan and UBSan clean. The `toast_notifier_vtbl` is a placeholder in the harness, so no row calls through it.
//! `addref=N` in the C output is the change in the object's count, and the Rust tests assert the new count.

use crate::{
    create_toast_notifier, debugstr_toast_template_type, dll_can_unload_now, dll_get_class_object,
    Class, Iid, Module, StubMethod, View, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL,
    E_OUTOFMEMORY, S_FALSE, TOAST_NOTIFICATION_CLASS, TOAST_NOTIFICATION_MANAGER_CLASS,
};

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn oracle_dll_entry_points() {
    // row 01: DllCanUnloadNow() = 0x1
    assert_eq!(dll_can_unload_now(), 0x1);
    assert_eq!(dll_can_unload_now(), S_FALSE);
    // row 02: DllGetClassObject(NULL, NULL, NULL) = 0x80040111
    assert_eq!(dll_get_class_object(), 0x8004_0111_u32 as i32);
    assert_eq!(dll_get_class_object(), CLASS_E_CLASSNOTAVAILABLE);
}

#[test]
fn oracle_dll_get_activation_factory() {
    // row 03: ToastNotificationManager -> 0, out = manager factory, addref +1
    let mut module = Module::default();
    let class = module
        .dll_get_activation_factory(Some(&wide(TOAST_NOTIFICATION_MANAGER_CLASS)))
        .unwrap();
    assert_eq!(class, Class::ToastNotificationManager);
    assert_eq!(module.ref_count(class), 2);

    // row 04: ToastNotification -> 0, out = notification factory, addref +1
    let mut module = Module::default();
    let class = module
        .dll_get_activation_factory(Some(&wide(TOAST_NOTIFICATION_CLASS)))
        .unwrap();
    assert_eq!(class, Class::ToastNotification);
    assert_eq!(module.ref_count(class), 2);

    // row 05: ToastNotificationManager2 -> 0x80040111, out NULL, no addref
    let mut module = Module::default();
    let result = module.dll_get_activation_factory(Some(&wide(
        "Windows.UI.Notifications.ToastNotificationManager2",
    )));
    assert_eq!(result, Err(CLASS_E_CLASSNOTAVAILABLE));
    assert_eq!(module.ref_count(Class::ToastNotificationManager), 1);

    // row 06: NULL class id -> 0x80040111, out NULL
    let mut module = Module::default();
    assert_eq!(
        module.dll_get_activation_factory(None),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );

    // row 07: "Windows.UI.Notifications.ToastNotification\0x" -> 0, out = notification factory, addref +1
    // (wcscmp stops at the NUL)
    let mut module = Module::default();
    let class = module
        .dll_get_activation_factory(Some(&wide("Windows.UI.Notifications.ToastNotification\0x")))
        .unwrap();
    assert_eq!(class, Class::ToastNotification);
    assert_eq!(module.ref_count(class), 2);
}

#[test]
fn oracle_debugstr_toast_template_type() {
    // row 08: type 0
    assert_eq!(
        debugstr_toast_template_type(0),
        "ToastTemplateType_ToastImageAndText01"
    );
    // row 09: type 4
    assert_eq!(
        debugstr_toast_template_type(4),
        "ToastTemplateType_ToastText01"
    );
    // row 10: type 7
    assert_eq!(
        debugstr_toast_template_type(7),
        "ToastTemplateType_ToastText04"
    );
    // row 11: type 8 is past ARRAY_SIZE
    assert_eq!(debugstr_toast_template_type(8), "{unknown 0x8}");
    // row 12: type -1 is out of range as an unsigned comparison
    assert_eq!(debugstr_toast_template_type(-1), "{unknown 0xffffffff}");
}

#[test]
fn oracle_manager_query_interface() {
    // row 13: IActivationFactory -> 0, out = manager factory, addref +1
    let mut module = Module::default();
    assert_eq!(
        module.query_interface(Class::ToastNotificationManager, Iid::ActivationFactory),
        Ok(View::ActivationFactory)
    );
    assert_eq!(module.ref_count(Class::ToastNotificationManager), 2);

    // row 14: IToastNotificationManagerStatics -> 0, out = statics, addref +1
    let mut module = Module::default();
    assert_eq!(
        module.query_interface(
            Class::ToastNotificationManager,
            Iid::ToastNotificationManagerStatics
        ),
        Ok(View::ToastNotificationManagerStatics)
    );
    assert_eq!(module.ref_count(Class::ToastNotificationManager), 2);

    // row 15: IToastNotifier -> 0x80004002, out NULL, no addref
    let mut module = Module::default();
    assert_eq!(
        module.query_interface(Class::ToastNotificationManager, Iid::ToastNotifier),
        Err(E_NOINTERFACE)
    );
    assert_eq!(module.ref_count(Class::ToastNotificationManager), 1);
}

#[test]
fn oracle_notification_factory_and_notifier_query_interface() {
    // row 16: IToastNotificationFactory -> 0, out = factory interface, addref +1
    let mut module = Module::default();
    assert_eq!(
        module.query_interface(Class::ToastNotification, Iid::ToastNotificationFactory),
        Ok(View::ToastNotificationFactory)
    );
    assert_eq!(module.ref_count(Class::ToastNotification), 2);

    // row 17: notifier QueryInterface(IToastNotifier) -> 0, addref +1
    let mut notifier = create_toast_notifier(true).unwrap();
    assert_eq!(
        notifier.query_interface(Iid::ToastNotifier),
        Ok(View::ToastNotifier)
    );
    assert_eq!(notifier.ref_count(), 2);

    // row 18: notifier QueryInterface(IActivationFactory) -> 0x80004002, out NULL, no addref
    let mut notifier = create_toast_notifier(true).unwrap();
    assert_eq!(
        notifier.query_interface(Iid::ActivationFactory),
        Err(E_NOINTERFACE)
    );
    assert_eq!(notifier.ref_count(), 1);
}

#[test]
fn oracle_not_implemented_methods() {
    // rows 19 to 23: each body returns E_NOTIMPL and does not write its output
    assert_eq!(
        StubMethod::ManagerFactoryActivateInstance.hresult(),
        0x8000_4001_u32 as i32
    );
    assert_eq!(StubMethod::ManagerGetTemplateContent.hresult(), E_NOTIMPL);
    assert_eq!(StubMethod::ManagerCreateToastNotifier.hresult(), E_NOTIMPL);
    assert_eq!(StubMethod::ToastNotifierShow.hresult(), E_NOTIMPL);
    assert_eq!(
        StubMethod::ToastFactoryCreateToastNotification.hresult(),
        E_NOTIMPL
    );
}

#[test]
fn oracle_create_toast_notifier() {
    // row 24: allocation succeeds -> S_OK, a new object with ref = 1
    let notifier = create_toast_notifier(true).unwrap();
    assert_eq!(notifier.ref_count(), 1);

    // row 25: allocation fails -> E_OUTOFMEMORY, the output is not written
    assert_eq!(create_toast_notifier(false).unwrap_err(), E_OUTOFMEMORY);
}
