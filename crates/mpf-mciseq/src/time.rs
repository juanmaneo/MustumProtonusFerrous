use crate::event::Track;
use crate::header::Header;
use crate::{
    MCI_FORMAT_MILLISECONDS, MCI_FORMAT_SMPTE_24, MCI_FORMAT_SMPTE_25, MCI_FORMAT_SMPTE_30,
};

const MS_PER_HOUR: u32 = 3_600_000;
const MS_PER_MINUTE: u32 = 60_000;
const MS_PER_SECOND: u32 = 1_000;

/// `MIDI_ConvertPulseToMS`: converts pulses to milliseconds at `tempo` microseconds per quarter note.
/// A zero division gives 0. In an SMPTE division the high byte is the negated frame rate and the
/// low byte the sub-frames per frame. Wine divides by zero when the sub-frame count is 0; here that
/// gives 0.
pub fn pulse_to_ms(division: u16, tempo: u32, pulse: u32) -> u32 {
    if division == 0 {
        0
    } else if division > 0x8000 {
        let frames = 256 - u32::from(division >> 8);
        let sub_frames = u32::from(division & 0xFF);
        pulse
            .wrapping_mul(1000)
            .checked_div(frames * sub_frames)
            .unwrap_or(0)
    } else {
        (f64::from(pulse) * (f64::from(tempo) / 1000.0) / f64::from(division)) as u32
    }
}

/// `MIDI_ConvertTimeFormatToMS`: converts an MCI time value to milliseconds. In the SMPTE formats
/// the bytes of `val` hold frames (top), seconds, minutes and hours (bottom). An unknown format gives 0.
pub fn time_format_to_ms(time_format: u32, val: u32) -> u32 {
    let frames = val >> 24;
    let seconds = (val >> 16) & 0xFF;
    let minutes = (val >> 8) & 0xFF;
    let hours = val & 0xFF;
    let clock = seconds * MS_PER_SECOND + minutes * MS_PER_MINUTE + hours * MS_PER_HOUR;
    match time_format {
        MCI_FORMAT_MILLISECONDS => val,
        MCI_FORMAT_SMPTE_24 => (frames * 125) / 3 + clock,
        MCI_FORMAT_SMPTE_25 => frames * 40 + clock,
        MCI_FORMAT_SMPTE_30 => (frames * 100) / 3 + clock,
        _ => 0,
    }
}

/// `MIDI_ConvertMSToTimeFormat`: packs milliseconds into the SMPTE layout of `time_format`. The frame
/// count comes from the milliseconds left in the minute, as in Wine, not from the second. The hour
/// byte is not masked, so a large hour count overlaps the minute byte. An unknown format gives 0.
pub fn ms_to_time_format(time_format: u32, ms: u32) -> u32 {
    let hours = ms / MS_PER_HOUR;
    let in_hour = ms - hours * MS_PER_HOUR;
    let minutes = in_hour / MS_PER_MINUTE;
    let in_minute = in_hour - minutes * MS_PER_MINUTE;
    let seconds = in_minute / MS_PER_SECOND;
    let pack = |frames: u32| (frames << 24) | (seconds << 16) | (minutes << 8) | hours;
    match time_format {
        MCI_FORMAT_MILLISECONDS => ms,
        MCI_FORMAT_SMPTE_24 => pack(in_minute * 3 / 125),
        MCI_FORMAT_SMPTE_25 => pack(in_minute / 40),
        MCI_FORMAT_SMPTE_30 => pack(in_minute * 3 / 100),
        _ => 0,
    }
}

/// `MIDI_GetMThdLengthMS`: the length in ms of the file. Format 2 tracks play one after another, so
/// their pulse counts add up; in other formats the longest track gives the length.
pub fn mthd_length_ms(header: &Header) -> u32 {
    let pulses = if header.format == 2 {
        header
            .tracks
            .iter()
            .fold(0u32, |sum, track| sum.wrapping_add(track.length))
    } else {
        header
            .tracks
            .iter()
            .fold(0u32, |longest, track| longest.max(track.length))
    };
    pulse_to_ms(header.division, header.tempo, pulses)
}

/// `MIDI_mciFindNextEvent`: the playing track whose next event comes first. Ties go to the lower
/// index. Returns the index, or `None` when no track plays, with the pulse (`u32::MAX` if none).
pub fn find_next_event(tracks: &[Track]) -> (Option<usize>, u32) {
    let mut best = (None, u32::MAX);
    for (index, track) in tracks.iter().enumerate() {
        if track.status && track.event_pulse < best.1 {
            best = (Some(index), track.event_pulse);
        }
    }
    best
}
