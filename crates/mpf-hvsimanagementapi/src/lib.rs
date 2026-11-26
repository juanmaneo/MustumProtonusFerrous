//! Port of the portable logic in Wine's `dlls/hvsimanagementapi/` (`main.c`, `host.c` and
//! `hvsimanagementapi.spec`).
//!
//! Ported:
//! - `DllGetClassObject` (`main.c`): [`dll_get_class_object`], [`CLASS_E_CLASSNOTAVAILABLE`]
//! - `DllGetActivationFactory` class-name dispatch (`main.c`): [`dll_get_activation_factory`]
//! - The static `isolated_host_statics` object (`host.c`): [`IsolatedHostStatics`], [`Iid`],
//!   [`Interface`]. This covers the `factory_QueryInterface` IID dispatch, `factory_AddRef` and
//!   `factory_Release`, and `isolated_host_statics_get_IsReady`.
//! - The `FIXME ... stub!` methods that return `E_NOTIMPL` (`host.c`): [`StubMethod`],
//!   [`stub_method`]
//! - The `HRESULT` values the port returns: [`Hresult`], [`S_OK`], [`E_NOINTERFACE`], [`E_NOTIMPL`]
//!
//! Skipped:
//! - `DllCanUnloadNow`: `-private` in `hvsimanagementapi.spec`, with no definition in this
//!   directory.
//! - `@ stub` exports: there are none. All three spec exports are `-private`, so the
//!   `__wine_spec_unimplemented_stub` model is not needed.
//! - The `FIXME` traces, and the `clsid`, `riid` and `out` arguments of `DllGetClassObject`,
//!   which the C body never reads or writes.
//! - COM plumbing: vtables and the `CONTAINING_RECORD` casts. The statics object's
//!   `IInspectable` methods forward to the factory (`DEFINE_IINSPECTABLE` in `private.h`), so
//!   they share the factory's functions. Out-pointers are modelled as return values.
//! - HSTRING handling: the class name is passed as a UTF-16 slice, which ends at the first NUL
//!   as `wcscmp` does.
//! - Atomicity: `InterlockedIncrement` and `InterlockedDecrement` become plain arithmetic.

mod entry;
mod host;

pub use entry::{dll_get_activation_factory, dll_get_class_object, CLASS_E_CLASSNOTAVAILABLE};
pub use host::{
    stub_method, Hresult, Iid, Interface, IsolatedHostStatics, StubMethod, E_NOINTERFACE,
    E_NOTIMPL, S_OK,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
