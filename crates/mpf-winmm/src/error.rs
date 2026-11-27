//! The error text of `midiOutGetErrorTextA/W` and `waveOutGetErrorTextA/W` from `dlls/winmm/winmm.c`
//! and `dlls/winmm/waveform.c`, with the STRINGTABLE rows of `dlls/winmm/winmm.rc`.

use crate::acp::wide_to_acp_z;

/// `MMSYSERR_NOERROR`, from `include/mmsystem.h`.
pub const MMSYSERR_NOERROR: u32 = 0;
pub const MMSYSERR_ERROR: u32 = 1;
pub const MMSYSERR_BADDEVICEID: u32 = 2;
pub const MMSYSERR_NOTENABLED: u32 = 3;
pub const MMSYSERR_ALLOCATED: u32 = 4;
pub const MMSYSERR_INVALHANDLE: u32 = 5;
pub const MMSYSERR_NODRIVER: u32 = 6;
pub const MMSYSERR_NOMEM: u32 = 7;
pub const MMSYSERR_NOTSUPPORTED: u32 = 8;
pub const MMSYSERR_BADERRNUM: u32 = 9;
pub const MMSYSERR_INVALFLAG: u32 = 10;
pub const MMSYSERR_INVALPARAM: u32 = 11;
pub const MMSYSERR_HANDLEBUSY: u32 = 12;
pub const MMSYSERR_LASTERROR: u32 = 21;

pub const WAVERR_BASE: u32 = 32;
pub const WAVERR_BADFORMAT: u32 = 32;
pub const WAVERR_STILLPLAYING: u32 = 33;
pub const WAVERR_UNPREPARED: u32 = 34;
pub const WAVERR_SYNC: u32 = 35;
pub const WAVERR_LASTERROR: u32 = 35;

pub const MIDIERR_BASE: u32 = 64;
pub const MIDIERR_UNPREPARED: u32 = 64;
pub const MIDIERR_STILLPLAYING: u32 = 65;
pub const MIDIERR_NOMAP: u32 = 66;
pub const MIDIERR_NOTREADY: u32 = 67;
pub const MIDIERR_NODEVICE: u32 = 68;
pub const MIDIERR_INVALIDSETUP: u32 = 69;
pub const MIDIERR_BADOPENMODE: u32 = 70;
pub const MIDIERR_LASTERROR: u32 = 71;

/// The text that `LoadStringW` finds for `id`. Ids without a STRINGTABLE row have no text.
pub fn error_text(id: u32) -> Option<&'static str> {
    match id {
        MMSYSERR_NOERROR => Some("The specified command was carried out."),
        MMSYSERR_ERROR => Some("Undefined external error."),
        MMSYSERR_BADDEVICEID => {
            Some("A device ID has been used that is out of range for your system.")
        }
        MMSYSERR_NOTENABLED => Some("The driver was not enabled."),
        MMSYSERR_ALLOCATED => Some(
            "The specified device is already in use. Wait until it is free, and then try again.",
        ),
        MMSYSERR_INVALHANDLE => Some("The specified device handle is invalid."),
        MMSYSERR_NODRIVER => Some("There is no driver installed on your system!"),
        MMSYSERR_NOMEM => Some(
            "Not enough memory available for this task. Quit one or more applications to increase available memory, and then try again.",
        ),
        MMSYSERR_NOTSUPPORTED => Some(
            "This function is not supported. Use the Capabilities function to determine which functions and messages the driver supports.",
        ),
        MMSYSERR_BADERRNUM => Some("An error number was specified that is not defined in the system."),
        MMSYSERR_INVALFLAG => Some("An invalid flag was passed to a system function."),
        MMSYSERR_INVALPARAM => Some("An invalid parameter was passed to a system function."),
        WAVERR_BADFORMAT => Some(
            "The specified format is not supported or cannot be translated. Use the Capabilities function to determine the supported formats.",
        ),
        WAVERR_STILLPLAYING => Some(
            "Cannot perform this operation while media data is still playing. Reset the device, or wait until the data is finished playing.",
        ),
        WAVERR_UNPREPARED => Some(
            "The wave header was not prepared. Use the Prepare function to prepare the header, and then try again.",
        ),
        WAVERR_SYNC => Some(
            "Cannot open the device without using the WAVE_ALLOWSYNC flag. Use the flag, and then try again.",
        ),
        MIDIERR_UNPREPARED => Some(
            "The MIDI header was not prepared. Use the Prepare function to prepare the header, and then try again.",
        ),
        MIDIERR_STILLPLAYING => Some(
            "Cannot perform this operation while media data is still playing. Reset the device, or wait until the data is finished playing.",
        ),
        MIDIERR_NOMAP => Some(
            "A MIDI map was not found. There may be a problem with the driver, or the MIDIMAP.CFG file may be corrupt or missing.",
        ),
        MIDIERR_NOTREADY => Some(
            "The port is transmitting data to the device. Wait until the data has been transmitted, and then try again.",
        ),
        MIDIERR_NODEVICE => Some(
            "The current MIDI Mapper setup refers to a MIDI device that is not installed on the system.",
        ),
        MIDIERR_INVALIDSETUP => Some(
            "The current MIDI setup is damaged. Copy the original MIDIMAP.CFG file to the Windows SYSTEM directory, and then try again.",
        ),
        _ => None,
    }
}

