//! Edge vectors. Most come from the same Wine build as `oracle_checks` (G04-G05,
//! M02, M04, M06, L04, L07-L10, N02, N08, N10-N27, A02-A04, A07-A09, U02,
//! P03-P16, Q03-Q06, S02-S06). The others follow from the C source: buffer
//! boundaries, family mismatches and whole-byte prefixes.

use crate::addr::{
    is_linklocal, is_loopback, is_unspecified, length_to_ipv4_mask, map_ipv6_to_ipv4,
    matches_prefix, IpAddress, Prefix,
};
use crate::guid::{guid_to_string, Guid};
use crate::luid::{luid_to_name, name_to_luid, NetLuid};
use crate::Error;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn v4(a: [u8; 4]) -> IpAddress {
    IpAddress::V4(a)
}

fn v6(a: [u8; 16]) -> IpAddress {
    IpAddress::V6(a)
}

fn ipv6(bytes: &[u8]) -> [u8; 16] {
    let mut out = [0u8; 16];
    out[..bytes.len()].copy_from_slice(bytes);
    out
}

fn prefix4(a: [u8; 4], length: u8) -> Prefix {
    Prefix {
        address: v4(a),
        length,
    }
}

fn prefix6(a: [u8; 16], length: u8) -> Prefix {
    Prefix {
        address: v6(a),
        length,
    }
}

#[test]
fn error_codes_match_win32() {
    assert_eq!(Error::NotEnoughMemory.code(), 8);
    assert_eq!(Error::InvalidParameter.code(), 87);
    assert_eq!(Error::InsufficientBuffer.code(), 122);
    assert_eq!(Error::InvalidName.code(), 123);
}

/// G04: the fields are formatted in full, with leading zeros.
#[test]
fn guid_leading_zeros() {
    let g = Guid {
        data1: 0x0000_abcd,
        data2: 0x00ff,
        data3: 0x0001,
        data4: [0, 1, 2, 3, 4, 5, 6, 7],
    };
    assert_eq!(
        guid_to_string(&g, 39),
        Ok("{0000ABCD-00FF-0001-0001-020304050607}".to_string())
    );
}

/// G05: the G01 GUID in a 38-unit buffer. The length limit is exactly 39, so one less fails.
#[test]
fn guid_buffer_boundary() {
    let g = Guid {
        data1: 0x1234_5678,
        data2: 0x1234,
        data3: 0x5678,
        data4: [0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78],
    };
    assert_eq!(guid_to_string(&g, 38), Err(Error::InsufficientBuffer));
    assert!(guid_to_string(&g, 39).is_ok());
    assert!(guid_to_string(&g, 0).is_err());
}

/// M02: a /1 mask. Its network-order bytes are 0x80 then zeros.
#[test]
fn mask_slash_one() {
    let mask = length_to_ipv4_mask(1).unwrap();
    assert_eq!(mask.to_ne_bytes(), [0x80, 0, 0, 0]);
}

/// M04: a /32 mask is all ones.
#[test]
fn mask_slash_32() {
    assert_eq!(length_to_ipv4_mask(32), Ok(u32::MAX));
}

/// M06: the largest u32 length is rejected.
#[test]
fn mask_length_u32_max_rejected() {
    assert_eq!(length_to_ipv4_mask(u32::MAX), Err(Error::InvalidParameter));
}

/// A /16 mask has network-order bytes 255.255.0.0.
#[test]
fn mask_slash_16_network_order() {
    assert_eq!(
        length_to_ipv4_mask(16).unwrap().to_ne_bytes(),
        [255, 255, 0, 0]
    );
}

/// L04: a type-1 interface, in a buffer of exactly its length plus the NUL.
#[test]
fn luid_other_exact_buffer() {
    assert_eq!(
        luid_to_name(&NetLuid::new(1, 0), 8),
        Ok("other_0".to_string())
    );
}

/// L07: type 0 has no prefix, so it is named `iftype0`.
#[test]
fn luid_type_zero() {
    assert_eq!(
        luid_to_name(&NetLuid::new(0, 0), 256),
        Ok("iftype0_0".to_string())
    );
}

/// L08: an empty buffer is always too small.
#[test]
fn luid_zero_buffer() {
    assert_eq!(
        luid_to_name(&NetLuid::new(6, 5), 0),
        Err(Error::NotEnoughMemory)
    );
}

/// L09: the IEEE 1394 prefix.
#[test]
fn luid_ieee1394() {
    assert_eq!(
        luid_to_name(&NetLuid::new(144, 3), 256),
        Ok("ieee1394_3".to_string())
    );
}

