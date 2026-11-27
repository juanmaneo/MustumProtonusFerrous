//! Port of the portable logic in Wine's `dlls/ctl3dv2.dll16/ctl3d.c` and `dlls/ctl3dv2.dll16/ctl3dv2.dll16.spec`.
//!
//! Ported:
//! - `CTL3D16_is_auto_subclass` state machine: `Ctl3dAutoSubclass`, `Ctl3dAutoSubclassEx`,
//!   `Ctl3dUnAutoSubclass`, `Ctl3dUnregister`, `Ctl3dIsAutoSubclass`: [`Ctl3d`]
//! - `Ctl3dGetVer` (`MAKEWORD(31, 2)`), `Ctl3dColorChange`, `Ctl3dEnabled`, `Ctl3dRegister`,
//!   `Ctl3dWinIniChange`: [`get_ver`], [`color_change`], [`enabled`], [`register`], [`win_ini_change`]
//! - Subclassing exports that report failure: [`subclass_dlg`], [`subclass_dlg_ex`],
//!   [`subclass_ctl`], [`subclass_ctl_ex`], [`unsubclass_ctl`]
//! - Colour hook and window procedures that return 0: [`ctl_color`], [`ctl_color_ex`],
//!   [`dlg_frame_paint`], [`dlg_proc`], [`btn_wnd_proc_3d`], [`edit_wnd_proc_3d`],
//!   [`list_wnd_proc_3d`], [`combo_wnd_proc_3d`], [`static_wnd_proc_3d`]
//! - `Ctl3dSetStyle` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`set_style`], [`wine_spec_unimplemented_stub`], [`StubException`]
//! - `ctl3dv2.dll16.spec` export table: [`EXPORTS`], [`ExportKind`], [`export_kind`]
//!
//! Skipped:
//! - The 16-bit calling convention (`pascal`, `-ret16`) and the segmented handle types. Arguments
//!   are plain integers, and the port ignores the ones the C body ignores.
//! - The `CTL3D16_is_auto_subclass` static. Wine keeps it process-wide; the port keeps it in a
//!   [`Ctl3d`] value that the caller owns.
//! - The `for (;;)` loop in `__wine_spec_unimplemented_stub`, which never returns. The port
//!   returns the record instead.

mod exports;
mod misc;
mod state;
mod stub;
mod subclass;
mod window;

pub use exports::{export_kind, ExportKind, EXPORTS};
pub use misc::{color_change, enabled, get_ver, register, win_ini_change};
pub use state::Ctl3d;
pub use stub::{
    set_style, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};
pub use subclass::{subclass_ctl, subclass_ctl_ex, subclass_dlg, subclass_dlg_ex, unsubclass_ctl};
pub use window::{
    btn_wnd_proc_3d, combo_wnd_proc_3d, ctl_color, ctl_color_ex, dlg_frame_paint, dlg_proc,
    edit_wnd_proc_3d, list_wnd_proc_3d, static_wnd_proc_3d,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
