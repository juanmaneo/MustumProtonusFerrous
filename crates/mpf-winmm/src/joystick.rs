//! `joyConfigChanged` and `joyGetNumDevs` from `dlls/winmm/joystick.c`. The DirectInput device
//! table and the `joyGetDevCaps` and `joyGetPos` paths are not modelled.

/// `JOYERR_NOERROR` and `JOYERR_PARMS`, from `include/mmsystem.h`.
pub const JOYERR_NOERROR: u32 = 0;
pub const JOYERR_PARMS: u32 = 165;

/// `ARRAY_SIZE(joysticks)` in `dlls/winmm/joystick.c`.
pub const JOYSTICK_SLOTS: u32 = 16;

/// `joyConfigChanged`. The C body is a stub that accepts only `flags == 0`.
pub fn joy_config_changed(flags: u32) -> u32 {
    if flags != 0 {
        JOYERR_PARMS
    } else {
        JOYERR_NOERROR
    }
}

/// `joyGetNumDevs`, which returns the size of the `joysticks` table.
pub fn joy_get_num_devs() -> u32 {
    JOYSTICK_SLOTS
}
