//! Port of `DllGetClassObject` and `DllGetActivationFactory` in Wine's `dlls/wpnapps/main.c`, and of the
//! default `DllCanUnloadNow` from `libs/winecrt0/dll_canunload.c`.

use crate::factory::{Class, StaticFactory};
use crate::hresult::{CLASS_E_CLASSNOTAVAILABLE, HRESULT, S_FALSE};
use crate::query::{Iid, View};

/// `RuntimeClass_Windows_UI_Notifications_ToastNotificationManager`.
pub const TOAST_NOTIFICATION_MANAGER_CLASS: &str =
    "Windows.UI.Notifications.ToastNotificationManager";

/// `RuntimeClass_Windows_UI_Notifications_ToastNotification`.
pub const TOAST_NOTIFICATION_CLASS: &str = "Windows.UI.Notifications.ToastNotification";

/// `DllCanUnloadNow`: `wpnapps` does not define it, so the winecrt0 default returns `S_FALSE`.
pub fn dll_can_unload_now() -> HRESULT {
    S_FALSE
}

/// `DllGetClassObject`: the body ignores its arguments and never writes `out`.
pub fn dll_get_class_object() -> HRESULT {
    CLASS_E_CLASSNOTAVAILABLE
}

/// The module state: the two static factories that `DllGetActivationFactory` hands out.
#[derive(Debug)]
pub struct Module {
    toast_manager: StaticFactory,
    toast_notification: StaticFactory,
}

impl Default for Module {
    fn default() -> Self {
        Self {
            toast_manager: StaticFactory::new(Class::ToastNotificationManager),
            toast_notification: StaticFactory::new(Class::ToastNotification),
        }
    }
}

impl Module {
    fn factory(&self, class: Class) -> &StaticFactory {
        match class {
            Class::ToastNotificationManager => &self.toast_manager,
            Class::ToastNotification => &self.toast_notification,
        }
    }

    fn factory_mut(&mut self, class: Class) -> &mut StaticFactory {
        match class {
            Class::ToastNotificationManager => &mut self.toast_manager,
            Class::ToastNotification => &mut self.toast_notification,
        }
    }

    /// `QueryInterface` on the static factory of `class`.
    pub fn query_interface(&mut self, class: Class, iid: Iid) -> Result<View, HRESULT> {
        self.factory_mut(class).query_interface(iid)
    }

    /// `Release` on the static factory of `class`: returns the new count.
    pub fn release(&mut self, class: Class) -> u32 {
        self.factory_mut(class).release()
    }

    pub fn ref_count(&self, class: Class) -> i32 {
        self.factory(class).ref_count()
    }

    /// `DllGetActivationFactory`. The class id is compared with `wcscmp`, so it ends at the first NUL, and a NULL
    /// `HSTRING` matches nothing. On a match, the static factory is queried for `IActivationFactory`, which adds
    /// a reference. The C output is NULL on failure, so only the matched class is returned.
    pub fn dll_get_activation_factory(
        &mut self,
        class_id: Option<&[u16]>,
    ) -> Result<Class, HRESULT> {
        let class = class_of(class_id).ok_or(CLASS_E_CLASSNOTAVAILABLE)?;
        self.query_interface(class, Iid::ActivationFactory)
            .map(|_| class)
    }
}

fn class_of(class_id: Option<&[u16]>) -> Option<Class> {
    let buffer = class_id?;
    let end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    let name = &buffer[..end];
    if name
        .iter()
        .copied()
        .eq(TOAST_NOTIFICATION_MANAGER_CLASS.encode_utf16())
    {
        Some(Class::ToastNotificationManager)
    } else if name
        .iter()
        .copied()
        .eq(TOAST_NOTIFICATION_CLASS.encode_utf16())
    {
        Some(Class::ToastNotification)
    } else {
        None
    }
}
