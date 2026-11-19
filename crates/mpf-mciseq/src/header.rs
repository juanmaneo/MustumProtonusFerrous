use crate::event::{read_mtrk, Track};
use crate::mmio::{MmioFile, Whence};
use crate::{MCIERR_INVALID_FILE, MCI_FORMAT_MILLISECONDS};

/// The `MThd` chunk and the `MTrk` headers that `MIDI_mciReadMThd` stores in the device.
pub struct Header {
    pub format: u16,
    pub division: u16,
    pub time_format: u32,
    pub tempo: u32,
    pub tracks: Vec<Track>,
}

/// `MIDI_mciReadMThd`: reads the header chunk at `offset`, then the `MTrk` header of each track.
/// SMPTE divisions are rejected, as in Wine, so the time format is always milliseconds. A format 0
/// file has one track, and more than 0x80 tracks are cut to 0x80.
pub fn read_mthd(file: &mut MmioFile, offset: u32) -> Result<Header, u32> {
    if file.seek(offset as i32, Whence::Set) as u32 != offset {
        return Err(MCIERR_INVALID_FILE);
    }
    let mut fourcc = [0u8; 4];
    if file.read(&mut fourcc) != fourcc.len() || &fourcc != b"MThd" {
        return Err(MCIERR_INVALID_FILE);
    }
    let toberead = file.read_long().ok_or(MCIERR_INVALID_FILE)?;
    if toberead < 6 {
        return Err(MCIERR_INVALID_FILE);
    }
    let format = file.read_word().ok_or(MCIERR_INVALID_FILE)?;
    let ntracks = file.read_word().ok_or(MCIERR_INVALID_FILE)?;
    let division = file.read_word().ok_or(MCIERR_INVALID_FILE)?;
    if division == 0 || division > 0x8000 {
        return Err(MCIERR_INVALID_FILE);
    }
    let ntracks = match format {
        0 => 1,
        1 | 2 => ntracks.min(0x80),
        _ => return Err(MCIERR_INVALID_FILE),
    };
    let extra = toberead - 6;
    if extra > 0 {
        file.seek(extra as i32, Whence::Cur);
    }
    let mut tracks = vec![Track::default(); usize::from(ntracks)];
    for track in &mut tracks {
        read_mtrk(file, track)?;
    }
    Ok(Header {
        format,
        division,
        time_format: MCI_FORMAT_MILLISECONDS,
        tempo: 500_000,
        tracks,
    })
}
