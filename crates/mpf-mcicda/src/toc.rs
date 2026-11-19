//! The `CDROM_TOC` table: track addresses, the lead-out, the disc serial and the data-track skip.

use crate::frame::Msf;

/// `MAXIMUM_NUMBER_TRACKS`.
const MAXIMUM_NUMBER_TRACKS: usize = 100;
/// The `Control & 4` bit that marks a data track.
const CONTROL_DATA: u8 = 0x04;

/// One `TRACKDATA` entry.
#[derive(Clone, Copy, Default)]
pub struct TrackData {
    pub control: u8,
    pub address: Msf,
}

impl TrackData {
    pub fn is_data_track(self) -> bool {
        (self.control & CONTROL_DATA) != 0
    }
}

/// `CDROM_TOC`. Track numbers index the table relative to `first_track`, as `FRAME_OF_TOC` does.
#[derive(Clone)]
pub struct Toc {
    pub first_track: u8,
    pub last_track: u8,
    tracks: [TrackData; MAXIMUM_NUMBER_TRACKS],
}

impl Toc {
    /// A table whose entries start at `first_track`. Entries past the given slice are zero.
    pub fn new(first_track: u8, last_track: u8, entries: &[TrackData]) -> Self {
        let mut tracks = [TrackData::default(); MAXIMUM_NUMBER_TRACKS];
        for (slot, &entry) in tracks.iter_mut().zip(entries) {
            *slot = entry;
        }
        Self {
            first_track,
            last_track,
            tracks,
        }
    }

    /// The entry at `index`. An index past the table reads a zero entry, where the C code reads
    /// out of bounds.
    fn entry(&self, index: u32) -> TrackData {
        self.tracks.get(index as usize).copied().unwrap_or_default()
    }

    pub fn track_data(&self, track: u32) -> TrackData {
        self.entry(track.wrapping_sub(u32::from(self.first_track)))
    }

    /// `FRAME_OF_TOC`.
    pub fn frame_of(&self, track: u32) -> u32 {
        self.track_data(track).address.frames()
    }

    pub fn first_frame(&self) -> u32 {
        self.frame_of(u32::from(self.first_track))
    }

    /// `FRAME_OF_TOC(toc, LastTrack + 1)`.
    pub fn lead_out_frame(&self) -> u32 {
        self.frame_of(u32::from(self.last_track) + 1)
    }

    pub fn contains_track(&self, track: u32) -> bool {
        (u32::from(self.first_track)..=u32::from(self.last_track)).contains(&track)
    }

    /// `LastTrack - FirstTrack + 1`. A malformed table gives a zero or negative count.
    pub fn track_count(&self) -> i32 {
        i32::from(self.last_track) - i32::from(self.first_track) + 1
    }

    /// `CDROM_Audio_GetSerial`.
    pub fn serial(&self) -> u32 {
        let mut serial = 0u32;
        for index in 0..self.track_count() {
            let address = self.entry(index as u32).address;
            serial = serial.wrapping_add(
                (u32::from(address.minute) << 16)
                    | (u32::from(address.second) << 8)
                    | u32::from(address.frame),
            );
        }
        if self.track_count() < 3 {
            let magic = u32::from(self.entry(0).address.frame);
            let length = self.lead_out_frame().wrapping_sub(self.first_frame());
            serial = serial.wrapping_add(magic.wrapping_add(length));
        }
        serial
    }

    /// `MCICDA_SkipDataTracks` on the table: the frame to seek to for `frame`, or `None` when
    /// `frame` lies outside the disc.
    pub fn skip_data_tracks(&self, frame: u32) -> Option<u32> {
        if !(self.first_frame()..self.lead_out_frame()).contains(&frame) {
            return None;
        }
        let first = u32::from(self.first_track);
        let last = u32::from(self.last_track);
        let mut track = last + 1;
        while track > first && self.frame_of(track) > frame {
            track -= 1;
        }
        while track <= last && self.track_data(track).is_data_track() {
            track += 1;
        }
        let mut frame = frame.max(self.frame_of(track));
        if frame == self.lead_out_frame() {
            frame -= 1;
        }
        Some(frame)
    }
}
