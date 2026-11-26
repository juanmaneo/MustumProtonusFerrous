//! Port of the portable logic in Wine's `dlls/magnification/main.c` and
//! `dlls/magnification/magnification.spec`.
//!
//! Ported:
//! - `MagInitialize` and `MagUninitialize`, the stub bodies, with the `ERROR_CALL_NOT_IMPLEMENTED`
//!   last error and the `BOOL` results: [`mag_initialize`], [`mag_uninitialize`],
//!   [`ERROR_CALL_NOT_IMPLEMENTED`]
//! - `magnification.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//! - The `__wine_spec_unimplemented_stub` record: [`wine_spec_unimplemented_stub`] and
//!   [`StubException`], plus one helper per `@ stub` export, named after its snake_case name,
//!   such as [`mag_get_color_effect`] for `MagGetColorEffect`
//!
//! Skipped:
//! - The `FIXME("stub!\n")` trace in both C bodies and the `WINE_DEFAULT_DEBUG_CHANNEL` channel.
//! - The thread's last-error slot: `SetLastError` is modelled as the `&mut u32` argument of
//!   [`mag_initialize`]. [`mag_uninitialize`] takes no such argument because its C body never
//!   touches the last error.
//! - The ordinal form of the stub call. Every `@ stub` export in `magnification.spec` has a name.

mod exports;
mod init;
mod stub;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use init::{mag_initialize, mag_uninitialize, ERROR_CALL_NOT_IMPLEMENTED};
pub use stub::{
    mag_get_color_effect, mag_get_fullscreen_color_effect, mag_get_fullscreen_transform,
    mag_get_image_scaling_callback, mag_get_input_transform, mag_get_window_filter_list,
    mag_get_window_source, mag_get_window_transform, mag_set_color_effect,
    mag_set_fullscreen_color_effect, mag_set_fullscreen_transform,
    mag_set_fullscreen_use_bitmap_smoothing, mag_set_image_scaling_callback,
    mag_set_input_transform, mag_set_lens_use_bitmap_smoothing, mag_set_window_filter_list,
    mag_set_window_source, mag_set_window_transform, mag_show_system_cursor,
    wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
