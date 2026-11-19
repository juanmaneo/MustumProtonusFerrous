use crate::mmio::{MmioFile, Whence};
use crate::{MCIERR_INTERNAL, MCIERR_INVALID_FILE};

/// The per-track state of `MCI_MIDITRACK`. The track number is not kept.
#[derive(Clone, Default)]
pub struct Track {
    /// Offset of the first event byte.
    pub first: u32,
    /// Offset one past the last byte of the track.
    pub last: u32,
    /// Offset of the next event.
    pub index: u32,
    /// Number of pulses in the track.
    pub length: u32,
    /// Pulse of the current event, counted from the start of the track.
    pub event_pulse: u32,
    /// Current event packed as `(hw << 16) | (b2 << 8) | b1`; see [read_next_event].
    pub event_data: u32,
    /// Byte length of the current event.
    pub event_length: u16,
    /// Whether the track is still playing (`wStatus`).
    pub status: bool,
    /// Last channel status byte, used for running status.
    pub last_command: u8,
}

/// `MIDI_mciReadNextEvent`: reads the event at `track.index`, then sets the pulse, data and length of
/// the event. Running status is handled through `track.last_command`.
///
/// The status byte and a running-status data byte that cannot be read are errors. Wine reads
/// them into an uninitialised variable, so this has no oracle behaviour.
pub fn read_next_event(file: &mut MmioFile, track: &mut Track) -> Result<(), u32> {
    if file.seek(track.index as i32, Whence::Set) as u32 != track.index {
        return Err(MCIERR_INVALID_FILE);
    }
    let (delta_len, delta) = file.read_var_len();
    let mut len = u32::from(delta_len) + 1;
    let mut b1 = file.read_byte().ok_or(MCIERR_INVALID_FILE)?;
    let mut b2 = 0u8;
    let mut hw = 0u16;
    match b1 {
        0xF0 | 0xF7 => {
            let (tl, tmp) = file.read_var_len();
            len = len.wrapping_add(u32::from(tl)).wrapping_add(tmp);
        }
        0xFF => {
            b2 = file.read_byte().unwrap_or(0);
            len += 1;
            let (tl, tmp) = file.read_var_len();
            len += u32::from(tl);
            hw = if len >= 0x10000 { 0xFFFF } else { len as u16 };
            len = len.wrapping_add(tmp);
        }
        _ => {
            if b1 & 0x80 != 0 {
                track.last_command = b1;
                b2 = file.read_byte().unwrap_or(0);
                len += 1;
            } else {
                b2 = b1;
                b1 = track.last_command;
            }
            match (b1 >> 4) & 0x07 {
                0 | 1 | 2 | 3 | 6 => {
                    hw = u16::from(file.read_byte().ok_or(MCIERR_INVALID_FILE)?);
                    len += 1;
                }
                _ => {}
            }
        }
    }
    if track.index.wrapping_add(len) > track.last {
        return Err(MCIERR_INTERNAL);
    }
    track.event_pulse = track.event_pulse.wrapping_add(delta);
    track.event_data = (u32::from(hw) << 16) | (u32::from(b2) << 8) | u32::from(b1);
    track.event_length = len as u16;
    Ok(())
}

/// `MIDI_mciReadMTrk`: reads an `MTrk` chunk header at the file position and scans its events for
/// the track length. Event errors end the scan without an error, as in Wine. The position is left
/// at the end of the track.
pub fn read_mtrk(file: &mut MmioFile, track: &mut Track) -> Result<(), u32> {
    let mut fourcc = [0u8; 4];
    if file.read(&mut fourcc) != fourcc.len() || &fourcc != b"MTrk" {
        return Err(MCIERR_INVALID_FILE);
    }
    let toberead = file.read_long().ok_or(MCIERR_INVALID_FILE)?;
    track.first = file.seek(0, Whence::Cur) as u32;
    track.last = track.first.wrapping_add(toberead);
    track.index = track.first;
    track.event_pulse = 0;

    while read_next_event(file, track).is_ok() && (track.event_data & 0xFFFF) != 0x2FFF {
        track.index = track.index.wrapping_add(u32::from(track.event_length));
        if matches!(track.event_data & 0xFFFF, 0x02FF | 0x03FF) {
            let len = track
                .event_length
                .wrapping_sub((track.event_data >> 16) as u16)
                .min(1023);
            file.skip(usize::from(len));
        }
    }
    track.length = track.event_pulse;
    track.status = true;
    track.index = track.first;
    track.event_pulse = 0;
    if file.seek(0, Whence::Cur) as u32 != track.last {
        file.seek(track.last as i32, Whence::Set);
    }
    Ok(())
}
