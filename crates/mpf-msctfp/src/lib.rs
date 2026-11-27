//! Port of the portable logic in Wine's `dlls/msctfp/msctfp.spec`, `dlls/msctfp/msctfp.idl`,
//! `include/rpcproxy.h` (the proxy DLL macros) and `libs/winecrt0/register.c`.
//!
//! Ported:
//! - `msctfp.spec` export table: [`EXPORTS`], [`Export`], [`find_export`], [`MODULE`]
//! - The `PSFactoryBuffer` CLSID from `msctfp.idl` and the CLSID that `GetProxyDllInfo` returns:
//!   [`CLSID_PSFACTORYBUFFER`], [`Guid`], [`get_proxy_dll_info`]
//! - `DllMain` from `REGISTER_PROXY_DLL_ROUTINES`: [`dll_main`], [`ProxyDll`], [`DLL_PROCESS_ATTACH`]
//! - The enumeration result behind `DllRegisterServer` and `DllUnregisterServer`:
//!   [`enumerate_registry_resources`], [`CallbackOutcome`], [`dll_register_server`],
//!   [`dll_unregister_server`], [`Hresult`], [`S_OK`]
//!
//! Skipped:
//! - The midl-generated `ProxyFileInfo` list and `CStdPSFactoryBuffer` (`dlldata.c` and
//!   `msctfp_p.c` are not in the reference), so [`get_proxy_dll_info`] returns only the CLSID.
//! - `DllGetClassObject` and `DllCanUnloadNow`. They forward to rpcrt4's `NdrDllGetClassObject`
//!   and `NdrDllCanUnloadNow` with the generated factory, and neither body is in the reference.
//! - The body of `register_resource`: `FindResourceW`, `LoadResource`, the ATL registrar and
//!   `IRegistrar_StringRegister` / `IRegistrar_StringUnregister`. Each callback outcome is an input.
//! - `NdrDllRegisterProxy` and `NdrDllUnregisterProxy`, which only build without
//!   `WINE_REGISTER_DLL`. `dlls/msctfp/Makefile.in` defines it.
//! - `@ stub` handling. msctfp has no `@ stub` exports, so there is no stub exception model.

mod dll_main;
mod exports;
mod proxy;
mod register;

pub use dll_main::{dll_main, ProxyDll, DLL_PROCESS_ATTACH};
pub use exports::{find_export, Export, EXPORTS, MODULE};
pub use proxy::{get_proxy_dll_info, Guid, CLSID_PSFACTORYBUFFER};
pub use register::{
    dll_register_server, dll_unregister_server, enumerate_registry_resources, CallbackOutcome,
    Hresult, S_OK,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
