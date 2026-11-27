//! Port of the portable logic in Wine's `dlls/olethk32/olethk32.spec`.
//!
//! Ported:
//! - `olethk32.spec` export table: [`EXPORTS`], [`Export`], [`export_stub`]
//! - The 14 `@ stub` exports and the `__wine_spec_unimplemented_stub` record each one raises:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], and one helper per export, such as
//!   [`invoke_on_32`]
//!
//! Skipped:
//! - `dlls/olethk32` has no `.c`, `.idl`, `.rc` or `.tlb` file, so the only code to port is the
//!   stub table.
//! - The ordinal form of the `function` argument, which `winebuild` uses for unnamed exports.
//!   Every `olethk32.spec` stub has a name, so the port takes only the name.
//! - The `__cdecl` calling convention, and the `EXCEPTION_RECORD` fields the record does not
//!   model (`ExceptionRecord` and `ExceptionAddress`).
//! - The `i386`-only build (`ENABLE_ARCHS = i386`) and the link and version flags in
//!   `dlls/olethk32/Makefile.in`.

mod exports;
mod stub;

pub use exports::{export_stub, Export, EXPORTS};
pub use stub::{
    c_sm16_release_handler_release32, callback_processing_3216, convert_hr1632_thunk,
    convert_hr3216_thunk, convert_obj_descriptor, i_unknown_obj32, int_op_initialize,
    int_op_uninitialize, invoke_on_32, thk_add_app_compat_flag, thk_mgr_initialize,
    thk_mgr_uninitialize, transform_hresult_1632, transform_hresult_3216,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
