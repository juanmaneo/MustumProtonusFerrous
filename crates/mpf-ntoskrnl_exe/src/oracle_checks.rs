//! Oracle-derived rows from Wine's value-level `Rtl*ByteSwap` behavior.

use super::{rtl_ulong_byte_swap, rtl_ulonglong_byte_swap, rtl_ushort_byte_swap};

const ROWS: &[(u64, u16, u32, u64)] = &[
    (0x0000000000000000, 0x0000, 0x00000000, 0x0000000000000000),
    (0x0000000000000001, 0x0100, 0x01000000, 0x0100000000000000),
    (0x0000000000000002, 0x0200, 0x02000000, 0x0200000000000000),
    (0x000000000000007f, 0x7f00, 0x7f000000, 0x7f00000000000000),
    (0x0000000000000080, 0x8000, 0x80000000, 0x8000000000000000),
    (0x00000000000000ff, 0xff00, 0xff000000, 0xff00000000000000),
    (0x0000000000000100, 0x0001, 0x00010000, 0x0001000000000000),
    (0x0000000000001234, 0x3412, 0x34120000, 0x3412000000000000),
    (0x0000000000008000, 0x0080, 0x00800000, 0x0080000000000000),
    (0x000000000000ffff, 0xffff, 0xffff0000, 0xffff000000000000),
    (0x0000000001020304, 0x0403, 0x04030201, 0x0403020100000000),
    (0x0000000089abcdef, 0xefcd, 0xefcdab89, 0xefcdab8900000000),
    (0x00000000ffffffff, 0xffff, 0xffffffff, 0xffffffff00000000),
    (0x0102030405060708, 0x0807, 0x08070605, 0x0807060504030201),
    (0x8000000000000000, 0x0000, 0x00000000, 0x0000000000000080),
    (0xffffffffffffffff, 0xffff, 0xffffffff, 0xffffffffffffffff),
    (0x1122334455667788, 0x8877, 0x88776655, 0x8877665544332211),
    (0xa5a55a5ac3c33c3c, 0x3c3c, 0x3c3cc3c3, 0x3c3cc3c35a5aa5a5),
];

#[test]
fn matches_sanitized_wine_oracle_rows() {
    assert_eq!(ROWS.len(), 18);
    for &(value, expected16, expected32, expected64) in ROWS {
        assert_eq!(rtl_ushort_byte_swap(value as u16), expected16);
        assert_eq!(rtl_ulong_byte_swap(value as u32), expected32);
        assert_eq!(rtl_ulonglong_byte_swap(value), expected64);
    }
}
