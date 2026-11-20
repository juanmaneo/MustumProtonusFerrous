//! Oracle rows from Wine's `dlls/iphlpapi/iphlpapi_main.c`, built verbatim under
//! ASan and UBSan. The function bodies were copied from the reference by line
//! range (60-76, 253-270, 916-939, 4469-4515, 4536-4572, 4577-4591, 4751-4776,
//! 4809-4814) and compiled with a Windows type shim. No vector was flagged by the
//! sanitizers, so none is excluded. Each expected value is that build's output.
//! Rows O01-O25 are here; the rest of the same run is in `edge_checks`.

use crate::addr::{
    is_linklocal, is_loopback, is_unspecified, length_to_ipv4_mask, map_ipv6_to_ipv4,
    matches_prefix, IpAddress, Prefix, INADDR_NONE,
};
use crate::guid::{guid_to_string, Guid};
use crate::luid::{luid_to_name, name_to_luid, NetLuid};
use crate::Error;

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

const G01: Guid = Guid {
    data1: 0x1234_5678,
    data2: 0x1234,
    data3: 0x5678,
    data4: [0x9a, 0xbc, 0xde, 0xf0, 0x12, 0x34, 0x56, 0x78],
};

const G02: Guid = Guid {
    data1: 0,
    data2: 0,
    data3: 0,
    data4: [0; 8],
};

const G03: Guid = Guid {
    data1: 0xffff_ffff,
    data2: 0xffff,
    data3: 0xffff,
    data4: [0xff; 8],
};

/// O01 (G01, A and W): a full GUID in a 39-unit buffer.
#[test]
fn o01_guid_full() {
    assert_eq!(
        guid_to_string(&G01, 39),
        Ok("{12345678-1234-5678-9ABC-DEF012345678}".to_string())
    );
}

/// O02 (G02, A and W): a 38-unit buffer is one short, so the call fails.
#[test]
fn o02_guid_short_buffer() {
    assert_eq!(guid_to_string(&G02, 38), Err(Error::InsufficientBuffer));
}

/// O03 (G03, A and W): all-ones GUID in a larger buffer.
#[test]
fn o03_guid_all_ones() {
    assert_eq!(
        guid_to_string(&G03, 100),
        Ok("{FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF}".to_string())
    );
}

/// O04 (M01): a zero-length mask.
#[test]
fn o04_mask_zero_length() {
    assert_eq!(length_to_ipv4_mask(0), Ok(0x0000_0000));
}

/// O05 (M03): a /24 mask, as Wine stores it (`0x00ffffff`).
#[test]
fn o05_mask_slash_24() {
    assert_eq!(length_to_ipv4_mask(24), Ok(0x00ff_ffff));
}

/// O06 (M05): a length of 33 is rejected, and Wine stores `INADDR_NONE`.
#[test]
fn o06_mask_too_long() {
    assert_eq!(length_to_ipv4_mask(33), Err(Error::InvalidParameter));
    assert_eq!(INADDR_NONE, 0xffff_ffff);
}

/// O07 (L01): an Ethernet interface name.
#[test]
fn o07_luid_ethernet() {
    assert_eq!(
        luid_to_name(&NetLuid::new(6, 5), 256),
        Ok("ethernet_5".to_string())
    );
}

/// O08 (L02): a loopback name fits in 15 units.
#[test]
fn o08_luid_loopback_fits() {
    assert_eq!(
        luid_to_name(&NetLuid::new(24, 1), 15),
        Ok("loopback_1".to_string())
    );
}

/// O09 (L03): `ethernet_5` is 10 units, so with its NUL it needs 11. A 10-unit buffer fails.
#[test]
fn o09_luid_buffer_one_short() {
    assert_eq!(
        luid_to_name(&NetLuid::new(6, 5), 10),
        Err(Error::NotEnoughMemory)
    );
}

/// O10 (L05): the largest 24-bit index.
#[test]
fn o10_luid_tunnel_max_index() {
    assert_eq!(
        luid_to_name(&NetLuid::new(131, 0x00ff_ffff), 256),
        Ok("tunnel_16777215".to_string())
    );
}

/// O11 (L06): a type with no prefix uses `iftype<n>`.
#[test]
fn o11_luid_unknown_type() {
    assert_eq!(
        luid_to_name(&NetLuid::new(999, 7), 256),
        Ok("iftype999_7".to_string())
    );
}

/// O12 (N01): parses an Ethernet name.
#[test]
fn o12_name_ethernet() {
    assert_eq!(name_to_luid(&wide("ethernet_5")), Ok(NetLuid::new(6, 5)));
}

/// O13 (N03): `iftype-1` gives type `~0`, which Wine rejects as an invalid name.
#[test]
fn o13_name_negative_iftype_rejected() {
    assert_eq!(name_to_luid(&wide("iftype-1_5")), Err(Error::InvalidName));
}

