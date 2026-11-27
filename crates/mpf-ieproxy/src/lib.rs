//! Port of the portable logic in Wine's `dlls/ieproxy/ieproxy.spec`, `dlls/ieproxy/Makefile.in`
//! and the `WINE_REGISTER_DLL` wrappers in `include/rpcproxy.h` that the spec exports.
//!
//! Ported:
//! - `ieproxy.spec` export table: [`EXPORTS`], [`Export`], [`find_export`], and the module name
//!   [`MODULE`] from `Makefile.in`
//! - winebuild's ordinal assignment for the `@` entries: [`assign_ordinals`], [`export_ordinal`],
//!   [`export_name`]
//! - the i386 `@N` stdcall suffix: [`i386_stdcall_bytes`]
//! - the callee that each wrapper returns, from `include/rpcproxy.h`: [`Export::delegate`]
//! - the `PSFactoryBuffer` coclass from the wrapper IDL files: [`PS_FACTORY_BUFFER`],
//!   [`PS_FACTORY_BUFFER_CLSID`], [`Coclass`], [`Guid`]
//!
//! Skipped:
//! - The callees' bodies: `NdrDllGetClassObject` and `NdrDllCanUnloadNow` in `dlls/rpcrt4/cpsf.c`,
//!   and `__wine_register_resources` and `__wine_unregister_resources` in `libs/winecrt0`. They are
//!   other components. The port records only which callee each export returns.
//! - The midl-generated proxy and stub tables in `dlldata.c`, which are not in the reference tree.
//! - The interface definitions that the wrapper IDL files `#include` from `ieautomation.idl` and
//!   `perhist.idl`.
//! - winebuild's parse checks ("Ordinal 0 is not valid", "ordinal redefined") and its MAX_ORDINALS
//!   limit.
//! - The `@ stub` model. `ieproxy.spec` has no `stub` entries, so no `StubException` is raised.

mod coclass;
mod exports;

pub use coclass::{Coclass, Guid, PS_FACTORY_BUFFER, PS_FACTORY_BUFFER_CLSID};
pub use exports::{
    assign_ordinals, export_name, export_ordinal, find_export, i386_stdcall_bytes, Export, EXPORTS,
    MODULE,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
