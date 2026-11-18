//! `ADPCM_StreamSize`: converts a byte count on one side of a stream into the
//! matching count on the other side, in whole blocks.

use crate::{Error, WaveFormat, WAVE_FORMAT_IMA_ADPCM, WAVE_FORMAT_PCM};

pub const ACM_STREAMSIZEF_SOURCE: u32 = 0;
pub const ACM_STREAMSIZEF_DESTINATION: u32 = 1;

/// `ACMDRVSTREAMSIZE` fields. The query fills the side it does not name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamSize {
    pub fdw_size: u32,
    pub cb_src_length: u32,
    pub cb_dst_length: u32,
}

/// `ADPCM_StreamSize`. Each count is in whole blocks of the side that the
/// block align sets. A zero divisor, or a count that holds no whole block,
/// gives [`Error::NotPossible`].
pub fn stream_size(
    src: &WaveFormat,
    dst: &WaveFormat,
    sized: &mut StreamSize,
) -> Result<(), Error> {
    match sized.fdw_size {
        ACM_STREAMSIZEF_DESTINATION => match (src.format_tag, dst.format_tag) {
            (WAVE_FORMAT_PCM, WAVE_FORMAT_IMA_ADPCM) => {
                let blocks = whole_blocks(sized.cb_dst_length, u32::from(dst.block_align))?;
                sized.cb_src_length = blocks
                    .wrapping_mul(u32::from(src.block_align))
                    .wrapping_mul(u32::from(dst.samples_per_block));
            }
            (WAVE_FORMAT_IMA_ADPCM, WAVE_FORMAT_PCM) => {
                let unit = u32::from(dst.block_align) * u32::from(src.samples_per_block);
                let blocks = whole_blocks(sized.cb_dst_length, unit)?;
                sized.cb_src_length = blocks.wrapping_mul(u32::from(src.block_align));
            }
            _ => return Err(Error::NotSupported),
        },
        ACM_STREAMSIZEF_SOURCE => match (src.format_tag, dst.format_tag) {
            (WAVE_FORMAT_PCM, WAVE_FORMAT_IMA_ADPCM) => {
                let unit = u32::from(src.block_align) * u32::from(dst.samples_per_block);
                let blocks = partial_blocks(sized.cb_src_length, unit)?;
                sized.cb_dst_length = blocks.wrapping_mul(u32::from(dst.block_align));
            }
            (WAVE_FORMAT_IMA_ADPCM, WAVE_FORMAT_PCM) => {
                let blocks = partial_blocks(sized.cb_src_length, u32::from(src.block_align))?;
                sized.cb_dst_length = blocks
                    .wrapping_mul(u32::from(dst.block_align))
                    .wrapping_mul(u32::from(src.samples_per_block));
            }
            _ => return Err(Error::NotSupported),
        },
        _ => return Err(Error::NotSupported),
    }
    Ok(())
}

/// The whole blocks in `len` bytes.
fn whole_blocks(len: u32, unit: u32) -> Result<u32, Error> {
    len.checked_div(unit)
        .filter(|&blocks| blocks > 0)
        .ok_or(Error::NotPossible)
}

/// The blocks needed for `len` bytes, counting a partial last block.
fn partial_blocks(len: u32, unit: u32) -> Result<u32, Error> {
    whole_blocks(len, unit)?;
    Ok(len.div_ceil(unit))
}
