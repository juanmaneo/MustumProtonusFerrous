//! `ACMDM_STREAM_SIZE` for the MPEG Layer-3 codec: converts a PCM byte count
//! to its MPEG byte count, or back, as Wine's `MPEG3_StreamSize` does.

use crate::{Error, WaveFormat, WAVE_FORMAT_MPEG, WAVE_FORMAT_MPEGLAYER3, WAVE_FORMAT_PCM};

pub const ACM_STREAMSIZEF_SOURCE: u32 = 0;
pub const ACM_STREAMSIZEF_DESTINATION: u32 = 1;

/// Samples in one MPEG Layer-3 frame.
const SAMPLES_PER_BLOCK: u32 = 1152;
/// The 3000 bytes that Wine adds to or removes from an encoded size.
const ENCODE_OVERHEAD: u32 = 3000;

/// `ACMDRVSTREAMSIZE`: the query, and the byte counts it converts between.
pub struct StreamSize {
    pub fdw_size: u32,
    pub cb_src_length: u32,
    pub cb_dst_length: u32,
}

/// `MPEG3_StreamSize`: given the size named by `size.fdw_size`, sets the other
/// size. Only PCM and MPEG Layer-3 or MPEG-1 streams convert.
pub fn stream_size(src: &WaveFormat, dst: &WaveFormat, size: &mut StreamSize) -> Result<(), Error> {
    match size.fdw_size {
        ACM_STREAMSIZEF_DESTINATION => {
            if src.format_tag == WAVE_FORMAT_PCM && is_mpeg(dst.format_tag) {
                let blocks = whole_blocks(
                    size.cb_dst_length.wrapping_sub(ENCODE_OVERHEAD),
                    frame_bytes(dst)?,
                )?;
                size.cb_src_length = blocks
                    .wrapping_mul(SAMPLES_PER_BLOCK)
                    .wrapping_mul(u32::from(src.block_align));
            } else if is_mpeg(src.format_tag) && dst.format_tag == WAVE_FORMAT_PCM {
                let blocks = whole_blocks(size.cb_dst_length, block_len(dst))?;
                size.cb_src_length = blocks.wrapping_mul(frame_bytes(src)?);
            } else {
                return Err(Error::NotSupported);
            }
        }
        ACM_STREAMSIZEF_SOURCE => {
            if src.format_tag == WAVE_FORMAT_PCM && is_mpeg(dst.format_tag) {
                let blocks = ceil_blocks(size.cb_src_length, block_len(src))?;
                size.cb_dst_length =
                    ENCODE_OVERHEAD.wrapping_add(blocks.wrapping_mul(frame_bytes(dst)?));
            } else if is_mpeg(src.format_tag) && dst.format_tag == WAVE_FORMAT_PCM {
                let blocks = ceil_blocks(size.cb_src_length, frame_bytes(src)?)?;
                size.cb_dst_length = blocks
                    .wrapping_mul(SAMPLES_PER_BLOCK)
                    .wrapping_mul(u32::from(dst.block_align));
            } else {
                return Err(Error::NotSupported);
            }
        }
        _ => return Err(Error::NotSupported),
    }
    Ok(())
}

fn is_mpeg(tag: u16) -> bool {
    tag == WAVE_FORMAT_MPEG || tag == WAVE_FORMAT_MPEGLAYER3
}

/// Bytes in one MPEG frame: `nAvgBytesPerSec * 1152 / nSamplesPerSec`.
fn frame_bytes(format: &WaveFormat) -> Result<u32, Error> {
    format
        .avg_bytes_per_sec
        .wrapping_mul(SAMPLES_PER_BLOCK)
        .checked_div(format.samples_per_sec)
        .ok_or(Error::NotPossible)
}

/// Bytes in one PCM block: `nBlockAlign * 1152`.
fn block_len(format: &WaveFormat) -> u32 {
    u32::from(format.block_align) * SAMPLES_PER_BLOCK
}

/// Whole units of `unit` in `total`, which must be at least one.
fn whole_blocks(total: u32, unit: u32) -> Result<u32, Error> {
    non_zero(total.checked_div(unit).ok_or(Error::NotPossible)?)
}

/// Units of `unit` in `total`, with a partial unit rounded up.
fn ceil_blocks(total: u32, unit: u32) -> Result<u32, Error> {
    let whole = total.checked_div(unit).ok_or(Error::NotPossible)?;
    non_zero(whole + u32::from(!total.is_multiple_of(unit)))
}

fn non_zero(blocks: u32) -> Result<u32, Error> {
    if blocks == 0 {
        Err(Error::NotPossible)
    } else {
        Ok(blocks)
    }
}
