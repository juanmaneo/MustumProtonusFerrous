//! Oracle-derived rows. Each test names its row from the C oracle output (`rows.txt`), which runs
//! the verbatim Wine functions with the inputs of `driver.c`.
//!
//! Sanitizer-flagged vectors are not rows. UBSan reports signed overflow in `MAPI32_FtAddFt`
//! (`INT64_MAX + 1`), `MAPI32_FtMulDwDw` (`0xffffffff * 0xffffffff`) and `MAPI32_FtNegFt`
//! (`INT64_MIN`). The port wraps on those inputs, and the edge checks cover the wrapped results.

use super::*;

/// Row r01: `FBinFromHex("00ff10")` returns TRUE and writes `00 ff 10`.
#[test]
fn r01_bin_from_hex_decodes_pairs() {
    assert_eq!(bin_from_hex(b"00ff10"), Some(vec![0x00, 0xff, 0x10]));
}

/// Row r02: `FBinFromHex("ABcd")` returns TRUE and accepts upper- and lowercase digits.
#[test]
fn r02_bin_from_hex_accepts_mixed_case() {
    assert_eq!(bin_from_hex(b"ABcd"), Some(vec![0xab, 0xcd]));
}

/// Row r03: `FBinFromHex("0g")` and `FBinFromHex("4G")` return FALSE. The C code writes nothing.
#[test]
fn r03_bin_from_hex_rejects_non_hex_digits() {
    assert_eq!(bin_from_hex(b"0g"), None);
    assert_eq!(bin_from_hex(b"4G"), None);
}

/// Row r04: `FBinFromHex("abc")` returns FALSE for odd length. The C code writes `ab` before it
/// fails. The port returns `None` and does not reproduce that partial write.
#[test]
fn r04_bin_from_hex_rejects_odd_length() {
    assert_eq!(bin_from_hex(b"abc"), None);
}

/// Row r05: `FBinFromHex("")` returns TRUE and writes nothing.
#[test]
fn r05_bin_from_hex_empty_is_ok() {
    assert_eq!(bin_from_hex(b""), Some(Vec::new()));
}

/// Row r06: `HexFromBin({00, ff, 1a})` writes `00FF1A`.
#[test]
fn r06_hex_from_bin_writes_uppercase_pairs() {
    assert_eq!(hex_from_bin(&[0x00, 0xff, 0x1a]), "00FF1A");
}

/// Row r07: `UlFromSzHex("1f")` returns 31.
#[test]
fn r07_ul_from_sz_hex_single_pair() {
    assert_eq!(ul_from_sz_hex(b"1f"), 31);
}

/// Row r08: `UlFromSzHex("1f2e")` returns 542, and `UlFromSzHex("0102zz")` stops at `zz` and
/// returns 18.
#[test]
fn r08_ul_from_sz_hex_accumulates_pairs() {
    assert_eq!(ul_from_sz_hex(b"1f2e"), 542);
    assert_eq!(ul_from_sz_hex(b"0102zz"), 18);
}

/// Row r09: `UlFromSzHex("g1")` returns 0, because the first pair is not valid.
#[test]
fn r09_ul_from_sz_hex_invalid_first_pair() {
    assert_eq!(ul_from_sz_hex(b"g1"), 0);
}

/// Row r10: `SwapPlong({0x11223344})` gives `0x44332211`.
#[test]
fn r10_swap_plong_reverses_bytes() {
    let mut data = [0x1122_3344];
    swap_plong(&mut data);
    assert_eq!(data, [0x4433_2211]);
}

/// Row r11: `SwapPword({0x1122, 0xaabb})` gives `{0x2211, 0xbbaa}`.
#[test]
fn r11_swap_pword_reverses_bytes() {
    let mut data = [0x1122, 0xaabb];
    swap_pword(&mut data);
    assert_eq!(data, [0x2211, 0xbbaa]);
}

/// Row r12: `UFromSz("123abc")` returns 123.
#[test]
fn r12_uf_from_sz_leading_digits() {
    assert_eq!(uf_from_sz(b"123abc"), 123);
}

/// Row r13: `UFromSz("abc")` returns 0.
#[test]
fn r13_uf_from_sz_no_digits() {
    assert_eq!(uf_from_sz(b"abc"), 0);
}

/// Row r14: `UFromSz("4294967296")` returns 0, because the value wraps at 32 bits.
#[test]
fn r14_uf_from_sz_wraps_at_32_bits() {
    assert_eq!(uf_from_sz(b"4294967296"), 0);
}

