//! Port of the portable logic in Wine's `dlls/cdosys/main.c`, plus the
//! return values of the exports that `cdosys.spec` lists but `main.c` does not define.
//!
//! Ported:
//! - `DllGetClassObject`: always `CLASS_E_CLASSNOTAVAILABLE`, ignoring its
//!   arguments and never writing `obj`: [`dll_get_class_object`], [`CLASS_E_CLASSNOTAVAILABLE`]
//! - `DllCanUnloadNow` (`libs/winecrt0/dll_canunload.c`): always `S_FALSE`: [`dll_can_unload_now`]
//! - `DllRegisterServer` and `DllUnregisterServer` (`libs/winecrt0/dll_register.c`):
//!   return `S_OK`, because cdosys has no `WINE_REGISTRY` resources to register:
//!   [`dll_register_server`], [`dll_unregister_server`]
//!
//! Skipped:
//! - `debugstr_guid` and the `FIXME` trace in `DllGetClassObject`: logging only.
//! - `__wine_register_resources` / `__wine_unregister_resources` for non-empty resource
//!   sets: the ATL registrar path in winecrt0 is COM and OS work, and cdosys never reaches it.
//! - `__wine_spec_unimplemented_stub`: `cdosys.spec` has no `stub` entries.

mod class_object;
mod registration;

pub use class_object::{dll_get_class_object, Hresult, CLASS_E_CLASSNOTAVAILABLE};
pub use registration::{
    dll_can_unload_now, dll_register_server, dll_unregister_server, S_FALSE, S_OK,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
