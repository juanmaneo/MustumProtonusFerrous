//! Replays of Wine's oracle rows. The oracle compiles the verbatim `gdi_handle_type` from
//! `dlls/wow64win/gdi.c` with `-fsanitize=address,undefined` and prints one line per input. Each
//! test rebuilds the row from the port and compares it with the printed text.

use super::*;

/// The printed oracle rows: the input handle, then the type bits Wine returns.
const ROWS: [(u64, u32); 20] = [
    (0x0000_0000_0000_0000, 0x0000_0000),
    (0x0000_0000_0000_0001, 0x0000_0000),
    (0x0000_0000_0000_ffff, 0x0000_0000),
    (0x0000_0000_0001_0000, 0x0001_0000),
    (0x0000_0000_0007_ffff, 0x0007_0000),
    (0x0000_0000_0008_0000, 0x0008_0000),
    (0x0000_0000_0009_0000, 0x0009_0000),
    (0x0000_0000_000a_0000, 0x000a_0000),
    (0x0000_0000_0010_0000, 0x0010_0000),
    (0x0000_0000_0030_0000, 0x0030_0000),
    (0x0000_0000_0041_0000, 0x0041_0000),
    (0x0000_0000_0046_0000, 0x0046_0000),
    (0x0000_0000_0050_0000, 0x0050_0000),
    (0x0000_0000_0066_0000, 0x0066_0000),
    (0x0000_0000_007f_0000, 0x007f_0000),
    (0x0000_0000_0080_0000, 0x0000_0000),
    (0x0000_0000_ffff_ffff, 0x007f_0000),
    (0x1234_5678_0009_abcd, 0x0009_0000),
    (0xffff_ffff_0080_0000, 0x0000_0000),
    (0xabcd_efab_007f_1234, 0x007f_0000),
];

#[test]
fn oracle_rows_match_wine_gdi_handle_type() {
    for (obj, expected) in ROWS {
        let line = format!("{obj:016x} {:08x}", gdi_handle_type(obj));
        assert_eq!(line, format!("{obj:016x} {expected:08x}"));
    }
}

#[test]
fn oracle_mask_matches_ntgdi_header() {
    assert_eq!(NTGDI_HANDLE_TYPE_MASK, 0x007f_0000);
    assert_eq!(
        gdi_handle_type(u64::from(NTGDI_HANDLE_TYPE_MASK)),
        NTGDI_HANDLE_TYPE_MASK
    );
}
