//! Port of the DWARF byte readers in Wine's `dlls/dbghelp/dwarf.c` at commit
//! `e2aa6cd75ae60acd90ff384feee171ce994f0f6d`.
//!
//! The C readers advance a `dwarf2_traverse_context_t` cursor through a section.
//! These functions take a byte slice and return the decoded value with the number
//! of bytes it occupies, so the caller moves its own cursor. None of them needs
//! module, section or symbol state.
//!
//! Ported (5 functions):
//! - `dwarf2_get_leb128_as_unsigned`: [`read_uleb128`].
//! - `dwarf2_get_leb128_as_signed`: [`read_sleb128`].
//! - `dwarf2_leb128_length`: [`leb128_len`].
//! - `dwarf2_get_addr`, with `dwarf2_get_u4` and `dwarf2_get_u8`: [`read_addr`].
//! - `dwarf2_parse_3264`, with `dwarf2_parse_u4` and `dwarf2_parse_u8`: [`read_initial_length`].
//!
//! Skipped: the abbreviation and DIE parsers, the location evaluator, the section
//! loader and the symbol builder. They need module state, symbol tables or the ELF
//! and PE section loaders.
//!
//! Divergences from C:
//! - A read past the end of the slice returns `None`. C reads the bytes beyond the input.
//! - A LEB128 value with bits at position 64 or above drops those bits. C shifts by 64 or more there, which is undefined behaviour and is flagged by UBSan. Sign extension still applies only when the encoding covers fewer than 64 bits, as in C.
//! - [`read_addr`] returns `None` for a word size other than 4 or 8. C returns 0 and logs a FIXME.
//! - FIXME and WARN diagnostics are not emitted.
//! - Values are 64-bit. C's `ULONG_PTR` is 32-bit on 32-bit targets, and the oracle runs on LP64.

/// The 32-bit initial length that announces 64-bit DWARF. The 8-byte length follows it.
const DWARF64_ESCAPE: u32 = 0xffff_ffff;

/// Decodes an unsigned LEB128 value from the start of `bytes`.
///
/// Returns the value and the number of bytes it occupies. `None` means `bytes` ends
/// before a byte without the continuation bit.
pub fn read_uleb128(bytes: &[u8]) -> Option<(u64, usize)> {
    let mut value = 0u64;
    let mut shift = 0u32;
    for (index, &byte) in bytes.iter().enumerate() {
        if let Some(bits) = u64::from(byte & 0x7f).checked_shl(shift) {
            value |= bits;
        }
        if (byte & 0x80) == 0 {
            return Some((value, index + 1));
        }
        shift += 7;
    }
    None
}

/// Decodes a signed LEB128 value from the start of `bytes`.
///
/// Returns the value and the number of bytes it occupies. The value is sign-extended
/// only when the encoding covers fewer than 64 bits.
pub fn read_sleb128(bytes: &[u8]) -> Option<(i64, usize)> {
    let mut value = 0u64;
    let mut shift = 0u32;
    for (index, &byte) in bytes.iter().enumerate() {
        if let Some(bits) = u64::from(byte & 0x7f).checked_shl(shift) {
            value |= bits;
        }
        shift += 7;
        if (byte & 0x80) == 0 {
            if shift < u64::BITS && (byte & 0x40) != 0 {
                value |= !0u64 << shift;
            }
            return Some((value as i64, index + 1));
        }
    }
    None
}

/// Returns the length of the LEB128 encoding at the start of `bytes`. `None` means the encoding is not terminated.
pub fn leb128_len(bytes: &[u8]) -> Option<usize> {
    bytes
        .iter()
        .position(|&byte| (byte & 0x80) == 0)
        .map(|index| index + 1)
}

/// Reads a little-endian unsigned address or offset of `word_size` bytes, which must be 4 or 8.
///
/// Returns `None` if `word_size` is not 4 or 8, or if `bytes` is shorter than `word_size`.
pub fn read_addr(bytes: &[u8], word_size: u8) -> Option<u64> {
    match word_size {
        4 => Some(u64::from(u32::from_le_bytes(*bytes.first_chunk::<4>()?))),
        8 => Some(u64::from_le_bytes(*bytes.first_chunk::<8>()?)),
        _ => None,
    }
}

/// An initial length or section offset, as [`read_initial_length`] decodes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InitialLength {
    /// The length or offset value.
    pub value: u64,
    /// The offset size in bytes: 4 for 32-bit DWARF and 8 for 64-bit DWARF.
    pub offset_size: u8,
    /// The bytes the encoding occupies: 4 for 32-bit DWARF and 12 for 64-bit DWARF.
    pub consumed: usize,
}

/// Ports `dwarf2_parse_3264`: reads a 32-bit initial length, or the 8-byte form that follows the `0xffffffff` escape.
///
/// Returns `None` if `bytes` is too short for the encoding it announces.
pub fn read_initial_length(bytes: &[u8]) -> Option<InitialLength> {
    let (head, rest) = bytes.split_first_chunk::<4>()?;
    let short = u32::from_le_bytes(*head);
    if short != DWARF64_ESCAPE {
        return Some(InitialLength {
            value: u64::from(short),
            offset_size: 4,
            consumed: 4,
        });
    }
    let value = read_addr(rest, 8)?;
    Some(InitialLength {
        value,
        offset_size: 8,
        consumed: 12,
    })
}

#[cfg(test)]
mod edge_checks;

#[cfg(test)]
mod oracle_checks;
