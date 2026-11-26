//! Port of the portable logic in Wine's `dlls/query/query_main.c` and `dlls/query/query.spec`.
//!
//! Ported:
//! - The five C bodies in `query_main.c`, with their return values and the `*ppIUnk = NULL`
//!   write: [`dll_get_class_object`], [`ci_state`], [`locate_catalogs_a`], [`locate_catalogs_w`]
//!   and [`load_ifilter`], with [`HRESULT`], [`E_NOTIMPL`], [`CLASS_E_CLASSNOTAVAILABLE`] and
//!   [`CI_E_NOT_RUNNING`]
//! - `query.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 38 `@ stub` exports, one helper each in `stub_exports.rs`, which call
//!   [`wine_spec_unimplemented_stub`]. It models `__wine_spec_unimplemented_stub` from
//!   `dlls/ntdll/exception.c` as the [`StubException`] record it raises.
//!
//! Skipped:
//! - The `FIXME` traces and the parameters the C bodies never read: `rclsid`, `iid`, `ppv`,
//!   `pwcsCat`, `pwcsMachine`, `pCiState`, `pwszScope`, `iBm`, `pwszMachine`, `pcMachine`,
//!   `pwszCat`, `pcCat`, `pwcsPath` and `pUnkOuter`. `CIState` and the `LocateCatalogs*`
//!   functions write nothing through their pointers.
//! - The `__cdecl` and `WINAPI` calling conventions. Helper names drop the leading `_` and the
//!   `@N` suffix of decorated exports, so `_AbortMerges@16` becomes `abort_merges`.
//! - The ordinal form of the `function` field. `query.spec` has no export without a name.
//! - `DllCanUnloadNow`, `DllRegisterServer` and `DllUnregisterServer`. `query.spec` lists them as
//!   `stdcall -private`, but no file in `dlls/query` defines them, so they are listed in
//!   [`EXPORTS`] as [`ExportKind::NoBody`] and nothing is modelled for them.

mod exports;
mod query_main;
mod stub;
mod stub_exports;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use query_main::{
    ci_state, dll_get_class_object, load_ifilter, locate_catalogs_a, locate_catalogs_w,
    CI_E_NOT_RUNNING, CLASS_E_CLASSNOTAVAILABLE, E_NOTIMPL, HRESULT,
};
pub use stub::{
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use stub_exports::*;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
