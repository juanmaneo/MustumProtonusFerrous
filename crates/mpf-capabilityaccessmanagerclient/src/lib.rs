//! Port of the portable logic in Wine's `dlls/capabilityaccessmanagerclient/`
//! (`capability.c`, `main.c`).
//!
//! Ported:
//! - `DllGetActivationFactory` class-name dispatch, with `wcscmp` semantics (the name
//!   ends at its first NUL): [`dll_get_activation_factory`]
//! - `DllGetClassObject`, which only returns `CLASS_E_CLASSNOTAVAILABLE`: [`dll_get_class_object`]
//! - `QueryInterface` identification, reference counting and `E_NOINTERFACE` for the
//!   factory/statics object ([`CapabilityStatics`]) and for `AppCapability`
//!   ([`AppCapability`])
//! - `cap_create` (name duplication, initial count 1), `get_CapabilityName`,
//!   `CheckAccess` (always `Allowed`), `add_AccessChanged` (always the dummy cookie),
//!   `remove_AccessChanged` (always `S_OK`)
//! - Every `FIXME` stub, which returns `E_NOTIMPL`
//!
//! Skipped:
//! - The `calloc` failure path of `cap_create` and its `E_OUTOFMEMORY` return: allocation
//!   failure cannot be induced from safe Rust.
//! - `DllCanUnloadNow`: the `.spec` file exports it, but Wine has no C body. Winebuild
//!   emits a stub that raises `EXCEPTION_WINE_STUB` and never returns, so there is no
//!   return value to model.
//! - COM vtables, HSTRING allocation and the IInspectable outer object. Names are
//!   `Vec<u16>` (an empty name is the NULL HSTRING), `QueryInterface` returns an
//!   [`Interface`] tag in place of `*out`, and there are no out pointers.

mod capability;
mod com;
mod dll;
mod statics;

pub use capability::{AppCapability, AppCapabilityAccessStatus, EventRegistrationToken};
pub use com::{
    Guid, Hresult, Interface, CLASS_E_CLASSNOTAVAILABLE, E_NOINTERFACE, E_NOTIMPL, S_OK,
};
pub use dll::{dll_get_activation_factory, dll_get_class_object};
pub use statics::CapabilityStatics;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
