//! Port of the portable logic in Wine's dlls/msxml6/msxml6.spec, dlls/msxml6/msxml6_tlb.idl and
//! include/msxml6.idl, which msxml6_tlb.idl includes.
//!
//! Ported:
//! - the msxml6.spec export table: [EXPORTS], [Export], [ExportKind], [export]
//! - the MSXML2 library record and its 12 coclass records from include/msxml6.idl:
//!   [TYPELIB], [TypeLib], [COCLASSES], [CoClass], [Threading], [coclass]
//! - the GUID layout and its lowercase uuid text form: [Guid]
//!
//! Skipped:
//! - The bodies of DllCanUnloadNow, DllRegisterServer and DllUnregisterServer. Each is
//!   stdcall -private with no forward target, and this reference has no C source for it, so no
//!   return value or stub record is modelled. The spec has no @ stub line.
//! - The body of the forwarded DllGetClassObject, which lives in dlls/msxml3, another component.
//! - Typelib registration from rsrc.rc and msxml6.manifest, and the VER_* values in Makefile.in.
//! - The interfaces, enums and dispinterfaces in include/msxml6.idl, and the helpstring and
//!   version attributes of the coclasses.

mod exports;
mod guid;
mod typelib;

pub use exports::{export, Export, ExportKind, EXPORTS};
pub use guid::Guid;
pub use typelib::{coclass, CoClass, Threading, TypeLib, COCLASSES, TYPELIB};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
