//! Oracle-derived test vectors for the ported nsiproxy.sys logic.
//!
//! The vectors come from the verbatim Wine C bodies in `dlls/nsiproxy.sys/` (`ip.c`, `tcp.c`,
//! `icmp_echo.c` and `device.c`), built with the minimal Windows type shims under
//! `target/canon/nsiproxy.sys/` (`oracle_pure.c` and `oracle_icmp.c`) with
//! `gcc -fsanitize=address,undefined -g`. The harnesses print 25 rows. ASan and UBSan report
//! nothing for any of them, so no row is excluded. The `// row NN` comments match the printed
//! output.
//!
//! Rows 01 to 03 print only the parsed prefix. The C code leaves the rest of the address
//! uninitialised, so those tests compare only the first `end / 2` bytes.
//!
//! Rows 07 and 09 assume a little-endian host, as the oracle ran on x86-64.

use super::*;
use std::net::Ipv6Addr;

fn v6(text: &str) -> [u8; 16] {
    text.parse::<Ipv6Addr>().unwrap().octets()
}

#[test]
fn str_to_in6_addr_rows() {
    // row 01
    let (addr, end) = str_to_in6_addr(b"00112233445566778899AABBCCDDEEFF");
    assert_eq!(end, 32);
    assert_eq!(addr, v6("0011:2233:4455:6677:8899:aabb:ccdd:eeff"));

    // row 02
    let (addr, end) = str_to_in6_addr(b"0011223344556677zz8899AABBCCDDEEFF");
    assert_eq!(end, 16);
    assert_eq!(addr[..8], v6("0011:2233:4455:6677::")[..8]);

    // row 03
    let (addr, end) = str_to_in6_addr(b"00112233445566778899AABBCCDDEEF");
    assert_eq!(end, 30);
    assert_eq!(
        addr[..15],
        v6("0011:2233:4455:6677:8899:aabb:ccdd:ee00")[..15]
    );
}

#[test]
fn mask_prefix_rows() {
    // row 04
    assert_eq!(mask_v4_to_prefix(0xffff_ff00), 24);
    // row 05
    assert_eq!(mask_v6_to_prefix(&v6("ffff:ffff:ffff:ffff::")), 64);
}

#[test]
fn find_ipv6_addr_scope_rows() {
    // row 06
    assert_eq!(find_ipv6_addr_scope(&v6("::"), None), 0);
    // row 07
    assert_eq!(find_ipv6_addr_scope(&v6("ff02::1"), None), 0x0200);
    // row 08
    assert_eq!(find_ipv6_addr_scope(&v6("2001:db8::1"), None), u32::MAX);

    let table = [Ipv6AddrScope {
        addr: v6("fe80::1"),
        scope: 0x20,
    }];
    // row 09
    assert_eq!(find_ipv6_addr_scope(&v6("fe80::1"), Some(&table)), 0x20);
    // row 10
    assert_eq!(
        find_ipv6_addr_scope(&v6("2001:db8::2"), Some(&table)),
        u32::MAX
    );
}

#[test]
fn icmp_checksum_rows() {
    let data = [0x00, 0x01, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7];
    // row 11
    assert_eq!(chksum(false, &data), 0x0d22);
    // row 12
    assert_eq!(chksum(false, &data[..7]), 0x0423);
    // row 13
    assert_eq!(chksum(true, &data), 0);

    let message = [0x80, 0x00, 0x00, 0x00, 0x12, 0x34, 0x00, 0x01];
    // row 14
    assert_eq!(
        ipv6_chksum(false, &v6("2001:db8::1"), &v6("ff02::1"), &message),
        0xca40
    );
    // row 15
    assert_eq!(
        ipv6_chksum(true, &v6("2001:db8::1"), &v6("ff02::1"), &message),
        0
    );
}

#[test]
fn errno_to_ip_status_rows() {
    // row 16
    assert_eq!(errno_to_ip_status(113), 11003);
    // row 17
    assert_eq!(errno_to_ip_status(111), 11050);
}

#[test]
fn icmp_echo_reply_struct_len_rows() {
    // row 18
    assert_eq!(icmp_echo_reply_struct_len(AF_INET, 32), 28);
    // row 19
    assert_eq!(icmp_echo_reply_struct_len(AF_INET, 64), 40);
    // row 20
    assert_eq!(icmp_echo_reply_struct_len(AF_INET6, 0), 36);
}

#[test]
fn nsiproxy_icmp_echo_check_rows() {
    let ipv4 = |dst_ipv4| IcmpEchoRequest {
        dst_family: AF_INET,
        dst_ipv4,
        dst_ipv6: [0; 16],
        bits: 32,
        opt_size: 0,
        req_size: 0,
    };
    // row 21
    assert_eq!(
        nsiproxy_icmp_echo_check(83, 28, &ipv4(0x0100_007f)),
        Err(STATUS_INVALID_PARAMETER)
    );
    // row 22
    assert_eq!(
        nsiproxy_icmp_echo_check(84, 28, &ipv4(0)),
        Err(STATUS_INVALID_ADDRESS_WILDCARD)
    );
    // row 23
    let ipv6 = IcmpEchoRequest {
        dst_family: AF_INET6,
        dst_ipv4: 0,
        dst_ipv6: v6("2001:db8::1"),
        bits: 0,
        opt_size: 0xffff_fffd,
        req_size: 0,
    };
    assert_eq!(nsiproxy_icmp_echo_check(84, 36, &ipv6), Ok(()));
}

#[test]
fn nsi_ioctl_rows() {
    // row 24
    assert_eq!(nsi_ioctl(0x0012_1000), Ok(IoctlOp::EnumerateAll));
    // row 25
    assert_eq!(nsi_ioctl(0x0012_1014), Err(STATUS_NOT_SUPPORTED));
}
