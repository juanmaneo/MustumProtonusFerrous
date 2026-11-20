//! Edge cases that the oracle rows do not reach: the buffer-too-short boundary, the
//! NUL terminator of the replacement, and the prefix check with a shorter prefix.

use crate::normalize_manufacturer;

/// The replacement string that Wine writes when the buffer starts with "AVM".
const REPLACEMENT: &[u8] = b"AVM-GmbH\0";

#[test]
fn buffer_too_short_for_replacement_is_not_replaced() {
    // 3..8 bytes starting with "AVM" are too short for the 9-byte replacement.
    for len in 3..9 {
        let mut buffer = *b"AVMXXXXXX";
        assert!(!normalize_manufacturer(&mut buffer[..len]), "len={len}");
        assert_eq!(&buffer[..len], &b"AVMXXXXXX"[..len], "len={len}");
    }
}

#[test]
fn buffer_exactly_replacement_size_is_replaced() {
    let mut buffer = *b"AVMXXXXXX";
    assert!(normalize_manufacturer(&mut buffer));
    assert_eq!(&buffer[..], REPLACEMENT);
}

#[test]
fn replacement_writes_nul_terminator() {
    let mut buffer = *b"AVMXXXXXX";
    normalize_manufacturer(&mut buffer);
    assert_eq!(buffer[8], 0, "replacement must NUL-terminate");
}

#[test]
fn prefix_check_requires_exact_avm() {
    // "AV" is not "AVM".
    let mut buffer = b"AV".to_vec();
    assert!(!normalize_manufacturer(&mut buffer));
    assert_eq!(&buffer, b"AV");

    // "AVMX" is too short for the replacement and not replaced even though "AVM" matches.
    let mut buffer = b"AVMX".to_vec();
    assert!(!normalize_manufacturer(&mut buffer));
    assert_eq!(&buffer, b"AVMX");

    // "AVM" with a different fourth byte is still replaced.
    let mut buffer = b"AVM!XXXXXX".to_vec();
    assert!(normalize_manufacturer(&mut buffer));
    assert_eq!(&buffer[..9], REPLACEMENT);
}

#[test]
fn empty_buffer_is_not_replaced() {
    let mut buffer: [u8; 0] = [];
    assert!(!normalize_manufacturer(&mut buffer));
}

#[test]
fn avm_prefix_is_replaced() {
    let mut buffer = b"AVM-GmbH\0".to_vec();
    assert!(normalize_manufacturer(&mut buffer));
    assert_eq!(&buffer, b"AVM-GmbH\0");
}

#[test]
fn lowercase_avm_is_not_replaced() {
    let mut buffer = b"avmXXXXXX".to_vec();
    assert!(!normalize_manufacturer(&mut buffer));
    assert_eq!(&buffer, b"avmXXXXXX");
}

#[test]
fn buffer_with_nul_after_avm_is_replaced() {
    // "AVM\0garbage\0" — the C code reads past the NUL, but strncmp stops at the NUL.
    let mut buffer = b"AVM\0garbage\0".to_vec();
    assert!(normalize_manufacturer(&mut buffer));
    assert_eq!(&buffer[..9], REPLACEMENT);
    assert_eq!(&buffer[9..], b"ge\0");
}
