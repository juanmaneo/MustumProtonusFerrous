//! The stubs that return `MMSYSERR_NOTSUPPORTED` without reading their arguments. The MIDI patch
//! functions are in `dlls/winmm/winmm.c`. The wave pitch and playback-rate functions, and
//! `mixerMessage`, are in `dlls/winmm/waveform.c`.

use crate::error::MMSYSERR_NOTSUPPORTED;

/// `midiOutCachePatches` from `dlls/winmm/winmm.c`.
pub fn midi_out_cache_patches() -> u32 {
    MMSYSERR_NOTSUPPORTED
}

/// `midiOutCacheDrumPatches` from `dlls/winmm/winmm.c`.
pub fn midi_out_cache_drum_patches() -> u32 {
    MMSYSERR_NOTSUPPORTED
}

/// `waveOutGetPitch` from `dlls/winmm/waveform.c`.
pub fn wave_out_get_pitch() -> u32 {
    MMSYSERR_NOTSUPPORTED
}

/// `waveOutSetPitch` from `dlls/winmm/waveform.c`.
pub fn wave_out_set_pitch() -> u32 {
    MMSYSERR_NOTSUPPORTED
}

/// `waveOutGetPlaybackRate` from `dlls/winmm/waveform.c`.
pub fn wave_out_get_playback_rate() -> u32 {
    MMSYSERR_NOTSUPPORTED
}

/// `waveOutSetPlaybackRate` from `dlls/winmm/waveform.c`.
pub fn wave_out_set_playback_rate() -> u32 {
    MMSYSERR_NOTSUPPORTED
}

/// `mixerMessage` from `dlls/winmm/waveform.c`, which returns `DWORD`.
pub fn mixer_message() -> u32 {
    MMSYSERR_NOTSUPPORTED
}
