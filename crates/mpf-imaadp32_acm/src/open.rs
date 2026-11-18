//! `ADPCM_StreamOpen` checks: which format pairs convert, and which ADPCM
//! formats have a block layout the convert loops can read and write.

use crate::format::{footprint, get_format_index};
use crate::{Error, WaveFormat, WAVE_FORMAT_IMA_ADPCM, WAVE_FORMAT_PCM};

pub const ACM_STREAMOPENF_ASYNC: u32 = 0x0000_0002;

/// Which convert loop the stream runs. Decode turns ADPCM into PCM and encode
/// turns PCM into ADPCM.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Decode,
    Encode,
}

/// An open stream: the two formats, the direction, and the encoder's step index
/// per channel (`stepIndexL` and `stepIndexR`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream {
    pub(crate) src: WaveFormat,
    pub(crate) dst: WaveFormat,
    pub(crate) direction: Direction,
    pub(crate) step_index: [u8; 2],
}

impl Stream {
    /// `ADPCM_Reset`: both step indices return to 0.
    pub(crate) fn reset(&mut self) {
        self.step_index = [0, 0];
    }
}

/// `ADPCM_StreamOpen`. Accepts ADPCM to 8- or 16-bit PCM, and 16-bit PCM to
/// ADPCM, at the same rate and channel count.
///
/// # Panics
///
/// Panics on an asynchronous stream, as Wine's `assert` does.
pub fn stream_open(fdw_open: u32, src: &WaveFormat, dst: &WaveFormat) -> Result<Stream, Error> {
    assert!(
        (fdw_open & ACM_STREAMOPENF_ASYNC) == 0,
        "asynchronous streams are not supported"
    );
    if get_format_index(src).is_none() || get_format_index(dst).is_none() {
        return Err(Error::NotPossible);
    }
    let direction = match (src.format_tag, dst.format_tag) {
        (WAVE_FORMAT_IMA_ADPCM, WAVE_FORMAT_PCM) => decode_direction(src, dst)?,
        (WAVE_FORMAT_PCM, WAVE_FORMAT_IMA_ADPCM) => encode_direction(src, dst)?,
        _ => return Err(Error::NotSupported),
    };
    Ok(Stream {
        src: src.clone(),
        dst: dst.clone(),
        direction,
        step_index: [0, 0],
    })
}

fn decode_direction(src: &WaveFormat, dst: &WaveFormat) -> Result<Direction, Error> {
    if src.samples_per_sec != dst.samples_per_sec
        || src.channels != dst.channels
        || !matches!((dst.bits_per_sample, dst.channels), (16, _) | (8, 1))
    {
        return Err(Error::NotSupported);
    }
    check_adpcm(src)?;
    Ok(Direction::Decode)
}

fn encode_direction(src: &WaveFormat, dst: &WaveFormat) -> Result<Direction, Error> {
    if src.samples_per_sec != dst.samples_per_sec
        || src.channels != dst.channels
        || src.bits_per_sample != 16
    {
        return Err(Error::NotSupported);
    }
    check_adpcm(dst)?;
    Ok(Direction::Encode)
}

/// The block layout checks on the ADPCM side. The samples per block must be
/// `1 mod 4`. The block must hold its footprint, and a stereo block must hold
/// whole 8-nibble groups. A zero block align has no layout.
fn check_adpcm(wfx: &WaveFormat) -> Result<(), Error> {
    let block_align = u32::from(wfx.block_align);
    let extra = u32::from(wfx.samples_per_block).wrapping_sub(1);
    if extra & 3 != 0
        || block_align == 0
        || footprint(wfx) < block_align
        || (wfx.channels == 2 && extra % 8 != 0)
    {
        return Err(Error::NotSupported);
    }
    Ok(())
}
