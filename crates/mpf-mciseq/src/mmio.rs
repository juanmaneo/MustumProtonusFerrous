/// The origin of a seek: `SEEK_SET` or `SEEK_CUR`.
pub enum Whence {
    Set,
    Cur,
}

/// A MIDI image with a read position, following `mmioSeek` and `mmioRead`.
pub struct MmioFile<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> MmioFile<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    /// `mmioSeek`: moves to `offset` from the origin and returns the new position. A target outside
    /// `0..=i32::MAX` fails with -1 and leaves the position unchanged. The position may pass the end.
    pub fn seek(&mut self, offset: i32, whence: Whence) -> i32 {
        let base = match whence {
            Whence::Set => 0,
            Whence::Cur => self.pos as i64,
        };
        let target = base + i64::from(offset);
        if (0..=i64::from(i32::MAX)).contains(&target) {
            self.pos = target as usize;
            target as i32
        } else {
            -1
        }
    }

    /// `mmioRead`: copies up to `buf.len()` bytes from the position and returns how many were copied.
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let rest = self.data.get(self.pos..).unwrap_or(&[]);
        let n = rest.len().min(buf.len());
        buf[..n].copy_from_slice(&rest[..n]);
        self.pos += n;
        n
    }

    /// Advances over `len` bytes, stopping at the end of the image, as `mmioRead` into a discarded buffer does.
    pub fn skip(&mut self, len: usize) {
        self.pos += len.min(self.data.len().saturating_sub(self.pos));
    }

    /// `MIDI_mciReadByte`: `None` when the image ends first.
    pub fn read_byte(&mut self) -> Option<u8> {
        let mut byte = [0u8];
        (self.read(&mut byte) == 1).then_some(byte[0])
    }

    /// `MIDI_mciReadWord`: big-endian, high byte first.
    pub fn read_word(&mut self) -> Option<u16> {
        let hi = self.read_byte()?;
        let lo = self.read_byte()?;
        Some((u16::from(hi) << 8) | u16::from(lo))
    }

    /// `MIDI_mciReadLong`: two big-endian words, the first one high.
    pub fn read_long(&mut self) -> Option<u32> {
        let hi = self.read_word()?;
        let lo = self.read_word()?;
        Some((u32::from(hi) << 16) | u32::from(lo))
    }

    /// `MIDI_mciReadVaryLen`: a variable-length quantity of seven-bit groups, high group first.
    /// Returns the number of bytes read and the value. A read failure gives `(0, 0)`.
    pub fn read_var_len(&mut self) -> (u16, u32) {
        let mut value = 0u32;
        let mut len = 0u16;
        loop {
            let Some(byte) = self.read_byte() else {
                return (0, 0);
            };
            value = (value << 7) | u32::from(byte & 0x7F);
            len = len.wrapping_add(1);
            if byte & 0x80 == 0 {
                return (len, value);
            }
        }
    }
}
