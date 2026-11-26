//! Port of the portable logic in Wine's `dlls/uiribbon/main.c` and `dlls/uiribbon/uiribbon.c`,
//! and the export table in `dlls/uiribbon/uiribbon.spec`.
//!
//! Ported:
//! - `DllGetClassObject` IID check and CLSID dispatch, and `DllCanUnloadNow`:
//!   [`dll_get_class_object`], [`dll_can_unload_now`]
//! - The class factory for `CLSID_UIRibbonFramework`: [`ClassFactory`] with its
//!   `QueryInterface`, `AddRef`, `Release`, `CreateInstance` and `LockServer`
//! - The `IUIFramework` object's `QueryInterface` and reference counting, and
//!   `UIRibbonFrameworkImpl_Create`: [`UIRibbonFramework`], [`ui_ribbon_framework_create`]
//! - The nine `FIXME ... return E_NOTIMPL` methods of `IUIFramework`: [`UIFrameworkMethod`],
//!   [`ui_framework_method`], [`UI_FRAMEWORK_METHODS`]
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`: they forward to winecrt0's
//!   `__wine_register_resources` and `__wine_unregister_resources`, which read the DLL's PE
//!   resources. The port has no PE image.
//! - `@ stub` exports: `uiribbon.spec` has none. All four exports are `-private`.
//! - The `pUnkOuter` argument of `CreateInstance` and `UIRibbonFrameworkImpl_Create`, which the C
//!   code never reads.
//! - The COM vtable layout, and `QueryInterface` for interfaces other than `IUnknown`,
//!   `IClassFactory` and `IUIFramework`.
//! - Thread safety (`InterlockedIncrement` and `InterlockedDecrement`) and the `HeapAlloc`
//!   failure path (`E_OUTOFMEMORY`).
//! - The `FIXME` traces and the method arguments, which the stub bodies never read.

mod class_factory;
mod com;
mod dll;
mod framework;
mod stubs;

pub use class_factory::ClassFactory;
pub use com::{
    Guid, Interface, CLSID_UIRIBBONFRAMEWORK, IID_ICLASSFACTORY, IID_IUIFRAMEWORK, IID_IUNKNOWN,
};
pub use dll::{dll_can_unload_now, dll_get_class_object};
pub use framework::{ui_ribbon_framework_create, UIRibbonFramework};
pub use stubs::{ui_framework_method, UIFrameworkMethod, UI_FRAMEWORK_METHODS};

/// `HRESULT`, as a signed 32-bit value.
pub type Hresult = i32;

pub const S_OK: Hresult = 0;
pub const S_FALSE: Hresult = 1;
pub const E_NOTIMPL: Hresult = 0x8000_4001u32 as i32;
pub const E_NOINTERFACE: Hresult = 0x8000_4002u32 as i32;
pub const CLASS_E_CLASSNOTAVAILABLE: Hresult = 0x8004_0111u32 as i32;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
