/// A font table, as `struct dwrite_fonttable` holds it. A table the font does not have is
/// [`Table::absent`], and every read from it gives nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Table<'a> {
    data: Option<&'a [u8]>,
}

impl<'a> Table<'a> {
    /// A table the font does not have.
    pub const fn absent() -> Self {
        Self { data: None }
    }

    /// A table with the given bytes.
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data: Some(data) }
    }

    /// `table_read_ensure`: the `size` bytes at `offset`, or `None` when they do not all lie
    /// inside the table.
    pub(crate) fn read_ensure(&self, offset: usize, size: usize) -> Option<&'a [u8]> {
        let data = self.data?;
        if size > data.len() || offset > data.len() - size {
            None
        } else {
            Some(&data[offset..offset + size])
        }
    }

    /// `table_read_be_word`: the big-endian `u16` at `offset`, or 0 when it is outside the table.
    pub(crate) fn be_word(&self, offset: usize) -> u16 {
        match self.read_ensure(offset, 2) {
            Some(&[hi, lo]) => u16::from_be_bytes([hi, lo]),
            _ => 0,
        }
    }

    /// The byte at `offset`, or 0 when it is outside the table.
    pub(crate) fn byte(&self, offset: usize) -> u8 {
        match self.read_ensure(offset, 1) {
            Some(&[value]) => value,
            _ => 0,
        }
    }
}
