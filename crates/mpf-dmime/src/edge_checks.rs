//! Boundary checks for byte-slice behavior around the C oracle rows.

use crate::variable_length::{read_variable_length_number, DecodeResult};

#[test]
fn empty_input_is_incomplete_without_consuming_bytes() {
    assert_eq!(
        read_variable_length_number(&[]),
        DecodeResult::Incomplete {
            value: 0,
            bytes_read: 0,
        }
    );
}

#[test]
fn every_single_byte_value_decodes_or_reports_its_continuation_state() {
    for byte in u8::MIN..=u8::MAX {
        let actual = read_variable_length_number(&[byte]);
        let expected = if byte & 0x80 == 0 {
            DecodeResult::Complete {
                value: u32::from(byte),
                bytes_read: 1,
            }
        } else {
            DecodeResult::Incomplete {
                value: u32::from(byte & 0x7f),
                bytes_read: 1,
            }
        };
        assert_eq!(actual, expected, "byte {byte:#04x}");
    }
}

#[test]
fn first_terminating_byte_leaves_following_input_unconsumed() {
    assert_eq!(
        read_variable_length_number(&[0x02, 0x80, 0x03]),
        DecodeResult::Complete {
            value: 2,
            bytes_read: 1,
        }
    );
}

#[test]
fn overlong_input_wraps_the_accumulator_as_a_u32() {
    assert_eq!(
        read_variable_length_number(&[0xff, 0xff, 0xff, 0xff, 0xff, 0x00]),
        DecodeResult::Complete {
            value: 0xffff_ff80,
            bytes_read: 6,
        }
    );
}
