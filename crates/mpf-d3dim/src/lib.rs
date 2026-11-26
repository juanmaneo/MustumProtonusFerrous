//! Port of the portable logic in Wine's `dlls/d3dim/d3dim.spec` and `dlls/d3dim/Makefile.in`.
//!
//! `dlls/d3dim` has no C sources, and no `.idl`, `.rc` or `.tlb` files: the component is the
//! spec file, and every export in it is `@ stub`. The port therefore models the export table and
//! the stub records.
//!
//! Ported:
//! - `d3dim.spec` export table: [`EXPORTS`], [`export_stub`]
//! - `@ stub` exports, modelled on `__wine_spec_unimplemented_stub`: [`wine_spec_unimplemented_stub`],
//!   [`StubException`], and one helper per export, named after it in snake_case, such as [`d3d_free`]
//! - The values the stub raises: [`EXCEPTION_WINE_STUB`], [`EXCEPTION_NONCONTINUABLE`]
//!
//! Skipped:
//! - Export behaviour beyond the stub. There is no C body in `dlls/d3dim` to port, so there is no
//!   return value to model. A stub never returns.
//! - `ENABLE_ARCHS = i386` in `Makefile.in`, and the `__cdecl` and `__stdcall` decorations. The
//!   port builds on every target.

mod exports;
mod stub;

pub use exports::{export_stub, EXPORTS};
pub use stub::{
    d3d_free, d3d_malloc, d3d_realloc, direct3d_create, direct3d_create_device,
    direct3d_create_texture, direct3d_get_sw_rast_zpix_fmts, direct3d_hal_clean_up,
    flush_d3d_devices, flush_d3d_devices2, palette_associate_notify, palette_update_notify,
    surface_flip_notify, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
