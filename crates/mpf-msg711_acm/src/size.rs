//! The `ACMDM_STREAM_SIZE` query (`G711_StreamSize`): the length that a
//! stream converts to, in either direction.

use crate::{Error, WaveFormat, WAVE_FORMAT_ALAW, WAVE_FORMAT_MULAW, WAVE_FORMAT_PCM};

pub const ACM_STREAMSIZEF_SOURCE: u32 = 0;
pub const ACM_STREAMSIZEF_DESTINATION: u32 = 1;

fn is_law(format_tag: u16) -> bool {
    format_tag == WAVE_FORMAT_ALAW || format_tag == WAVE_FORMAT_MULAW
}

/// `G711_StreamSize`. For `ACM_STREAMSIZEF_DESTINATION`, `length` is the
/// destination length and the result is the source length. For
/// `ACM_STREAMSIZEF_SOURCE` it is the reverse. The arithmetic wraps as Wine's
/// `DWORD` does. PCM to PCM and unknown queries are `NotSupported`.
pub fn stream_size(
    query: u32,
    src: &WaveFormat,
    dst: &WaveFormat,
    length: u32,
) -> Result<u32, Error> {
    let (src_tag, dst_tag) = (src.format_tag, dst.format_tag);
    match query {
        ACM_STREAMSIZEF_DESTINATION => {
            if src_tag == WAVE_FORMAT_PCM && is_law(dst_tag) {
                Ok(length.wrapping_mul(2))
            } else if is_law(src_tag) && dst_tag == WAVE_FORMAT_PCM {
                Ok(length / 2)
            } else if is_law(src_tag) && is_law(dst_tag) {
                Ok(length)
            } else {
                Err(Error::NotSupported)
            }
        }
        ACM_STREAMSIZEF_SOURCE => {
            if src_tag == WAVE_FORMAT_PCM && is_law(dst_tag) {
                Ok(length / 2)
            } else if is_law(src_tag) && dst_tag == WAVE_FORMAT_PCM {
                Ok(length.wrapping_mul(2))
            } else if is_law(src_tag) && is_law(dst_tag) {
                Ok(length)
            } else {
                Err(Error::NotSupported)
            }
        }
        _ => Err(Error::NotSupported),
    }
}
