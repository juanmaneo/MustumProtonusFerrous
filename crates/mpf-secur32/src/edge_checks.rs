//! Additional parser boundary and opaque-byte checks beyond the sanitizer oracle rows.

use super::parse_alpn_protocol_list;

#[test]
fn empty_input_is_an_empty_valid_list() {
    assert_eq!(parse_alpn_protocol_list(&[]), Some(vec![]));
}

#[test]
fn maximum_length_identifier_accepts_every_byte_value() {
    let payload: Vec<u8> = (0..=u8::MAX).skip(1).collect();
    let mut input = vec![u8::MAX];
    input.extend_from_slice(&payload);

    assert_eq!(
        parse_alpn_protocol_list(&input),
        Some(vec![payload.as_slice()])
    );
}

#[test]
fn parser_accepts_zero_and_non_utf8_bytes_as_opaque_protocol_data() {
    let input = [3, 0, 0x80, 0xff, 1, 0xfe];
    assert_eq!(
        parse_alpn_protocol_list(&input),
        Some(vec![&input[1..4], &input[5..6]])
    );
}

#[test]
fn parser_rejects_a_zero_length_entry_even_after_complete_entries() {
    assert_eq!(parse_alpn_protocol_list(&[1, b'a', 0]), None);
}

#[test]
fn parser_rejects_each_kind_of_truncated_entry() {
    assert_eq!(parse_alpn_protocol_list(&[1]), None);
    assert_eq!(parse_alpn_protocol_list(&[2, b'a']), None);
    assert_eq!(parse_alpn_protocol_list(&[1, b'a', 2, b'b']), None);
}
