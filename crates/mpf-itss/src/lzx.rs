//! The LZX decompressor state and LZXdecompress: the block loop, verbatim, aligned and
//! uncompressed blocks, the repeated offsets, the window and the E8 translation.

use crate::bits::Bits;
use crate::e8;
use crate::huffman::{read_lengths, Table};
use crate::slots::{posn_slots, BASE, EXTRA};
use crate::DecrError;

const MIN_WINDOW: u32 = 1 << 15;
const MAX_WINDOW: u32 = 1 << 21;
const NUM_CHARS: usize = 256;
const MIN_MATCH: usize = 2;
const NUM_PRIMARY_LENGTHS: usize = 7;
const NUM_SECONDARY_LENGTHS: usize = 249;
const PRETREE_SYMBOLS: usize = 20;
const PRETREE_BITS: u32 = 6;
const ALIGNED_SYMBOLS: usize = 8;
const ALIGNED_BITS: u32 = 7;
const MAIN_SYMBOLS: usize = NUM_CHARS + 50 * 8;
const MAIN_BITS: u32 = 12;
const LENGTH_SYMBOLS: usize = NUM_SECONDARY_LENGTHS + 1;
const LENGTH_BITS: u32 = 12;
const BLOCK_VERBATIM: u32 = 1;
const BLOCK_ALIGNED: u32 = 2;
const BLOCK_UNCOMPRESSED: u32 = 3;
/// The E8 translation is undone for the first this many frames only.
const E8_FRAMES: u32 = 32768;

/// An LZX decompressor, the LZXstate of lzx.c. It keeps the window and the Huffman tables
/// between calls, so one stream is decoded by successive calls to [Lzx::decompress].
pub struct Lzx {
    window: Vec<u8>,
    window_posn: usize,
    r: [usize; 3],
    main_elements: usize,
    header_read: bool,
    block_type: u32,
    block_length: usize,
    block_remaining: usize,
    frames_read: u32,
    intel_filesize: i32,
    intel_curpos: i32,
    intel_started: bool,
    pretree: Table,
    maintree: Table,
    length: Table,
    aligned: Table,
}

impl Lzx {
    /// LZXinit. `window` is the window size in bytes. It must be a power of two from 2^15 to
    /// 2^21, the sizes the LZX format allows, or this returns `None`.
    pub fn new(window: u32) -> Option<Self> {
        if !window.is_power_of_two() || !(MIN_WINDOW..=MAX_WINDOW).contains(&window) {
            return None;
        }
        let mut lzx = Lzx {
            window: vec![0; window as usize],
            window_posn: 0,
            r: [1; 3],
            main_elements: NUM_CHARS + 8 * posn_slots(window),
            header_read: false,
            block_type: 0,
            block_length: 0,
            block_remaining: 0,
            frames_read: 0,
            intel_filesize: 0,
            intel_curpos: 0,
            intel_started: false,
            pretree: Table::new(PRETREE_SYMBOLS, PRETREE_BITS),
            maintree: Table::new(MAIN_SYMBOLS, MAIN_BITS),
            length: Table::new(LENGTH_SYMBOLS, LENGTH_BITS),
            aligned: Table::new(ALIGNED_SYMBOLS, ALIGNED_BITS),
        };
        lzx.reset();
        Some(lzx)
    }

    /// LZXreset: starts a new stream. The window and the symbol tables are kept, as in lzx.c.
    pub fn reset(&mut self) {
        self.r = [1; 3];
        self.header_read = false;
        self.frames_read = 0;
        self.block_remaining = 0;
        self.block_type = 0;
        self.intel_curpos = 0;
        self.intel_started = false;
        self.window_posn = 0;
        self.maintree.lens.fill(0);
        self.length.lens.fill(0);
    }

    /// LZXdecompress: decodes the next `outlen` bytes of the stream from `input`, which holds the
    /// compressed data from where the previous call stopped. Input past its end reads as zeros.
    pub fn decompress(&mut self, input: &[u8], outlen: usize) -> Result<Vec<u8>, DecrError> {
        let w = self.window.len();
        if outlen > w {
            return Err(DecrError::IllegalData);
        }
        let mut bits = Bits::new(input);
        let mut posn = self.window_posn;
        let mut r = self.r;

        if !self.header_read {
            let mut i = 0;
            let mut j = 0;
            if bits.read(1) != 0 {
                i = bits.read(16);
                j = bits.read(16);
            }
            self.intel_filesize = ((i << 16) | j) as i32;
            self.header_read = true;
        }

        let mut togo = outlen;
        while togo > 0 {
            if self.block_remaining == 0 {
                self.block_header(&mut bits, &mut r)?;
            }
            if bits.overrun() {
                return Err(DecrError::IllegalData);
            }
            while self.block_remaining > 0 && togo > 0 {
                let this_run = self.block_remaining.min(togo);
                togo -= this_run;
                self.block_remaining -= this_run;

                posn &= w - 1;
                if posn + this_run > w {
                    return Err(DecrError::DataFormat);
                }
                match self.block_type {
                    BLOCK_VERBATIM | BLOCK_ALIGNED => {
                        let aligned = self.block_type == BLOCK_ALIGNED;
                        self.decode_codes(&mut bits, &mut posn, &mut r, aligned, this_run)?
                    }
                    BLOCK_UNCOMPRESSED => {
                        let src = bits.stored(this_run)?;
                        self.window[posn..posn + this_run].copy_from_slice(src);
                        posn += this_run;
                    }
                    _ => return Err(DecrError::IllegalData),
                }
            }
        }

        let start = (posn + w - outlen) % w;
        let mut out: Vec<u8> = (0..outlen).map(|i| self.window[(start + i) % w]).collect();
        self.window_posn = posn;
        self.r = r;

        let frames = self.frames_read;
        self.frames_read = self.frames_read.wrapping_add(1);
        if frames < E8_FRAMES && self.intel_filesize != 0 {
            e8::undo_e8(
                &mut out,
                &mut self.intel_curpos,
                self.intel_filesize,
                self.intel_started,
            );
        }
        Ok(out)
    }

