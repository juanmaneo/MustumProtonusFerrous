//! Port of the portable logic in Wine's `dlls/msxml4/msxml4.spec`, `dlls/msxml4/msxml4_tlb.idl`,
//! `dlls/msxml4/msxml4.manifest`, `dlls/msxml4/Makefile.in` and `dlls/msxml4/rsrc.rc`.
//!
//! The directory has no `.c` files, so the port models the constants and the export table that
//! these files define.
//!
//! Ported:
//! - `msxml4.spec` export table: [`EXPORTS`], [`ExportEntry`], [`ExportTarget`], [`export_entry`],
//!   including the `DllGetClassObject` forward to `msxml3`
//! - `msxml4_tlb.idl` library and coclass constants: [`Guid`], [`LIBID_MSXML2`], [`COCLASSES`],
//!   [`coclass_clsid`]
//! - `msxml4.manifest` assembly identity: [`assembly_identity`], [`AssemblyIdentity`]
//! - `Makefile.in` module values and the `rsrc.rc` manifest resource: [`MODULE_NAME`],
//!   [`FILE_DESCRIPTION`], [`PRODUCT_VERSION`], [`RT_MANIFEST`], [`MANIFEST_FILE`]
//!
//! Skipped:
//! - The bodies of `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`. The spec
//!   lists them as `-private` with no target, and this directory has no implementation for them.
//!   Only their export records are ported, and no behaviour is modelled.
//! - The `DllGetClassObject` body, which the spec forwards to `msxml3`. That DLL is not part of
//!   this crate.
//! - The typelib interfaces, enums and dispatch IDs in `msxml4_tlb.idl`.
//! - The `WINE_MANIFEST` macro. Its expansion is not in the tree checked, so only the type `24`
//!   and the file name it passes are ported.
//! - `@ stub` exports. The spec has none, so no `__wine_spec_unimplemented_stub` record is modelled.

mod exports;
mod guid;
mod resources;

pub use exports::{export_entry, ExportEntry, ExportTarget, EXPORTS};
pub use guid::{coclass_clsid, Guid, COCLASSES, LIBID_MSXML2};
pub use resources::{
    assembly_identity, AssemblyIdentity, FILE_DESCRIPTION, MANIFEST_FILE, MODULE_NAME,
    PRODUCT_VERSION, RT_MANIFEST,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
