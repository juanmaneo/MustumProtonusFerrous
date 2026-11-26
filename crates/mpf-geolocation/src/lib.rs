//! Port of the portable logic in Wine's `dlls/geolocation/main.c` and `dlls/geolocation/weakref.c`.
//!
//! Ported:
//! - `DllGetActivationFactory`, `DllGetClassObject`, and `DllCanUnloadNow` (the `libs/winecrt0`
//!   default): `dll_get_activation_factory`, `dll_get_class_object`, `dll_can_unload_now`
//! - `factory_QueryInterface`, `factory_AddRef`, `factory_Release`, `factory_ActivateInstance`:
//!   `factory_query_interface`, `factory_add_ref`, `factory_release`, `factory_activate_instance`
//! - `geolocator_QueryInterface`, `geolocator_AddRef`, `geolocator_Release`:
//!   `geolocator_query_interface`, `geolocator_add_ref`, `geolocator_release`
//! - The `weakref.c` control block: strong and weak counts, `weak_reference_Resolve`,
//!   `GetWeakReference`, and the `IWeakReference` and `IWeakReferenceSource` entries
//! - Stub members: `E_NOTIMPL` for every member except `add_PositionChanged`, which returns
//!   `S_OK`: `stub_hresult`, `Member`
//!
//! Skipped:
//! - `calloc` and `free`: modelled by the `Heap` counts and liveness flags.
//! - `HSTRING` and `WindowsGetStringRawBuffer`: the class name is a `&str`, and a NULL HSTRING
//!   reads as empty.
//! - `widl` types and vtable layout: `Iid` lists the IIDs the port distinguishes, and `Iface`
//!   stands in for interface pointers.
//! - `Interlocked*` and the compare-exchange loop in `weak_reference_Resolve`: plain arithmetic,
//!   since the model is single-threaded.
//! - `TRACE` and `FIXME` logging.
//! - The location service and the async and event objects: the stub members never reach them.
//! - `CONTAINING_RECORD` pointer arithmetic: objects are addressed by index.

mod dll;
mod factory;
mod geolocator;
mod heap;
mod hresult;
mod iid;
mod stub;
mod weakref;

pub use dll::{dll_can_unload_now, dll_get_activation_factory, dll_get_class_object};
pub use factory::{
    factory_activate_instance, factory_add_ref, factory_query_interface, factory_release, Factory,
};
pub use geolocator::{geolocator_add_ref, geolocator_query_interface, geolocator_release};
pub use heap::{Alloc, Heap, Iface, Instance};
pub use hresult::{
    failed, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, S_FALSE,
    S_OK,
};
pub use iid::{inspectable_base, Iid};
pub use stub::{stub_hresult, Member};
pub use weakref::{
    weak_reference_add_ref, weak_reference_query_interface, weak_reference_release,
    weak_reference_resolve, weak_reference_source_add_ref,
    weak_reference_source_get_weak_reference, weak_reference_source_init,
    weak_reference_source_query_interface, weak_reference_source_release,
    weak_reference_strong_add_ref, weak_reference_strong_release,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
