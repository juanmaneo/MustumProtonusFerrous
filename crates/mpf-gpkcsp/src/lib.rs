//! Port of the portable logic in Wine's `dlls/gpkcsp/gpkcsp.spec`. The component has no `.c`
//! sources, so the port covers the spec's export table and the `@ stub` records it raises.
//!
//! Ported:
//! - `gpkcsp.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 23 `@ stub` CP* exports: [`wine_spec_unimplemented_stub`], [`StubException`] and one
//!   helper per export, such as [`cp_acquire_context`]
//! - The `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE` values the record carries
//!
//! Skipped:
//! - `DllMain`, `DllRegisterServer` and `DllUnregisterServer` (`@ stdcall -private`): the reference
//!   defines no body for them, so only their export-table entries are ported.
//! - The `ExceptionRecord` (always NULL) and `ExceptionAddress` fields, which [`StubException`]
//!   does not carry.
//! - The CP* parameters and calling conventions: the stub never reads them.
//! - The ordinal form of `function`: every export in `gpkcsp.spec` has a name.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    cp_acquire_context, cp_create_hash, cp_decrypt, cp_derive_key, cp_destroy_hash, cp_destroy_key,
    cp_encrypt, cp_export_key, cp_gen_key, cp_gen_random, cp_get_hash_param, cp_get_key_param,
    cp_get_prov_param, cp_get_user_key, cp_hash_data, cp_hash_session_key, cp_import_key,
    cp_release_context, cp_set_hash_param, cp_set_key_param, cp_set_prov_param, cp_sign_hash,
    cp_verify_signature, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
