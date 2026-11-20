//! The LZX bit reader: the ENSURE_BITS, PEEK_BITS and REMOVE_BITS macros of lzx.c.
//!
//! Bits are taken MSB first from little-endian 16-bit words. Bytes past the end of the input read
//! as zero, the same as the zero padding the oracle harness gives its input.

use crate::DecrError;

/// The compressed input, a byte position in it, and the bit buffer loaded from it.
pub(crate) struct Bits<'a> {
    input: &'a [u8],
    pos: usize,
    left: u32,
    buf: u32,
}

impl<'a> Bits<'a> {
    pub(crate) fn new(input: &'a [u8]) -> Self {
        Bits {
            input,
            pos: 0,
            left: 0,
            buf: 0,
        }
    }

    fn byte(&self, i: usize) -> u8 {
        self.input.get(i).copied().unwrap_or(0)
    }

    /// INIT_BITSTREAM: drops the buffered bits. The byte position is kept.
    pub(crate) fn init(&mut self) {
        self.left = 0;
        self.buf = 0;
    }

    /// ENSURE_BITS: loads words until at least `n` bits are buffered, `n` at most 17.
    pub(crate) fn ensure(&mut self, n: u32) {
        while self.left < n {
            let word = (u32::from(self.byte(self.pos + 1)) << 8) | u32::from(self.byte(self.pos));
            self.buf |= word << (16 - self.left);
            self.left += 16;
            self.pos += 2;
        }
    }

    /// PEEK_BITS: the next `n` buffered bits, which must be loaded with [Bits::ensure] first.
    pub(crate) fn peek(&self, n: u32) -> u32 {
        self.buf >> (32 - n)
    }

    /// Whether any bit of `mask` is set in the buffer.
    pub(crate) fn test_bit(&self, mask: u32) -> bool {
        self.buf & mask != 0
    }

    /// REMOVE_BITS: drops the next `n` buffered bits. [Bits::ensure] must have loaded them.
    pub(crate) fn consume(&mut self, n: u32) {
        self.buf <<= n;
        self.left -= n;
    }

    /// READ_BITS: takes the next `n` bits, 1 to 17 of them.
    pub(crate) fn read(&mut self, n: u32) -> u32 {
        self.ensure(n);
        let v = self.peek(n);
        self.consume(n);
        v
    }

    /// The start of an uncompressed block: loads 16 bits, then gives back the last word loaded if
    /// more than 16 bits were buffered, so the stored words start on a byte boundary.
    pub(crate) fn align_word(&mut self) {
        self.ensure(16);
        if self.left > 16 {
            self.pos -= 2;
        }
    }

    /// Reads four little-endian bytes at the byte position, which then advances past them.
    pub(crate) fn take_u32(&mut self) -> usize {
        let start = self.pos;
        self.pos += 4;
        u32::from_le_bytes(std::array::from_fn(|k| self.byte(start + k))) as usize
    }

    /// Takes `len` bytes of stored data at the byte position.
    pub(crate) fn stored(&mut self, len: usize) -> Result<&'a [u8], DecrError> {
        let end = self.pos + len;
        if end > self.input.len() {
            return Err(DecrError::IllegalData);
        }
        let data = &self.input[self.pos..end];
        self.pos = end;
        Ok(data)
    }

    /// Advances the byte position past a pad byte.
    pub(crate) fn skip_byte(&mut self) {
        self.pos += 1;
    }

    /// The exhaustion check of LZXdecompress. The decoder may read up to two bytes past the end of
    /// the input, but only while 16 bits of the buffer are still unused.
    pub(crate) fn overrun(&self) -> bool {
        let end = self.input.len();
        self.pos > end && (self.pos > end + 2 || self.left < 16)
    }
}
