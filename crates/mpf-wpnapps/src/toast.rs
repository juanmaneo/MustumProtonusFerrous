//! Port of `struct toast_notifier` and `create_toast_notifier` in Wine's `dlls/wpnapps/toast_notification.c`.

use crate::hresult::{E_NOINTERFACE, E_OUTOFMEMORY, HRESULT};
use crate::query::{notifier_view, Iid, View};
use crate::refcount::RefCount;

/// An `IToastNotifier` object. It is created with `ref = 1`. The C `Release` frees it at zero; here the caller
/// drops the value instead.
#[derive(Debug)]
pub struct ToastNotifier {
    ref_count: RefCount,
}

impl ToastNotifier {
    /// `toast_notifier_QueryInterface`. A successful query adds a reference.
    pub fn query_interface(&mut self, iid: Iid) -> Result<View, HRESULT> {
        let view = notifier_view(iid).ok_or(E_NOINTERFACE)?;
        self.add_ref();
        Ok(view)
    }

    /// `toast_notifier_AddRef`: returns the new count.
    pub fn add_ref(&mut self) -> u32 {
        self.ref_count.add_ref()
    }

    /// `toast_notifier_Release`: returns the new count.
    pub fn release(&mut self) -> u32 {
        self.ref_count.release()
    }

    pub fn ref_count(&self) -> i32 {
        self.ref_count.get()
    }
}

/// `create_toast_notifier`. `alloc_succeeds` models whether `calloc` returns memory. On failure the output is not
/// written, and the result is `E_OUTOFMEMORY`.
pub fn create_toast_notifier(alloc_succeeds: bool) -> Result<ToastNotifier, HRESULT> {
    if !alloc_succeeds {
        return Err(E_OUTOFMEMORY);
    }
    Ok(ToastNotifier {
        ref_count: RefCount::initial(),
    })
}
