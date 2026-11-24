//! Cursor arithmetic of the `IEnumVARIANT` implementations in `msscript.c`.

use crate::{HResult, S_FALSE, S_OK};

/// The result of `module_enum_Next`: how many items were fetched, the new cursor, and the status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Next {
    pub fetched: u32,
    pub pos: u32,
    pub hr: HResult,
}

/// The result of `module_enum_Skip`: the new cursor and the status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skip {
    pub pos: u32,
    pub hr: HResult,
}

/// Fetches up to `celt` items from `pos` among `total`. The status is `S_OK` only when all `celt`
/// items were fetched.
pub fn next(total: u32, pos: u32, celt: u32) -> Next {
    let fetched = celt.min(total.wrapping_sub(pos));
    let hr = if fetched == celt { S_OK } else { S_FALSE };
    Next {
        fetched,
        pos: pos.wrapping_add(fetched),
        hr,
    }
}

/// Advances `pos` by `celt`. If fewer than `celt` items remain, the cursor moves to `total` and the
/// status is `S_FALSE`.
pub fn skip(total: u32, pos: u32, celt: u32) -> Skip {
    if total.wrapping_sub(pos) < celt {
        Skip {
            pos: total,
            hr: S_FALSE,
        }
    } else {
        Skip {
            pos: pos.wrapping_add(celt),
            hr: S_OK,
        }
    }
}
