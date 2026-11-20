//! Expected masks from Wine's verbatim `wctype` implementation compiled with
//! GCC AddressSanitizer and UndefinedBehaviorSanitizer.

use crate::wctype;

const ORACLE_ROWS: &[(&[u8], u16)] = &[
    (b"alnum", 263),
    (b"alpha", 259),
    (b"cntrl", 32),
    (b"digit", 4),
    (b"graph", 279),
    (b"lower", 2),
    (b"print", 343),
    (b"punct", 16),
    (b"space", 8),
    (b"upper", 1),
    (b"xdigit", 128),
    (b"ALNUM", 0),
    (b"Alnum", 0),
    (b"", 0),
    (b"unknown", 0),
    (b"alnumx", 0),
    (b"xalnum", 0),
    (b"white", 0),
    (b"blank", 0),
    (b"tolower", 0),
];

#[test]
fn matches_wine_oracle_rows() {
    for &(property, expected) in ORACLE_ROWS {
        assert_eq!(wctype(property), expected, "property {property:?}");
    }
}