/// L10: a one-unit buffer cannot hold `atm_12`.
#[test]
fn luid_one_unit_buffer() {
    assert_eq!(
        luid_to_name(&NetLuid::new(37, 12), 1),
        Err(Error::NotEnoughMemory)
    );
}

/// The buffer boundary: `ethernet_5` needs 10 units with its NUL, so 11 is the first that fits.
#[test]
fn luid_buffer_boundary() {
    assert!(luid_to_name(&NetLuid::new(6, 5), 10).is_err());
    assert_eq!(
        luid_to_name(&NetLuid::new(6, 5), 11),
        Ok("ethernet_5".to_string())
    );
}

/// `NetLuid::new` keeps only the low 24 bits of the index.
#[test]
fn luid_index_masked_to_24_bits() {
    assert_eq!(NetLuid::new(24, 0x0100_0001).index, 1);
    assert_eq!(
        luid_to_name(&NetLuid::new(24, 0x0100_0001), 256),
        Ok("loopback_1".to_string())
    );
}

/// N02: `iftype` followed by a decimal type.
#[test]
fn name_iftype_decimal() {
    assert_eq!(name_to_luid(&wide("iftype999_7")), Ok(NetLuid::new(999, 7)));
}

/// N08: a name with no `_` separator.
#[test]
fn name_without_separator() {
    assert_eq!(name_to_luid(&wide("ethernet")), Err(Error::InvalidName));
}

/// N10: an empty index parses as 0.
#[test]
fn name_empty_index() {
    assert_eq!(name_to_luid(&wide("loopback_")), Ok(NetLuid::new(24, 0)));
}

/// N11: the type is truncated to 16 bits, so `65537` becomes 1.
#[test]
fn name_iftype_truncated_to_16_bits() {
    assert_eq!(name_to_luid(&wide("iftype65537_9")), Ok(NetLuid::new(1, 9)));
}

/// N12: an empty prefix is not a known name.
#[test]
fn name_empty_prefix() {
    assert_eq!(name_to_luid(&wide("_5")), Err(Error::InvalidName));
}

/// N13: a leading `+` is accepted by `wcstol`.
#[test]
fn name_iftype_plus_sign() {
    assert_eq!(name_to_luid(&wide("iftype+7_2")), Ok(NetLuid::new(7, 2)));
}

/// N14: parsing stops at the second `_`.
#[test]
fn name_index_stops_at_second_separator() {
    assert_eq!(name_to_luid(&wide("ethernet_5_9")), Ok(NetLuid::new(6, 5)));
}

/// N15: an index past 24 bits keeps its low 24 bits.
#[test]
fn name_index_truncated_to_24_bits() {
    assert_eq!(
        name_to_luid(&wide("tunnel_16777217")),
        Ok(NetLuid::new(131, 1))
    );
}

/// N16: prefixes are case-sensitive.
#[test]
fn name_prefix_is_case_sensitive() {
    assert_eq!(name_to_luid(&wide("Ethernet_5")), Err(Error::InvalidName));
}

/// N17: `wcstol` stops at the first non-digit in an `iftype` name.
#[test]
fn name_iftype_letter_gives_zero() {
    assert_eq!(name_to_luid(&wide("iftypeX_1")), Ok(NetLuid::new(0, 1)));
}

/// N18: a space is skipped before the digits of an `iftype` name.
#[test]
fn name_iftype_space_skipped() {
    assert_eq!(name_to_luid(&wide("iftype 7_2")), Ok(NetLuid::new(7, 2)));
}

/// N19: an index past the range of `long` saturates, then keeps its low 24 bits.
#[test]
fn name_index_overflow_saturates() {
    assert_eq!(
        name_to_luid(&wide("ethernet_99999999999999999999999")),
        Ok(NetLuid::new(6, 0x00ff_ffff))
    );
}

/// N20: a large negative index saturates at the minimum of `long`, whose low 24 bits are 0.
#[test]
fn name_negative_index_overflow_saturates() {
    assert_eq!(
        name_to_luid(&wide("ethernet_-99999999999999999999999")),
        Ok(NetLuid::new(6, 0))
    );
}

/// N21: an `iftype` value that saturates at `long` max has low 32 bits of `0xffffffff`, so Wine rejects it.
#[test]
fn name_iftype_overflow_rejected() {
    assert_eq!(
        name_to_luid(&wide("iftype99999999999999999999_1")),
        Err(Error::InvalidName)
    );
}

/// N22: the `long` minimum has low 32 bits 0, so the type is 0.
#[test]
fn name_iftype_negative_overflow_gives_zero() {
    assert_eq!(
        name_to_luid(&wide("iftype-99999999999999999999_1")),
        Ok(NetLuid::new(0, 1))
    );
}

