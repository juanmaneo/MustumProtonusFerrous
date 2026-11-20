use crate::rtf_char_to_hex;

#[test]
fn maps_ascii_boundaries_like_the_source_arithmetic() {
    assert_eq!(rtf_char_to_hex(0), -87);
    assert_eq!(rtf_char_to_hex(b'G'), 16);
    assert_eq!(rtf_char_to_hex(b'z'), 35);
}
