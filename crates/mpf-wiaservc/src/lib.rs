//! Port of the portable logic in Wine's `dlls/wiaservc/factory.c`, `wiadevmgr.c`, `service.c` and
//! `wiaservc.spec`.
//!
//! Ported:
//! - the class factory registered for `CLSID_WiaDevMgr`: [`ClassFactory`]
//! - the `IWiaDevMgr` and `IEnumWIA_DEV_INFO` objects with their reference counts and `E_NOTIMPL`
//!   methods: [`WiaDevMgr`], [`EnumWiaDevInfo`]
//! - the service entry point: [`Service`], [`ServiceHost`], [`ServiceStatus`], covering
//!   `ServiceMain`, `UpdateStatus`, `ServiceHandler` and `StartCount`
//! - the `wiaservc.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - the `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`], one helper per stub
//!   export, and [`STUB_EXPORTS`]
//!
//! Skipped:
//! - The `TRACE`, `FIXME` and `ERR` output and the `debugstr_*` helpers.
//! - The `HeapAlloc` and `HeapFree` heap: allocation success is an input, and freeing an object at
//!   zero references is not modelled.
//! - The COM vtables, `CONTAINING_RECORD` and the `IUnknown` pointer casts.
//! - Arguments the C bodies never read: the `EnumDeviceInfo` flag, the `Next` element array, the
//!   `Clone` output, the `ServiceMain` argv, and the `ServiceHandler` event type, data and context.
//!   The `E_NOTIMPL` return values and the `*fetched` and `*count` writes are modelled.
//! - The Win32 and COM calls themselves: [`ServiceHost`] stands in for them.
//! - `DllRegisterServer` and `DllUnregisterServer`: `wiaservc.spec` exports them, but no C body
//!   exists in the reference directory.

mod com;
mod devmgr;
mod enum_dev_info;
mod exports;
mod factory;
mod service;
mod stubs;

pub use com::{
    Guid, Hresult, CLASS_E_NOAGGREGATION, CLSID_WIADEVMGR, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY,
    IID_ICLASSFACTORY, IID_IENUMWIA_DEV_INFO, IID_IUNKNOWN, IID_IWIADEVMGR,
};
pub use devmgr::WiaDevMgr;
pub use enum_dev_info::EnumWiaDevInfo;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use factory::ClassFactory;
pub use service::{Service, ServiceHost, ServiceStatus};
pub use stubs::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
