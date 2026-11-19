//! Frame arithmetic and the packed `MSF` and `TMSF` values of `mmsystem.h`.

/// `CDFRAMES_PERSEC`.
pub const FRAMES_PER_SECOND: u32 = 75;
/// `CDFRAMES_PERMIN`.
pub const FRAMES_PER_MINUTE: u32 = 60 * FRAMES_PER_SECOND;

/// A disc address.
#[derive(Clone, Copy, Default)]
pub struct Msf {
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
}

impl Msf {
    /// `FRAME_OF_ADDR`.
    pub fn frames(self) -> u32 {
        u32::from(self.minute) * FRAMES_PER_MINUTE
            + u32::from(self.second) * FRAMES_PER_SECOND
            + u32::from(self.frame)
    }

    /// The address of `frames`. The minute wraps to a byte, as `MCI_MAKE_MSF` truncates it.
    pub fn from_frames(frames: u32) -> Self {
        Self {
            minute: (frames / FRAMES_PER_MINUTE) as u8,
            second: ((frames / FRAMES_PER_SECOND) % 60) as u8,
            frame: (frames % FRAMES_PER_SECOND) as u8,
        }
    }

    /// `MCI_MSF_MINUTE`, `MCI_MSF_SECOND` and `MCI_MSF_FRAME`: one byte each, from the low byte.
    pub fn from_mci_msf(value: u32) -> Self {
        Self {
            minute: value as u8,
            second: (value >> 8) as u8,
            frame: (value >> 16) as u8,
        }
    }

    /// `MCI_MAKE_MSF`.
    pub fn to_mci_msf(self) -> u32 {
        u32::from(self.minute) | (u32::from(self.second) << 8) | (u32::from(self.frame) << 16)
    }
}

/// `MCI_MAKE_TMSF`: the track in the low byte, then the offset as an MSF in the upper three.
pub fn pack_tmsf(track: u8, offset: Msf) -> u32 {
    u32::from(track) | (offset.to_mci_msf() << 8)
}

/// `MCI_TMSF_TRACK` and the offset that `MCI_TMSF_MINUTE`, `MCI_TMSF_SECOND` and
/// `MCI_TMSF_FRAME` read from the upper three bytes.
pub fn unpack_tmsf(value: u32) -> (u8, Msf) {
    (value as u8, Msf::from_mci_msf(value >> 8))
}