/// `LoadStringW` into `buf`: copies at most `buf.len() - 1` units and NUL-terminates. Returns the
/// number of units copied, which is 0 when `id` has no text. `buf` must not be empty.
fn load_string_w(id: u32, buf: &mut [u16]) -> u32 {
    let units: Vec<u16> = error_text(id)
        .unwrap_or("")
        .encode_utf16()
        .take(buf.len() - 1)
        .collect();
    buf[..units.len()].copy_from_slice(&units);
    buf[units.len()] = 0;
    units.len() as u32
}

/// The W body shared by the four error-text functions. A NULL buffer is `MMSYSERR_INVALPARAM` and
/// an empty one is `MMSYSERR_NOERROR`. An id outside the MMSYSERR range and `base..=last`, or one
/// without text, is `MMSYSERR_BADERRNUM`.
fn get_error_text_w(error: u32, dst: Option<&mut [u16]>, base: u32, last: u32) -> u32 {
    let Some(dst) = dst else {
        return MMSYSERR_INVALPARAM;
    };
    if dst.is_empty() {
        return MMSYSERR_NOERROR;
    }
    let in_range = error <= MMSYSERR_LASTERROR || (base..=last).contains(&error);
    if in_range && load_string_w(error, dst) > 0 {
        MMSYSERR_NOERROR
    } else {
        MMSYSERR_BADERRNUM
    }
}

/// The A body shared by the error-text functions. `alloc` stands in for the `malloc` of `dst.len()`
/// wide units, and returns `None` when that allocation fails. The converted text is written only
/// when the W body succeeds.
pub(crate) fn get_error_text_a(
    error: u32,
    dst: Option<&mut [u8]>,
    base: u32,
    last: u32,
    alloc: impl FnOnce(usize) -> Option<Vec<u16>>,
) -> u32 {
    let Some(dst) = dst else {
        return MMSYSERR_INVALPARAM;
    };
    if dst.is_empty() {
        return MMSYSERR_NOERROR;
    }
    let Some(mut wide) = alloc(dst.len()) else {
        return MMSYSERR_NOMEM;
    };
    let ret = get_error_text_w(error, Some(wide.as_mut_slice()), base, last);
    if ret == MMSYSERR_NOERROR {
        wide_to_acp_z(&wide, dst);
    }
    ret
}

/// `midiOutGetErrorTextW`.
pub fn midi_out_get_error_text_w(error: u32, dst: Option<&mut [u16]>) -> u32 {
    get_error_text_w(error, dst, MIDIERR_BASE, MIDIERR_LASTERROR)
}

/// `midiOutGetErrorTextA`. The wide buffer comes from `Vec`, which aborts rather than failing, so
/// this wrapper never returns `MMSYSERR_NOMEM`.
pub fn midi_out_get_error_text_a(error: u32, dst: Option<&mut [u8]>) -> u32 {
    get_error_text_a(error, dst, MIDIERR_BASE, MIDIERR_LASTERROR, |len| {
        Some(vec![0; len])
    })
}

/// `waveOutGetErrorTextW`.
pub fn wave_out_get_error_text_w(error: u32, dst: Option<&mut [u16]>) -> u32 {
    get_error_text_w(error, dst, WAVERR_BASE, WAVERR_LASTERROR)
}

/// `waveOutGetErrorTextA`. As with [`midi_out_get_error_text_a`], the wide buffer cannot fail.
pub fn wave_out_get_error_text_a(error: u32, dst: Option<&mut [u8]>) -> u32 {
    get_error_text_a(error, dst, WAVERR_BASE, WAVERR_LASTERROR, |len| {
        Some(vec![0; len])
    })
}
