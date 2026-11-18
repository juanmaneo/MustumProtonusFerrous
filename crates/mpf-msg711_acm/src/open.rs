//! The stream open checks (`G711_StreamOpen`), which pick the convert loop.

use crate::convert::Convert;
use crate::format::get_format_index;
use crate::{Error, WaveFormat, WAVE_FORMAT_ALAW, WAVE_FORMAT_MULAW, WAVE_FORMAT_PCM};

pub const ACM_STREAMOPENF_ASYNC: u32 = 2;

/// `G711_StreamOpen`. Both formats must be listed. Only the G.711 to 16-bit
/// PCM and PCM to G.711 directions, and the A-law to mu-law directions, have
/// convert loops. Those need the same sample rate and channel count, and the
/// PCM side must be 16-bit. Other pairs, including PCM to PCM, are
/// `NotSupported`. Wine asserts that the stream is not asynchronous, and so
/// does this function.
pub fn stream_open(fdw_open: u32, src: &WaveFormat, dst: &WaveFormat) -> Result<Convert, Error> {
    assert!(
        (fdw_open & ACM_STREAMOPENF_ASYNC) == 0,
        "msg711.acm does not support asynchronous streams"
    );
    if get_format_index(src).is_none() || get_format_index(dst).is_none() {
        return Err(Error::NotPossible);
    }
    let same_layout = src.samples_per_sec == dst.samples_per_sec && src.channels == dst.channels;
    let convert = match (src.format_tag, dst.format_tag) {
        (WAVE_FORMAT_ALAW, WAVE_FORMAT_PCM) if same_layout && dst.bits_per_sample == 16 => {
            Convert::AlawToPcm16
        }
        (WAVE_FORMAT_PCM, WAVE_FORMAT_ALAW) if same_layout && src.bits_per_sample == 16 => {
            Convert::PcmToAlaw
        }
        (WAVE_FORMAT_MULAW, WAVE_FORMAT_PCM) if same_layout && dst.bits_per_sample == 16 => {
            Convert::MulawToPcm16
        }
        (WAVE_FORMAT_PCM, WAVE_FORMAT_MULAW) if same_layout && src.bits_per_sample == 16 => {
            Convert::PcmToMulaw
        }
        (WAVE_FORMAT_MULAW, WAVE_FORMAT_ALAW) if same_layout => Convert::MulawToAlaw,
        (WAVE_FORMAT_ALAW, WAVE_FORMAT_MULAW) if same_layout => Convert::AlawToMulaw,
        _ => return Err(Error::NotSupported),
    };
    Ok(convert)
}
