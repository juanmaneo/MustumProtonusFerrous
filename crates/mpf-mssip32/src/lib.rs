//! Port of the portable logic in Wine's `dlls/mssip32/mssip32.spec` and `dlls/mssip32/Makefile.in`.
//!
//! Ported:
//! - `mssip32.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The seven `@ stub` CryptSIP exports, each as a helper such as [`crypt_sip_get_info`], and
//!   the `__wine_spec_unimplemented_stub` record they raise: [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//!
//! Skipped:
//! - The bodies of `DllRegisterServer` and `DllUnregisterServer`. Both are `@ stdcall -private`,
//!   and `dlls/mssip32` has no source that defines them.
//! - The CryptSIP implementations. Every CryptSIP export is `@ stub`, so the reference has no
//!   body; winebuild only emits the call to `__wine_spec_unimplemented_stub`.
//! - Stub records for exports without a name, where `function` is the ordinal. The spec has no
//!   `-noname` exports.
//! - The per-architecture code winebuild emits around each stub (`nop` padding, PIC thunks).

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    crypt_sip_create_indirect_data, crypt_sip_get_info, crypt_sip_get_reg_working_flags,
    crypt_sip_get_signed_data_msg, crypt_sip_put_signed_data_msg, crypt_sip_remove_signed_data_msg,
    crypt_sip_verify_indirect_data, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
