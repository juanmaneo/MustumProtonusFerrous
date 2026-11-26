//! Port of the portable logic in Wine's `dlls/windowscodecsext/main.c` and `dlls/windowscodecsext/windowscodecsext.spec`.
//!
//! Ported:
//! - `DllGetClassObject`: [`dll_get_class_object`]
//! - `WICCreateColorTransform_Proxy` control flow, with the COM calls behind [`ColorTransformHost`]:
//!   [`wic_create_color_transform_proxy`]
//! - `IWICColorTransform_Initialize_Proxy_W` forwarding, with the vtable call behind
//!   [`ColorTransformVtable`]: [`initialize_proxy_w`]
//! - `windowscodecsext.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The `HRESULT` values the bodies return or test: [`hresult`]
//!
//! Skipped:
//! - The `FIXME` and `TRACE` output, including the `debugstr_guid` formatting of the arguments.
//! - The COM runtime (`CoInitializeEx`, `CoCreateInstance`, `CoUninitialize`) and the WIC
//!   interfaces. The port calls them through the host and vtable traits.
//! - The NULL `iface` case of `IWICColorTransform_Initialize_Proxy_W`: the port takes a
//!   reference, and the C body would dereference the NULL receiver.
//! - `tests/transform.c`, which `dlls/windowscodecsext/Makefile.in` does not build.
//! - `@ stub` exports: the spec has none, so no `__wine_spec_unimplemented_stub` model.

mod exports;
mod hresult;
mod proxy;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use hresult::{
    failed, succeeded, CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL,
    E_OUTOFMEMORY, HRESULT, RPC_E_CHANGED_MODE, S_OK,
};
pub use proxy::{
    dll_get_class_object, initialize_proxy_w, wic_create_color_transform_proxy, ColorTransformHost,
    ColorTransformVtable,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
