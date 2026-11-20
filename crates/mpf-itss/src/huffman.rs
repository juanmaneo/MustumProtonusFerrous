//! Canonical Huffman tables and the code length reader of lzx.c.

use crate::bits::Bits;
use crate::DecrError;

/// Code lengths are kept this far past the symbol count. READ_LENGTHS may write past the last
/// symbol it was asked for, by up to 50 entries (LZX_LENTABLE_SAFETY).
const LENTABLE_SAFETY: usize = 64;

/// A canonical Huffman code: the lookup table built by make_decode_table and the code lengths
/// it was built from.
pub(crate) struct Table {
    symbols: usize,
    bits: u32,
    table: Vec<u16>,
    pub(crate) lens: Vec<u8>,
}

impl Table {
    /// A table of `symbols` symbols that decodes codes of up to `bits` bits in one lookup. All
    /// entries start at zero.
    pub(crate) fn new(symbols: usize, bits: u32) -> Self {
        Table {
            symbols,
            bits,
            table: vec![0; (1 << bits) + 2 * symbols],
            lens: vec![0; symbols + LENTABLE_SAFETY],
        }
    }

    /// make_decode_table: builds the lookup table from the code lengths. Codes of up to `bits`
    /// bits fill the table directly. Longer codes follow tree nodes that are allocated after the
    /// direct entries, two table entries per node.
    pub(crate) fn build(&mut self) -> Result<(), DecrError> {
        let nbits = self.bits as usize;
        let lens = &self.lens[..self.symbols];
        let table = &mut self.table;

        let mut table_mask = 1usize << nbits;
        let mut bit_mask = table_mask >> 1;
        let mut next = bit_mask;
        let mut pos = 0usize;
        let mut bit_num = 1usize;

        while bit_num <= nbits {
            for (sym, &len) in lens.iter().enumerate() {
                if usize::from(len) == bit_num {
                    let leaf = pos;
                    pos += bit_mask;
                    if pos > table_mask {
                        return Err(DecrError::IllegalData);
                    }
                    table[leaf..pos].fill(sym as u16);
                }
            }
            bit_mask >>= 1;
            bit_num += 1;
        }

        if pos != table_mask {
            table[pos..table_mask].fill(0);
            pos <<= 16;
            table_mask <<= 16;
            bit_mask = 1 << 15;

            while bit_num <= 16 {
                for (sym, &len) in lens.iter().enumerate() {
                    if usize::from(len) == bit_num {
                        let mut leaf = pos >> 16;
                        for fill in 0..bit_num - nbits {
                            if table[leaf] == 0 {
                                if 2 * next + 1 >= table.len() {
                                    return Err(DecrError::IllegalData);
                                }
                                table[2 * next] = 0;
                                table[2 * next + 1] = 0;
                                table[leaf] = next as u16;
                                next += 1;
                            }
                            leaf = usize::from(table[leaf]) << 1;
                            if (pos >> (15 - fill)) & 1 != 0 {
                                leaf += 1;
                            }
                        }
                        table[leaf] = sym as u16;
                        pos += bit_mask;
                        if pos > table_mask {
                            return Err(DecrError::IllegalData);
                        }
                    }
                }
                bit_mask >>= 1;
                bit_num += 1;
            }
        }

        if pos == table_mask {
            return Ok(());
        }
        if lens.iter().any(|&len| len != 0) {
            return Err(DecrError::IllegalData);
        }
        Ok(())
    }

    /// READ_HUFFSYM: decodes one symbol and consumes its code. [read_lengths] keeps every length
    /// in 0..=16, so a code fits in the 16 bits that [Bits::ensure] loads. A link from a node leads
    /// to a later node or to a symbol, so the walk ends even in a table that a failed build left.
    pub(crate) fn decode(&self, bits: &mut Bits) -> usize {
        bits.ensure(16);
        let mut sym = usize::from(self.table[bits.peek(self.bits) as usize]);
        let mut j = 1u32 << (32 - self.bits);
        while sym >= self.symbols {
            j >>= 1;
            sym <<= 1;
            if bits.test_bit(j) {
                sym |= 1;
            }
            sym = usize::from(self.table[sym]);
        }
        bits.consume(u32::from(self.lens[sym]));
        sym
    }
}

/// lzx_read_lens: reads the code lengths of symbols `first..last` into `lens`. Each length is
/// coded as a symbol of `pretree` that gives the change from the old length, mod 17. Symbols
/// 17 to 19 are runs of zeros or of one repeated change.
pub(crate) fn read_lengths(
    bits: &mut Bits,
    pretree: &mut Table,
    lens: &mut [u8],
    first: usize,
    last: usize,
) -> Result<(), DecrError> {
    for len in &mut pretree.lens[..pretree.symbols] {
        *len = bits.read(4) as u8;
    }
    pretree.build()?;

    let mut x = first;
    while x < last {
        let z = pretree.decode(bits);
        match z {
            17 => {
                let run = bits.read(4) as usize + 4;
                lens[x..x + run].fill(0);
                x += run;
            }
            18 => {
                let run = bits.read(5) as usize + 20;
                lens[x..x + run].fill(0);
                x += run;
            }
            19 => {
                let run = bits.read(1) as usize + 4;
                let z = pretree.decode(bits);
                let v = delta(lens[x], z);
                if !(0..=16).contains(&v) {
                    return Err(DecrError::IllegalData);
                }
                lens[x..x + run].fill(v as u8);
                x += run;
            }
            _ => {
                lens[x] = delta(lens[x], z) as u8;
                x += 1;
            }
        }
    }
    Ok(())
}

/// The new code length `old - z`, plus 17 when that is negative. Only a run of 19 can decode a
/// `z` above 16, and then the result can still be negative, which [read_lengths] rejects. Any
/// other result is in 0..=16.
fn delta(old: u8, z: usize) -> i32 {
    let d = i32::from(old) - z as i32;
    if d < 0 {
        d + 17
    } else {
        d
    }
}
