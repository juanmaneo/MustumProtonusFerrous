//! Port of the portable logic in Wine's `dlls/cryptdll/cryptdll.spec` and `dlls/cryptdll/Makefile.in`.
//!
//! `dlls/cryptdll` has no `.c` source, so this port is partial: it models what the DLL defines.
//!
//! Ported:
//! - `cryptdll.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - the eleven `@ stub` exports and the `__wine_spec_unimplemented_stub` record each raises:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, such as
//!   [`cd_build_vect`]
//! - the `MODULE = cryptdll.dll` file name that `winebuild` passes to every stub
//!
//! Skipped:
//! - The `@ stdcall` forwards `MD5Final`, `MD5Init` and `MD5Update` to `advapi32`. Their code is in
//!   advapi32, so the port records the forward targets and does not implement MD5.
//! - The `__cdecl` calling convention and the assembly that `winebuild` generates for each stub.
//! - The `for (;;)` loop in `__wine_spec_unimplemented_stub`: the port returns the record instead.
//! - `IMPORTS` and `EXTRADLLFLAGS` in `Makefile.in`, which are build settings, not code.

mod exports;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use stub::{
    cd_build_integrity_vect, cd_build_vect, cd_find_common_csystem,
    cd_find_common_csystem_with_key, cd_generate_random_bits, cd_locate_check_sum,
    cd_locate_csystem, cd_locate_rng, cd_register_check_sum, cd_register_csystem, cd_register_rng,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
