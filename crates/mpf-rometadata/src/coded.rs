//! Bit widths, coded index sizes and coded value decoding from `assembly.c`: `bit_width`,
//! `index_size`, `assembly_get_coded_index_size` and `metadata_coded_value_as_token`.
//!
//! Divergence: the C code asserts when a coded value's tag selects past the end of the table
//! list. [`coded_value_as_token`] returns `None` in that case.

use crate::token_from_rid;

/// The bit width for `n` (C `bit_width`). It counts the bits of `(n - 1) >> 1` and wraps for
/// `n == 0`, which gives 32.
pub fn bit_width(n: u32) -> u32 {
    let mut bits = 1;
    let mut rest = n.wrapping_sub(1) >> 1;
    while rest != 0 {
        bits += 1;
        rest >>= 1;
    }
    bits
}

/// The size in bytes of a simple table index for a table with `rows` rows (C `index_size`).
pub fn index_size(rows: u32) -> u32 {
    if rows < 1 << 16 {
        2
    } else {
        4
    }
}

/// The size in bytes of a coded index over `tables` (C `assembly_get_coded_index_size`). Each
/// entry is the row count of one table, or `None` for a slot that is unused (`TABLE_MAX`).
/// `tables` must not be empty.
pub fn coded_index_size(tables: &[Option<u32>]) -> u32 {
    assert!(!tables.is_empty(), "a coded index has at least one table");
    let tag_bits = bit_width(tables.len() as u32);
    let max_rows = tables.iter().flatten().copied().fold(0, u32::max);
    if max_rows < 1 << (16 - tag_bits) {
        2
    } else {
        4
    }
}

/// Decodes a coded index value into a token (C `metadata_coded_value_as_token`). The low bits of
/// `value` select one of `tables`, and the remaining bits are the row. `tables` must not be
/// empty. Returns `None` where the C code asserts, which is when the tag is past the end of
/// `tables`.
pub fn coded_value_as_token(value: u32, tables: &[u32]) -> Option<u32> {
    assert!(!tables.is_empty(), "a coded index has at least one table");
    let tag_bits = bit_width(tables.len() as u32);
    let tag = value & ((1 << tag_bits) - 1);
    let table = *tables.get(tag as usize)?;
    Some(token_from_rid(value >> tag_bits, table << 24))
}
