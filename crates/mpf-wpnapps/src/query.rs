//! IID dispatch of the `QueryInterface` implementations in `toast_notification.c`.

/// The IIDs that `QueryInterface` distinguishes. `Other` stands for any other GUID.
///
/// `Unknown` is `IUnknown` (`00000000-0000-0000-C000-000000000046`), `Inspectable` is
/// `IInspectable` (`AF86E2E0-B12D-4C6A-9C5A-D7AA65101E90`), and `ActivationFactory` is
/// `IActivationFactory` (`00000035-0000-0000-C000-000000000046`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iid {
    Unknown,
    Inspectable,
    ActivationFactory,
    ToastNotificationManagerStatics,
    ToastNotificationFactory,
    ToastNotifier,
    Other,
}

/// The interface view that a successful query returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum View {
    ActivationFactory,
    ToastNotificationManagerStatics,
    ToastNotificationFactory,
    ToastNotifier,
}

/// `toast_manager_factory_QueryInterface`: `IUnknown`, `IInspectable` and `IActivationFactory` select
/// the activation factory view; `IToastNotificationManagerStatics` selects the statics view.
pub(crate) fn manager_view(iid: Iid) -> Option<View> {
    match iid {
        Iid::Unknown | Iid::Inspectable | Iid::ActivationFactory => Some(View::ActivationFactory),
        Iid::ToastNotificationManagerStatics => Some(View::ToastNotificationManagerStatics),
        Iid::ToastNotificationFactory | Iid::ToastNotifier | Iid::Other => None,
    }
}

/// `toast_act_factory_QueryInterface`: `IUnknown`, `IInspectable` and `IActivationFactory` select
/// the activation factory view; `IToastNotificationFactory` selects the factory view.
pub(crate) fn toast_factory_view(iid: Iid) -> Option<View> {
    match iid {
        Iid::Unknown | Iid::Inspectable | Iid::ActivationFactory => Some(View::ActivationFactory),
        Iid::ToastNotificationFactory => Some(View::ToastNotificationFactory),
        Iid::ToastNotificationManagerStatics | Iid::ToastNotifier | Iid::Other => None,
    }
}

/// `toast_notifier_QueryInterface`: `IUnknown`, `IInspectable` and `IToastNotifier` are accepted.
pub(crate) fn notifier_view(iid: Iid) -> Option<View> {
    match iid {
        Iid::Unknown | Iid::Inspectable | Iid::ToastNotifier => Some(View::ToastNotifier),
        Iid::ActivationFactory
        | Iid::ToastNotificationManagerStatics
        | Iid::ToastNotificationFactory
        | Iid::Other => None,
    }
}
