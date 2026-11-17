//! C-locale character classification from Wine's `dlls/msvcrt/ctype.c`.
//!
//! Wine classifies through `_isctype`, whose table covers `-1..=255` where
//! `-1` is EOF. The C locale defines classes only for ASCII, so EOF and bytes
//! above 127 belong to no class. `toupper` and `tolower` map only ASCII.

fn ascii_byte(c: i32) -> Option<u8> {
    u8::try_from(c).ok()
}

pub fn isalpha(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_alphabetic())
}

pub fn isdigit(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_digit())
}

pub fn isalnum(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_alphanumeric())
}

pub fn isupper(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_uppercase())
}

pub fn islower(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_lowercase())
}

pub fn isxdigit(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_hexdigit())
}

/// `isspace` includes vertical tab, which Rust's `is_ascii_whitespace` omits.
pub fn isspace(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| matches!(b, b' ' | b'\t'..=b'\r'))
}

pub fn iscntrl(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_control())
}

pub fn isgraph(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_graphic())
}

pub fn isprint(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b == b' ' || b.is_ascii_graphic())
}

pub fn ispunct(c: i32) -> bool {
    ascii_byte(c).is_some_and(|b| b.is_ascii_punctuation())
}

/// Maps ASCII lowercase to uppercase; every other value is returned unchanged.
pub fn toupper(c: i32) -> i32 {
    ascii_byte(c).map_or(c, |b| i32::from(b.to_ascii_uppercase()))
}

/// Maps ASCII uppercase to lowercase; every other value is returned unchanged.
pub fn tolower(c: i32) -> i32 {
    ascii_byte(c).map_or(c, |b| i32::from(b.to_ascii_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_digits_and_alnum() {
        assert!(isalpha(i32::from(b'a')) && isalpha(i32::from(b'Z')));
        assert!(!isalpha(i32::from(b'5')) && !isalpha(-1) && !isalpha(0xe9));
        assert!(isdigit(i32::from(b'0')) && isdigit(i32::from(b'9')));
        assert!(!isdigit(i32::from(b'a')));
        assert!(isalnum(i32::from(b'q')) && isalnum(i32::from(b'7')));
        assert!(!isalnum(i32::from(b'_')));
        assert!(isupper(i32::from(b'A')) && !isupper(i32::from(b'a')));
        assert!(islower(i32::from(b'a')) && !islower(i32::from(b'A')));
        assert!(isxdigit(i32::from(b'F')) && isxdigit(i32::from(b'9')));
        assert!(!isxdigit(i32::from(b'g')));
    }

    #[test]
    fn space_includes_vertical_tab() {
        for b in [b' ', b'\t', b'\n', 0x0b, 0x0c, b'\r'] {
            assert!(isspace(i32::from(b)));
        }
        assert!(!isspace(i32::from(b'a')) && !isspace(0x85) && !isspace(-1));
    }

    #[test]
    fn control_graph_print_punct() {
        assert!(iscntrl(0) && iscntrl(0x1f) && iscntrl(0x7f));
        assert!(!iscntrl(i32::from(b' ')));
        assert!(isgraph(i32::from(b'!')) && !isgraph(i32::from(b' ')));
        assert!(isprint(i32::from(b' ')) && !isprint(0x1f));
        assert!(ispunct(i32::from(b'!')) && !ispunct(i32::from(b'a')));
        assert!(!ispunct(i32::from(b' ')));
    }

    #[test]
    fn eof_and_high_bytes_are_unclassified() {
        assert!(!isalpha(-1) && !isdigit(-1) && !isspace(-1));
        assert!(!isalnum(256) && !isprint(0x80));
    }

    #[test]
    fn case_mapping_is_ascii_only() {
        assert_eq!(toupper(i32::from(b'a')), i32::from(b'A'));
        assert_eq!(toupper(i32::from(b'A')), i32::from(b'A'));
        assert_eq!(toupper(0xe9), 0xe9);
        assert_eq!(toupper(-1), -1);
        assert_eq!(tolower(i32::from(b'Z')), i32::from(b'z'));
        assert_eq!(tolower(i32::from(b'1')), i32::from(b'1'));
        assert_eq!(tolower(0xc9), 0xc9);
    }
}
