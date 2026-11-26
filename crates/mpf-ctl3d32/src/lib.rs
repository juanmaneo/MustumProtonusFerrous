//! Port of the portable logic in Wine's `dlls/ctl3d32/ctl3d32.c`.
//!
//! Ported:
//! - `CTL3D_is_auto_subclass` state machine: `Ctl3dAutoSubclass`,
//!   `Ctl3dAutoSubclassEx`, `Ctl3dUnAutoSubclass`, `Ctl3dUnregister`,
//!   `Ctl3dIsAutoSubclass`: [`Ctl3d`]
//! - `Ctl3dGetVer` (`MAKEWORD(31, 2)`): [`get_ver`]
//!
//! Skipped:
//! - `Ctl3dDlgFramePaint`: forwards to `DefWindowProcA`.
//! - `BtnWndProc3d`, `EditWndProc3d`, `ListWndProc3d`, `ComboWndProc3d`,
//!   `StaticWndProc3d`, `Ctl3dDlgProc`: window procedures that always return 0.
//! - `Ctl3dRegister`, `Ctl3dEnabled`, `Ctl3dColorChange`, `Ctl3dCtlColor`,
//!   `Ctl3dCtlColorEx`, `Ctl3dSubclassCtl`, `Ctl3dSubclassCtlEx`,
//!   `Ctl3dSubclassDlg`, `Ctl3dSubclassDlgEx`, `Ctl3dUnsubclassCtl`,
//!   `Ctl3dWinIniChange`: fixed-value or no-op stubs that never read or write the flag.

/// The `CTL3D_is_auto_subclass` flag. Wine keeps it in a process-wide static;
/// this type holds the same single bit per instance, and it defaults to FALSE.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ctl3d {
    auto_subclass: bool,
}

impl Ctl3d {
    /// `Ctl3dAutoSubclass`. Sets the flag and returns TRUE. `hinst` is ignored.
    pub fn auto_subclass(&mut self, _hinst: usize) -> bool {
        self.auto_subclass = true;
        true
    }

    /// `Ctl3dAutoSubclassEx`. Sets the flag and returns TRUE. `hinst` and `ty` are ignored.
    pub fn auto_subclass_ex(&mut self, _hinst: usize, _ty: u32) -> bool {
        self.auto_subclass = true;
        true
    }

    /// `Ctl3dUnAutoSubclass`. Clears the flag and returns FALSE (bug-compatible).
    pub fn un_auto_subclass(&mut self) -> bool {
        self.auto_subclass = false;
        false
    }

    /// `Ctl3dUnregister`. Clears the flag and returns TRUE. `hinst` is ignored.
    pub fn unregister(&mut self, _hinst: usize) -> bool {
        self.auto_subclass = false;
        true
    }

    /// `Ctl3dIsAutoSubclass`.
    pub fn is_auto_subclass(&self) -> bool {
        self.auto_subclass
    }
}

/// `Ctl3dGetVer`: `MAKEWORD(31, 2)`, so the low byte is 31 and the high byte is 2.
pub fn get_ver() -> u16 {
    u16::from_le_bytes([31, 2])
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
