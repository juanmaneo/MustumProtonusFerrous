//! The pixel decoders `msvideo1_decode_8bit` and `msvideo1_decode_16bit` of `msvideo1.c`.
//!
//! A frame is a grid of 4x4 blocks, read in row order. Each block starts with two stream bytes,
//! `byte_a` and `byte_b`. Their values select a skip run, a 2-colour block, an 8-colour block or
//! a 1-colour block. The 8-bit and 16-bit decoders share [walk] and differ in the colour words.

use crate::frame::Grid;
use crate::IcResult;

/// `msvideo1_decode_8bit`: decodes `buf` into the 8-bit frame `pixels`, `stride` bytes per row.
/// `pixels` must hold every block of the frame, or the call returns [crate::IcError::BadSize].
pub fn decode_8bit(
    width: i32,
    height: i32,
    buf: &[u8],
    pixels: &mut [u8],
    stride: i32,
) -> IcResult<()> {
    let grid = Grid::new(width, height, stride, pixels.len())?;
    walk(grid, buf, pixels, |stream, [byte_a, byte_b]| {
        let flags = u16::from_le_bytes([byte_a, byte_b]);
        if byte_b < 0x80 {
            Some(two_colour(flags, stream.take()?))
        } else if byte_b >= 0x90 {
            Some(eight_colour(flags, stream.take()?))
        } else {
            Some([byte_a; 16])
        }
    });
    Ok(())
}

/// `msvideo1_decode_16bit`: decodes `buf` into the 16-bit frame `pixels`, with each pixel stored
/// as a little-endian word and `stride` words per row. `pixels` must hold every block of the
/// frame, as for [decode_8bit]; a trailing odd byte is never written.
pub fn decode_16bit(
    width: i32,
    height: i32,
    buf: &[u8],
    pixels: &mut [u8],
    stride: i32,
) -> IcResult<()> {
    let grid = Grid::new(width, height, stride, pixels.len() / 2)?;
    walk(grid, buf, pixels, |stream, [byte_a, byte_b]| {
        let flags = u16::from_le_bytes([byte_a, byte_b]);
        if byte_b >= 0x80 {
            return Some([flags; 16]);
        }
        let head: [u8; 4] = stream.take()?;
        let mut colors = [0u16; 8];
        colors[0] = u16::from_le_bytes([head[0], head[1]]);
        colors[1] = u16::from_le_bytes([head[2], head[3]]);
        if (colors[0] & 0x8000) == 0 {
            return Some(two_colour(flags, [colors[0], colors[1]]));
        }
        let tail: [u8; 12] = stream.take()?;
        let (pairs, _) = tail.as_chunks::<2>();
        for (slot, pair) in colors[2..].iter_mut().zip(pairs) {
            *slot = u16::from_le_bytes(*pair);
        }
        Some(eight_colour(flags, colors))
    });
    Ok(())
}

/// The bytes of a stream, read front to back. Reads past the end fail, which ends decoding.
struct Stream<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl Stream<'_> {
    fn take<const N: usize>(&mut self) -> Option<[u8; N]> {
        let bytes = self.buf.get(self.pos..self.pos + N)?;
        let mut out = [0u8; N];
        out.copy_from_slice(bytes);
        self.pos += N;
        Some(out)
    }
}

/// A pixel of either depth: one byte for 8-bit frames, a little-endian word for 16-bit frames.
trait Pixel: Copy {
    const BYTES: usize;
    fn store(self, out: &mut [u8]);
}

impl Pixel for u8 {
    const BYTES: usize = 1;

    fn store(self, out: &mut [u8]) {
        out[0] = self;
    }
}

impl Pixel for u16 {
    const BYTES: usize = 2;

    fn store(self, out: &mut [u8]) {
        out[..2].copy_from_slice(&self.to_le_bytes());
    }
}

/// The block loop shared by both depths. `decode_block` gets the stream positioned after the
/// two header bytes, and returns the 16 pixels of a non-skip block, or `None` when the stream
/// ends. A skip code uses up its own block and the blocks that its count names; a count of -1
/// skips every block left in the frame.
fn walk<P: Pixel>(
    grid: Grid,
    buf: &[u8],
    pixels: &mut [u8],
    decode_block: impl Fn(&mut Stream<'_>, [u8; 2]) -> Option<[P; 16]>,
) {
    let mut stream = Stream { buf, pos: 0 };
    let mut skip: i64 = 0;
    for block_row in 0..grid.high {
        for block_col in 0..grid.wide {
            if skip != 0 {
                skip -= 1;
                continue;
            }
            let Some(header) = stream.take::<2>() else {
                return;
            };
            let [byte_a, byte_b] = header;
            if (byte_b & 0xFC) == 0x84 {
                skip = ((i64::from(byte_b) - 0x84) << 8) + i64::from(byte_a) - 1;
                continue;
            }
            let Some(block) = decode_block(&mut stream, header) else {
                return;
            };
            let first = 4 * block_row * grid.stride + 4 * block_col;
            for (i, pixel) in block.into_iter().enumerate() {
                let at = first + (i / 4) * grid.stride + i % 4;
                pixel.store(&mut pixels[at * P::BYTES..]);
            }
        }
    }
}

/// The 2-colour block: pixel `i` takes colour 1 when its flag bit is 0, and colour 0 otherwise.
fn two_colour<P: Copy>(flags: u16, colors: [P; 2]) -> [P; 16] {
    std::array::from_fn(|i| colors[usize::from(((flags >> i) & 1) ^ 1)])
}

/// The 8-colour block: each 2x2 quadrant of the block picks one pair of the eight colours.
fn eight_colour<P: Copy>(flags: u16, colors: [P; 8]) -> [P; 16] {
    std::array::from_fn(|i| {
        let (row, col) = (i / 4, i % 4);
        colors[((row & 2) << 1) + (col & 2) + usize::from(((flags >> i) & 1) ^ 1)]
    })
}
