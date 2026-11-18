//! xactengine3_7 subset: the notification type maps and field selection behind Wine's XACT3 wrapper.
//!
//! Reference: `dlls/xactengine3_7/xact_dll.c`, built with `XACT3_VER=0x0307` (`Makefile.in`).
//!
//! Ported:
//! - `xact_notification_type_from_fact`: [`xact_notification_type_from_fact`].
//! - `fact_notification_type_from_xact`: [`fact_notification_type_from_xact`].
//! - the `flags` chain in `unwrap_notificationdesc`: [`notify_flags`], which selects the
//!   optional fields a notification carries.
//!
//! Skipped:
//! - `unwrap_notificationdesc` itself: it copies COM interface pointers into FACT objects.
//! - `wrapper_lookup_compare`, `wrapper_lookup_destroy`, `wrapper_remove_entry`,
//!   `wrapper_add_entry`, `wrapper_find_entry`: a red-black tree keyed on live FACT pointers.
//! - `fact_notification_cb`: callback from a running FACT engine.
//! - `IXACT3CueImpl_*`, `IXACT3SoundBankImpl_*`, `IXACT3WaveImpl_*`, `IXACT3WaveBankImpl_*`,
//!   `IXACT3EngineImpl_*`: COM methods over live engine, bank, cue and wave handles.
//! - `XACT3CF_*`, `DllMain`, `DllGetClassObject`: COM class factory and DLL entry points.
//! - `XACT_Internal_Malloc`, `XACT_Internal_Free`, `XACT_Internal_Realloc`: process heap wrappers.
//!
//! The `FIXME` trace on an unknown type is logging only and is not ported; the maps still
//! return `0` for it. Both maps are the identity on notification numbers 1 to 18, because the
//! FACT and XACT3 constants have the same values in this build.

/// `NOTIFY_SoundBank`: the notification carries a sound bank.
pub const NOTIFY_SOUND_BANK: u32 = 0x01;
/// `NOTIFY_WaveBank`: the notification carries a wave bank.
pub const NOTIFY_WAVE_BANK: u32 = 0x02;
/// `NOTIFY_Cue`: the notification carries a cue instance.
pub const NOTIFY_CUE: u32 = 0x04;
/// `NOTIFY_Wave`: the notification carries a wave instance.
pub const NOTIFY_WAVE: u32 = 0x08;
/// `NOTIFY_cueIndex`: the notification carries a cue index.
pub const NOTIFY_CUE_INDEX: u32 = 0x10;
/// `NOTIFY_waveIndex`: the notification carries a wave index.
pub const NOTIFY_WAVE_INDEX: u32 = 0x20;

/// Last notification number, `WAVEBANKSTREAMING_INVALIDCONTENT`.
const LAST_NOTIFICATION_TYPE: u8 = 18;

const CUE_PREPARED: u8 = 1;
const CUE_PLAY: u8 = 2;
const CUE_STOP: u8 = 3;
const CUE_DESTROYED: u8 = 4;
const MARKER: u8 = 5;
const SOUND_BANK_DESTROYED: u8 = 6;
const WAVE_BANK_DESTROYED: u8 = 7;
const LOCAL_VARIABLE_CHANGED: u8 = 8;
const WAVE_PREPARED: u8 = 12;
const WAVE_PLAY: u8 = 13;
const WAVE_STOP: u8 = 14;
const WAVE_LOOPED: u8 = 15;
const WAVE_DESTROYED: u8 = 16;
const WAVE_BANK_PREPARED: u8 = 17;
const WAVE_BANK_STREAMING_INVALID_CONTENT: u8 = 18;

/// `xact_notification_type_from_fact`: maps a FACT notification number to the XACT3 one.
///
/// Numbers outside 1 to 18 map to `0`.
pub fn xact_notification_type_from_fact(ty: u8) -> u8 {
    if (1..=LAST_NOTIFICATION_TYPE).contains(&ty) {
        ty
    } else {
        0
    }
}

/// `fact_notification_type_from_xact`: maps an XACT3 notification number to the FACT one.
///
/// Numbers outside 1 to 18 map to `0`.
pub fn fact_notification_type_from_xact(ty: u8) -> u8 {
    if (1..=LAST_NOTIFICATION_TYPE).contains(&ty) {
        ty
    } else {
        0
    }
}

/// The `flags` chain in `unwrap_notificationdesc`, keyed on the FACT notification number.
///
/// Types the chain does not list (`GUICONNECTED`, `GUIDISCONNECTED`, `GLOBALVARIABLECHANGED`,
/// and anything unknown) select no fields.
pub fn notify_flags(fact_type: u8) -> u32 {
    match fact_type {
        CUE_PREPARED | CUE_PLAY | CUE_STOP | CUE_DESTROYED | MARKER | LOCAL_VARIABLE_CHANGED => {
            NOTIFY_SOUND_BANK | NOTIFY_CUE_INDEX | NOTIFY_CUE
        }
        WAVE_BANK_DESTROYED | WAVE_BANK_PREPARED | WAVE_BANK_STREAMING_INVALID_CONTENT => {
            NOTIFY_WAVE_BANK
        }
        SOUND_BANK_DESTROYED => NOTIFY_SOUND_BANK,
        WAVE_PLAY | WAVE_STOP | WAVE_LOOPED => {
            NOTIFY_SOUND_BANK | NOTIFY_CUE_INDEX | NOTIFY_CUE | NOTIFY_WAVE_BANK | NOTIFY_WAVE
        }
        WAVE_PREPARED | WAVE_DESTROYED => NOTIFY_WAVE_BANK | NOTIFY_WAVE_INDEX | NOTIFY_WAVE,
        _ => 0,
    }
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
