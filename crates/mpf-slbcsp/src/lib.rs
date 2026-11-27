//! Port of the portable logic in Wine's `dlls/slbcsp/slbcsp.spec`.
//!
//! The reference has no C sources and no `.idl`, `.rc` or `.tlb` files, so the port is partial.
//! Its only definitions are the export table and the `@ stub` entries, which `winebuild` points at
//! `__wine_spec_unimplemented_stub`.
//!
//! Ported:
//! - `slbcsp.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - the 25 `@ stub` CP* exports, each modelled by a helper that returns the record
//!   `__wine_spec_unimplemented_stub` raises: [`wine_spec_unimplemented_stub`],
//!   [`StubException`] and the `cp_*` helpers, such as [`cp_acquire_context`]
//! - `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE`
//!
//! Skipped:
//! - `DllRegisterServer` and `DllUnregisterServer`, which are `@ stdcall -private` in the spec.
//!   The reference never defines them, so they appear in [`EXPORTS`] only, with no record or
//!   return value modelled.
//! - The rest of the reference: it contains no executable code.
//! - The `__cdecl` calling convention and the stub glue that `winebuild` generates.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    cp_acquire_context, cp_create_hash, cp_decrypt, cp_derive_key, cp_destroy_hash, cp_destroy_key,
    cp_duplicate_hash, cp_duplicate_key, cp_encrypt, cp_export_key, cp_gen_key, cp_gen_random,
    cp_get_hash_param, cp_get_key_param, cp_get_prov_param, cp_get_user_key, cp_hash_data,
    cp_hash_session_key, cp_import_key, cp_release_context, cp_set_hash_param, cp_set_key_param,
    cp_set_prov_param, cp_sign_hash, cp_verify_signature, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
