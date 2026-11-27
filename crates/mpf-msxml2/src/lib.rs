//! Port of the portable logic in Wine's `dlls/msxml2/msxml2.spec` and
//! `dlls/msxml2/msxml2_tlb.idl`.
//!
//! Ported:
//! - `msxml2.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`] and
//!   [`ExportKind::forward_target`] for the `DllGetClassObject` forwarder
//! - The `uuid(...)` attributes of `msxml2_tlb.idl`: the `LIBID_MSXML2`, `IID_`, `DIID_` and
//!   `CLSID_` constants, [`TYPELIB_GUIDS`], [`typelib_guid`], and [`Guid`] with
//!   [`Guid::to_braced_string`]
//! - The `DOMNodeType` enum of `msxml2_tlb.idl`: [`DOM_NODE_TYPE`] with its `NODE_*`
//!   constants, [`Member`], [`member_value`] and [`member_name`]
//!
//! Skipped:
//! - The bodies of `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`: the spec
//!   exports them, but `dlls/msxml2` has no C source for them. The table records them as
//!   [`ExportKind::Undefined`] only.
//! - The `DllGetClassObject` target `msxml3.DllGetClassObject`, which lives in another DLL.
//! - The coclass, interface and dispinterface bodies of the IDL (methods, properties,
//!   `helpstring`, `version(2.6)`, `importlib` and `import`), since they declare no code.
//! - The DISPIDs of the dispinterface: they come from `msxml2did.h` and `idispids.h`, which
//!   are not in `dlls/msxml2`.

mod enums;
mod exports;
mod guid;
mod typelib;

pub use enums::*;
pub use exports::{export_kind, ExportKind, EXPORTS};
pub use guid::Guid;
pub use typelib::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
