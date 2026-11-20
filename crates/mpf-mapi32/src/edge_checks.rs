//! Edge tests for the branches that the oracle rows do not reach: NUL truncation, bad first
//! digits, wrap-around, and the exhaustive digit and property-tag sets.

use super::*;

#[test]
fn bin_from_hex_invalid_high_digit_is_none() {
    assert_eq!(bin_from_hex(b"g0"), None);
}

#[test]
fn bin_from_hex_stops_at_nul() {
    assert_eq!(bin_from_hex(b"00\x0011"), Some(vec![0]));
}

#[test]
fn hex_from_bin_empty_is_empty() {
    assert_eq!(hex_from_bin(&[]), "");
}

#[test]
fn ul_from_sz_hex_stops_at_nul() {
    assert_eq!(ul_from_sz_hex(b"1\x002"), 0);
}

#[test]
fn ul_from_sz_hex_ignores_trailing_single_char() {
    assert_eq!(ul_from_sz_hex(b"1f2"), 31);
}

#[test]
fn ul_from_sz_hex_wraps_at_32_bits() {
    assert_eq!(ul_from_sz_hex(b"ffffffffffffff"), 268_435_439);
}

#[test]
fn uf_from_sz_leading_space_is_zero() {
    assert_eq!(uf_from_sz(b" 12"), 0);
}

#[test]
fn uf_from_sz_stops_at_nul() {
    assert_eq!(uf_from_sz(b"12\x0034"), 12);
}

#[test]
fn cb_of_encoded_null_is_zero() {
    assert_eq!(cb_of_encoded(None), 0);
}

#[test]
fn cb_of_encoded_five_bytes_is_six() {
    assert_eq!(cb_of_encoded(Some(b"abcde")), 6);
}

#[test]
fn cb_of_encoded_stops_at_nul() {
    assert_eq!(cb_of_encoded(Some(b"abcde\x00xx")), 6);
}

#[test]
fn lstrlen_w_empty_is_zero() {
    assert_eq!(lstrlen_w(&[]), 0);
}

#[test]
fn lstrlen_w_without_nul_counts_whole_slice() {
    assert_eq!(lstrlen_w(&[0x41]), 1);
}

#[test]
fn lstrlen_w_stops_at_nul() {
    assert_eq!(lstrlen_w(&[0x41, 0, 0x42]), 1);
}

#[test]
fn lstrcpy_w_dest_too_short_is_none() {
    assert_eq!(lstrcpy_w(&mut [0u16; 2], &[0x41, 0x42, 0]), None);
}

#[test]
fn lstrcpy_w_src_without_nul_is_none() {
    assert_eq!(lstrcpy_w(&mut [0u16; 4], &[0x41, 0x42]), None);
}

#[test]
fn lstrcpy_w_exact_fit_copies_nul() {
    let mut dest = [0u16; 3];
    assert_eq!(lstrcpy_w(&mut dest, &[0x41, 0x42, 0]), Some(6));
    assert_eq!(dest, [0x41, 0x42, 0]);
}

#[test]
fn lstrcpy_w_copies_only_to_src_nul() {
    let mut dest = [0xffff; 4];
    assert_eq!(lstrcpy_w(&mut dest, &[0x41, 0, 0x42]), Some(4));
    assert_eq!(dest, [0x41, 0, 0xffff, 0xffff]);
}

#[test]
fn swap_plong_is_an_involution() {
    let original = [0x0102_0304, 0xdead_beef];
    let mut data = original;
    swap_plong(&mut data);
    assert_eq!(data, [0x0403_0201, 0xefbe_adde]);
    swap_plong(&mut data);
    assert_eq!(data, original);
}

#[test]
fn swap_pword_is_an_involution() {
    let original = [0x0102, 0xbeef];
    let mut data = original;
    swap_pword(&mut data);
    swap_pword(&mut data);
    assert_eq!(data, original);
}

#[test]
fn file_time_round_trips_negative_one() {
    assert_eq!(FileTime::from_i64(-1).to_i64(), -1);
}

#[test]
fn file_time_round_trips_extremes() {
    for value in [i64::MIN, i64::MAX, 0, 1 << 32] {
        assert_eq!(FileTime::from_i64(value).to_i64(), value);
    }
}

#[test]
fn file_time_high_half_is_bit_32() {
    assert_eq!(FileTime { low: 0, high: 1 }.to_i64(), 1 << 32);
    assert_eq!(FileTime::from_i64(1 << 32), FileTime { low: 0, high: 1 });
}

#[test]
fn ft_add_ft_wraps_at_max() {
    assert_eq!(
        ft_add_ft(FileTime::from_i64(i64::MAX), FileTime::from_i64(1)),
        i64::MIN
    );
}

#[test]
fn ft_neg_ft_wraps_at_min() {
    assert_eq!(ft_neg_ft(FileTime::from_i64(i64::MIN)), i64::MIN);
}

#[test]
fn ft_mul_dw_dw_max_squared_wraps() {
    assert_eq!(ft_mul_dw_dw(u32::MAX, u32::MAX), -8_589_934_591);
}

#[test]
fn bin_from_hex_accepts_exactly_hex_digits() {
    let digits: Vec<u8> = b"0123456789ABCDEFabcdef".to_vec();
    let high: Vec<u8> = (1..=255u8)
        .filter(|&c| bin_from_hex(&[c, b'0']).is_some())
        .collect();
    let low: Vec<u8> = (1..=255u8)
        .filter(|&c| bin_from_hex(&[b'0', c]).is_some())
        .collect();
    assert_eq!(high, digits);
    assert_eq!(low, digits);
}

#[test]
fn fbad_prop_tag_accepts_only_known_types_with_or_without_mv_flag() {
    const BASE: [u32; 17] = [
        PT_UNSPECIFIED,
        PT_NULL,
        PT_I2,
        PT_LONG,
        PT_R4,
        PT_DOUBLE,
        PT_CURRENCY,
        PT_APPTIME,
        PT_ERROR,
        PT_BOOLEAN,
        PT_OBJECT,
        PT_I8,
        PT_STRING8,
        PT_UNICODE,
        PT_SYSTIME,
        PT_CLSID,
        PT_BINARY,
    ];
    let mut expected: Vec<u32> = BASE.iter().flat_map(|&t| [t, t | MV_FLAG]).collect();
    expected.sort_unstable();
    let accepted: Vec<u32> = (0..=0xFFFF).filter(|&tag| !fbad_prop_tag(tag)).collect();
    assert_eq!(accepted.len(), 34);
    assert_eq!(accepted, expected);
}

#[test]
fn fbad_prop_tag_high_bits_13_to_15_are_bad() {
    assert!(fbad_prop_tag(0x2003));
    assert!(fbad_prop_tag(0x8003));
    assert!(fbad_prop_tag(0xE003));
}

#[test]
fn fbad_column_set_only_rejects_exact_error_type() {
    assert!(!fbad_column_set(&[0x0001_100A]));
}

#[test]
fn find_prop_index_skips_bad_tag() {
    assert_eq!(find_prop_index(&[0x0005_0009], 0x0005_0000), None);
}

#[test]
fn find_prop_index_typed_mismatch_is_none() {
    assert_eq!(find_prop_index(&[0x0002_0002], 0x0002_0003), None);
}

#[test]
fn find_prop_index_empty_is_none() {
    assert_eq!(find_prop_index(&[], 0x0002_0003), None);
}

#[test]
fn prop_type_and_id_split_the_tag() {
    assert_eq!(prop_type(0x0002_0003), PT_LONG);
    assert_eq!(prop_id(0x0002_0003), 2);
}
