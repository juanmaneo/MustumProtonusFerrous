/// `S_OK` from `winerror.h`.
pub const S_OK: i32 = 0;

/// `DMUSIC32_CreateCDirectMusicEmulatePort`. Undocumented, unimplemented in Wine:
/// the arguments are ignored and the call always reports success.
pub fn create_cdirectmusic_emulate_port(_ptr1: usize, _ptr2: usize, _ptr3: usize) -> i32 {
    S_OK
}

/// `DMUSIC32_EnumLegacyDevices`. Undocumented, unimplemented in Wine:
/// the arguments are ignored and the call always reports success.
pub fn enum_legacy_devices(_ptr1: usize, _ptr2: usize) -> i32 {
    S_OK
}
