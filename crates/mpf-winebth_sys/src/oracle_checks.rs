//! Rows captured from Wine's `dbus.c` `parse_uuid`, compiled verbatim with
//! `gcc -fsanitize=address,undefined -g` (`target/canon/winebth.sys/oracle.c`). No row hit a
//! sanitizer report, so none is excluded.

use super::{parse_uuid, Uuid};

const ZERO: [u8; 8] = [0; 8];

fn uuid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Option<Uuid> {
    Some(Uuid {
        data1,
        data2,
        data3,
        data4,
    })
}

#[rustfmt::skip]
fn rows() -> Vec<(&'static [u8], Option<Uuid>)> {
    vec![
        (b"00000000-0000-0000-0000-000000000000", uuid(0, 0, 0, ZERO)),
        (b"00112233-4455-6677-8899-aabbccddeeff", uuid(0x0011_2233, 0x4455, 0x6677, [0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff])),
        (b"ffffffff-ffff-ffff-ffff-ffffffffffff", uuid(u32::MAX, u16::MAX, u16::MAX, [0xff; 8])),
        (b"AABBCCDD-EEFF-1122-3344-55667788990A", uuid(0xaabb_ccdd, 0xeeff, 0x1122, [0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x0a])),
        (b"12345678-9abc-def0-1234-56789abcdef0", uuid(0x1234_5678, 0x9abc, 0xdef0, [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0])),
        (b"00000001-0002-0003-0408-090a0b0c0d0e", uuid(1, 2, 3, [4, 8, 9, 10, 11, 12, 13, 14])),
        (b"deadbeef-cafe-babe-0123-456789abcdef", uuid(0xdead_beef, 0xcafe, 0xbabe, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
        (b"01234567-89ab-cdef-0123-456789abcdef", uuid(0x0123_4567, 0x89ab, 0xcdef, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
        (b"00000000-0000-0000-0000-00000000000", None),
        (b"00000000-0000-0000-0000-0000000000000", None),
        (b"00000000_0000-0000-0000-000000000000", None),
        (b"g0000000-0000-0000-0000-000000000000", None),
        (b"00000000-0000-0000-0000-00000000000g", uuid(0, 0, 0, ZERO)),
        (b"00000000-0000-0000-0000-000000000000 ", None),
        (b"+0000000-0000-0000-0000-000000000000", uuid(0, 0, 0, ZERO)),
        (b"00000000- 001-0000-0000-000000000000", uuid(0, 1, 0, ZERO)),
        (b"00000000-0000-+001-0000-000000000000", uuid(0, 0, 1, ZERO)),
        (b"00000000-0000-0000--101-020304050607", uuid(0, 0, 0, [0xff, 1, 2, 3, 4, 5, 6, 7])),
        (b"0x000000-0000-0000-0000-000000000000", uuid(0, 0, 0, ZERO)),
        (b"00000000-0000-0000-0000-000000000000x", None),
        (b"00000000-1234a-000-0000-000000000000", uuid(0, 0x234a, 0, ZERO)),
        (b"00000000--001-0000-0000-000000000000", uuid(0, 0xffff, 0, ZERO)),
        (b"00000000-0000-0000- 123-456789abcdef", uuid(0, 0, 0, [0x12, 0x03, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef])),
        (b"00000000-0000-0000-+100-000000000000", uuid(0, 0, 0, [0x01, 0, 0, 0, 0, 0, 0, 0])),
    ]
}

#[test]
fn parser_matches_wine_oracle_rows() {
    for (text, expected) in rows() {
        assert_eq!(
            parse_uuid(text),
            expected,
            "{}",
            String::from_utf8_lossy(text)
        );
    }
}
