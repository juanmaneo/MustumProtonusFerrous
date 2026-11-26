//! Port of the portable logic in Wine's `dlls/dhtmled.ocx/main.c` and `edit.c`.
//!
//! Ported:
//! - `dhtml_edit_qi` IID-to-interface table: [`Iid`], [`Target`], [`dhtml_edit_qi`]
//! - `dhtml_edit_create` and the object reference count: [`dhtml_edit_create`], [`ObjectRef`]
//! - `ClassFactory_*`: [`class_factory_query_interface`], [`class_factory_add_ref`],
//!   [`class_factory_release`], [`class_factory_create_instance`], [`class_factory_lock_server`]
//! - `DllMain` and `DllGetClassObject`: [`dll_main`], [`dll_get_class_object`]
//! - `MulDiv` (kernelbase) and the default control extent: [`mul_div`], [`default_extent`]
//! - `OleObject_SetExtent` / `OleObject_GetExtent`: [`object_set_extent`], [`object_get_extent`]
//! - Fixed returns of `OleObject_DoVerb`, `ViewObjectEx_GetViewStatus`, the connection point
//!   and service provider exports: [`ole_object_do_verb`], [`view_object_ex_get_view_status`],
//!   [`connection_point_container_find_connection_point`], [`connection_point_query_interface`],
//!   [`service_provider_query_service`]
//! - Stub exports that log FIXME and return `E_NOTIMPL` or `S_OK`: [`stub_result`]
//!
//! Skipped:
//! - `load_typelib`, `get_typeinfo`, `release_typelib`, and `DHTMLEdit_GetTypeInfo*`: ITypeLib
//!   loading through OS COM.
//! - `HeapAlloc` failure path (`E_OUTOFMEMORY`) and `GetDC` / `GetDeviceCaps`: OS calls. The DPI
//!   is an input to [`default_extent`].
//! - `OleObject_SetClientSite`, `GetClientSite`, and the other COM pointer-ownership paths.
//! - Per-export stub bodies: they are modelled by [`StubReturn`] kind, not one row per export.
//! - `ConnectionPoint_AddRef`/`Release`, `DataObject`, `ServiceProvider` vtable plumbing.

mod classfactory;
mod dll;
mod extent;
mod hresult;
mod object;
mod ole;
mod qi;
mod stubs;

pub use classfactory::{
    class_factory_add_ref, class_factory_create_instance, class_factory_lock_server,
    class_factory_query_interface, class_factory_release,
};
pub use dll::{
    dll_get_class_object, dll_main, Clsid, DllMainAction, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH,
};
pub use extent::{
    default_extent, mul_div, object_get_extent, object_set_extent, Sizel, DVASPECT_CONTENT,
};
pub use hresult::{
    is_success, HResult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, CONNECT_E_NOCONNECTION,
    DV_E_DVASPECT, E_FAIL, E_NOINTERFACE, E_NOTIMPL, E_POINTER, S_OK,
};
pub use object::{dhtml_edit_create, ObjectRef};
pub use ole::{
    connection_point_container_find_connection_point, connection_point_query_interface,
    ole_object_do_verb, service_provider_query_service, view_object_ex_get_view_status,
    OLEIVERB_INPLACEACTIVATE, VIEWSTATUS_OPAQUE, VIEWSTATUS_SOLIDBKGND,
};
pub use qi::{dhtml_edit_qi, Iid, Target};
pub use stubs::{stub_result, StubReturn};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
