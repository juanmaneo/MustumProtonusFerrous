//! Port of the portable logic in Wine's `dlls/dataexchange/main.c`.
//!
//! Ported:
//! - `dataexchange_QueryInterface` IID dispatch. The statics and interop tables reach
//!   the same code through their IInspectable forwarders: [`DataExchange::query_interface`],
//!   [`factory_interface`]
//! - `dataexchange_AddRef` and `dataexchange_Release`: [`DataExchange::add_ref`],
//!   [`DataExchange::release`]
//! - `DllGetActivationFactory`: [`dll_get_activation_factory`]
//! - `DllGetClassObject`, which only returns `CLASS_E_CLASSNOTAVAILABLE`: [`dll_get_class_object`]
//! - `core_dragdrop_manager_QueryInterface`, `_AddRef` and `_Release`:
//!   [`CoreDragDropManager::query_interface`], [`CoreDragDropManager::add_ref`],
//!   [`CoreDragDropManager::release`]
//! - `create_core_dragdrop_manager`, the window precheck and the reference handling
//!   around the query: [`create_core_dragdrop_manager`], [`Window`]
//! - `dragdrop_manager_interop_GetForWindow`: [`DataExchange::get_for_window`]
//! - `HRESULT_FROM_WIN32`: [`hresult_from_win32`]
//! - `FIXME` stubs, modelled by their return values. Factory, statics and manager
//!   methods return `E_NOTIMPL`. `add_TargetRequested` returns `S_OK` and token
//!   `0xdeadbeef` and does not store the handler.
//!
//! Skipped:
//! - `IsWindow` and `GetDesktopWindow`: the handle is passed in as a [`Window`] classification.
//! - `calloc` failure: passed in as the `allocation_ok` flag.
//! - Vtable plumbing and the `DEFINE_IINSPECTABLE` forwarders. The GUID bytes are not in
//!   `dlls/dataexchange`, so IIDs are symbolic ([`Iid`]).
//! - `manager->hwnd`: Wine writes it and never reads it.
//! - Out-pointers that the stubs never write (`HSTRING`, `IID **`, `TrustLevel`,
//!   `ICoreDragDropManager **`).
//! - `free`: Rust drops the manager when its count reaches zero.

mod factory;
mod hresult;
mod iid;
mod manager;
mod refcount;

pub use factory::{dll_get_activation_factory, dll_get_class_object, DataExchange};
pub use hresult::{
    hresult_from_win32, Hresult, CLASS_E_CLASSNOTAVAILABLE, ERROR_INVALID_WINDOW_HANDLE,
    E_ACCESSDENIED, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, S_OK,
};
pub use iid::{factory_interface, FactoryInterface, Iid};
pub use manager::{create_core_dragdrop_manager, CoreDragDropManager, Window};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
