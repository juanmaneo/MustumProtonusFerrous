//! Boundary and cursor-consumption checks for the PA19 VLI readers.

use crate::{read_svli, read_uvli, VliError};

#[test]
fn the_first_terminator_controls_consumption() {
    assert_eq!(read_uvli(&[0x82, 0x01, 0x80]), Ok((2, 1)));
    assert_eq!(read_svli(&[0xc2, 0x01, 0x80]), Ok((-2, 1)));
}

#[test]
fn a_terminator_after_the_nine_byte_limit_is_not_read() {
    assert_eq!(
        read_uvli(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80]),
        Err(VliError::Unterminated)
    );
    assert_eq!(
        read_svli(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80]),
        Err(VliError::Unterminated)
    );
}
