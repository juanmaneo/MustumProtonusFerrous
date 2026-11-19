//! MSZIP inflate, from `dlls/d3dxof/mszip.c` (taken from the cabinet code by Stuart Caie).
//!
//! A chunk is the signature `CK` followed by deflate blocks, and it expands to at most one
//! 32 KiB window. Wine decodes with lookup tables that read whole bytes ahead, so it reads
//! past the end of a truncated chunk and writes past the window without noticing. This port
//! reads one bit at a time and stops at the first bit that is missing.

use crate::huffman::Code;
use crate::InflateError;

/// `CAB_INPUTMAX`: the largest chunk that `mszip_decompress` accepts.
const INPUT_MAX: usize = 32768 + 6144;

/// `ZIPWSIZE`: the window, which bounds the output.
const WINDOW_SIZE: usize = 0x8000;

/// The order in which a dynamic block stores the code-length code lengths (`Zipborder`).
const BORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

/// Base lengths and extra bits of the length symbols 257 to 285 (`Zipcplens`, `Zipcplext`).
const LEN_BASE: [usize; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [usize; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];

/// Base distances and extra bits of the distance symbols 0 to 29 (`Zipcpdist`, `Zipcpdext`).
const DIST_BASE: [usize; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [usize; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

/// Inflates one chunk, as `mszip_decompress` does: `input` holds the `CK` signature and the
/// deflate blocks, and the result is the first `outlen` bytes of the output.
pub fn decompress(input: &[u8], outlen: usize) -> Result<Vec<u8>, InflateError> {
    if input.len() > INPUT_MAX || outlen > WINDOW_SIZE {
        return Err(InflateError::DataFormat);
    }
    let mut bits = Bits::new(input);
    if bits.bits(16)? != 0x4b43 {
        return Err(InflateError::IllegalData);
    }
    let mut out = Vec::new();
    loop {
        let header = bits.bits(3)?;
        match header >> 1 {
            0 => stored(&mut bits, &mut out)?,
            1 => fixed(&mut bits, &mut out)?,
            2 => dynamic(&mut bits, &mut out)?,
            _ => return Err(InflateError::IllegalData),
        }
        if header & 1 == 1 {
            break;
        }
    }
    if out.len() < outlen {
        return Err(InflateError::OutputShort);
    }
    out.truncate(outlen);
    Ok(out)
}

/// A stored block: `LEN`, its complement `NLEN`, then `LEN` bytes (`fdi_Zipinflate_stored`).
fn stored(bits: &mut Bits<'_>, out: &mut Vec<u8>) -> Result<(), InflateError> {
    bits.align();
    let len = bits.bits(16)?;
    let nlen = bits.bits(16)?;
    if len + nlen != 0xffff {
        return Err(InflateError::IllegalData);
    }
    check_room(out, len)?;
    for _ in 0..len {
        out.push(bits.bits(8)? as u8);
    }
    Ok(())
}

/// A fixed-Huffman block with the code lengths of RFC 1951 section 3.2.6
/// (`fdi_Zipinflate_fixed`). Its distance code has 30 codes, so it is incomplete, and Wine
/// accepts that.
fn fixed(bits: &mut Bits<'_>, out: &mut Vec<u8>) -> Result<(), InflateError> {
    let mut lengths = [8u8; 288];
    lengths[144..256].fill(9);
    lengths[256..280].fill(7);
    codes(bits, out, &Code::new(&lengths), &Code::new(&[5u8; 30]))
}

/// A dynamic block: the code-length code, the literal and distance code lengths, then the data
/// (`fdi_Zipinflate_dynamic`). Wine does not check the distance code here, and the port checks
/// it only when a copy uses it.
fn dynamic(bits: &mut Bits<'_>, out: &mut Vec<u8>) -> Result<(), InflateError> {
    let nl = 257 + bits.bits(5)?;
    let nd = 1 + bits.bits(5)?;
    let nb = 4 + bits.bits(4)?;
    let mut cl_lengths = [0u8; 19];
    for &symbol in &BORDER[..nb] {
        cl_lengths[symbol] = bits.bits(3)? as u8;
    }
    let cl = Code::new(&cl_lengths);
    if !cl.is_well_formed() {
        return Err(InflateError::IllegalData);
    }

    let total = nl + nd;
    let mut lengths: Vec<u8> = Vec::with_capacity(total);
    let mut previous = 0;
    while lengths.len() < total {
        let (value, repeat) = match cl.decode(bits)? {
            16 => (previous, 3 + bits.bits(2)?),
            17 => (0, 3 + bits.bits(3)?),
            18 => (0, 11 + bits.bits(7)?),
            length => (length as u8, 1),
        };
        if lengths.len() + repeat > total {
            return Err(InflateError::IllegalData);
        }
        lengths.resize(lengths.len() + repeat, value);
        previous = value;
    }

    let literal = Code::new(&lengths[..nl]);
    if !literal.is_well_formed() {
        return Err(InflateError::IllegalData);
    }
    codes(bits, out, &literal, &Code::new(&lengths[nl..]))
}

/// Literal, length and distance symbols up to the end-of-block symbol (`fdi_Zipinflate_codes`).
fn codes(
    bits: &mut Bits<'_>,
    out: &mut Vec<u8>,
    literal: &Code,
    distance: &Code,
) -> Result<(), InflateError> {
    loop {
        match literal.decode(bits)? {
            symbol @ 0..=255 => {
                check_room(out, 1)?;
                out.push(symbol as u8);
            }
            256 => return Ok(()),
            symbol @ 257..=285 => {
                let index = symbol - 257;
                let length = LEN_BASE[index] + bits.bits(LEN_EXTRA[index])?;
                let slot = distance.decode(bits)?;
                if slot >= DIST_BASE.len() {
                    return Err(InflateError::IllegalData);
                }
                let dist = DIST_BASE[slot] + bits.bits(DIST_EXTRA[slot])?;
                if dist > out.len() {
                    return Err(InflateError::IllegalData);
                }
                check_room(out, length)?;
                for _ in 0..length {
                    let byte = out[out.len() - dist];
                    out.push(byte);
                }
            }
            _ => return Err(InflateError::IllegalData),
        }
    }
}

/// Refuses output past the window. Wine writes past its buffer here.
fn check_room(out: &[u8], count: usize) -> Result<(), InflateError> {
    if out.len() + count > WINDOW_SIZE {
        return Err(InflateError::IllegalData);
    }
    Ok(())
}

/// Reads the deflate bit stream: bits come least significant first from each byte.
pub(crate) struct Bits<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Bits<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Self { input, pos: 0 }
    }

    pub(crate) fn bit(&mut self) -> Result<usize, InflateError> {
        let byte = *self
            .input
            .get(self.pos / 8)
            .ok_or(InflateError::InputExhausted)?;
        let bit = (byte >> (self.pos % 8)) & 1;
        self.pos += 1;
        Ok(usize::from(bit))
    }

    pub(crate) fn bits(&mut self, count: usize) -> Result<usize, InflateError> {
        let mut value = 0;
        for i in 0..count {
            value |= self.bit()? << i;
        }
        Ok(value)
    }

    /// Skips to the next byte boundary.
    pub(crate) fn align(&mut self) {
        self.pos = self.pos.next_multiple_of(8);
    }
}
