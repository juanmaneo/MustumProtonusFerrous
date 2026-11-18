//! `ACMDM_STREAM_OPEN` checks for the MPEG Layer-3 decoder. The libmpg123
//! handle that Wine creates on success is not modelled.

use crate::format::get_format_index;
use crate::{Error, WaveFormat, WAVE_FORMAT_MPEG, WAVE_FORMAT_MPEGLAYER3, WAVE_FORMAT_PCM};

pub const ACM_STREAMOPENF_ASYNC: u32 = 2;

const MPEGLAYER3_WFX_EXTRA_BYTES: u16 = 12;
const MPEGLAYER3_ID_MPEG: u16 = 1;

/// `MPEG3_StreamOpen`: checks that a stream from `src` to `dst` can be
/// decoded. The output must be 16-bit PCM with the input's rate and channels.
///
/// # Panics
///
/// Panics on an asynchronous stream, as Wine's `assert` does.
pub fn stream_open(fdw_open: u32, src: &WaveFormat, dst: &WaveFormat) -> Result<(), Error> {
    assert!(
        (fdw_open & ACM_STREAMOPENF_ASYNC) == 0,
        "asynchronous streams are not supported"
    );
    if get_format_index(src).is_none() || get_format_index(dst).is_none() {
        return Err(Error::NotPossible);
    }
    if dst.format_tag != WAVE_FORMAT_PCM {
        return Err(Error::NotSupported);
    }
    if src.format_tag != WAVE_FORMAT_MPEGLAYER3 && src.format_tag != WAVE_FORMAT_MPEG {
        return Err(Error::NotSupported);
    }
    if src.format_tag == WAVE_FORMAT_MPEGLAYER3
        && (src.cb_size < MPEGLAYER3_WFX_EXTRA_BYTES || src.mpeg_id != MPEGLAYER3_ID_MPEG)
    {
        return Err(Error::NotPossible);
    }
    if src.samples_per_sec != dst.samples_per_sec
        || src.channels != dst.channels
        || dst.bits_per_sample != 16
    {
        return Err(Error::NotSupported);
    }
    Ok(())
}
