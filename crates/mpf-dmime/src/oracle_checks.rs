//! Twenty-four output rows from the ASan/UBSan-checked C oracle in `target/canon/dmime/oracle.c`.

use crate::variable_length::{read_variable_length_number, DecodeResult};

#[test]
fn oracle_r01_to_r24() {
    let rows: [(&str, &[u8], DecodeResult); 24] = [
        (
            "R01",
            &[0x00],
            DecodeResult::Complete {
                value: 0,
                bytes_read: 1,
            },
        ),
        (
            "R02",
            &[0x01],
            DecodeResult::Complete {
                value: 1,
                bytes_read: 1,
            },
        ),
        (
            "R03",
            &[0x7f],
            DecodeResult::Complete {
                value: 127,
                bytes_read: 1,
            },
        ),
        (
            "R04",
            &[0x80, 0x00],
            DecodeResult::Complete {
                value: 0,
                bytes_read: 2,
            },
        ),
        (
            "R05",
            &[0x80, 0x01],
            DecodeResult::Complete {
                value: 1,
                bytes_read: 2,
            },
        ),
        (
            "R06",
            &[0x81, 0x00],
            DecodeResult::Complete {
                value: 128,
                bytes_read: 2,
            },
        ),
        (
            "R07",
            &[0x81, 0x01],
            DecodeResult::Complete {
                value: 129,
                bytes_read: 2,
            },
        ),
        (
            "R08",
            &[0xff, 0x7f],
            DecodeResult::Complete {
                value: 16_383,
                bytes_read: 2,
            },
        ),
        (
            "R09",
            &[0x81, 0x80, 0x00],
            DecodeResult::Complete {
                value: 16_384,
                bytes_read: 3,
            },
        ),
        (
            "R10",
            &[0xff, 0xff, 0x7f],
            DecodeResult::Complete {
                value: 2_097_151,
                bytes_read: 3,
            },
        ),
        (
            "R11",
            &[0x81, 0x80, 0x80, 0x00],
            DecodeResult::Complete {
                value: 2_097_152,
                bytes_read: 4,
            },
        ),
        (
            "R12",
            &[0xff, 0xff, 0xff, 0x7f],
            DecodeResult::Complete {
                value: 268_435_455,
                bytes_read: 4,
            },
        ),
        (
            "R13",
            &[0x80, 0x80, 0x80, 0x80, 0x00],
            DecodeResult::Complete {
                value: 0,
                bytes_read: 5,
            },
        ),
        (
            "R14",
            &[0x81, 0x80, 0x80, 0x80, 0x00],
            DecodeResult::Complete {
                value: 268_435_456,
                bytes_read: 5,
            },
        ),
        (
            "R15",
            &[0xff, 0xff, 0xff, 0xff, 0x7f],
            DecodeResult::Complete {
                value: u32::MAX,
                bytes_read: 5,
            },
        ),
        (
            "R16",
            &[0x81, 0x80, 0x80, 0x80, 0x80, 0x00],
            DecodeResult::Complete {
                value: 0,
                bytes_read: 6,
            },
        ),
        (
            "R17",
            &[0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
            DecodeResult::Complete {
                value: u32::MAX,
                bytes_read: 6,
            },
        ),
        (
            "R18",
            &[],
            DecodeResult::Incomplete {
                value: 0,
                bytes_read: 0,
            },
        ),
        (
            "R19",
            &[0x80],
            DecodeResult::Incomplete {
                value: 0,
                bytes_read: 1,
            },
        ),
        (
            "R20",
            &[0x81, 0x80],
            DecodeResult::Incomplete {
                value: 128,
                bytes_read: 2,
            },
        ),
        (
            "R21",
            &[0xff, 0xff, 0xff, 0xff],
            DecodeResult::Incomplete {
                value: 268_435_455,
                bytes_read: 4,
            },
        ),
        (
            "R22",
            &[0x81, 0x80, 0x80, 0x80, 0x80],
            DecodeResult::Incomplete {
                value: 268_435_456,
                bytes_read: 5,
            },
        ),
        (
            "R23",
            &[0x7f, 0x80],
            DecodeResult::Complete {
                value: 127,
                bytes_read: 1,
            },
        ),
        (
            "R24",
            &[0x80, 0x01, 0x7f],
            DecodeResult::Complete {
                value: 1,
                bytes_read: 2,
            },
        ),
    ];

    for (name, bytes, expected) in rows {
        assert_eq!(read_variable_length_number(bytes), expected, "{name}");
    }
}
