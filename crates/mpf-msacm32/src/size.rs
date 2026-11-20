//! Stream size arithmetic (`PCM_StreamSize`).

use crate::format::WaveFormat;
use crate::Error;

/// `ACM_STREAMSIZEF_SOURCE`.
pub const SIZE_SOURCE: u32 = 0;
/// `ACM_STREAMSIZEF_DESTINATION`.
pub const SIZE_DESTINATION: u32 = 1;

/// The `ACMDRVSTREAMSIZE` fields: the query and the two lengths it converts between.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StreamSize {
    pub fdw_size: u32,
    pub src_length: u32,
    pub dst_length: u32,
}

/// `PCM_StreamSize`. Converts a destination length to a source length, or the
/// reverse, rounded to whole blocks of each format.
pub fn stream_size(src: &WaveFormat, dst: &WaveFormat, s: &mut StreamSize) -> Result<(), Error> {
    let src_mask = block_mask(src.block_align);
    let dst_mask = block_mask(dst.block_align);
    match s.fdw_size {
        SIZE_DESTINATION => {
            let dst_length = s.dst_length & dst_mask;
            s.src_length =
                round(dst_length, src.avg_bytes_per_sec, dst.avg_bytes_per_sec)? & src_mask;
        }
        SIZE_SOURCE => {
            let src_length = s.src_length & src_mask;
            s.dst_length =
                round(src_length, dst.avg_bytes_per_sec, src.avg_bytes_per_sec)? & dst_mask;
        }
        _ => return Err(Error::NotSupported),
    }
    Ok(())
}

/// The mask that rounds a length down to a multiple of the block alignment, as
/// Wine computes it. A block alignment of zero gives an all-zero mask.
fn block_mask(block_align: u16) -> u32 {
    !u32::from(block_align).wrapping_sub(1)
}

/// `PCM_round`: `a * b / c` rounded up, computed in doubles as Wine does. A quotient
/// beyond `u32` saturates, where Wine's conversion is undefined. Wine asserts
/// that `c` is not zero, so the port refuses it.
fn round(a: u32, b: u32, c: u32) -> Result<u32, Error> {
    if c == 0 {
        return Err(Error::NotPossible);
    }
    Ok(((f64::from(a) * f64::from(b) + f64::from(c) - 1.0) / f64::from(c)) as u32)
}
