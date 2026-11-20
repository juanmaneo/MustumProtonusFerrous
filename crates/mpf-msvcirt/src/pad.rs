//! `ostream::writepad_len` in Wine's `msvcirt.c`: lays out a field of text and fill
//! characters to the stream width.

use crate::{FLAGS_INTERNAL, FLAGS_LEFT};

/// Lays out `str1` and `str2` in a field of `width` characters, padded with `fill`.
///
/// The `left` and `internal` flags pick where the padding goes. With `left`, both
/// strings come before it. With `internal`, `str1` comes before it and `str2` after.
/// With neither, the padding comes first and `str1` and `str2` follow it. A field
/// already wider than `width` gets no padding.
pub fn writepad(flags: u32, width: i32, fill: u8, str1: &[u8], str2: &[u8]) -> Vec<u8> {
    let side = flags & (FLAGS_LEFT | FLAGS_INTERNAL);
    let mut out = Vec::new();
    if side != 0 {
        out.extend_from_slice(str1);
        if (flags & FLAGS_INTERNAL) == 0 {
            out.extend_from_slice(str2);
        }
    }
    let pad = usize::try_from(width)
        .unwrap_or(0)
        .saturating_sub(str1.len() + str2.len());
    out.resize(out.len() + pad, fill);
    if side != FLAGS_LEFT {
        if side == 0 {
            out.extend_from_slice(str1);
        }
        out.extend_from_slice(str2);
    }
    out
}
