//! The flag masks of `sndPlaySoundA` and `sndPlaySoundW` in `dlls/winmm/playsound.c`, and
//! `mmsystemGetVersion` from the same file. `MULTIMEDIA_PlaySound` is not modelled.

/// `SND_*` values from `include/mmsystem.h`. `SND_SYNC` is 0, so the mask needs no bit for it.
pub const SND_ASYNC: u32 = 0x0001;
pub const SND_NODEFAULT: u32 = 0x0002;
pub const SND_MEMORY: u32 = 0x0004;
pub const SND_LOOP: u32 = 0x0008;
pub const SND_NOSTOP: u32 = 0x0010;
pub const SND_ALIAS_ID: u32 = 0x0011_0000;
pub const SND_FILENAME: u32 = 0x0002_0000;
pub const SND_RESOURCE: u32 = 0x0004_0004;

/// The bits that `sndPlaySoundA` and `sndPlaySoundW` keep. `SND_PURGE` and the other bits are cleared.
pub const SND_FLAGS_MASK: u32 = SND_RESOURCE
    | SND_ALIAS_ID
    | SND_FILENAME
    | SND_ASYNC
    | SND_LOOP
    | SND_MEMORY
    | SND_NODEFAULT
    | SND_NOSTOP;

/// `MMSYSTEM_VERSION`, the value that `mmsystemGetVersion` returns for version 3.10.
pub const MMSYSTEM_VERSION: u32 = 0x030a;

/// The flags that `sndPlaySoundA` and `sndPlaySoundW` pass to `MULTIMEDIA_PlaySound`. Both use the
/// same mask, so the unicode argument does not change the flags.
pub fn snd_play_sound_flags(flags: u32) -> u32 {
    flags & SND_FLAGS_MASK
}

/// `mmsystemGetVersion`.
pub fn mmsystem_get_version() -> u32 {
    MMSYSTEM_VERSION
}
