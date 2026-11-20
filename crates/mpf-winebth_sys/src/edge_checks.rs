//! Cases the oracle rows do not reach: the C string bound, the length check, and a non-hex byte
//! at each point where the scanner or a literal dash must match.

use super::{parse_uuid, Uuid};

const VALID: &[u8] = b"00112233-4455-6677-8899-aabbccddeeff";

#[test]
fn c_string_ends_at_first_nul() {
    let mut terminated = VALID.to_vec();
    terminated.extend_from_slice(b"\0ignored");
    assert_eq!(parse_uuid(&terminated), parse_uuid(VALID));
    assert!(parse_uuid(&terminated).is_some());
}

#[test]
fn empty_and_nul_only_strings_fail_the_length_check() {
    assert_eq!(parse_uuid(b""), None);
    assert_eq!(parse_uuid(b"\0"), None);
}

#[test]
fn non_hex_byte_at_each_scanned_field_start_fails() {
    // The first byte of every field and every literal dash. A non-hex byte in the second
    // position of a field is not checked: `%2hhx` stops early and the next conversion fails.
    for index in [0, 8, 9, 13, 14, 18, 19, 21, 23, 24, 26, 28, 30, 32, 34] {
        let mut text = VALID.to_vec();
        text[index] = b'g';
        assert_eq!(parse_uuid(&text), None, "index {index}");
    }
}

#[test]
fn dash_replaced_by_other_byte_fails() {
    let mut text = VALID.to_vec();
    text[23] = b'x';
    assert_eq!(parse_uuid(&text), None);
}

#[test]
fn one_byte_too_long_or_short_fails() {
    assert_eq!(parse_uuid(&VALID[..35]), None);
    let mut long = VALID.to_vec();
    long.push(b'0');
    assert_eq!(parse_uuid(&long), None);
}

#[test]
fn uppercase_and_lowercase_hex_decode_the_same() {
    let upper = parse_uuid(b"AABBCCDD-EEFF-1122-3344-55667788990A");
    let lower = parse_uuid(b"aabbccdd-eeff-1122-3344-55667788990a");
    assert_eq!(upper, lower);
    assert_eq!(
        upper,
        Some(Uuid {
            data1: 0xaabb_ccdd,
            data2: 0xeeff,
            data3: 0x1122,
            data4: [0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x0a],
        })
    );
}
