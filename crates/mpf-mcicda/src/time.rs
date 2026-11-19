//! `MCICDA_CalcFrame` and `MCICDA_CalcTime`: the conversions between MCI times and frame
//! addresses. The TMSF forms read the table of contents.

use crate::device::CdRom;
use crate::frame::{pack_tmsf, unpack_tmsf, Msf, FRAMES_PER_SECOND};
use crate::mci::{
    MCI_COLONIZED3_RETURN, MCI_COLONIZED4_RETURN, MCI_FORMAT_MILLISECONDS, MCI_FORMAT_MSF,
};

/// `MCICDA_CalcFrame`: the frame that `time` names in time format `tf`. Zero when the TMSF track
/// is not on the disc.
pub fn calc_frame(dev: &mut dyn CdRom, tf: u32, time: u32) -> u32 {
    match tf {
        MCI_FORMAT_MILLISECONDS => {
            time.wrapping_sub(1)
                .wrapping_mul(FRAMES_PER_SECOND)
                .wrapping_add(500)
                / 1000
        }
        MCI_FORMAT_MSF => Msf::from_mci_msf(time).frames(),
        _ => {
            let (track, offset) = unpack_tmsf(time);
            let Ok(toc) = dev.read_toc() else {
                return 0;
            };
            if !toc.contains_track(u32::from(track)) {
                return 0;
            }
            toc.frame_of(u32::from(track)) + offset.frames()
        }
    }
}

/// `MCICDA_CalcTime`: the MCI time of `frame` in time format `tf`, with the return flag that the C
/// code sets alongside it. Zero when a TMSF frame is outside the disc.
pub fn calc_time(dev: &mut dyn CdRom, tf: u32, frame: u32) -> (u32, u32) {
    match tf {
        MCI_FORMAT_MILLISECONDS => (frame.wrapping_mul(1000) / FRAMES_PER_SECOND + 1, 0),
        MCI_FORMAT_MSF => (Msf::from_frames(frame).to_mci_msf(), MCI_COLONIZED3_RETURN),
        _ => calc_tmsf(dev, frame),
    }
}

/// The TMSF branch of `MCICDA_CalcTime`: the track that holds `frame`, and the offset into it.
fn calc_tmsf(dev: &mut dyn CdRom, frame: u32) -> (u32, u32) {
    let Ok(toc) = dev.read_toc() else {
        return (0, 0);
    };
    if !(toc.first_frame()..=toc.lead_out_frame()).contains(&frame) {
        return (0, 0);
    }
    let last = u32::from(toc.last_track);
    let mut track = u32::from(toc.first_track);
    while track <= last && toc.frame_of(track) <= frame {
        track += 1;
    }
    track -= 1;
    let offset = Msf::from_frames(frame.wrapping_sub(toc.frame_of(track)));
    (pack_tmsf(track as u8, offset), MCI_COLONIZED4_RETURN)
}
