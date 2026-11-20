//! Boundary behavior not represented by the oracle's NUL-terminated arguments.

use crate::wctype;

#[test]
fn bounded_c_string_and_non_ascii_edges() {
    assert_eq!(wctype(b"alpha\0ignored"), 0x103);
    assert_eq!(wctype(b"\0alpha"), 0);
    assert_eq!(wctype(b""), 0);
    assert_eq!(wctype(b"alp"), 0);
    assert_eq!(wctype(b" alpha"), 0);
    assert_eq!(wctype(b"ALPHA"), 0);
    assert_eq!(wctype(&[0xff, b'a', b'l', b'p', b'h', b'a']), 0);
    assert_eq!(wctype(b"alpha\n"), 0);
    assert_eq!(wctype(b"print\0"), 0x157);
}
