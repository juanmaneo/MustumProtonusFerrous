//! Port of the portable logic in Wine's `dlls/ctl3d.dll16/ctl3d.c` and `dlls/ctl3d.dll16/ctl3d.dll16.spec`.
//!
//! Ported:
//! - The `CTL3D16_is_auto_subclass` flag and the methods that set, clear and read it: [`Ctl3d`],
//!   covering `Ctl3dAutoSubclass`, `Ctl3dAutoSubclassEx`, `Ctl3dUnAutoSubclass`, `Ctl3dUnregister`
//!   and `Ctl3dIsAutoSubclass`
//! - `Ctl3dGetVer` (`MAKEWORD(31, 2)`): [`ctl3d_get_ver`]
//! - The exports whose C bodies return a fixed value and never touch the flag: [`ctl3d_subclass_dlg`],
//!   [`ctl3d_subclass_ctl`], [`ctl3d_ctl_color`], [`ctl3d_enabled`], [`ctl3d_color_change`],
//!   [`ctl3d_register`], [`ctl3d_ctl_color_ex`], [`ctl3d_dlg_frame_paint`],
//!   [`ctl3d_subclass_dlg_ex`], [`ctl3d_win_ini_change`], [`ctl3d_subclass_ctl_ex`] and
//!   [`ctl3d_unsubclass_ctl`]
//! - The window procedures, which return 0: [`btn_wnd_proc3d`], [`edit_wnd_proc3d`],
//!   [`list_wnd_proc3d`], [`combo_wnd_proc3d`], [`static_wnd_proc3d`] and [`ctl3d_dlg_proc`]
//! - The `ctl3d.dll16.spec` export table: [`EXPORTS`], [`Export`], [`ExportKind`], [`export_kind`]
//! - `Ctl3dSetStyle` (`@ stub`) and the `__wine_spec_unimplemented_stub` record it raises:
//!   [`ctl3d_set_style`], [`wine_spec_unimplemented_stub`], [`StubException`]
//!
//! Skipped:
//! - The `pascal` calling convention, the `-ret16` spec flag, and the `ENABLE_ARCHS`,
//!   `EXTRADLLFLAGS` and link settings in `dlls/ctl3d.dll16/Makefile.in`.
//! - The ordinal form of the stub's `function` argument, which `winebuild` uses for unnamed
//!   exports. Every `ctl3d.dll16.spec` export has a name, so the port takes only the name.
//! - The process-wide static: [`Ctl3d`] holds one flag per instance.
//! - The `hinst`, `hwnd`, `msg`, `wparam`, `lparam` and `type` arguments of the fixed-value
//!   exports and window procedures. The C bodies never read them, so the port accepts them and
//!   ignores them.

mod auto_subclass;
mod exports;
mod fixed;
mod stub;
mod window_proc;

pub use auto_subclass::Ctl3d;
pub use exports::{export_kind, Export, ExportKind, EXPORTS};
pub use fixed::{
    ctl3d_color_change, ctl3d_ctl_color, ctl3d_ctl_color_ex, ctl3d_dlg_frame_paint, ctl3d_enabled,
    ctl3d_get_ver, ctl3d_register, ctl3d_subclass_ctl, ctl3d_subclass_ctl_ex, ctl3d_subclass_dlg,
    ctl3d_subclass_dlg_ex, ctl3d_unsubclass_ctl, ctl3d_win_ini_change,
};
pub use stub::{
    ctl3d_set_style, wine_spec_unimplemented_stub, StubException, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_WINE_STUB,
};
pub use window_proc::{
    btn_wnd_proc3d, combo_wnd_proc3d, ctl3d_dlg_proc, edit_wnd_proc3d, list_wnd_proc3d,
    static_wnd_proc3d,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
