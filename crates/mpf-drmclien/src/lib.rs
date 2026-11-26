//! Port of the portable logic in Wine's `dlls/drmclien/drmclien.spec`.
//!
//! `dlls/drmclien` has no C sources: `Makefile.in` sets `MODULE = drmclien.dll` and only the spec
//! file defines the exports. The port covers that export table and the stub behaviour of its
//! `@ stub` exports.
//!
//! Ported:
//! - `drmclien.spec` entry table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The 28 `@ stub` exports: [`wine_spec_unimplemented_stub`], [`StubException`],
//!   [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`], and one helper per export
//!   (`can_decrypt`, `decrypt`, ..., `cdrmlite_crypto_restore_licenses`) that raises the record
//!
//! Skipped:
//! - `DllMain`, `DllRegisterServer` and `DllUnregisterServer` (`@ stdcall -private`): the reference
//!   has no C body for them, so there is no behaviour to port. They appear only in [`EXPORTS`].
//! - The ordinal form of the stub's function argument: every entry in `drmclien.spec` has a name.
//! - The `-Wb,--prefer-native` flag from `Makefile.in`: winebuild only sets a PE header bit.
//! - The raised exception: the port returns the record instead of raising it.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    bind, bind_ex, can_decrypt, can_decrypt_ex, cdrmlite_crypto_backup_licenses,
    cdrmlite_crypto_ctor, cdrmlite_crypto_dtor, cdrmlite_crypto_encrypt_fast,
    cdrmlite_crypto_encrypt_indirect_fast, cdrmlite_crypto_generate_new_license_ex,
    cdrmlite_crypto_get_licenses, cdrmlite_crypto_get_public_key,
    cdrmlite_crypto_query_xfer_to_pm_ex, cdrmlite_crypto_restore_licenses, create_pm_license,
    decrypt, encrypt, generate_new_license, get_pm_license_file_name, get_pm_license_size,
    get_version, init_app_certs, key_exchange, process_response, query_xfer_to_pm, request_license,
    set_app_sec, set_license_store, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
