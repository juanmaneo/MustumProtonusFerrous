//! Port of the portable logic in Wine's `dlls/rtscom/rtscom_main.c` and `dlls/rtscom/rtscom.spec`.
//!
//! Ported:
//! - `DllGetClassObject` CLSID dispatch and `DllCanUnloadNow`: [`dll_get_class_object`],
//!   [`dll_can_unload_now`]
//! - The class factory for `CLSID_RealTimeStylus`: [`class_factory_query_interface`],
//!   [`class_factory_add_ref`], [`class_factory_release`], [`class_factory_create_instance`],
//!   [`class_factory_lock_server`]
//! - The `RealTimeStylus` object's `QueryInterface` and reference counting, and the IID check
//!   in `realtime_stylus_create`: [`RealTimeStylus`], [`realtime_stylus_create`]
//! - The 31 `FIXME ... return E_NOTIMPL` methods of `IRealTimeStylus`: [`StubMethod`],
//!   [`stub_method`], [`STUB_METHODS`]
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`: they forward to winecrt0's
//!   `__wine_register_resources` and `__wine_unregister_resources`, which read the DLL's PE
//!   resources. The port has no PE image.
//! - `@ stub` exports: `rtscom.spec` has none. All four exports are `-private`.
//! - The `outer` argument of `IClassFactory::CreateInstance`, which the C code never reads.
//! - The COM vtable layout, and `QueryInterface` for interfaces other than `IUnknown`,
//!   `IClassFactory` and `IRealTimeStylus`.
//! - Thread safety (`InterlockedIncrement` and `InterlockedDecrement`) and the `malloc`
//!   failure path (`E_OUTOFMEMORY`).
//! - The `FIXME` traces and the method arguments, which the stub bodies never read.

mod class_factory;
mod com;
mod dll;
mod realtime_stylus;
mod stubs;

pub use class_factory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release,
};
pub use com::{
    Guid, Interface, CLSID_REALTIMESTYLUS, IID_ICLASSFACTORY, IID_IREALTIMESTYLUS, IID_IUNKNOWN,
};
pub use dll::{dll_can_unload_now, dll_get_class_object};
pub use realtime_stylus::{realtime_stylus_create, RealTimeStylus};
pub use stubs::{stub_method, StubMethod, STUB_METHODS};

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
