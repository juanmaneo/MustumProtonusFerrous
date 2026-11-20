//! `parse_schema` from Wine's `dlls/urlmon/internet.c`: the scheme is the text
//! before the first colon.

use crate::wide_str;

const COLON: u16 = b':' as u16;

/// `E_POINTER`, the error when the scheme and its NUL do not fit in the buffer.
const E_POINTER: u32 = 0x8000_4003;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// `E_POINTER`
    Pointer,
}

impl Error {
    /// The `HRESULT` that Wine returns for this error.
    pub fn hresult(self) -> u32 {
        match self {
            Error::Pointer => E_POINTER,
        }
    }
}

/// The output of [`parse_schema`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    /// The length of the scheme. Wine sets `*rsize` on every call.
    pub rsize: u32,
    /// The scheme followed by a NUL, when it fits in the buffer.
    pub result: Result<Vec<u16>, Error>,
}

/// Splits the scheme off `url` for a buffer of `size` code units. `url` is read
/// up to its first NUL. A scheme that needs `size` or more code units fails.
pub fn parse_schema(url: &[u16], size: u32) -> Schema {
    let url = wide_str(url);
    let len = url.iter().position(|&c| c == COLON).unwrap_or(0);
    let rsize = len as u32;
    if rsize >= size {
        return Schema {
            rsize,
            result: Err(Error::Pointer),
        };
    }
    let mut result = url[..len].to_vec();
    result.push(0);
    Schema {
        rsize,
        result: Ok(result),
    }
}
