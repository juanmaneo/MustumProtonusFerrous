//! Shared item-window arithmetic in the file and job enumerators.

use crate::{HResult, S_FALSE, S_OK};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub hresult: HResult,
    pub count: u32,
    pub next_index: u32,
}

/// Model the count and index update in `EnumBackgroundCopy{Files,Jobs}_Next`.
pub fn next(total: u32, index: u32, requested: u32) -> Window {
    let count = requested.min(total.saturating_sub(index));
    Window {
        hresult: if count == requested { S_OK } else { S_FALSE },
        count,
        next_index: index + count,
    }
}

/// Model the index update in `EnumBackgroundCopy{Files,Jobs}_Skip`.
pub fn skip(total: u32, index: u32, requested: u32) -> Window {
    let remaining = total.saturating_sub(index);
    if requested > remaining {
        Window {
            hresult: S_FALSE,
            count: 0,
            next_index: total,
        }
    } else {
        Window {
            hresult: S_OK,
            count: requested,
            next_index: index + requested,
        }
    }
}
