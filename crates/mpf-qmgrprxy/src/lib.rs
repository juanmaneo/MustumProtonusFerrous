//! Port of the portable logic in Wine's `dlls/qmgrprxy/qmgrprxy.spec`, `dlls/qmgrprxy/qmgrprxy.idl`
//! and `dlls/qmgrprxy/Makefile.in`, and of the proxy DLL macros in `include/rpcproxy.h`.
//!
//! The DLL has no `.c` sources. Its code is the `dlldata.c` that widl generates, which expands the
//! `include/rpcproxy.h` macros, plus `__wine_register_resources` from `libs/winecrt0/register.c`.
//!
//! Ported:
//! - `qmgrprxy.spec` export table: [`EXPORTS`], [`Export`], [`export`], [`args_size`]
//! - `Makefile.in` and `qmgrprxy.idl` values: [`MODULE`], [`IMPORTS`], [`Guid`],
//!   [`PS_FACTORY_BUFFER_CLSID`], [`guid_string`]
//! - `DllMain` from `REGISTER_PROXY_DLL_ROUTINES`: [`dll_main`], [`DllMainOutcome`],
//!   [`DLL_PROCESS_ATTACH`]
//! - `DllRegisterServer` and `DllUnregisterServer` routing under `WINE_REGISTER_DLL`:
//!   [`register_route`], [`RegisterRoute`]
//! - `__wine_register_resources` and `__wine_unregister_resources` with no `WINE_REGISTRY`
//!   resources: [`wine_register_resources`], [`wine_unregister_resources`], [`Hresult`], [`S_OK`],
//!   [`TRUE`]
//!
//! Skipped:
//! - `DllGetClassObject`, `DllCanUnloadNow` and `GetProxyDllInfo`. They forward to rpcrt4's
//!   `NdrDllGetClassObject` and `NdrDllCanUnloadNow` with the proxy file list that widl generates
//!   from the interfaces in `bits*.idl`. Neither the generated list nor rpcrt4 is ported. These
//!   exports appear only in [`EXPORTS`].
//! - `NdrDllRegisterProxy` and `NdrDllUnregisterProxy`, the [`RegisterRoute::NdrProxy`] route.
//!   `dlldata_EXTRADEFS = -DWINE_REGISTER_DLL` in `Makefile.in` selects the other route.
//! - The `register_resource` callback. It runs the ATL registrar for each `WINE_REGISTRY`
//!   resource, and this DLL has none.
//! - The `PROXY_CLSID` and `GET_DLL_CLSID` selection, and the generated proxy and stub code for
//!   the `bits*.idl` interfaces.

mod dll;
mod exports;
mod module;

pub use dll::{
    dll_main, register_route, wine_register_resources, wine_unregister_resources, DllMainOutcome,
    Hresult, RegisterRoute, DLL_PROCESS_ATTACH, S_OK, TRUE,
};
pub use exports::{args_size, export, Export, EXPORTS};
pub use module::{guid_string, Guid, IMPORTS, MODULE, PS_FACTORY_BUFFER_CLSID};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
