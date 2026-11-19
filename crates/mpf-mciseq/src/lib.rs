//! Deterministic part of Wine's `dlls/mciseq/mcimidi.c`, the MCI MIDI sequencer: parsing of
//! MIDI files over an in-memory image, conversion of pulses and MCI time formats to
//! milliseconds, and the argument checks of seek and play.
//!
//! Ported:
//! - mmr2mci: [mmr2mci].
//! - MIDI_mciReadByte, MIDI_mciReadWord, MIDI_mciReadLong, MIDI_mciReadVaryLen: [MmioFile].
//! - MIDI_mciReadNextEvent: [read_next_event]; MIDI_mciReadMTrk: [read_mtrk]; MIDI_mciReadMThd: [read_mthd].
//! - MIDI_ConvertPulseToMS: [pulse_to_ms]; MIDI_ConvertTimeFormatToMS: [time_format_to_ms];
//!   MIDI_ConvertMSToTimeFormat: [ms_to_time_format]; MIDI_GetMThdLengthMS: [mthd_length_ms].
//! - MIDI_mciFindNextEvent: [find_next_event].
//! - MIDI_mciSeek, after its NULL check: [seek_target].
//! - MIDI_mciPlay, before its player thread is started: [play_range].
//!
//! Skipped:
//! - mmioOpen, mmioClose and the element name: file handles.
//! - MIDI_drvOpen, MIDI_drvClose, MIDI_mciGetOpenDev and MCIMIDI_DriverProc: driver state and message routing.
//! - MIDI_mciOpen, MIDI_mciClose, MIDI_mciStop, MIDI_mciPause and MIDI_mciResume: device handles and the player state.
//! - MIDI_player, MIDI_Starter and ensurePlayerThread: the player thread, midiOut calls and timing.
//! - MIDI_mciNotify and the mciDriverNotify calls: the caller's notification window.
//! - MIDI_mciSet, MIDI_mciStatus, MIDI_mciGetDevCaps and MIDI_mciInfo: MCI parameter blocks and device capabilities.
//! - The NULL-parameter check of MIDI_mciSeek and the NULL check of MIDI_mciPlay: the caller passes the block.
//! - Copyright and name text: the text bytes are skipped, the wide strings are not stored.
//! - WARN, FIXME and TRACE output.

mod event;
mod header;
mod mmio;
mod time;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use event::{read_mtrk, read_next_event, Track};
pub use header::{read_mthd, Header};
pub use mmio::{MmioFile, Whence};
pub use time::{
    find_next_event, ms_to_time_format, mthd_length_ms, pulse_to_ms, time_format_to_ms,
};

pub const MCIERR_INVALID_FILE: u32 = 296;
pub const MCIERR_INTERNAL: u32 = 277;
pub const MCIERR_OUT_OF_MEMORY: u32 = 264;
pub const MCIERR_MISSING_PARAMETER: u32 = 273;
pub const MCIERR_FILE_NOT_FOUND: u32 = 275;
pub const MCIERR_OUTOFRANGE: u32 = 282;
pub const MCIERR_FLAGS_NOT_COMPATIBLE: u32 = 284;
pub const MCIERR_SEQ_PORT_INUSE: u32 = 337;
pub const MCIERR_SEQ_PORT_NONEXISTENT: u32 = 338;
pub const MCIERR_SEQ_PORT_MAPNODEVICE: u32 = 339;

pub const MMSYSERR_BADDEVICEID: u32 = 2;
pub const MMSYSERR_ALLOCATED: u32 = 4;
pub const MMSYSERR_NOMEM: u32 = 7;
pub const MIDIERR_INVALIDSETUP: u32 = 69;

pub const MCI_FORMAT_MILLISECONDS: u32 = 0;
pub const MCI_FORMAT_SMPTE_24: u32 = 4;
pub const MCI_FORMAT_SMPTE_25: u32 = 5;
pub const MCI_FORMAT_SMPTE_30: u32 = 6;
pub const MCI_FORMAT_SMPTE_30DROP: u32 = 7;

pub const MCI_WAIT: u32 = 2;
pub const MCI_FROM: u32 = 4;
pub const MCI_TO: u32 = 8;
pub const MCI_SEEK_TO_START: u32 = 256;
pub const MCI_SEEK_TO_END: u32 = 512;

/// `mmr2mci`: maps a MMSYSERR or MIDIERR code to the MCIERR code that MCI reports.
pub fn mmr2mci(ret: u32) -> u32 {
    match ret {
        MMSYSERR_ALLOCATED => MCIERR_SEQ_PORT_INUSE,
        MMSYSERR_NOMEM => MCIERR_OUT_OF_MEMORY,
        MMSYSERR_BADDEVICEID => MCIERR_SEQ_PORT_NONEXISTENT,
        MIDIERR_INVALIDSETUP => MCIERR_SEQ_PORT_MAPNODEVICE,
        other => other,
    }
}

/// The flags and target of `MCI_PLAY` that the range check reads.
pub struct PlayParms {
    pub from: u32,
    pub to: u32,
}

/// The argument checks of `MIDI_mciSeek` and the position it sets, in ms.
/// `MCI_NOTIFY` and the stop of a running player are left to the caller.
pub fn seek_target(flags: u32, to: u32, time_format: u32) -> Result<u32, u32> {
    let position = flags & (MCI_SEEK_TO_START | MCI_SEEK_TO_END | MCI_TO);
    if position == 0 {
        return Err(MCIERR_MISSING_PARAMETER);
    }
    if position & (position - 1) != 0 {
        return Err(MCIERR_FLAGS_NOT_COMPATIBLE);
    }
    Ok(if flags & MCI_TO != 0 {
        time_format_to_ms(time_format, to)
    } else if flags & MCI_SEEK_TO_START != 0 {
        0
    } else {
        u32::MAX
    })
}

/// The range checks of `MIDI_mciPlay`. Returns the start, which becomes the position, and the end,
/// both in ms. The end is `u32::MAX` when no end is given. The player thread is the caller's.
pub fn play_range(
    has_file: bool,
    flags: u32,
    time_format: u32,
    parms: Option<PlayParms>,
    position: u32,
) -> Result<(u32, u32), u32> {
    if !has_file {
        return Err(MCIERR_FILE_NOT_FOUND);
    }
    let end = match parms.as_ref() {
        Some(p) if flags & MCI_TO != 0 => time_format_to_ms(time_format, p.to),
        _ => u32::MAX,
    };
    let start = match parms.as_ref() {
        Some(p) if flags & MCI_FROM != 0 => time_format_to_ms(time_format, p.from),
        _ => position,
    };
    if end < start {
        return Err(MCIERR_OUTOFRANGE);
    }
    Ok((start, end))
}
