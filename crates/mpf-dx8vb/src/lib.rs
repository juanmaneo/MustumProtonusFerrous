//! Port of the portable logic in Wine's `dlls/dx8vb/main.c` and `dlls/dx8vb/dx8vb.spec`.
//!
//! Ported:
//! - `DllGetClassObject` dispatch (riid check, class lookup, allocation result): [`dll_get_class_object`]
//! - Class factory `QueryInterface`, `AddRef`, `Release` and `LockServer`: [`ClassFactory`], [`lock_server`]
//! - The 135 `@ stub` exports of `dx8vb.spec`, modelled on `__wine_spec_unimplemented_stub`:
//!   [`STUB_EXPORTS`], [`unimplemented_stub`]
//!
//! Skipped:
//! - `classfactory_CreateInstance`: the object table holds only `GUID_NULL`, whose creation
//!   callback is NULL, so there is no object constructor to model.
//! - `DllCanUnloadNow`, `DllRegisterServer`, `DllUnregisterServer`: declared `-private` in the
//!   spec, with no body in `main.c`, so no return value can be derived.
//! - `GetProcessHeap`, `HeapAlloc`, `HeapFree`, `Interlocked*`: OS calls. The allocation result
//!   is passed in as a bool.

mod class_factory;
mod stubs;

pub use class_factory::{
    dll_get_class_object, lock_server, ClassFactory, Guid, HResult, CLASS_E_CLASSNOTAVAILABLE,
    E_NOINTERFACE, E_OUTOFMEMORY, GUID_NULL, IID_CLASS_FACTORY, IID_UNKNOWN, S_OK,
};
pub use stubs::{
    unimplemented_stub, StubException, EXCEPTION_WINE_STUB, STUB_EXPORTS, WINE_SPEC_FILE_NAME,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
