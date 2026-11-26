//! Port of the portable logic in Wine's `dlls/ia2comproxy/ia2comproxy.spec`, `Makefile.in` and the
//! `uuid(...)` attribute of `ia2_classes.idl`.
//!
//! The directory has no C sources. Its four exports are generated at build time by the midl
//! proxy code (`DLLDATA_ROUTINES` in `include/rpcproxy.h`), so this port is partial and models
//! only the data the reference defines. None of the exports is an `@ stub`, so none is modelled
//! with `wine_spec_unimplemented_stub`.
//!
//! Ported:
//! - `ia2comproxy.spec` export table: [`EXPORTS`], [`Export`], [`CallingConvention`], [`find_export`]
//! - `PSFactoryBuffer` coclass CLSID: [`CLSID_PS_FACTORY_BUFFER`], [`Guid`] and its IDL text form
//! - `Makefile.in` module constants: [`MODULE`], [`IMPORTS`], [`DLLDATA_EXTRADEFS`], [`is_import`],
//!   [`has_dlldata_define`]
//!
//! Skipped:
//! - The bodies of `DllGetClassObject`, `DllCanUnloadNow`, `DllRegisterServer` and
//!   `DllUnregisterServer`: they are generated into `dlldata.c` at build time and are not in
//!   `dlls/ia2comproxy`.
//! - The proxy and stub code that widl generates from `ia2_classes.idl`.
//! - The `threading(both)` attribute on the coclass, which is a widl attribute.

mod exports;
mod guid;
mod module;

pub use exports::{find_export, CallingConvention, Export, EXPORTS};
pub use guid::{Guid, CLSID_PS_FACTORY_BUFFER};
pub use module::{has_dlldata_define, is_import, DLLDATA_EXTRADEFS, IMPORTS, MODULE};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
