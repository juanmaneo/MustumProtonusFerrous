//! Port of the portable logic in Wine's `dlls/initpki/initpki.spec` and `dlls/initpki/Makefile.in`.
//!
//! The reference directory has no C sources, so there is no executable body to port. It defines
//! an export table and the `@ stub` exports that winebuild points at `__wine_spec_unimplemented_stub`.
//!
//! Ported:
//! - `initpki.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - `DllInstall` and `InitializePKI` (`@ stub`), and the `__wine_spec_unimplemented_stub` record
//!   they raise: [`dll_install`], [`initialize_pki`], [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer` (`@ stdcall -private`): the reference has no C
//!   definition for either, so there is no behaviour to port. The export table records them only.
//! - The `NULL` pointer case of the stub arguments: winebuild always passes string literals, so
//!   the port takes `&'static str`.
//! - The ordinal form of the function argument: every `initpki.spec` export has a name.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    dll_install, initialize_pki, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
