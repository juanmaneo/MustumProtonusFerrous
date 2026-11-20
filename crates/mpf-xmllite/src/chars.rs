//! Character classes from reader.c and writer.c over UTF-16 code units. The C compares each
//! range with two bounds; the same ranges are written here as range patterns.

/// `is_char`: the XML Char production. The surrogate units are included.
pub fn is_char(ch: u16) -> bool {
    matches!(ch, 0x09 | 0x0a | 0x0d | 0x20..=0xfffd)
}

/// `is_wchar_space`: the XML white space characters.
pub fn is_wchar_space(ch: u16) -> bool {
    matches!(ch, 0x09 | 0x0a | 0x0d | 0x20)
}

/// `is_pubchar`: the PubidChar production as the C tests it. The C's range `'-'..';'` leaves
/// out `'`, `(`, `)`, `*`, `+` and `,`, though its comment lists them, so they are rejected here.
pub fn is_pubchar(ch: u16) -> bool {
    matches!(
        ch,
        0x0a | 0x0d | 0x20 | 0x21 | 0x23..=0x25 | 0x2d..=0x3b | 0x3d | 0x3f..=0x5a
            | 0x5f | 0x61..=0x7a
    )
}

/// `is_ncnamestartchar`: the NCName start characters. The surrogate units are included.
pub fn is_ncnamestartchar(ch: u16) -> bool {
    matches!(
        ch,
        0x41..=0x5a
            | 0x5f
            | 0x61..=0x7a
            | 0xc0..=0xd6
            | 0xd8..=0xf6
            | 0xf8..=0x2ff
            | 0x370..=0x37d
            | 0x37f..=0x1fff
            | 0x200c..=0x200d
            | 0x2070..=0x218f
            | 0x2c00..=0x2fef
            | 0x3001..=0xdfff
            | 0xf900..=0xfdcf
            | 0xfdf0..=0xfffd
    )
}

/// `is_namestartchar`: the Name start characters, which add ':' to the NCName ones.
pub fn is_namestartchar(ch: u16) -> bool {
    ch == 0x3a || is_ncnamestartchar(ch)
}

/// `is_ncnamechar`: the NCName characters. The surrogate units are included.
pub fn is_ncnamechar(ch: u16) -> bool {
    matches!(
        ch,
        0x2d..=0x2e
            | 0x30..=0x39
            | 0x41..=0x5a
            | 0x5f
            | 0x61..=0x7a
            | 0xb7
            | 0xc0..=0xd6
            | 0xd8..=0xf6
            | 0xf8..=0x2ff
            | 0x300..=0x36f
            | 0x370..=0x37d
            | 0x37f..=0x1fff
            | 0x200c..=0x200d
            | 0x203f..=0x2040
            | 0x2070..=0x218f
            | 0x2c00..=0x2fef
            | 0x3001..=0xdfff
            | 0xf900..=0xfdcf
            | 0xfdf0..=0xfffd
    )
}

/// `is_namechar`: the Name characters, which add ':' to the NCName ones.
pub fn is_namechar(ch: u16) -> bool {
    ch == 0x3a || is_ncnamechar(ch)
}

pub(crate) fn is_high_surrogate(ch: u16) -> bool {
    matches!(ch, 0xd800..=0xdbff)
}

pub(crate) fn is_low_surrogate(ch: u16) -> bool {
    matches!(ch, 0xdc00..=0xdfff)
}
