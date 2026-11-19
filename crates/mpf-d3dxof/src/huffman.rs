//! Canonical Huffman codes, the tables that `fdi_Ziphuft_build` makes for the MSZIP decoder.

use crate::mszip::Bits;
use crate::InflateError;

/// The longest code length in deflate.
const MAX_BITS: usize = 15;

/// A prefix code over the symbols that have a nonzero length, in the canonical order of RFC 1951
/// section 3.2.2. A code is well formed when it is complete, or when it is a single code of
/// length one. Wine accepts only those for the literal and code-length codes.
pub(crate) struct Code {
    /// The number of symbols with each code length.
    counts: [usize; MAX_BITS + 1],
    /// The symbols sorted by code length, then by value.
    symbols: Vec<usize>,
    well_formed: bool,
}

impl Code {
    /// Builds the code in which `lengths[symbol]` is the code length of `symbol`, or zero for no
    /// code. Lengths are at most 15.
    pub(crate) fn new(lengths: &[u8]) -> Self {
        let mut counts = [0; MAX_BITS + 1];
        for &length in lengths {
            counts[usize::from(length)] += 1;
        }
        let space = 1 << MAX_BITS;
        let used: usize = (1..=MAX_BITS)
            .map(|len| counts[len] << (MAX_BITS - len))
            .sum();
        if used > space {
            return Self {
                counts: [0; MAX_BITS + 1],
                symbols: Vec::new(),
                well_formed: false,
            };
        }
        let longest = (1..=MAX_BITS)
            .rev()
            .find(|&len| counts[len] != 0)
            .unwrap_or(0);
        let mut symbols: Vec<usize> = (0..lengths.len())
            .filter(|&symbol| lengths[symbol] != 0)
            .collect();
        symbols.sort_by_key(|&symbol| (lengths[symbol], symbol));
        Self {
            counts,
            symbols,
            well_formed: used == space || longest == 1,
        }
    }

    /// Reads one symbol a bit at a time. The code read so far is compared with the first code of
    /// each length, until it falls among that length's codes.
    pub(crate) fn decode(&self, bits: &mut Bits<'_>) -> Result<usize, InflateError> {
        let mut code = 0;
        let mut first = 0;
        let mut index = 0;
        for &count in &self.counts[1..] {
            code |= bits.bit()?;
            if code < first + count {
                return Ok(self.symbols[index + code - first]);
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        Err(InflateError::IllegalData)
    }

    pub(crate) fn is_well_formed(&self) -> bool {
        self.well_formed
    }
}
