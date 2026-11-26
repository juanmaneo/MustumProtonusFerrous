//! Port of `toast_manager_statics` and `toast_factory` in Wine's `dlls/wpnapps/toast_notification.c`: the two
//! static activation factories that `DllGetActivationFactory` hands out.

use crate::hresult::{E_NOINTERFACE, HRESULT};
use crate::query::{manager_view, toast_factory_view, Iid, View};
use crate::refcount::RefCount;

/// The runtime classes that `DllGetActivationFactory` serves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    ToastNotificationManager,
    ToastNotification,
}

/// A static activation factory. It starts with `ref = 1` and is never freed: its `Release` does not call `free`.
#[derive(Debug)]
pub struct StaticFactory {
    class: Class,
    ref_count: RefCount,
}

impl StaticFactory {
    pub fn new(class: Class) -> Self {
        Self {
            class,
            ref_count: RefCount::initial(),
        }
    }

    /// `QueryInterface`. The accepted IIDs depend on the class. A successful query adds a reference.
    pub fn query_interface(&mut self, iid: Iid) -> Result<View, HRESULT> {
        let view = match self.class {
            Class::ToastNotificationManager => manager_view(iid),
            Class::ToastNotification => toast_factory_view(iid),
        }
        .ok_or(E_NOINTERFACE)?;
        self.add_ref();
        Ok(view)
    }

    /// `AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count.add_ref()
    }

    /// `Release`: returns the new count. The static object is never freed.
    pub fn release(&mut self) -> u32 {
        self.ref_count.release()
    }

    pub fn ref_count(&self) -> i32 {
        self.ref_count.get()
    }
}