/// Row r15: `CbOfEncoded("")` returns 3, and `CbOfEncoded("abcdefgh")` returns 9.
#[test]
fn r15_cb_of_encoded_empty_and_eight() {
    assert_eq!(cb_of_encoded(Some(b"")), 3);
    assert_eq!(cb_of_encoded(Some(b"abcdefgh")), 9);
}

/// Row r16: `CbOfEncoded("abcd")` returns 6.
#[test]
fn r16_cb_of_encoded_four() {
    assert_eq!(cb_of_encoded(Some(b"abcd")), 6);
}

/// Row r17: `FtAddFt(1, 2)` is 3, `FtSubFt(5, 12)` is 7 (`right - left`), `FtMulDw(3, 100)` is
/// 300, and `FtNegFt(5)` is -5.
#[test]
fn r17_ft_add_sub_mul_neg() {
    assert_eq!(ft_add_ft(ft(1), ft(2)), 3);
    assert_eq!(ft_sub_ft(ft(5), ft(12)), 7);
    assert_eq!(ft_mul_dw(3, ft(100)), 300);
    assert_eq!(ft_neg_ft(ft(5)), -5);
}

/// Row r18: `FtMulDwDw(4000000000, 2)` is 8000000000, and `FtAddFt(-5, 3)` is -2.
#[test]
fn r18_ft_mul_dw_dw_and_signed_add() {
    assert_eq!(ft_mul_dw_dw(4_000_000_000, 2), 8_000_000_000);
    assert_eq!(ft_add_ft(ft(-5), ft(3)), -2);
}

/// Row r19: `MNLS_lstrlenW("AB")` returns 2, and `MNLS_lstrcpyW` copies it with its NUL, which is
/// 6 bytes.
#[test]
fn r19_lstrlen_and_lstrcpy_w() {
    let src = [0x41, 0x42, 0];
    let mut dest = [0u16; 8];
    assert_eq!(lstrlen_w(&src), 2);
    assert_eq!(lstrcpy_w(&mut dest, &src), Some(6));
    assert_eq!(dest[..3], src);
}

/// Row r20: `PpropFindProp` returns index 1 for `0x00020003`, index 2 for `0x00030000` (matched by
/// id, since its type is unspecified), and index 1 for `0x00050000` (skipping the invalid tag
/// `0x00050009`).
#[test]
fn r20_pprop_find_prop_index() {
    let tags = [0x0001_001E, 0x0002_0003, 0x0003_0003];
    assert_eq!(find_prop_index(&tags, 0x0002_0003), Some(1));
    assert_eq!(find_prop_index(&tags, 0x0003_0000), Some(2));
    assert_eq!(
        find_prop_index(&[0x0005_0009, 0x0005_0003], 0x0005_0000),
        Some(1)
    );
}

/// Row r21: `LpValFindProp(0x00050000)` over `{0x00050009, 0x00050003}` returns index 0, because
/// it matches by id and does not check the type.
#[test]
fn r21_lval_find_prop_index_ignores_type() {
    assert_eq!(
        lval_find_prop_index(0x0005_0000, &[0x0005_0009, 0x0005_0003]),
        Some(0)
    );
}

/// Row r22: `FBadPropTag` is FALSE for `0x0003`, `0x1003` and `0x100A`, and TRUE for `0x0009` and
/// `0x2102`.
#[test]
fn r22_fbad_prop_tag_known_and_unknown_types() {
    let bad = [0x0003, 0x1003, 0x0009, 0x2102, 0x100A].map(fbad_prop_tag);
    assert_eq!(bad, [false, false, true, true, false]);
}

/// Row r23: `FBadColumnSet` is FALSE for `{0x0001001E, 0x00020003}`, TRUE for `{0x0001000A}` (type
/// `PT_ERROR`) and `{0x00020009}` (unknown type), and FALSE for an empty set.
#[test]
fn r23_fbad_column_set_rejects_error_and_unknown() {
    assert!(!fbad_column_set(&[0x0001_001E, 0x0002_0003]));
    assert!(fbad_column_set(&[0x0001_000A]));
    assert!(fbad_column_set(&[0x0002_0009]));
    assert!(!fbad_column_set(&[]));
}

/// The `FILETIME` with the bit pattern of `value`, as the oracle driver builds it.
fn ft(value: i64) -> FileTime {
    FileTime::from_i64(value)
}
