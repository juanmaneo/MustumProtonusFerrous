//! The code point classes that `opentype.c` uses when it shapes text. Each class is a fixed list of
//! Unicode ranges, and the ranges below are the ones the C code lists in its comments.

/// `opentype_is_zero_width`: the code points that take no advance.
pub fn is_zero_width(codepoint: u32) -> bool {
    matches!(
        codepoint,
        0x034f | 0x061c | 0xfeff | 0x180b..=0x180e | 0x200b..=0x200f
    )
}

/// `opentype_is_default_ignorable`: the code points that a renderer skips when no font covers
/// them. The C early return for code points below 0x80 is implied by the ranges, which all start
/// above it.
pub fn is_default_ignorable(codepoint: u32) -> bool {
    matches!(
        codepoint,
        0x00ad
            | 0x034f
            | 0x061c
            | 0x17b4..=0x17b5
            | 0x180b..=0x180e
            | 0x200b..=0x200f
            | 0x202a..=0x202e
            | 0x2060..=0x206f
            | 0xfe00..=0xfe0f
            | 0xfeff
            | 0xfff0..=0xfff8
            | 0x1d173..=0x1d17a
            | 0xe0000..=0xe0fff
    )
}

/// `opentype_is_whitespace`: the white space code points, including the tabs, line breaks and
/// the space separators.
pub fn is_whitespace(codepoint: u32) -> bool {
    matches!(
        codepoint,
        0x09..=0x0d
            | 0x20
            | 0x85
            | 0xa0
            | 0x1680
            | 0x2000..=0x200a
            | 0x2028
            | 0x2029
            | 0x202f
            | 0x205f
            | 0x3000
    )
}
