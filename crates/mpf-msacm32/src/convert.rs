//! Stream open (`PCM_StreamOpen`) and the convert loop (`PCM_StreamConvert` with the
//! `cvt*K` and `cvt*C` routines).

use crate::format::WaveFormat;
use crate::sample::{mix, read, wider, write, Depth};
use crate::Error;

/// `ACM_STREAMOPENF_ASYNC`. Wine asserts that a PCM stream is not asynchronous.
pub const STREAMOPEN_ASYNC: u32 = 2;

/// One side of a stream: the depth of its samples, whether it has two channels, and
/// its block alignment. Any channel count other than one is treated as stereo, as
/// Wine's table lookup does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Side {
    depth: Depth,
    stereo: bool,
    block_align: usize,
}

impl Side {
    /// The bytes of one frame: one sample per channel.
    fn frame_bytes(self) -> usize {
        self.depth.bytes() * if self.stereo { 2 } else { 1 }
    }
}

/// The source and destination bytes that one conversion used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Usage {
    pub src_used: usize,
    pub dst_used: usize,
}

/// An open PCM stream, built by [`stream_open`] from layouts it has checked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Converter {
    src: Side,
    dst: Side,
    src_rate: u32,
    dst_rate: u32,
}

/// `PCM_StreamOpen`: checks the source and destination layouts and returns the
/// converter for them. Only the conversions that Wine implements are accepted.
pub fn stream_open(fdw_open: u32, src: &WaveFormat, dst: &WaveFormat) -> Result<Converter, Error> {
    if fdw_open & STREAMOPEN_ASYNC != 0 {
        return Err(Error::NotSupported);
    }
    let converter = Converter {
        src: side(src)?,
        dst: side(dst)?,
        src_rate: src.samples_per_sec,
        dst_rate: dst.samples_per_sec,
    };
    if !supported(
        converter.resamples(),
        converter.src.depth,
        converter.dst.depth,
    ) {
        return Err(Error::NotSupported);
    }
    Ok(converter)
}

/// Checks one side of a stream. A 24-bit sample must be packed, with a block of three
/// bytes per channel, and a block must hold one whole frame.
fn side(fmt: &WaveFormat) -> Result<Side, Error> {
    let depth = match fmt.bits_per_sample {
        8 => Depth::Bits8,
        16 => Depth::Bits16,
        24 if u32::from(fmt.block_align) == 3 * u32::from(fmt.channels) => Depth::Bits24,
        _ => return Err(Error::NotSupported),
    };
    let side = Side {
        depth,
        stereo: fmt.channels != 1,
        block_align: usize::from(fmt.block_align),
    };
    if side.block_align < side.frame_bytes() {
        return Err(Error::NotSupported);
    }
    Ok(side)
}

/// The depth pairs that Wine has a converter for: 24-bit output only from 24-bit
/// input with a rate change, and 24-bit input only without one.
fn supported(resample: bool, src: Depth, dst: Depth) -> bool {
    match (src, dst) {
        (_, Depth::Bits24) => resample && matches!(src, Depth::Bits24),
        (Depth::Bits24, _) => !resample,
        _ => true,
    }
}

impl Converter {
    /// Whether the stream changes the sample rate, which selects the rate-changing
    /// converters.
    pub fn resamples(&self) -> bool {
        self.src_rate != self.dst_rate
    }

    /// The index of this conversion in Wine's converter tables.
    pub fn table_index(&self) -> u32 {
        12 * self.src.depth as u32
            + 4 * self.dst.depth as u32
            + 2 * u32::from(!self.src.stereo)
            + u32::from(!self.dst.stereo)
    }

    /// `PCM_StreamConvert`. The lengths of `src` and `dst` stand for `cbSrcLength` and
    /// `cbDstLength`. Only whole blocks are converted, and the result reports the
    /// bytes used on each side.
    pub fn convert(&self, src: &[u8], dst: &mut [u8]) -> Usage {
        let nsrc = src.len() / self.src.block_align;
        let ndst = dst.len() / self.dst.block_align;
        if self.resamples() {
            let (consumed, produced) = self.resample(src, nsrc, dst, ndst);
            Usage {
                src_used: consumed * self.src.block_align,
                dst_used: produced * self.dst.block_align,
            }
        } else {
            let frames = nsrc.min(ndst);
            let (src_frame, dst_frame) = (self.src.frame_bytes(), self.dst.frame_bytes());
            for i in 0..frames {
                self.convert_frame(&src[i * src_frame..], &mut dst[i * dst_frame..]);
            }
            Usage {
                src_used: frames * self.src.block_align,
                dst_used: frames * self.dst.block_align,
            }
        }
    }

    /// The rate-changing loop. The error term grows by the destination rate for each
    /// source frame, and each time it passes the source rate a destination frame is
    /// written from the current source frame. A full destination stops the loop
    /// before the current source frame is counted as consumed. Returns the source
    /// frames consumed and the destination frames produced.
    fn resample(&self, src: &[u8], nsrc: usize, dst: &mut [u8], ndst: usize) -> (usize, usize) {
        let (src_frame, dst_frame) = (self.src.frame_bytes(), self.dst.frame_bytes());
        let mut error = self.src_rate / 2;
        let (mut consumed, mut produced) = (0, 0);
        'frames: while consumed < nsrc {
            error = error.wrapping_add(self.dst_rate);
            while error > self.src_rate {
                if produced == ndst {
                    break 'frames;
                }
                error -= self.src_rate;
                self.convert_frame(
                    &src[consumed * src_frame..],
                    &mut dst[produced * dst_frame..],
                );
                produced += 1;
            }
            consumed += 1;
        }
        (consumed, produced)
    }

    /// Converts the frame at the start of `src` into the frame at the start of `dst`.
    /// Mono to stereo repeats the sample, stereo to mono mixes the two channels, and
    /// stereo to stereo keeps both.
    fn convert_frame(&self, src: &[u8], dst: &mut [u8]) {
        let (sd, dd) = (self.src.depth, self.dst.depth);
        let left = read(sd, src);
        let db = dd.bytes();
        match (self.src.stereo, self.dst.stereo) {
            (false, false) => write(dd, left, dst),
            (false, true) => {
                write(dd, left, dst);
                write(dd, left, &mut dst[db..]);
            }
            (true, false) => {
                let right = read(sd, &src[sd.bytes()..]);
                write(dd, mix(wider(sd, dd), left, right), dst);
            }
            (true, true) => {
                write(dd, left, dst);
                write(dd, read(sd, &src[sd.bytes()..]), &mut dst[db..]);
            }
        }
    }
}
