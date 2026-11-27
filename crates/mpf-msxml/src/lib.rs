//! Port of the portable logic in Wine's `dlls/msxml/msxml.spec`, `dlls/msxml/msxml.rgs` and
//! `dlls/msxml/Makefile.in`, and of the forward helpers in `tools/winebuild/import.c`.
//!
//! Ported:
//! - the `msxml.spec` export table: [`EXPORTS`], [`Export`], [`export`]
//! - the `MODULE` name from `Makefile.in`: [`MODULE_NAME`]
//! - the forward split and file name that `winebuild` applies to `msxml3.DllGetClassObject`:
//!   [`split_forward`], [`get_dll_name`]
//! - the `ForceRemove` class keys of `msxml.rgs`: [`COM_CLASSES`], [`ComClass`], [`com_class`]
//!
//! Skipped:
//! - The bodies of `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`. The spec binds
//!   them to functions of the same name, and `dlls/msxml` has no C sources that define them.
//! - The body of `DllGetClassObject`, which the spec forwards to `msxml3`.
//! - The `ForceRemove` semantics of `msxml.rgs` (remove the key, then add it) and the registrar.
//! - `msxml_tlb.idl` and `rsrc.rc`: type library and resource inputs with no runtime logic.
//! - Ordinal assignment (`assign_ordinals` in `tools/winebuild/parser.c`). The spec has no explicit
//!   ordinals, so the table records none.
//! - `__wine_spec_unimplemented_stub`. The spec has no `@ stub` exports.

mod exports;
mod forward;
mod registry;

pub use exports::{export, Export, EXPORTS, MODULE_NAME};
pub use forward::{get_dll_name, split_forward};
pub use registry::{com_class, ComClass, COM_CLASSES};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
