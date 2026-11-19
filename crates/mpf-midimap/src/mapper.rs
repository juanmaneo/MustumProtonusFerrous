//! The `MIDIHDR` and volume handlers of Wine's `midimap.c`: `modPrepare`, `modUnprepare` and
//! `modGetVolume`. They need no port state, only the instance's existence, so they are free
//! functions. The [`Driver`](crate::Driver) that dispatches to them is at the crate root.

use crate::Instance;
use crate::{
    MidiHdr, MHDR_DONE, MHDR_INQUEUE, MHDR_PREPARED, MIDIERR_STILLPLAYING, MMSYSERR_ERROR,
    MMSYSERR_INVALPARAM, MMSYSERR_NOERROR,
};

/// `offsetof(MIDIHDR, dwOffset)`: the smallest `MIDIHDR` size that `prepare` accepts.
const MIDIHDR_DWOFFSET: usize = 44;

/// `modPrepare`: marks the header prepared, and clears `MHDR_DONE` and `MHDR_INQUEUE`.
pub fn prepare(inst: Option<&Instance>, hdr: Option<&mut MidiHdr>, size: usize) -> u32 {
    let hdr = match header_args(inst, hdr, size) {
        Ok(hdr) => hdr,
        Err(ret) => return ret,
    };
    if (hdr.flags & MHDR_PREPARED) != 0 {
        return MMSYSERR_NOERROR;
    }
    hdr.flags = (hdr.flags | MHDR_PREPARED) & !(MHDR_DONE | MHDR_INQUEUE);
    MMSYSERR_NOERROR
}

/// `modUnprepare`: clears `MHDR_PREPARED`, unless the header is still queued.
pub fn unprepare(inst: Option<&Instance>, hdr: Option<&mut MidiHdr>, size: usize) -> u32 {
    let hdr = match header_args(inst, hdr, size) {
        Ok(hdr) => hdr,
        Err(ret) => return ret,
    };
    if (hdr.flags & MHDR_PREPARED) == 0 {
        return MMSYSERR_NOERROR;
    }
    if (hdr.flags & MHDR_INQUEUE) != 0 {
        return MIDIERR_STILLPLAYING;
    }
    hdr.flags &= !MHDR_PREPARED;
    MMSYSERR_NOERROR
}

/// `modGetVolume`: reports `0xFFFFFFFF`, the value C reports before any `MODM_SETVOLUME`.
pub fn get_volume(volume: Option<&mut u32>) -> u32 {
    let Some(volume) = volume else {
        return MMSYSERR_INVALPARAM;
    };
    *volume = 0xFFFF_FFFF;
    MMSYSERR_NOERROR
}

fn header_args<'a>(
    inst: Option<&Instance>,
    hdr: Option<&'a mut MidiHdr>,
    size: usize,
) -> Result<&'a mut MidiHdr, u32> {
    if inst.is_none() {
        return Err(MMSYSERR_ERROR);
    }
    match hdr {
        Some(hdr) if size >= MIDIHDR_DWOFFSET && hdr.has_data => Ok(hdr),
        _ => Err(MMSYSERR_INVALPARAM),
    }
}
