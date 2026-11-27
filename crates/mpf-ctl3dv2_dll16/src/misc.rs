/// `Ctl3dGetVer`: `MAKEWORD(31, 2)`, so the low byte is 31 and the high byte is 2.
pub fn get_ver() -> u16 {
    u16::from_le_bytes([31, 2])
}

/// `Ctl3dColorChange`: returns TRUE and does nothing else.
pub fn color_change() -> bool {
    true
}

/// `Ctl3dEnabled`: returns FALSE, so the 3D look is never reported as enabled.
pub fn enabled() -> bool {
    false
}

/// `Ctl3dRegister`: returns TRUE. `hinst` is ignored.
pub fn register(_hinst: u16) -> bool {
    true
}

/// `Ctl3dWinIniChange`: does nothing.
pub fn win_ini_change() {}
