//! Port of the portable logic in Wine's `dlls/ctapi32/ctapi32.c` and `unixlib.c`.
//!
//! Ported:
//! - `CT_init`, `CT_data`, `CT_close` and the `IS8` return narrowing: [`ct_init`], [`ct_data`], [`ct_close`]
//! - Unix call table (`enum ctapi_funcs`, native and WoW64 `ct_data` slot): [`UnixFunc`], [`Entry`], [`entry`], [`UNIX_FUNCS_COUNT`]
//! - Unix-side forwarding and `wow64_ct_data` pointer widening: [`unix_ct_init`], [`unix_ct_data`], [`unix_ct_close`], [`wow64_ct_data`]
//! - `attach` status codes and `CT_*` lookup order: [`attach`]; `detach` status: [`detach`]
//! - `load_functions` name selection (`library` value, `libctapi.so` fallback, `MAX_PATH` limit): [`registry_library`], [`narrow_name`], [`soname`], [`load_functions`]
//! - `DllMain` reason switch: [`dll_main`]
//!
//! Skipped:
//! - `dlopen`, `dlsym`, `dlclose` and the `libctapi.so` binary: the caller supplies their results.
//! - `RegOpenKeyExW`, `RegQueryValueExW`, `RegCloseKey`, `WideCharToMultiByte`: the caller supplies the value read.
//! - `__wine_init_unix_call`, `DisableThreadLibraryCalls`, the `WINE_UNIX_CALL` transport, `MESSAGE` and `TRACE` output.
//! - Dispatch through `__wine_unix_call_wow64_funcs`: only its slot table is modelled ([`entry`]).
//! - Soname buffer overflow: Wine writes a truncated, unterminated prefix and `dlopen` then reads past it, which is undefined. The port falls back to `libctapi.so`.

mod dllmain;
mod exports;
mod loader;
mod unixlib;

pub use dllmain::{dll_main, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
pub use exports::{ct_close, ct_data, ct_init};
pub use loader::{
    load_functions, narrow_name, registry_library, soname, FALLBACK_LIBCTAPI, MAX_PATH, REG_SZ,
};
pub use unixlib::{
    attach, detach, entry, unix_ct_close, unix_ct_data, unix_ct_init, wow64_ct_data, CtCloseParams,
    CtDataParams, CtInitParams, CtLibrary, Entry, NtStatus, UnixFunc, Wow64CtDataParams,
    STATUS_DLL_NOT_FOUND, STATUS_ENTRYPOINT_NOT_FOUND, STATUS_SUCCESS, UNIX_FUNCS_COUNT,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
