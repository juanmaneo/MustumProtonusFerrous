//! Boundary and branch tests for the port. Expectations here are not oracle-derived unless a comment says so.

use crate::refcount::RefCount;
use crate::{
    create_toast_notifier, debugstr_toast_template_type, succeeded, Class, Iid, Module, StubMethod,
    View, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, S_FALSE, S_OK,
    TOAST_NOTIFICATION_CLASS, TOAST_NOTIFICATION_MANAGER_CLASS,
};

const ALL_IIDS: [Iid; 7] = [
    Iid::Unknown,
    Iid::Inspectable,
    Iid::ActivationFactory,
    Iid::ToastNotificationManagerStatics,
    Iid::ToastNotificationFactory,
    Iid::ToastNotifier,
    Iid::Other,
];

const ALL_CLASSES: [Class; 2] = [Class::ToastNotificationManager, Class::ToastNotification];

const ALL_STUBS: [StubMethod; 20] = [
    StubMethod::ManagerFactoryGetIids,
    StubMethod::ManagerFactoryGetRuntimeClassName,
    StubMethod::ManagerFactoryGetTrustLevel,
    StubMethod::ManagerFactoryActivateInstance,
    StubMethod::ManagerCreateToastNotifier,
    StubMethod::ManagerGetTemplateContent,
    StubMethod::ToastFactoryGetIids,
    StubMethod::ToastFactoryGetRuntimeClassName,
    StubMethod::ToastFactoryGetTrustLevel,
    StubMethod::ToastFactoryActivateInstance,
    StubMethod::ToastFactoryCreateToastNotification,
    StubMethod::ToastNotifierGetIids,
    StubMethod::ToastNotifierGetRuntimeClassName,
    StubMethod::ToastNotifierGetTrustLevel,
    StubMethod::ToastNotifierShow,
    StubMethod::ToastNotifierHide,
    StubMethod::ToastNotifierGetSetting,
    StubMethod::ToastNotifierAddToSchedule,
    StubMethod::ToastNotifierRemoveFromSchedule,
    StubMethod::ToastNotifierGetScheduledToastNotifications,
];

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[test]
fn hresult_constants_and_succeeded() {
    // Not oracle-derived: the values come from winerror.h as transcribed in hresult.rs.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE, 0x8000_4002_u32 as i32);
    assert_eq!(E_NOTIMPL, 0x8000_4001_u32 as i32);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000E_u32 as i32);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111_u32 as i32);

    // Not oracle-derived: SUCCEEDED is the sign test on HRESULT.
    assert!(succeeded(S_OK));
    assert!(succeeded(S_FALSE));
    assert!(succeeded(i32::MAX));
    assert!(!succeeded(E_NOTIMPL));
    assert!(!succeeded(i32::MIN));
    assert!(!succeeded(-1));
}

#[test]
fn class_id_match_edge_cases() {
    // Not oracle-derived: boundaries of the wcscmp match. Each input is a NUL-truncated UTF-16 buffer.
    let manager = wide(TOAST_NOTIFICATION_MANAGER_CLASS);
    let notification = wide(TOAST_NOTIFICATION_CLASS);

    let cases: [(&[u16], Option<Class>); 9] = [
        (&[], None),
        (&[0], None),
        (&manager, Some(Class::ToastNotificationManager)),
        (&notification, Some(Class::ToastNotification)),
        // A prefix of a class name is not a match.
        (&manager[..manager.len() - 1], None),
        // A longer name is not a match.
        (&wide("Windows.UI.Notifications.ToastNotificationX"), None),
        // wcscmp is case sensitive.
        (&wide("windows.ui.notifications.toastnotification"), None),
        // A NUL in the first position truncates the name to empty.
        (&wide("\0Windows.UI.Notifications.ToastNotification"), None),
        // Text after the NUL is ignored, and the prefix before it matches.
        (
            &wide("Windows.UI.Notifications.ToastNotification\0junk"),
            Some(Class::ToastNotification),
        ),
    ];
    for (input, expected) in cases {
        let mut module = Module::default();
        let got = module.dll_get_activation_factory(Some(input));
        match expected {
            Some(class) => assert_eq!(got, Ok(class), "input {input:?}"),
            None => assert_eq!(got, Err(CLASS_E_CLASSNOTAVAILABLE), "input {input:?}"),
        }
    }
}

