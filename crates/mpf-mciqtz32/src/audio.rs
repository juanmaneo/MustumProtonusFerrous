//! The volume that `MCIQTZ_mciSetAudio` sets.

use crate::abi::{
    MCIERR_OUTOFRANGE, MCI_DGV_SETAUDIO_ITEM, MCI_DGV_SETAUDIO_VALUE, MCI_DGV_SETAUDIO_VOLUME,
    MCI_TEST,
};

/// The largest value `MCI_SETAUDIO` accepts for the volume.
pub const MAX_VOLUME: u32 = 1000;

/// The DirectShow volume in hundredths of a decibel for an MCI value from 0 to [MAX_VOLUME].
/// Zero is silence, and other values follow `2000 * (log10(value) - 3)`.
pub fn volume_for(value: u32) -> i32 {
    if value == 0 {
        -10000
    } else {
        (2000.0 * (f64::from(value).log10() - 3.0)) as i32
    }
}

/// `MCI_SETAUDIO`. Only the volume item with a value is acted on. Any other request returns
/// `Ok(None)`. Values above [MAX_VOLUME] return [MCIERR_OUTOFRANGE], even with `MCI_TEST`. The
/// caller passes `Some(volume)` to `put_Volume` and maps its result with `com_status`.
pub fn setaudio(flags: u32, item: u32, value: u32) -> Result<Option<i32>, u32> {
    if flags & MCI_DGV_SETAUDIO_ITEM == 0
        || item != MCI_DGV_SETAUDIO_VOLUME
        || flags & MCI_DGV_SETAUDIO_VALUE == 0
    {
        return Ok(None);
    }
    if value > MAX_VOLUME {
        return Err(MCIERR_OUTOFRANGE);
    }
    if flags & MCI_TEST != 0 {
        return Ok(None);
    }
    Ok(Some(volume_for(value)))
}
