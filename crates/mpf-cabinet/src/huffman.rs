//! The `make_decode_table` of `fdi.c`, which builds the two-level canonical
//! Huffman decode table from the code lengths.

/// Why [`make_decode_table`] rejected its input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableError {
    /// The code lengths overrun the table or do not form a complete code.
    Invalid,
    /// The output table is shorter than the code needs.
    TableTooSmall,
}

/// The output table. An access past its end sets `overflow` instead of
/// panicking, so the build finishes and the caller reports `TableTooSmall`.
struct Slots<'a> {
    table: &'a mut [u16],
    overflow: bool,
}

impl Slots<'_> {
    fn get(&mut self, index: u32) -> u16 {
        match self.table.get(index as usize) {
            Some(&value) => value,
            None => {
                self.overflow = true;
                0
            }
        }
    }

    fn set(&mut self, index: u32, value: u16) {
        match self.table.get_mut(index as usize) {
            Some(slot) => *slot = value,
            None => self.overflow = true,
        }
    }
}

/// `make_decode_table`: fills `table` with the decode table for the canonical
/// code whose symbol `i` has code length `lengths[i]`. Codes of up to `nbits`
/// bits are looked up directly. Longer codes go through nodes that are
/// allocated from the top of the table, two entries per node. A zero length
/// means the symbol has no code.
pub fn make_decode_table(nbits: u32, lengths: &[u8], table: &mut [u16]) -> Result<(), TableError> {
    if nbits > 15 || lengths.len() > usize::from(u16::MAX) {
        return Err(TableError::Invalid);
    }
    let mut slots = Slots {
        table,
        overflow: false,
    };
    let mut bit_num: u32 = 1;
    let mut pos: u32 = 0;
    let mut table_mask: u32 = 1 << nbits;
    let mut bit_mask: u32 = table_mask >> 1;
    let mut next_symbol: u32 = bit_mask;

    while bit_num <= nbits {
        for (sym, &len) in lengths.iter().enumerate() {
            if u32::from(len) == bit_num {
                let leaf = pos;
                pos += bit_mask;
                if pos > table_mask {
                    return Err(TableError::Invalid);
                }
                for index in leaf..pos {
                    slots.set(index, sym as u16);
                }
            }
        }
        bit_mask >>= 1;
        bit_num += 1;
    }

    if pos != table_mask {
        for index in pos..table_mask {
            slots.set(index, 0);
        }
        pos <<= 16;
        table_mask <<= 16;
        bit_mask = 1 << 15;

        while bit_num <= 16 {
            for (sym, &len) in lengths.iter().enumerate() {
                if u32::from(len) == bit_num {
                    let mut leaf = pos >> 16;
                    for fill in 0..bit_num - nbits {
                        if slots.get(leaf) == 0 {
                            slots.set(next_symbol << 1, 0);
                            slots.set((next_symbol << 1) + 1, 0);
                            slots.set(leaf, next_symbol as u16);
                            next_symbol += 1;
                        }
                        leaf = u32::from(slots.get(leaf)) << 1;
                        if ((pos >> (15 - fill)) & 1) == 1 {
                            leaf += 1;
                        }
                    }
                    slots.set(leaf, sym as u16);
                    pos += bit_mask;
                    if pos > table_mask {
                        return Err(TableError::Invalid);
                    }
                }
            }
            bit_mask >>= 1;
            bit_num += 1;
        }
    }

    if slots.overflow {
        return Err(TableError::TableTooSmall);
    }
    if pos == table_mask || lengths.iter().all(|&len| len == 0) {
        Ok(())
    } else {
        Err(TableError::Invalid)
    }
}
