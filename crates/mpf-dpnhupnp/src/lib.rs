//! Port of the portable logic in Wine's `dlls/dpnhupnp/main.c`.
//!
//! Ported:
//! - `DirectPlayNATHelpCreate` (stub returning `E_NOTIMPL`): [`direct_play_nat_help_create`]
//! - `DllGetClassObject` (returns `E_FAIL`): [`dll_get_class_object`]
//! - `HRESULT` values used by those stubs: [`E_NOTIMPL`], [`E_FAIL`]
//! - Export list of `dpnhupnp.spec` with stdcall argument byte counts: [`stdcall_arg_bytes`]
//!
//! Skipped:
//! - `DllCanUnloadNow`, `DllRegisterServer`, `DllUnregisterServer`: listed in
//!   `dpnhupnp.spec` but not defined in `main.c`. Only their export entries are modelled.
//! - `FIXME` tracing and `WINE_DEFAULT_DEBUG_CHANNEL`.
//! - Writes through `ppv`: neither C body stores to it, so the port takes it as
//!   `Option<&mut usize>` and leaves it untouched.
//! - The `GUID` values are passed through unread; `Guid` is only a stand-in for `REFIID`.

mod exports;
mod stubs;

pub use exports::{stdcall_arg_bytes, EXPORTS};
pub use stubs::{
    direct_play_nat_help_create, dll_get_class_object, Guid, Hresult, E_FAIL, E_NOTIMPL,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