/// O14 (N04): `iftype` with no digits is only six units, so it is not an `iftype` name.
#[test]
fn o14_name_bare_iftype_rejected() {
    assert_eq!(name_to_luid(&wide("iftype_5")), Err(Error::InvalidName));
}

/// O15 (N05): `wcstol` stops at the first non-digit, so `iftypeabc` gives type 0.
#[test]
fn o15_name_iftype_non_digits_gives_zero() {
    assert_eq!(name_to_luid(&wide("iftypeabc_1")), Ok(NetLuid::new(0, 1)));
}

/// O16 (N06): `wcstol` skips leading white space, and the index stops at `x`.
#[test]
fn o16_name_wireless_with_space() {
    assert_eq!(
        name_to_luid(&wide("wireless_ 12x")),
        Ok(NetLuid::new(71, 12))
    );
}

/// O17 (N09): a negative index keeps its low 24 bits.
#[test]
fn o17_name_negative_index_wraps_to_24_bits() {
    assert_eq!(
        name_to_luid(&wide("ethernet_-3")),
        Ok(NetLuid::new(6, 0x00ff_fffd))
    );
}

/// O18 (N07): an unknown prefix is rejected.
#[test]
fn o18_name_unknown_prefix() {
    assert_eq!(name_to_luid(&wide("unknown_1")), Err(Error::InvalidName));
}

/// O19 (A01): `127.0.0.1` is loopback, not link-local, not unspecified.
#[test]
fn o19_addr_ipv4_loopback() {
    let a = IpAddress::V4([127, 0, 0, 1]);
    assert!(is_loopback(&a));
    assert!(!is_linklocal(&a));
    assert!(!is_unspecified(&a));
}

/// O20 (A05): `::1` is loopback.
#[test]
fn o20_addr_ipv6_loopback() {
    let mut b = [0u8; 16];
    b[15] = 1;
    let a = IpAddress::V6(b);
    assert!(is_loopback(&a));
    assert!(!is_linklocal(&a));
    assert!(!is_unspecified(&a));
}

/// O21 (A06): `fe80::1` is link-local, not loopback.
#[test]
fn o21_addr_ipv6_linklocal() {
    let mut b = [0u8; 16];
    b[0] = 0xfe;
    b[1] = 0x80;
    b[15] = 1;
    let a = IpAddress::V6(b);
    assert!(!is_loopback(&a));
    assert!(is_linklocal(&a));
}

/// O22 (U01, U03): the unspecified IPv4 and IPv6 addresses.
#[test]
fn o22_addr_unspecified() {
    assert!(is_unspecified(&IpAddress::V4([0; 4])));
    assert!(is_unspecified(&IpAddress::V6([0; 16])));
}

/// O23 (P01, P02): `192.168.1.77` is in `192.168.0.0/16`, not in `192.169.0.0/16`.
#[test]
fn o23_prefix_ipv4() {
    let addr = IpAddress::V4([192, 168, 1, 77]);
    let inside = Prefix {
        address: IpAddress::V4([192, 168, 0, 0]),
        length: 16,
    };
    let outside = Prefix {
        address: IpAddress::V4([192, 169, 0, 0]),
        length: 16,
    };
    assert!(matches_prefix(&addr, &inside));
    assert!(!matches_prefix(&addr, &outside));
}

/// O24 (Q01, Q02): `2001:db8::1` is in `2001:db8::/32`, not in `2001:db9::/32`.
#[test]
fn o24_prefix_ipv6() {
    let mut addr = [0u8; 16];
    addr[..4].copy_from_slice(&[0x20, 0x01, 0x0d, 0xb8]);
    addr[15] = 1;
    let mut inside = [0u8; 16];
    inside[..4].copy_from_slice(&[0x20, 0x01, 0x0d, 0xb8]);
    let mut outside = [0u8; 16];
    outside[..4].copy_from_slice(&[0x20, 0x01, 0x0d, 0xb9]);
    assert!(matches_prefix(
        &IpAddress::V6(addr),
        &Prefix {
            address: IpAddress::V6(inside),
            length: 32
        }
    ));
    assert!(!matches_prefix(
        &IpAddress::V6(addr),
        &Prefix {
            address: IpAddress::V6(outside),
            length: 32
        }
    ));
}

/// O25 (S01, S02): `::ffff:1.2.3.4` maps to `3.4.1.2` (Wine's byte order), and `::1` does not map.
#[test]
fn o25_map_ipv4_mapped() {
    let mut mapped = [0u8; 16];
    mapped[10] = 0xff;
    mapped[11] = 0xff;
    mapped[12..].copy_from_slice(&[1, 2, 3, 4]);
    assert_eq!(map_ipv6_to_ipv4(&mapped), Some([3, 4, 1, 2]));

    let mut loopback = [0u8; 16];
    loopback[15] = 1;
    assert_eq!(map_ipv6_to_ipv4(&loopback), None);
}
