//! Port of the portable logic in Wine's `dlls/mscat32/mscat32.spec` and `dlls/mscat32/Makefile.in`.
//!
//! `dlls/mscat32` has no C sources. Its exports are forwards to `wintrust` and `@ stub` records,
//! so the port keeps the export table and the stub records.
//!
//! Ported:
//! - `mscat32.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`export`], with the
//!   ordinals and `PRIVATE` marks from the reference `winebuild --def` output
//! - `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], [`stub_for_export`], and one helper per
//!   stub: [`crypt_cat_verify_member`], [`crypt_cat_cdf_enum_attributes`],
//!   [`crypt_cat_cdf_enum_members`], [`ms_cat_construct_hash_tag`], [`ms_cat_free_hash_tag`]
//! - The `EXCEPTION_WINE_STUB` and `EXCEPTION_NONCONTINUABLE` values: [`EXCEPTION_WINE_STUB`],
//!   [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - The 30 forwarded exports, including `DllRegisterServer` and `DllUnregisterServer`. Their bodies
//!   are in `wintrust.dll`, which this DLL does not contain, and the loader's forward resolution
//!   needs the OS.
//! - The `#@ stub CatalogCompactHashDatabase` line. It is commented out, so it is not an export.
//! - The effect of `PRIVATE` on import libraries (`tools/winebuild/import.c`). It is a link-time
//!   property. [`Export::private`] records the mark without acting on it.

mod exports;
mod stub;

pub use exports::{export, Export, ExportKind, EXPORTS};
pub use stub::{
    crypt_cat_cdf_enum_attributes, crypt_cat_cdf_enum_members, crypt_cat_verify_member,
    ms_cat_construct_hash_tag, ms_cat_free_hash_tag, stub_for_export, wine_spec_unimplemented_stub,
    StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