    /// The header of a block: its type and length, then its trees, or for an uncompressed block
    /// the three repeated offsets that the stored data starts after.
    fn block_header(&mut self, bits: &mut Bits, r: &mut [usize; 3]) -> Result<(), DecrError> {
        if self.block_type == BLOCK_UNCOMPRESSED {
            if self.block_length & 1 != 0 {
                bits.skip_byte();
            }
            bits.init();
        }

        self.block_type = bits.read(3);
        let i = bits.read(16);
        let j = bits.read(8);
        self.block_length = ((i << 8) | j) as usize;
        self.block_remaining = self.block_length;

        match self.block_type {
            BLOCK_ALIGNED => {
                for len in &mut self.aligned.lens[..ALIGNED_SYMBOLS] {
                    *len = bits.read(3) as u8;
                }
                self.aligned.build()?;
                self.read_trees(bits)
            }
            BLOCK_VERBATIM => self.read_trees(bits),
            BLOCK_UNCOMPRESSED => {
                self.intel_started = true;
                bits.align_word();
                *r = [bits.take_u32(), bits.take_u32(), bits.take_u32()];
                Ok(())
            }
            _ => Err(DecrError::IllegalData),
        }
    }

    /// The main and length trees, which a verbatim or aligned block starts with.
    fn read_trees(&mut self, bits: &mut Bits) -> Result<(), DecrError> {
        read_lengths(
            bits,
            &mut self.pretree,
            &mut self.maintree.lens,
            0,
            NUM_CHARS,
        )?;
        read_lengths(
            bits,
            &mut self.pretree,
            &mut self.maintree.lens,
            NUM_CHARS,
            self.main_elements,
        )?;
        self.maintree.build()?;
        if self.maintree.lens[0xE8] != 0 {
            self.intel_started = true;
        }
        read_lengths(
            bits,
            &mut self.pretree,
            &mut self.length.lens,
            0,
            NUM_SECONDARY_LENGTHS,
        )?;
        self.length.build()
    }

    /// The symbols of a verbatim or aligned block, until `this_run` bytes are written. A match can
    /// run past `this_run`, as in lzx.c, but never past the end of the window.
    fn decode_codes(
        &mut self,
        bits: &mut Bits,
        posn: &mut usize,
        r: &mut [usize; 3],
        aligned: bool,
        mut this_run: usize,
    ) -> Result<(), DecrError> {
        while this_run > 0 {
            let sym = self.maintree.decode(bits);
            if sym < NUM_CHARS {
                self.window[*posn] = sym as u8;
                *posn += 1;
                this_run -= 1;
                continue;
            }

            let main = sym - NUM_CHARS;
            let mut len = main & NUM_PRIMARY_LENGTHS;
            if len == NUM_PRIMARY_LENGTHS {
                len += self.length.decode(bits);
            }
            len += MIN_MATCH;

            let slot = main >> 3;
            let off = match slot {
                0 => r[0],
                1 => {
                    r.swap(0, 1);
                    r[0]
                }
                2 => {
                    r.swap(0, 2);
                    r[0]
                }
                _ => {
                    let off = if aligned {
                        aligned_offset(bits, &self.aligned, slot)
                    } else {
                        verbatim_offset(bits, slot)
                    };
                    *r = [off, r[0], r[1]];
                    off
                }
            };

            copy_match(&mut self.window, *posn, off, len)?;
            *posn += len;
            this_run = this_run.saturating_sub(len);
        }
        Ok(())
    }
}

/// The offset of a match in a verbatim block: the slot base plus its extra bits. Slot 3 is the
/// offset 1.
fn verbatim_offset(bits: &mut Bits, slot: usize) -> usize {
    if slot == 3 {
        1
    } else {
        (BASE[slot] - 2 + bits.read(EXTRA[slot])) as usize
    }
}

/// The offset of a match in an aligned block. The low three bits come from the aligned tree
/// when the slot has more than three extra bits, and from the tree alone when it has three.
fn aligned_offset(bits: &mut Bits, aligned: &Table, slot: usize) -> usize {
    let extra = EXTRA[slot];
    let base = (BASE[slot] - 2) as usize;
    if extra > 3 {
        base + ((bits.read(extra - 3) as usize) << 3) + aligned.decode(bits)
    } else if extra == 3 {
        base + aligned.decode(bits)
    } else if extra > 0 {
        base + bits.read(extra) as usize
    } else {
        1
    }
}

/// Copies `len` bytes from `off` bytes back to `posn`, byte by byte, so an overlapping match
/// repeats its own output. Both positions wrap around the window.
pub(crate) fn copy_match(
    window: &mut [u8],
    posn: usize,
    off: usize,
    len: usize,
) -> Result<(), DecrError> {
    let w = window.len();
    if off > w || posn + len > w {
        return Err(DecrError::DataFormat);
    }
    let src = (posn + w - off) % w;
    for k in 0..len {
        window[posn + k] = window[(src + k) % w];
    }
    Ok(())
}
