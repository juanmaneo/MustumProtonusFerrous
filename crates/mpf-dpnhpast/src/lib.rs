//! Port of the portable logic in Wine's `dlls/dpnhpast/main.c`, plus the
//! `libs/winecrt0` default entry points that its `-private` spec exports link to.
//!
//! Ported:
//! - `DirectPlayNATHelpCreate` (stub, returns `E_NOTIMPL`): [`direct_play_nat_help_create`]
//! - `DllGetClassObject` (stub, returns `E_FAIL`): [`dll_get_class_object`]
//! - `DllCanUnloadNow` (`libs/winecrt0/dll_canunload.c`, returns `S_FALSE`): [`dll_can_unload_now`]
//! - `DllRegisterServer` / `DllUnregisterServer` (`libs/winecrt0/dll_register.c`), which
//!   wrap `__wine_register_resources` / `__wine_unregister_resources` (`libs/winecrt0/register.c`).
//!   The result fold over per-resource outcomes: [`dll_register_server`], [`dll_unregister_server`]
//! - `SUCCEEDED`: [`succeeded`]
//!
//! Skipped:
//! - `EnumResourceNamesW`, `FindResourceW`, `LoadResource`, `SizeofResource`, `HeapAlloc`:
//!   OS resource and heap access. The caller supplies each resource's outcome as a [`ResourceStep`].
//! - `AtlCreateRegistrar` and `IRegistrar_*`: the ATL COM registrar. Its HRESULTs enter as [`ResourceStep`] values.
//! - `MultiByteToWideChar` (UTF-8 script decoding) and `GetModuleFileNameW`.
//! - COM vtables and `DllMain`, which dpnhpast does not define.
//!
//! dpnhpast ships no `WINE_REGISTRY` resources, so its real `DllRegisterServer`
//! returns `S_OK` (oracle row 07).

mod class_object;
mod hresult;
mod nat_help;
mod registration;

pub use class_object::{dll_can_unload_now, dll_get_class_object};
pub use hresult::{
    succeeded, Hresult, E_FAIL, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, S_FALSE, S_OK,
};
pub use nat_help::direct_play_nat_help_create;
pub use registration::{dll_register_server, dll_unregister_server, ResourceStep};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
