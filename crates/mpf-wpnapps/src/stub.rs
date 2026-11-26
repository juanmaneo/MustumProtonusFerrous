//! The `E_NOTIMPL` bodies of the manager, toast factory and notifier methods in Wine's
//! `dlls/wpnapps/toast_notification.c`. Each body returns `E_NOTIMPL`, and at most emits a FIXME first.
//! The statics' `GetIids`, `GetRuntimeClassName` and `GetTrustLevel` forward to the activation factory's bodies.

use crate::hresult::{E_NOTIMPL, HRESULT};

/// One `E_NOTIMPL` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StubMethod {
    ManagerFactoryGetIids,
    ManagerFactoryGetRuntimeClassName,
    ManagerFactoryGetTrustLevel,
    ManagerFactoryActivateInstance,
    ManagerCreateToastNotifier,
    ManagerGetTemplateContent,
    ToastFactoryGetIids,
    ToastFactoryGetRuntimeClassName,
    ToastFactoryGetTrustLevel,
    ToastFactoryActivateInstance,
    ToastFactoryCreateToastNotification,
    ToastNotifierGetIids,
    ToastNotifierGetRuntimeClassName,
    ToastNotifierGetTrustLevel,
    ToastNotifierShow,
    ToastNotifierHide,
    ToastNotifierGetSetting,
    ToastNotifierAddToSchedule,
    ToastNotifierRemoveFromSchedule,
    ToastNotifierGetScheduledToastNotifications,
}

impl StubMethod {
    /// The value the C body returns.
    pub fn hresult(self) -> HRESULT {
        E_NOTIMPL
    }
}
