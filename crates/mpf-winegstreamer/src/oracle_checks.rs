//! Checks the port against rows recorded from the verbatim C code of `wg_channel_mask_from_gst`
//! and `wg_channel_position_from_gst` (`dlls/winegstreamer/wg_format.c`), built with ASan and
//! UBSan, with no reports. Each row is the channel positions of one layout and the mask that
//! the C function returns for them.

use crate::channel_mask;

#[rustfmt::skip]
const ROWS: &[(&[i32], u32)] = &[
    (&[], 0x0000_0000),
    (&[-2], 0x0000_0004),
    (&[0], 0x0000_0001),
    (&[1], 0x0000_0002),
    (&[0, 1], 0x0000_0003),
    (&[0, 1, 2], 0x0000_0007),
    (&[0, 1, 2, 3], 0x0000_000f),
    (&[0, 1, 2, 3, 4, 5], 0x0000_003f),
    (&[0, 1, 2, 3, 4, 5, 6, 7], 0x0000_00ff),
    (&[8], 0x0000_0100),
    (&[10], 0x0000_0200),
    (&[0, 1, 2, 3, 4, 5, 8, 10, 11, 15, 12, 14, 13], 0x0000_7f3f),
    (&[0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 15, 12, 14, 13, 16, 20, 17], 0x0003_ffff),
    (&[0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 15, 12, 14, 13, 16, 17], 0x0002_ffff),
    (&[1, 0], 0x0000_0000),
    (&[0, 0], 0x0000_0000),
    (&[0, 2, 1], 0x0000_0000),
    (&[9], 0x0000_0000),
    (&[18], 0x0000_0000),
    (&[19], 0x0000_0000),
    (&[0, -2], 0x0000_0005),
    (&[-2, 0], 0x0000_0000),
    (&[-1], 0x0000_0000),
    (&[-3], 0x0000_0000),
    (&[21], 0x0000_0000),
];

#[test]
fn oracle_rows_match_wine() {
    assert_eq!(ROWS.len(), 25);
    for (positions, expected) in ROWS {
        assert_eq!(
            channel_mask(positions),
            *expected,
            "positions: {positions:?}"
        );
    }
}
