//! CD-ROM table of contents helpers: `frame_to_msf` and `track_to_frame` from
//! `cdrom.c`, and `VOLUME_GetAudioCDSerial` from `device.c`.

const CD_SECS: u32 = 60;
const CD_FRAMES: u32 = 75;

/// The fields of a `CDROM_TOC` that the helpers read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Toc {
    pub first_track: u8,
    pub last_track: u8,
    /// The `Address` field of each `TRACK_DATA` entry, from `first_track` through
    /// the lead-out at `last_track + 1`.
    pub addresses: Vec<[u8; 4]>,
}

/// Converts a frame count to minutes, seconds and frames. Minutes wrap to 8 bits, as in C.
pub fn frame_to_msf(frame: u32) -> [u8; 3] {
    let frames = frame % CD_FRAMES;
    let seconds = (frame / CD_FRAMES) % CD_SECS;
    let minutes = frame / CD_FRAMES / CD_SECS;
    [minutes as u8, seconds as u8, frames as u8]
}

/// Returns the absolute frame where `track` starts, or `None` when the TOC has no entry for it.
pub fn track_to_frame(toc: &Toc, track: u8) -> Option<u32> {
    let index = usize::from(track.checked_sub(toc.first_track)?);
    toc.addresses
        .get(index)
        .map(|&address| frame_of_address(address))
}

/// Computes the serial of an audio CD from its TOC, as `VOLUME_GetAudioCDSerial`.
///
/// The serial adds each audio track's minutes, seconds and frames, packed into 24 bits.
/// A disc with fewer than three tracks also adds its length in frames.
pub fn audio_cd_serial(toc: &Toc) -> Option<u32> {
    let tracks = usize::from(toc.last_track.checked_sub(toc.first_track)?) + 1;
    let audio = toc.addresses.get(..tracks)?;
    let mut serial = audio
        .iter()
        .fold(0u32, |serial, &[_, minutes, seconds, frames]| {
            serial.wrapping_add(
                (u32::from(minutes) << 16) | (u32::from(seconds) << 8) | u32::from(frames),
            )
        });
    if tracks < 3 {
        let lead_out = toc.addresses.get(tracks)?;
        let start = frame_of_address(toc.addresses[0]);
        serial = serial.wrapping_add(frame_of_address(*lead_out).wrapping_sub(start));
    }
    Some(serial)
}

fn frame_of_address(address: [u8; 4]) -> u32 {
    (u32::from(address[1]) * CD_SECS + u32::from(address[2])) * CD_FRAMES + u32::from(address[3])
}
