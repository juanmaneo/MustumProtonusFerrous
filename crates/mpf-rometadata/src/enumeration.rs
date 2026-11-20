//! Token enumerations from `mdtables.c`: `token_enum_range_create`, `token_enum_list_create`,
//! `token_enum_get_entries`, and the pure parts of `import_CountEnum` and `import_ResetEnum`.
//! The COM wrappers and the `HCORENUM` handle are not ported; a [`TokenEnum`] owns its state.
//!
//! Divergence: C asserts that a range starts above zero before it counts the range.
//! [`TokenEnum::count`] does not assert, and for `start == 0` it returns `end + 1` with wrapping
//! arithmetic.

use crate::token_from_rid;

/// An enumeration over either a range of rows or an explicit list of tokens, with a cursor.
pub enum TokenEnum {
    Range {
        token_type: u32,
        start: u32,
        end: u32,
        pos: u32,
    },
    List {
        tokens: Vec<u32>,
        pos: u32,
    },
}

impl TokenEnum {
    /// Enumerates rows `start..=end` of `token_type` (C `token_enum_range_create`). Panics where C
    /// asserts that `start <= end`.
    pub fn range(token_type: u32, start: u32, end: u32) -> Self {
        assert!(start <= end, "a range starts at or before its end");
        Self::Range {
            token_type,
            start,
            end,
            pos: start,
        }
    }

    /// Enumerates `tokens` (C `token_enum_list_create`). Panics on an empty list, where C asserts.
    pub fn list(tokens: Vec<u32>) -> Self {
        assert!(!tokens.is_empty(), "a token list is not empty");
        Self::List { tokens, pos: 0 }
    }

    /// The number of entries (C `import_CountEnum`). A range counts with wrapping arithmetic.
    pub fn count(&self) -> u32 {
        match self {
            Self::Range { start, end, .. } => end.wrapping_sub(*start).wrapping_add(1),
            Self::List { tokens, .. } => tokens.len() as u32,
        }
    }

    /// Moves the cursor to entry `index` (C `import_ResetEnum`). For a range the index is an
    /// offset from `start`, and the addition wraps as in C.
    pub fn reset(&mut self, index: u32) {
        match self {
            Self::Range { start, pos, .. } => *pos = start.wrapping_add(index),
            Self::List { pos, .. } => *pos = index,
        }
    }

    /// The cursor position, as C `pos`.
    pub fn position(&self) -> u32 {
        match self {
            Self::Range { pos, .. } | Self::List { pos, .. } => *pos,
        }
    }

    /// Copies up to `buf.len()` entries from the cursor into `buf` and advances the cursor (C
    /// `token_enum_get_entries`). Returns the number of entries written. A range wraps its cursor
    /// at `u32::MAX`, as the C code does.
    pub fn get_entries(&mut self, buf: &mut [u32]) -> usize {
        match self {
            Self::Range {
                token_type,
                end,
                pos,
                ..
            } => {
                let mut n = 0;
                while *pos <= *end && n < buf.len() {
                    buf[n] = token_from_rid(*pos, *token_type);
                    *pos = pos.wrapping_add(1);
                    n += 1;
                }
                n
            }
            Self::List { tokens, pos } => {
                let start = *pos as usize;
                if start >= tokens.len() {
                    return 0;
                }
                let n = (tokens.len() - start).min(buf.len());
                buf[..n].copy_from_slice(&tokens[start..start + n]);
                *pos += n as u32;
                n
            }
        }
    }
}
