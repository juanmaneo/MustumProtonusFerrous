//! Stream size arithmetic (`ADPCM_StreamSize`).

use crate::format::{samples_per_block, WaveFormat};
use crate::{Error, WAVE_FORMAT_ADPCM, WAVE_FORMAT_PCM};

/// `ACM_STREAMSIZEF_SOURCE`.
pub const SIZE_SOURCE: u32 = 0;
/// `ACM_STREAMSIZEF_DESTINATION`.
pub const SIZE_DESTINATION: u32 = 1;

/// `ACMDRVSTREAMSIZE`: the query and the two lengths it converts between.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StreamSize {
    pub fdw_size: u32,
    pub src_length: u32,
    pub dst_length: u32,
}

/// `ADPCM_StreamSize`. Only PCM to ADPCM and ADPCM to PCM are supported.
pub fn stream_size(src: &WaveFormat, dst: &WaveFormat, s: &mut StreamSize) -> Result<(), Error> {
    let pcm_to_adpcm = src.format_tag == WAVE_FORMAT_PCM && dst.format_tag == WAVE_FORMAT_ADPCM;
    let adpcm_to_pcm = src.format_tag == WAVE_FORMAT_ADPCM && dst.format_tag == WAVE_FORMAT_PCM;
    match s.fdw_size {
        SIZE_DESTINATION if pcm_to_adpcm => {
            let spb = spb_of(dst)?;
            let blocks = block_count(s.dst_length, u32::from(dst.block_align), false)?;
            s.src_length = blocks
                .wrapping_mul(u32::from(src.block_align))
                .wrapping_mul(spb);
        }
        SIZE_DESTINATION if adpcm_to_pcm => {
            let spb = spb_of(src)?;
            let per = u32::from(dst.block_align).wrapping_mul(spb);
            let blocks = block_count(s.dst_length, per, false)?;
            s.src_length = blocks.wrapping_mul(u32::from(src.block_align));
        }
        SIZE_SOURCE if pcm_to_adpcm => {
            let spb = spb_of(dst)?;
            let per = u32::from(src.block_align).wrapping_mul(spb);
            let blocks = block_count(s.src_length, per, true)?;
            s.dst_length = blocks.wrapping_mul(u32::from(dst.block_align));
        }
        SIZE_SOURCE if adpcm_to_pcm => {
            let spb = spb_of(src)?;
            let blocks = block_count(s.src_length, u32::from(src.block_align), true)?;
            s.dst_length = blocks
                .wrapping_mul(u32::from(dst.block_align))
                .wrapping_mul(spb);
        }
        _ => return Err(Error::NotSupported),
    }
    Ok(())
}

/// The samples per block of an ADPCM layout, from its block alignment and channels.
fn spb_of(fmt: &WaveFormat) -> Result<u32, Error> {
    samples_per_block(fmt.block_align, fmt.channels)
        .map(u32::from)
        .ok_or(Error::NotPossible)
}

/// The number of blocks in `total` bytes of `per`-byte blocks, rounded up when
/// asked. Wine refuses a count of zero.
fn block_count(total: u32, per: u32, round_up: bool) -> Result<u32, Error> {
    let whole = total
        .checked_div(per)
        .filter(|&n| n != 0)
        .ok_or(Error::NotPossible)?;
    Ok(if round_up && !total.is_multiple_of(per) {
        whole.wrapping_add(1)
    } else {
        whole
    })
}
