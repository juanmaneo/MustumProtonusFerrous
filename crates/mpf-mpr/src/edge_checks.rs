use crate::{ctox, decode_hex_pair, decode_value_name, hex_digit, select_resource, value_prefix};

#[test]
fn hexadecimal_helpers_cover_ascii_and_nibble_edges() {
    for value in 0..=9 {
        assert_eq!(ctox(b'0' + value), Some(value));
        assert_eq!(hex_digit(value), b'0' + value);
    }
    for value in 10..=15 {
        assert_eq!(ctox(b'A' + value - 10), Some(value));
        assert_eq!(ctox(b'a' + value - 10), Some(value));
        assert_eq!(hex_digit(value), b'A' + value - 10);
    }

    assert_eq!(ctox(b'/'), None);
    assert_eq!(ctox(b'G'), None);
    assert_eq!(ctox(b'g'), None);
    assert_eq!(hex_digit(0xff), b'F');
    assert_eq!(decode_hex_pair(b'a', b'F'), Some(0xaf));
    assert_eq!(decode_hex_pair(b'G', b'0'), None);
    assert_eq!(decode_hex_pair(b'0', b'g'), None);
}

#[test]
fn decoding_matches_the_value_name_scan_rules() {
    assert_eq!(value_prefix(0x0a), "X-0A-");
    assert_eq!(decode_value_name(b"", 1), None);
    assert_eq!(decode_value_name(b"X-01-", 1), None);
    assert_eq!(decode_value_name(b"X-02-00", 1), None);
    assert_eq!(decode_value_name(b"x-01-00", 1), None);
    assert_eq!(decode_value_name(b"X-01-aF09", 1), Some(vec![0xaf, 0x09]));
    assert_eq!(decode_value_name(b"X-01-G000", 1), Some(vec![]));
    assert_eq!(decode_value_name(b"X-01-0G00", 1), Some(vec![]));
    assert_eq!(decode_value_name(b"X-01-ABX1", 1), Some(vec![0xab]));
    assert_eq!(decode_value_name(b"X-01-A", 1), Some(vec![]));
}

#[test]
fn resource_selection_applies_the_decoded_prefix_filter() {
    let name = b"X-01-616263";
    assert_eq!(select_resource(name, 1, b"ab"), Some(b"abc".to_vec()));
    assert_eq!(select_resource(name, 1, b"abcd"), None);
    assert_eq!(select_resource(name, 1, b"ax"), None);
    assert_eq!(select_resource(name, 1, b""), Some(b"abc".to_vec()));
    assert_eq!(select_resource(b"X-01-6Z", 1, b""), Some(vec![]));
}
