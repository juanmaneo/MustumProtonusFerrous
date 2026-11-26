//! Port of the portable logic in Wine's `dlls/d3dim700/d3dim700.spec` and `dlls/d3dim700/Makefile.in`.
//!
//! The DLL has no `.c`, `.idl`, `.rc` or `.tlb` sources. All 18 exports in the spec are `@ stub`,
//! so the port covers the export table and the stub records that winebuild emits for them.
//!
//! Ported:
//! - `d3dim700.spec` export table: [`EXPORTS`], [`export_stub`]
//! - The 18 `@ stub` exports, one helper each, and the `__wine_spec_unimplemented_stub` record they
//!   raise: [`wine_spec_unimplemented_stub`], [`StubException`], with `EXCEPTION_WINE_STUB` and
//!   `EXCEPTION_NONCONTINUABLE` from `include/winternl.h` and `include/winnt.h`
//!
//! Skipped:
//! - The bodies of the 18 exports. The spec gives no C code for them, so the stub record stands in.
//! - The ordinal form of the function argument. Every export has a name, so winebuild always passes
//!   the name.
//! - The `__cdecl` calling convention, and the `for (;;)` loop around `RtlRaiseException`. The port
//!   returns the record instead.
//! - `VER_PRODUCTVERSION` in `Makefile.in`, which is version resource data, not logic.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::{
    create_texture, d3d_break_vb_lock, d3d_free, d3d_malloc, d3d_realloc, d3d_texture_update,
    destroy_texture, direct3d_create, direct3d_create_device, direct3d_hal_clean_up,
    flush_d3d_devices, get_lod, get_priority, palette_associate_notify, palette_update_notify,
    set_lod, set_priority, surface_flip_notify, wine_spec_unimplemented_stub, StubException,
    EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