/// N23: spaces only give type 0.
#[test]
fn name_iftype_spaces_only() {
    assert_eq!(name_to_luid(&wide("iftype   _4")), Ok(NetLuid::new(0, 4)));
}

/// N24 and N25: the separator at index 256 is the last that is accepted.
#[test]
fn name_separator_length_boundary() {
    let mut fits = wide("iftype");
    fits.extend(vec![0x20u16; 250]);
    fits.extend(wide("_1"));
    assert_eq!(name_to_luid(&fits), Ok(NetLuid::new(0, 1)));

    let mut too_long = wide("iftype");
    too_long.extend(vec![0x20u16; 251]);
    too_long.extend(wide("_1"));
    assert_eq!(name_to_luid(&too_long), Err(Error::InvalidName));
}

/// N26: an empty name has no separator.
#[test]
fn name_empty() {
    assert_eq!(name_to_luid(&[]), Err(Error::InvalidName));
}

/// N27: the first NUL ends the name, so the `_` after it is never seen.
#[test]
fn name_embedded_nul_ends_name() {
    assert_eq!(
        name_to_luid(&[0x65, 0x74, 0x68, 0, 0x5f, 0x35, 0]),
        Err(Error::InvalidName)
    );
}

/// A02: `10.0.0.1` is not loopback.
#[test]
fn addr_ipv4_private_not_loopback() {
    assert!(!is_loopback(&v4([10, 0, 0, 1])));
}

/// A03: any address in `127.0.0.0/8` is loopback.
#[test]
fn addr_ipv4_loopback_range() {
    assert!(is_loopback(&v4([127, 255, 255, 224])));
}

/// A04: IPv4 has no link-local test in this path, even for `169.254.0.0/16`.
#[test]
fn addr_ipv4_link_local_block_not_linklocal() {
    assert!(!is_linklocal(&v4([169, 254, 1, 1])));
}

/// A07: `febf::/10` is the top of the link-local range.
#[test]
fn addr_ipv6_linklocal_upper_edge() {
    assert!(is_linklocal(&v6(ipv6(&[
        0xfe, 0xbf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]))));
}

/// A08: `fec0::/10` is site-local, not link-local.
#[test]
fn addr_ipv6_sitelocal_not_linklocal() {
    assert!(!is_linklocal(&v6(ipv6(&[
        0xfe, 0xc0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]))));
}

/// A09: `::2` is neither loopback nor link-local.
#[test]
fn addr_ipv6_other() {
    let a = v6(ipv6(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]));
    assert!(!is_loopback(&a));
    assert!(!is_linklocal(&a));
}

/// A check on the first IPv6 byte alone: `ff80::1` fails the `0xfe` test.
#[test]
fn addr_ipv6_first_byte_check() {
    assert!(!is_linklocal(&v6(ipv6(&[
        0xff, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]))));
}

/// U02: `0.0.0.1` is specified.
#[test]
fn addr_unspecified_ipv4_nonzero() {
    assert!(!is_unspecified(&v4([0, 0, 0, 1])));
}

/// P03: `10.31.255.1` is in `10.16.0.0/12`, whose second byte has its top nibble set.
#[test]
fn prefix_partial_byte_inside() {
    assert!(matches_prefix(
        &v4([10, 31, 255, 1]),
        &prefix4([10, 16, 0, 0], 12)
    ));
}

/// P04: `10.32.0.1` is outside `10.16.0.0/12`.
#[test]
fn prefix_partial_byte_outside() {
    assert!(!matches_prefix(
        &v4([10, 32, 0, 1]),
        &prefix4([10, 16, 0, 0], 12)
    ));
}

/// P05: `/0` matches the whole family.
#[test]
fn prefix_zero_length_matches_family() {
    assert!(matches_prefix(&v4([1, 2, 3, 4]), &prefix4([0; 4], 0)));
}

/// P06 and P07: a `/32` matches only the exact address.
#[test]
fn prefix_full_length() {
    assert!(matches_prefix(
        &v4([1, 2, 3, 4]),
        &prefix4([1, 2, 3, 4], 32)
    ));
    assert!(!matches_prefix(
        &v4([1, 2, 3, 5]),
        &prefix4([1, 2, 3, 4], 32)
    ));
}

/// P08: `/33` is past the width of IPv4.
#[test]
fn prefix_past_ipv4_width() {
    assert!(!matches_prefix(
        &v4([1, 2, 3, 4]),
        &prefix4([1, 2, 3, 4], 33)
    ));
}

