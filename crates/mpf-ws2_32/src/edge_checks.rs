//! Boundary and malformed-input checks. Expectations marked `oracle` were observed from the
//! sanitized C harness in `target/canon/ws2_32`; the others follow directly from its logic.

use super::inet_addr::{inet_addr, InetAddrError, INADDR_NONE};

#[test]
fn null_input_is_distinguished_from_the_all_ones_value() {
    assert_eq!(inet_addr(None), Err(InetAddrError::NullInput));
    assert_eq!(inet_addr(Some(b"255.255.255.255")), Ok(INADDR_NONE));
}

#[test]
fn parsing_stops_at_a_c_string_terminator() {
    // oracle: "1.2.3.4\0junk"
    assert_eq!(inet_addr(Some(b"1.2.3.4\0junk")), Ok(0x0403_0201));
    // oracle: "\0 1.2.3.4"
    assert_eq!(inet_addr(Some(b"\0 1.2.3.4")), Ok(INADDR_NONE));
}

#[test]
fn rejects_bad_component_counts_and_separators() {
    // oracle: "1.2.3.4.5", "1.2.3.4.", "1.2.3.", "1.2.3.-1", "1.2.3.0x", "0x.1", "08"
    for input in [
        &b"1.2.3.4.5"[..],
        b"1.2.3.4.",
        b"1.2.3.",
        b"1.2.3.-1",
        b"1.2.3.0x",
        b"1.2.3.0x1g",
        b"0x.1",
        b"08",
        b"09.1",
    ] {
        assert_eq!(inet_addr(Some(input)), Ok(INADDR_NONE), "input: {input:?}");
    }
}

#[test]
fn rejects_values_that_overflow_their_byte_slot() {
    // oracle: "0777.1.1.1" has a 511 first component; "1.2.3.0a" stops at "a"
    for input in [
        &b"4294967296"[..],
        b"1.2.16777216",
        b"0777.1.1.1",
        b"1.2.3.0a",
    ] {
        assert_eq!(inet_addr(Some(input)), Ok(INADDR_NONE), "input: {input:?}");
    }
}

#[test]
fn accepts_ascii_space_as_the_legacy_terminator() {
    // oracle: the same address followed by each ASCII white space byte
    for whitespace in [b' ', b'\t', b'\n', b'\r', 0x0b, 0x0c] {
        let mut input = b"1.2.3.4".to_vec();
        input.push(whitespace);
        input.extend_from_slice(b"ignored");
        assert_eq!(inet_addr(Some(&input)), Ok(0x0403_0201));
    }
}

#[test]
fn two_and_three_part_forms_use_the_last_component_width() {
    // oracle: "1.65536" is 1.1.0.0 and "1.65535" is 1.0.255.255 (little-endian u32 values below)
    assert_eq!(inet_addr(Some(b"1.65536")), Ok(0x0000_0101));
    assert_eq!(inet_addr(Some(b"1.65535")), Ok(0xffff_0001));
    assert_eq!(inet_addr(Some(b"1.16777215")), Ok(0xffff_ff01));
    assert_eq!(inet_addr(Some(b"1.2.65535")), Ok(0xffff_0201));
    assert_eq!(inet_addr(Some(b"1.8.1.1")), Ok(0x0101_0801));
}

#[test]
fn accepts_upper_case_hex_and_trailing_spaces() {
    // oracle: "0X1F.0x2.3.4" and "1.2.3.4  "
    assert_eq!(inet_addr(Some(b"0X1F.0x2.3.4")), Ok(0x0403_021f));
    assert_eq!(inet_addr(Some(b"1.2.3.4  ")), Ok(0x0403_0201));
}

#[test]
fn single_space_is_zero_and_longer_space_runs_are_not() {
    assert_eq!(inet_addr(Some(b" ")), Ok(0));
    assert_eq!(inet_addr(Some(b"  ")), Ok(INADDR_NONE));
}
