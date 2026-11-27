/// `Ctl3dGetVer`, ordinal 1: `MAKEWORD(31, 2)`, so the low byte is 31 and the high byte is 2.
pub fn ctl3d_get_ver() -> u16 {
    u16::from_le_bytes([31, 2])
}

/// `Ctl3dSubclassDlg`, ordinal 2. Always FALSE.
pub fn ctl3d_subclass_dlg(_hwnd: u16, _types: u16) -> bool {
    false
}

/// `Ctl3dSubclassCtl`, ordinal 3. Always FALSE.
pub fn ctl3d_subclass_ctl(_hwnd: u16) -> bool {
    false
}

/// `Ctl3dCtlColor`, ordinal 4. Always returns a NULL brush.
pub fn ctl3d_ctl_color(_hdc: u16, _hwnd: i32) -> usize {
    0
}

/// `Ctl3dEnabled`, ordinal 5. Always FALSE.
pub fn ctl3d_enabled() -> bool {
    false
}

/// `Ctl3dColorChange`, ordinal 6. Always TRUE.
pub fn ctl3d_color_change() -> bool {
    true
}

/// `Ctl3dRegister`, ordinal 12. Always FALSE and does not touch the auto-subclass flag.
pub fn ctl3d_register(_hinst: u16) -> bool {
    false
}

/// `Ctl3dCtlColorEx`, ordinal 18. Always returns a NULL brush.
pub fn ctl3d_ctl_color_ex(_msg: u16, _wparam: u16, _lparam: isize) -> usize {
    0
}

/// `Ctl3dDlgFramePaint`, ordinal 20. Always returns 0.
pub fn ctl3d_dlg_frame_paint(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: isize) -> isize {
    0
}

/// `Ctl3dSubclassDlgEx`, ordinal 21. Always FALSE.
pub fn ctl3d_subclass_dlg_ex(_hwnd: u16, _types: u32) -> bool {
    false
}

/// `Ctl3dWinIniChange`, ordinal 22. Does nothing.
pub fn ctl3d_win_ini_change() {}

/// `Ctl3dSubclassCtlEx`, ordinal 25. Always FALSE.
pub fn ctl3d_subclass_ctl_ex(_hwnd: u16, _ty: i16) -> bool {
    false
}

/// `Ctl3dUnsubclassCtl`, ordinal 26. Always FALSE.
pub fn ctl3d_unsubclass_ctl(_hwnd: u16) -> bool {
    false
}