/// P09 and P10: `/23` fixes the first two bytes and the top seven bits of the third.
#[test]
fn prefix_slash_23() {
    assert!(matches_prefix(
        &v4([1, 2, 3, 4]),
        &prefix4([1, 2, 2, 0], 23)
    ));
    assert!(!matches_prefix(
        &v4([1, 2, 4, 0]),
        &prefix4([1, 2, 2, 0], 23)
    ));
}

/// P12 and P13: `/1` compares the top bit only.
#[test]
fn prefix_slash_1() {
    assert!(!matches_prefix(
        &v4([127, 255, 255, 255]),
        &prefix4([128, 0, 0, 0], 1)
    ));
    assert!(matches_prefix(
        &v4([128, 0, 0, 1]),
        &prefix4([128, 0, 0, 0], 1)
    ));
}

/// P14: `/7` compares the top seven bits, so `11` matches `10`.
#[test]
fn prefix_slash_7() {
    assert!(matches_prefix(
        &v4([11, 0, 0, 0]),
        &prefix4([10, 0, 0, 0], 7)
    ));
}

/// P15 and P16: `/24` compares three whole bytes.
#[test]
fn prefix_slash_24_whole_bytes() {
    assert!(matches_prefix(
        &v4([10, 1, 2, 3]),
        &prefix4([10, 1, 2, 0], 24)
    ));
    assert!(!matches_prefix(
        &v4([10, 1, 2, 3]),
        &prefix4([10, 1, 3, 0], 24)
    ));
}

/// `/8` is a whole byte, so the partial-byte branch is not taken.
#[test]
fn prefix_slash_8_whole_byte() {
    assert!(matches_prefix(
        &v4([10, 9, 9, 9]),
        &prefix4([10, 0, 0, 0], 8)
    ));
    assert!(!matches_prefix(
        &v4([11, 0, 0, 0]),
        &prefix4([10, 0, 0, 0], 8)
    ));
}

/// Q03: a `/129` is past the width of IPv6.
#[test]
fn prefix_ipv6_past_width() {
    assert!(!matches_prefix(&v6([0; 16]), &prefix6([0; 16], 129)));
}

/// Q04 and Q05: `/0` matches any IPv6 address.
#[test]
fn prefix_ipv6_zero_length() {
    assert!(matches_prefix(
        &v6(ipv6(&[0x20, 0x01])),
        &prefix6([0; 16], 0)
    ));
}

/// Q06: a `/128` matches only the exact address.
#[test]
fn prefix_ipv6_full_length() {
    let mut a = [0u8; 16];
    a[15] = 1;
    assert!(matches_prefix(&v6(a), &prefix6(a, 128)));
}

/// A zero-length prefix of the other family does not match (Wine checks family first).
#[test]
fn prefix_family_mismatch_is_false() {
    assert!(!matches_prefix(&v4([1, 2, 3, 4]), &prefix6([0; 16], 0)));
    assert!(!matches_prefix(&v6([0; 16]), &prefix4([0; 4], 0)));
}

/// S02 to S06: the non-mapped inputs of the oracle run all fail.
#[test]
fn map_rejects_non_mapped() {
    let mut loopback = [0u8; 16];
    loopback[15] = 1;
    assert_eq!(map_ipv6_to_ipv4(&loopback), None);
    assert_eq!(map_ipv6_to_ipv4(&[0; 16]), None);

    let mut fffe = [0u8; 16];
    fffe[14] = 0xff;
    fffe[15] = 0xfe;
    assert_eq!(map_ipv6_to_ipv4(&fffe), None);

    let mut fffe_mapped = [0u8; 16];
    fffe_mapped[10] = 0xff;
    fffe_mapped[11] = 0xfe;
    fffe_mapped[12..].copy_from_slice(&[1, 2, 3, 4]);
    assert_eq!(map_ipv6_to_ipv4(&fffe_mapped), None);

    let mut leading = [0u8; 16];
    leading[..4].copy_from_slice(&[0x20, 0x01, 0x0d, 0xb8]);
    leading[10] = 0xff;
    leading[11] = 0xff;
    leading[12..].copy_from_slice(&[1, 2, 3, 4]);
    assert_eq!(map_ipv6_to_ipv4(&leading), None);
}

/// `::ffff:0.0.0.0` maps to `0.0.0.0`, which is still a success.
#[test]
fn map_zero_ipv4_is_success() {
    let mut mapped = [0u8; 16];
    mapped[10] = 0xff;
    mapped[11] = 0xff;
    assert_eq!(map_ipv6_to_ipv4(&mapped), Some([0, 0, 0, 0]));
}

/// The unspecified IPv6 address is not loopback and not link-local.
#[test]
fn addr_unspecified_ipv6_is_not_loopback() {
    let a = v6([0; 16]);
    assert!(is_unspecified(&a));
    assert!(!is_loopback(&a));
}
