use super::*;

/// Ten copies of `first`, then `last`: an 11-byte LEB128 encoding that C reads with shifts of 70.
fn overlong(first: u8, last: u8) -> Vec<u8> {
    let mut bytes = vec![first; 10];
    bytes.push(last);
    bytes
}

#[test]
fn empty_input_returns_none() {
    assert_eq!(read_uleb128(&[]), None);
    assert_eq!(read_sleb128(&[]), None);
    assert_eq!(leb128_len(&[]), None);
    assert_eq!(read_addr(&[], 4), None);
    assert_eq!(read_addr(&[], 8), None);
    assert_eq!(read_initial_length(&[]), None);
}

#[test]
fn unterminated_leb128_returns_none() {
    assert_eq!(read_uleb128(&[0x80]), None);
    assert_eq!(read_uleb128(&[0xff, 0xff]), None);
    assert_eq!(read_sleb128(&[0xc0, 0xbb]), None);
    assert_eq!(leb128_len(&[0x80, 0x80]), None);
}

#[test]
fn leb128_stops_at_the_terminator_and_ignores_trailing_bytes() {
    assert_eq!(read_uleb128(&[0x05, 0x80, 0x01]), Some((5, 1)));
    assert_eq!(read_sleb128(&[0x7e, 0xff]), Some((-2, 1)));
    assert_eq!(leb128_len(&[0x80, 0x80, 0x00, 0x01]), Some(3));
}

#[test]
fn non_minimal_leb128_encodings_decode() {
    assert_eq!(read_uleb128(&[0x80, 0x00]), Some((0, 2)));
    assert_eq!(read_sleb128(&[0xff, 0x7f]), Some((-1, 2)));
}

#[test]
fn sleb128_sign_extends_only_inside_64_bits() {
    assert_eq!(read_sleb128(&[0x40]), Some((-64, 1)));
    assert_eq!(read_sleb128(&[0x3f]), Some((63, 1)));
    // The sign bit of the last byte lands at bit 62, so the shift is 63 and the sign extends.
    assert_eq!(
        read_sleb128(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7f]),
        Some((-(1 << 56), 9))
    );
    assert_eq!(
        read_sleb128(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x40]),
        Some((-(1 << 55), 8))
    );
}

#[test]
fn leb128_drops_bits_at_position_64_and_above() {
    // C shifts by 70 for the 11th byte, which is undefined behaviour. The port drops those bits.
    assert_eq!(read_uleb128(&overlong(0xff, 0x7f)), Some((u64::MAX, 11)));
    assert_eq!(read_uleb128(&overlong(0x80, 0x01)), Some((0, 11)));
    assert_eq!(read_sleb128(&overlong(0xff, 0x7f)), Some((-1, 11)));
    // Shift 70 is past 64 bits, so the sign bit of the last byte does not extend.
    assert_eq!(read_sleb128(&overlong(0x80, 0x7f)), Some((0, 11)));
}

#[test]
fn read_addr_rejects_unsupported_word_sizes() {
    for word_size in [0u8, 1, 2, 3, 5, 6, 7, 16] {
        assert_eq!(
            read_addr(&[0; 16], word_size),
            None,
            "word size {word_size}"
        );
    }
}

#[test]
fn read_addr_needs_the_whole_word() {
    assert_eq!(read_addr(&[1, 2, 3], 4), None);
    assert_eq!(read_addr(&[1, 2, 3, 4, 5, 6, 7], 8), None);
}

#[test]
fn read_addr_ignores_bytes_after_the_word() {
    assert_eq!(
        read_addr(&[0x78, 0x56, 0x34, 0x12, 0xff], 4),
        Some(0x1234_5678)
    );
}

#[test]
fn read_initial_length_needs_the_full_encoding() {
    assert_eq!(read_initial_length(&[0xff, 0xff, 0xff]), None);
    assert_eq!(read_initial_length(&[0xff, 0xff, 0xff, 0xff]), None);
    assert_eq!(
        read_initial_length(&[0xff, 0xff, 0xff, 0xff, 1, 0, 0, 0, 0, 0, 0]),
        None
    );
}

#[test]
fn read_initial_length_ignores_bytes_after_the_length() {
    assert_eq!(
        read_initial_length(&[0x10, 0, 0, 0, 0xaa]),
        Some(InitialLength {
            value: 0x10,
            offset_size: 4,
            consumed: 4,
        })
    );
    assert_eq!(
        read_initial_length(&[0xff, 0xff, 0xff, 0xff, 0x2a, 0, 0, 0, 0, 0, 0, 0, 0x99]),
        Some(InitialLength {
            value: 0x2a,
            offset_size: 8,
            consumed: 12,
        })
    );
}

#[test]
fn initial_length_is_copyable_and_debuggable() {
    let length = InitialLength {
        value: 42,
        offset_size: 8,
        consumed: 12,
    };
    assert_eq!(Clone::clone(&length), length);
    assert_eq!(
        format!("{length:?}"),
        "InitialLength { value: 42, offset_size: 8, consumed: 12 }"
    );
    assert_ne!(
        length,
        InitialLength {
            value: 42,
            offset_size: 4,
            consumed: 4,
        }
    );
}
