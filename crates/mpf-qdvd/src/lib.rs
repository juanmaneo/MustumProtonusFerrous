//! Port of the portable logic in Wine's `dlls/qdvd/`: `qdvd_main.c` (`DllGetClassObject` and the
//! class factories), `graph.c` (the DVD graph builder) and `navigator.c` (the navigator's
//! `QueryInterface` hook, its pin table, and its method tables).
//!
//! Ported:
//! - `DllGetClassObject` and the class factory's `QueryInterface`, `AddRef`, `Release`,
//!   `LockServer`, and the `outer` check of `CreateInstance`: [`dll_get_class_object`],
//!   [`class_factory_query_interface`], [`class_factory_add_ref`], [`class_factory_release`],
//!   [`class_factory_lock_server`], [`class_factory_create_instance_gate`]
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`, the `winecrt0` forwarders
//!   that `qdvd.spec` exports: [`dll_can_unload_now`], [`dll_register_server`],
//!   [`dll_unregister_server`]
//! - The graph builder's reference counts and `QueryInterface`: [`graph::GraphBuilder`]
//! - `navigator_query_interface` and `navigator_get_pin`: [`navigator_query_interface`],
//!   [`navigator_get_pin`]
//! - The explicit C functions that return `E_NOTIMPL`: [`graph::get_filtergraph`],
//!   [`graph::get_dvd_interface`], [`graph::render_dvd_video_volume`], the 40 `IDvdControl2`
//!   methods in [`control2`], and the 39 `IDvdInfo2` methods in [`info2`]
//!
//! `qdvd.spec` has no `@ stub` exports, so no `__wine_spec_unimplemented_stub` model is needed.
//!
//! Skipped:
//! - Aggregation. The graph builder is created with its own inner `IUnknown` as the outer object,
//!   so the `outer` argument is only the flag that the gate reads.
//! - The `strmbase` base class, `navigator_create` and `navigator_destroy`. They allocate and wire
//!   the filter, and the port has no allocator.
//! - The `calloc` failure path (`E_OUTOFMEMORY`) and the `free` on the last release.
//! - The bodies of `__wine_register_resources` and `__wine_unregister_resources`, which the caller
//!   supplies as closures.
//! - The `TRACE`, `WARN` and `FIXME` logging, and the `debugstr_*` formatting.

pub mod control2;
pub mod graph;
pub mod info2;

mod class_factory;
mod dll;
mod guid;
mod hresult;
mod navigator;

pub use class_factory::{
    class_factory_add_ref, class_factory_create_instance_gate, class_factory_lock_server,
    class_factory_query_interface, class_factory_release, ClassFactory,
};
pub use dll::{
    dll_can_unload_now, dll_get_class_object, dll_register_server, dll_unregister_server,
};
pub use guid::{
    is_equal_guid, Guid, CLSID_DVDGRAPHBUILDER, CLSID_DVDNAVIGATOR, IID_ICLASSFACTORY,
    IID_IDVDCONTROL2, IID_IDVDGRAPHBUILDER, IID_IDVDINFO2, IID_IUNKNOWN,
};
pub use hresult::{
    succeeded, Hresult, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_NOINTERFACE, E_NOTIMPL, S_FALSE, S_OK,
};
pub use navigator::{navigator_get_pin, navigator_query_interface, NavigatorInterface};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
