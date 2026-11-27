//! Port of the portable logic in Wine's `dlls/actxprxy/usrmarshal.c` and of the `DllCanUnloadNow`
//! export that `dlls/actxprxy/Makefile.in` generates through `include/rpcproxy.h`.
//!
//! Ported:
//! - `IEnumShellItems_Next_Proxy` and `IEnumObjects_Next_Proxy`: [`enum_next_proxy`]
//! - `IEnumShellItems_Next_Stub`: [`enum_shell_items_next_stub`]
//! - `IEnumObjects_Next_Stub`: [`enum_objects_next_stub`]
//! - `DllCanUnloadNow`, which returns `NdrDllCanUnloadNow(&gPFactory)` from `dlls/rpcrt4/cpsf.c`:
//!   [`dll_can_unload_now`]
//! - The `HRESULT` values the bodies compare against: [`Hresult`], [`S_OK`], [`S_FALSE`]
//!
//! Skipped:
//! - The `TRACE` calls, and the `rgelt` and `riid` pointers, which the C bodies pass through
//!   without reading.
//! - The pointer-forwarding proxies and stubs (`IServiceProvider_QueryService`,
//!   `ICatInformation_*`, `IPrint_Print`, `IEnumOleDocumentViews_Next`, `IModalWindow_Show`,
//!   `IFolderView2_GetGroupBy`, `IParentAndItem_GetParentAndItem`). Each is a cast and a call: into
//!   a widl-generated `*_Remote*` function in the proxy direction, or into the server's interface
//!   method in the stub direction. The generated sources are not in `dlls/actxprxy`, and the
//!   interface methods belong to the implementing server.
//! - `GetProxyDllInfo`, `DllGetClassObject`, `DllRegisterServer` and `DllUnregisterServer`. They are
//!   generated from `include/rpcproxy.h` and point at the generated proxy tables, at
//!   `NdrDllGetClassObject`, or at `__wine_register_resources`.
//! - `actxprxy.spec` has no `@ stub` entries, so no `__wine_spec_unimplemented_stub` record is
//!   modelled.

mod dlldata;
mod usrmarshal;

pub use dlldata::dll_can_unload_now;
pub use usrmarshal::{
    enum_next_proxy, enum_objects_next_stub, enum_shell_items_next_stub, Hresult, NextFn, S_FALSE,
    S_OK,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
