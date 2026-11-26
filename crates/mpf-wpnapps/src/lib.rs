//! Port of the portable logic in Wine's `dlls/wpnapps/main.c`, `dlls/wpnapps/toast_notification.c`
//! and `dlls/wpnapps/wpnapps.spec`.
//!
//! Ported:
//! - `DllCanUnloadNow`, which `wpnapps` does not define itself: it links the default from
//!   `libs/winecrt0/dll_canunload.c`: [`dll_can_unload_now`]
//! - `DllGetClassObject`: [`dll_get_class_object`]
//! - `DllGetActivationFactory` and its `wcscmp` class-id match: [`Module::dll_get_activation_factory`],
//!   [`TOAST_NOTIFICATION_MANAGER_CLASS`], [`TOAST_NOTIFICATION_CLASS`], [`Class`]
//! - The `QueryInterface` IID tables of the static factories and of `struct toast_notifier`: [`Iid`],
//!   [`View`], [`Module::query_interface`], [`ToastNotifier::query_interface`]
//! - `AddRef` and `Release` counts with the `LONG` count returned as `ULONG`: [`ToastNotifier`],
//!   [`Module::ref_count`]
//! - `create_toast_notifier` with its success and `E_OUTOFMEMORY` paths. `CreateToastNotifierWithId`
//!   ignores its app id and calls it too: [`create_toast_notifier`]
//! - `debugstr_ToastTemplateType`: [`debugstr_toast_template_type`]
//! - The `E_NOTIMPL` bodies of the manager, factory and notifier methods: [`StubMethod`]
//! - `HRESULT` values from `winerror.h`, and `SUCCEEDED`: [`succeeded`]
//!
//! Skipped:
//! - `WindowsGetStringRawBuffer` HSTRING storage: the class id is a UTF-16 slice, and `None` is a NULL `HSTRING`.
//! - COM vtable dispatch and the `DEFINE_IINSPECTABLE` delegation. The statics and the toast factory forward
//!   `GetIids`, `GetRuntimeClassName` and `GetTrustLevel` to the activation factory, and their
//!   `IActivationFactory` view is the same object. The Rust methods are called directly.
//! - `calloc` and `free`: the OOM outcome is an input, and dropping the object replaces `free`.
//! - `FIXME` and `TRACE` output, and `debugstr_guid` and `debugstr_hstring`.
//! - `DllGetClassObject` arguments: the C body ignores them and never writes `out`.
//! - The `factory` out-pointer of `DllGetActivationFactory`: the Rust result is the [`Class`] that was
//!   handed out, or the error. The C body writes NULL on failure.
//! - The parameters of the `E_NOTIMPL` methods, and `GetTemplateContent`'s `type`: those bodies never read them.

mod debugstr;
mod dll;
mod factory;
mod hresult;
mod query;
mod refcount;
mod stub;
mod toast;

pub use debugstr::debugstr_toast_template_type;
pub use dll::{
    dll_can_unload_now, dll_get_class_object, Module, TOAST_NOTIFICATION_CLASS,
    TOAST_NOTIFICATION_MANAGER_CLASS,
};
pub use factory::Class;
pub use hresult::{
    succeeded, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, HRESULT,
    S_FALSE, S_OK,
};
pub use query::{Iid, View};
pub use stub::StubMethod;
pub use toast::{create_toast_notifier, ToastNotifier};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