#[test]
fn null_hstring_matches_nothing_and_adds_no_reference() {
    // Not oracle-derived: a NULL HSTRING reads as the empty string, so it matches no class and no count changes.
    let mut module = Module::default();
    assert_eq!(
        module.dll_get_activation_factory(None),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    for class in ALL_CLASSES {
        assert_eq!(module.ref_count(class), 1);
    }
}

#[test]
fn successful_activation_adds_exactly_one_reference() {
    // Not oracle-derived: repeated activations keep counting on the same static factory.
    let mut module = Module::default();
    let manager = wide(TOAST_NOTIFICATION_MANAGER_CLASS);
    assert_eq!(
        module.dll_get_activation_factory(Some(&manager)),
        Ok(Class::ToastNotificationManager)
    );
    assert_eq!(
        module.dll_get_activation_factory(Some(&manager)),
        Ok(Class::ToastNotificationManager)
    );
    assert_eq!(module.ref_count(Class::ToastNotificationManager), 3);
    assert_eq!(module.ref_count(Class::ToastNotification), 1);
}

#[test]
fn failed_query_interface_does_not_add_a_reference() {
    // Not oracle-derived: every (class, IID) pair the view tables reject leaves the count unchanged.
    for class in ALL_CLASSES {
        let mut module = Module::default();
        for iid in ALL_IIDS {
            let before = module.ref_count(class);
            match module.query_interface(class, iid) {
                Ok(_) => assert_eq!(module.ref_count(class), before + 1),
                Err(hr) => {
                    assert_eq!(hr, E_NOINTERFACE);
                    assert_eq!(module.ref_count(class), before);
                }
            }
        }
    }
}

#[test]
fn manager_view_table_covers_every_iid() {
    // Not oracle-derived: the statics' table as a whole. Each accepted IID maps to one view.
    let mut module = Module::default();
    let accepted: Vec<(Iid, View)> = ALL_IIDS
        .iter()
        .filter_map(|&iid| {
            module
                .query_interface(Class::ToastNotificationManager, iid)
                .ok()
                .map(|view| (iid, view))
        })
        .collect();
    assert_eq!(
        accepted,
        vec![
            (Iid::Unknown, View::ActivationFactory),
            (Iid::Inspectable, View::ActivationFactory),
            (Iid::ActivationFactory, View::ActivationFactory),
            (
                Iid::ToastNotificationManagerStatics,
                View::ToastNotificationManagerStatics
            ),
        ]
    );
}

#[test]
fn toast_factory_view_table_covers_every_iid() {
    // Not oracle-derived: the notification factory's table as a whole.
    let mut module = Module::default();
    let accepted: Vec<(Iid, View)> = ALL_IIDS
        .iter()
        .filter_map(|&iid| {
            module
                .query_interface(Class::ToastNotification, iid)
                .ok()
                .map(|view| (iid, view))
        })
        .collect();
    assert_eq!(
        accepted,
        vec![
            (Iid::Unknown, View::ActivationFactory),
            (Iid::Inspectable, View::ActivationFactory),
            (Iid::ActivationFactory, View::ActivationFactory),
            (
                Iid::ToastNotificationFactory,
                View::ToastNotificationFactory
            ),
        ]
    );
}

#[test]
fn notifier_view_table_covers_every_iid() {
    // Not oracle-derived: the notifier's table as a whole.
    let accepted: Vec<Iid> = ALL_IIDS
        .into_iter()
        .filter(|&iid| {
            create_toast_notifier(true)
                .unwrap()
                .query_interface(iid)
                .is_ok()
        })
        .collect();
    assert_eq!(
        accepted,
        vec![Iid::Unknown, Iid::Inspectable, Iid::ToastNotifier]
    );
}

#[test]
fn static_release_counts_down_without_freeing() {
    // Not oracle-derived: Release reports the new count. The static object is kept, so the count can reach zero.
    let mut module = Module::default();
    assert_eq!(module.release(Class::ToastNotification), 0);
    assert_eq!(module.ref_count(Class::ToastNotification), 0);
    // Releasing past zero wraps the signed count, and the ULONG reads back as 0xffffffff.
    assert_eq!(module.release(Class::ToastNotification), u32::MAX);
    assert_eq!(module.ref_count(Class::ToastNotification), -1);
}

#[test]
fn notifier_release_reaches_zero_and_add_ref_counts_up() {
    // Not oracle-derived: AddRef and Release on a notifier, including the free point where the C body calls free.
    let mut notifier = create_toast_notifier(true).unwrap();
    assert_eq!(notifier.add_ref(), 2);
    assert_eq!(notifier.release(), 1);
    assert_eq!(notifier.release(), 0);
    assert_eq!(notifier.ref_count(), 0);
}

#[test]
fn ref_count_release_past_zero_wraps_as_ulong() {
    // Not oracle-derived: InterlockedDecrement wraps, and the new value is returned as a ULONG.
    let mut count = RefCount::initial();
    assert_eq!(count.release(), 0);
    assert_eq!(count.get(), 0);
    assert_eq!(count.release(), u32::MAX);
    assert_eq!(count.get(), -1);
    assert_eq!(count.add_ref(), 0);
}

#[test]
fn debugstr_boundaries() {
    // Not oracle-derived: the last valid index, the first invalid index and the extremes of the signed range.
    assert_eq!(
        debugstr_toast_template_type(7),
        "ToastTemplateType_ToastText04"
    );
    assert_eq!(debugstr_toast_template_type(8), "{unknown 0x8}");
    assert_eq!(
        debugstr_toast_template_type(i32::MAX),
        "{unknown 0x7fffffff}"
    );
    assert_eq!(
        debugstr_toast_template_type(i32::MIN),
        "{unknown 0x80000000}"
    );
}

#[test]
fn not_implemented_methods_return_e_notimpl() {
    // Not oracle-derived for the 15 methods that have no oracle row: each body returns E_NOTIMPL.
    for method in ALL_STUBS {
        assert_eq!(method.hresult(), E_NOTIMPL, "{method:?}");
    }
}

#[test]
fn create_toast_notifier_allocation_failure_is_out_of_memory() {
    // Not oracle-derived: the failure path returns no object, and only the failure code.
    assert_eq!(create_toast_notifier(false).unwrap_err(), E_OUTOFMEMORY);
}

#[test]
fn derived_traits_are_exercised() {
    // Not oracle-derived: derived Clone, Debug and Eq are part of the public surface, so each is called once.
    for class in ALL_CLASSES {
        assert_eq!(Clone::clone(&class), class);
        assert!(!format!("{class:?}").is_empty());
    }
    for iid in ALL_IIDS {
        assert_eq!(Clone::clone(&iid), iid);
        assert!(!format!("{iid:?}").is_empty());
    }
    for view in [
        View::ActivationFactory,
        View::ToastNotificationManagerStatics,
        View::ToastNotificationFactory,
        View::ToastNotifier,
    ] {
        assert_eq!(Clone::clone(&view), view);
        assert!(!format!("{view:?}").is_empty());
    }
    for method in ALL_STUBS {
        assert_eq!(Clone::clone(&method), method);
        assert!(!format!("{method:?}").is_empty());
    }
    let module = Module::default();
    assert!(format!("{module:?}").contains("ToastNotification"));
    let notifier = create_toast_notifier(true).unwrap();
    assert!(format!("{notifier:?}").contains("ToastNotifier"));
    assert!(format!("{:?}", RefCount::initial()).contains('1'));
}
