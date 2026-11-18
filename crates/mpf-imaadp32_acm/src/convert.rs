//! `ADPCM_StreamConvert`: the block loop around the codec. The stream's
//! direction picks decode or encode.

use crate::codec::{decode_block, encode_block, Layout, State};
use crate::format::footprint;
use crate::open::{Direction, Stream};
use crate::Error;

pub const ACM_STREAMCONVERTF_START: u32 = 0x0000_0010;

/// The bytes that a convert call read from `src` and wrote to `dst`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Usage {
    pub src_used: usize,
    pub dst_used: usize,
}

/// `ADPCM_StreamConvert`. Converts as many whole blocks as both buffers hold.
/// `ACM_STREAMCONVERTF_START` resets the encoder's step index first.
///
/// Blocks overlap when the footprint is larger than the block align. The last
/// block must then fit in the ADPCM buffer, or the call returns
/// [`Error::InvalidParam`] without converting anything. Wine reads past the end
/// of the buffer in that case.
pub fn stream_convert(
    stream: &mut Stream,
    fdw: u32,
    src: &[u8],
    dst: &mut [u8],
) -> Result<Usage, Error> {
    if fdw & ACM_STREAMCONVERTF_START != 0 {
        stream.reset();
    }
    let (adpcm, pcm) = match stream.direction {
        Direction::Decode => (&stream.src, &stream.dst),
        Direction::Encode => (&stream.dst, &stream.src),
    };
    let channels = usize::from(adpcm.channels);
    let stride = usize::from(adpcm.block_align);
    let block_footprint = footprint(adpcm) as usize;
    let samples = usize::from(adpcm.samples_per_block);
    let layout = Layout {
        channels,
        samples,
        sample_bytes: usize::from(pcm.bits_per_sample / 8),
    };
    let pcm_block = samples * channels * layout.sample_bytes;
    let (adpcm_len, pcm_len) = match stream.direction {
        Direction::Decode => (src.len(), dst.len()),
        Direction::Encode => (dst.len(), src.len()),
    };
    let blocks = (adpcm_len / stride).min(pcm_len / pcm_block);
    if blocks > 0 && (blocks - 1) * stride + block_footprint > adpcm_len {
        return Err(Error::InvalidParam);
    }
    match stream.direction {
        Direction::Decode => {
            for block in 0..blocks {
                let input = &src[block * stride..block * stride + block_footprint];
                let output = &mut dst[block * pcm_block..(block + 1) * pcm_block];
                decode_block(input, output, layout);
            }
            Ok(Usage {
                src_used: blocks * stride,
                dst_used: blocks * pcm_block,
            })
        }
        Direction::Encode => {
            let mut states = [State::default(); 2];
            for (state, &index) in states.iter_mut().zip(&stream.step_index) {
                state.index = i32::from(index);
            }
            for block in 0..blocks {
                let input = &src[block * pcm_block..(block + 1) * pcm_block];
                let output = &mut dst[block * stride..block * stride + block_footprint];
                encode_block(input, output, layout, &mut states);
            }
            stream.step_index = [states[0].index as u8, states[1].index as u8];
            Ok(Usage {
                src_used: blocks * pcm_block,
                dst_used: blocks * stride,
            })
        }
    }
}
