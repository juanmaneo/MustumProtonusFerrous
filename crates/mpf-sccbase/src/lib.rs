//! Port of the portable logic in Wine's `dlls/sccbase/sccbase.spec` and of the
//! `__wine_spec_unimplemented_stub` that its `@ stub` exports call in `dlls/ntdll/exception.c`.
//!
//! The reference directory has no `.c`, `.idl`, `.rc` or `.tlb` files, so this port covers the
//! spec file only: its declarations and the stub records they raise.
//!
//! Ported:
//! - `sccbase.spec` declarations: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 24 `@ stub` CryptoAPI provider entry points, `CPAcquireContext` to `CPVerifySignature`,
//!   one helper each, for example [`cp_acquire_context`]. Each returns the record that
//!   `__wine_spec_unimplemented_stub` raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`]
//! - `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE`: [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer` (`stdcall -private`). The reference has no
//!   body for either. winebuild keeps `FLAG_PRIVATE` entries out of the export table and emits
//!   no stub for them, so only their table entries are ported.
//! - The ordinal form of the stub's function argument. Every `@ stub` in the spec has a name.
//! - The endless `for (;;)` loop of the C stub, and the `__cdecl` calling convention.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    cp_acquire_context, cp_acquire_context_w, cp_create_hash, cp_decrypt, cp_derive_key,
    cp_destroy_hash, cp_destroy_key, cp_encrypt, cp_export_key, cp_gen_key, cp_gen_random,
    cp_get_hash_param, cp_get_key_param, cp_get_prov_param, cp_get_user_key, cp_hash_data,
    cp_hash_session_key, cp_import_key, cp_release_context, cp_set_hash_param, cp_set_key_param,
    cp_set_prov_param, cp_sign_hash, cp_verify_signature, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
