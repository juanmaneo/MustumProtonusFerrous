//! Port of the portable logic in Wine's `dlls/winmm/` (`winmm.c`, `waveform.c`, `time.c`,
//! `driver.c`, `playsound.c`, `joystick.c`, `mmio.c`) and `dlls/winmm/winmm.spec`.
//!
//! Ported:
//! - `midiOutGetErrorTextA/W` and `waveOutGetErrorTextA/W`, with the `MMSYSERR`, `WAVERR` and
//!   `MIDIERR` values and the STRINGTABLE text: [`midi_out_get_error_text_w`],
//!   [`midi_out_get_error_text_a`], [`wave_out_get_error_text_w`], [`wave_out_get_error_text_a`]
//! - `WINMM_CheckCallback`: [`check_callback`] and the `CALLBACK_*` values
//! - `timeSetEvent` and `timeKillEvent` over the 16-slot table, without the callback: [`TimerTable`]
//! - `mmioStringToFOURCCA` and `mmioStringToFOURCCW`, `mmioFOURCC` and `RtlUpperChar` for `a` to `z`:
//!   [`mmio_string_to_fourcc_a`], [`mmio_string_to_fourcc_w`], [`mmio_fourcc`], [`rtl_upper_char`]
//! - `DefDriverProc`: [`def_driver_proc`]
//! - the flag mask of `sndPlaySoundA` and `sndPlaySoundW`, and `mmsystemGetVersion`:
//!   [`snd_play_sound_flags`], [`mmsystem_get_version`]
//! - `joyConfigChanged` and `joyGetNumDevs`: [`joy_config_changed`], [`joy_get_num_devs`]
//! - the functions that return `MMSYSERR_NOTSUPPORTED`: [`midi_out_cache_patches`],
//!   [`midi_out_cache_drum_patches`], [`wave_out_get_pitch`], [`wave_out_set_pitch`],
//!   [`wave_out_get_playback_rate`], [`wave_out_set_playback_rate`], [`mixer_message`]
//! - the `winmm.spec` export table: [`EXPORTS`], [`ExportKind`], [`ExportName`], [`export_kind`]
//! - the `@ stub` exports and the `__wine_spec_unimplemented_stub` record they raise:
//!   [`wine_spec_unimplemented_stub`], [`StubException`], [`StubFunction`],
//!   [`mmio_install_io_proc16`], [`stub_ordinal_3`], [`stub_ordinal_4`]
//!
//! Skipped:
//! - `timeGetTime`, `timeGetDevCaps` and `timeBeginPeriod`, which `dlls/kernel32/sync.c` defines.
//! - `MULTIMEDIA_PlaySound` and the playback. Of the timer, the callback (`lpFunc`), `dwUser`, the
//!   timer service thread and the synchronous kill are not modelled.
//! - The `DefDriverProc` driver identifier, driver handle and `LPARAM` values.
//! - Non-ASCII `CP_ACP` text (the shim turns those units into `?`) and `RtlUpperChar` beyond `a` to
//!   `z`, which `dlls/ntdll/rtlstr.c` defines.
//! - The `TRACE` and `WARN` logging, and the calling convention of the exported functions.
//! - The parts of `driver.c`, `joystick.c`, `lolvldrv.c`, `mci.c`, `mmio.c`, `playsound.c`,
//!   `time.c`, `waveform.c` and `winmm.c` that need drivers, threads, the registry or the OS.

mod acp;
mod callback;
mod driver;
mod error;
mod exports;
mod fourcc;
mod joystick;
mod not_supported;
mod playsound;
mod stub;
mod time;

pub use acp::{wide_to_acp_byte, wide_to_acp_z};
pub use callback::{
    check_callback, CALLBACK_EVENT, CALLBACK_FUNCTION, CALLBACK_NULL, CALLBACK_THREAD,
    CALLBACK_TYPEMASK, CALLBACK_WINDOW,
};
pub use driver::def_driver_proc;
pub use error::{
    error_text, midi_out_get_error_text_a, midi_out_get_error_text_w, wave_out_get_error_text_a,
    wave_out_get_error_text_w, MIDIERR_BADOPENMODE, MIDIERR_BASE, MIDIERR_INVALIDSETUP,
    MIDIERR_LASTERROR, MIDIERR_NODEVICE, MIDIERR_NOMAP, MIDIERR_NOTREADY, MIDIERR_STILLPLAYING,
    MIDIERR_UNPREPARED, MMSYSERR_ALLOCATED, MMSYSERR_BADDEVICEID, MMSYSERR_BADERRNUM,
    MMSYSERR_ERROR, MMSYSERR_HANDLEBUSY, MMSYSERR_INVALFLAG, MMSYSERR_INVALHANDLE,
    MMSYSERR_INVALPARAM, MMSYSERR_LASTERROR, MMSYSERR_NODRIVER, MMSYSERR_NOERROR, MMSYSERR_NOMEM,
    MMSYSERR_NOTENABLED, MMSYSERR_NOTSUPPORTED, WAVERR_BADFORMAT, WAVERR_BASE, WAVERR_LASTERROR,
    WAVERR_STILLPLAYING, WAVERR_SYNC, WAVERR_UNPREPARED,
};
pub use exports::{export_kind, ExportKind, ExportName, EXPORTS};
pub use fourcc::{
    mmio_fourcc, mmio_string_to_fourcc_a, mmio_string_to_fourcc_w, rtl_upper_char, MMIO_TOUPPER,
};
pub use joystick::{
    joy_config_changed, joy_get_num_devs, JOYERR_NOERROR, JOYERR_PARMS, JOYSTICK_SLOTS,
};
pub use not_supported::{
    midi_out_cache_drum_patches, midi_out_cache_patches, mixer_message, wave_out_get_pitch,
    wave_out_get_playback_rate, wave_out_set_pitch, wave_out_set_playback_rate,
};
pub use playsound::{
    mmsystem_get_version, snd_play_sound_flags, MMSYSTEM_VERSION, SND_ALIAS_ID, SND_ASYNC,
    SND_FILENAME, SND_FLAGS_MASK, SND_LOOP, SND_MEMORY, SND_NODEFAULT, SND_NOSTOP, SND_RESOURCE,
};
pub use stub::{
    mmio_install_io_proc16, stub_ordinal_3, stub_ordinal_4, wine_spec_unimplemented_stub,
    StubException, StubFunction, EXCEPTION_NONCONTINUABLE, EXCEPTION_WINE_STUB,
};
pub use time::{TimerEntry, TimerTable, TIMERR_NOCANDO, TIMERR_NOERROR};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
