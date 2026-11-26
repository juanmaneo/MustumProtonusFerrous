//! Edge-case tests for the ported nsiproxy.sys logic.
//!
//! These cover the boundaries and branches that the oracle vectors do not reach. Each expectation
//! that is not oracle-derived says so in a comment.

use super::*;
use std::net::Ipv6Addr;

fn v6(text: &str) -> [u8; 16] {
    text.parse::<Ipv6Addr>().unwrap().octets()
}

fn ipv4_request() -> IcmpEchoRequest {
    IcmpEchoRequest {
        dst_family: AF_INET,
        dst_ipv4: 0x0100_007f,
        dst_ipv6: [0; 16],
        bits: 32,
        opt_size: 0,
        req_size: 0,
    }
}

fn ipv6_request() -> IcmpEchoRequest {
    IcmpEchoRequest {
        dst_family: AF_INET6,
        dst_ipv4: 0,
        dst_ipv6: v6("2001:db8::1"),
        bits: 0,
        opt_size: 0,
        req_size: 0,
    }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/ws2def.h, include/ipexport.h and
    // include/ntstatus.h. EHOSTUNREACH is the Linux errno value.
    assert_eq!(IPPROTO_ICMPV6, 58);
    assert_eq!(IP_STATUS_BASE, 11000);
    assert_eq!(IP_DEST_HOST_UNREACHABLE, 11003);
    assert_eq!(IP_GENERAL_FAILURE, 11050);
    assert_eq!(EHOSTUNREACH, 113);
    assert_eq!(AF_INET, 2);
    assert_eq!(AF_INET6, 23);
    assert_eq!(STATUS_INVALID_PARAMETER, 0xc000_000d);
    assert_eq!(STATUS_INVALID_ADDRESS_WILDCARD, 0xc000_0208);
    assert_eq!(STATUS_NOT_SUPPORTED, 0xc000_00bb);
    assert_eq!(FILE_DEVICE_NETWORK, 0x12);
    assert_eq!(METHOD_BUFFERED, 0);
    assert_eq!(FILE_ANY_ACCESS, 0);
}

#[test]
fn ctl_code_packs_its_fields() {
    // Not oracle-derived: CTL_CODE from include/winioctl.h, worked by hand. Access sits at bit 14,
    // the function at bit 2 and the method in the low two bits.
    assert_eq!(ctl_code(0, 0, 0, 1), 0x4000);
    assert_eq!(ctl_code(0x12, 0, 3, 0), 0x0012_0003);
    assert_eq!(ctl_code(0x12, 0x403, 0, 0), 0x0012_100c);
}

#[test]
fn ioctl_codes_are_ctl_code_of_the_wine_functions() {
    // Not oracle-derived: CTL_CODE(FILE_DEVICE_NETWORK, 0x400..0x404, METHOD_BUFFERED,
    // FILE_ANY_ACCESS) from include/wine/nsi.h. Only 0x121000 is an oracle row, as row 24.
    assert_eq!(IOCTL_NSIPROXY_WINE_ENUMERATE_ALL, 0x0012_1000);
    assert_eq!(IOCTL_NSIPROXY_WINE_GET_ALL_PARAMETERS, 0x0012_1004);
    assert_eq!(IOCTL_NSIPROXY_WINE_GET_PARAMETER, 0x0012_1008);
    assert_eq!(IOCTL_NSIPROXY_WINE_ICMP_ECHO, 0x0012_100c);
    assert_eq!(IOCTL_NSIPROXY_WINE_CHANGE_NOTIFICATION, 0x0012_1010);
}

#[test]
fn nsi_ioctl_dispatches_each_wine_code() {
    // Not oracle-derived: each code maps to its case label in the switch in device.c.
    assert_eq!(nsi_ioctl(0x0012_1004), Ok(IoctlOp::GetAllParameters));
    assert_eq!(nsi_ioctl(0x0012_1008), Ok(IoctlOp::GetParameter));
    assert_eq!(nsi_ioctl(0x0012_100c), Ok(IoctlOp::IcmpEcho));
    assert_eq!(nsi_ioctl(0x0012_1010), Ok(IoctlOp::ChangeNotification));
}

#[test]
fn nsi_ioctl_rejects_unknown_codes() {
    // Not oracle-derived: the C default case returns STATUS_NOT_SUPPORTED. 0x121018 is the next
    // function number, 0x00221000 has another device type, and 0 is not a code.
    assert_eq!(nsi_ioctl(0x0012_1018), Err(STATUS_NOT_SUPPORTED));
    assert_eq!(nsi_ioctl(0x0022_1000), Err(STATUS_NOT_SUPPORTED));
    assert_eq!(nsi_ioctl(0), Err(STATUS_NOT_SUPPORTED));
}

#[test]
fn icmp_echo_reply_struct_len_branches() {
    // Not oracle-derived: for AF_INET only bits == 32 selects the 32-bit reply. AF_INET6 ignores
    // bits. Other families give 0, including 10, which is the Linux AF_INET6 value.
    assert_eq!(icmp_echo_reply_struct_len(AF_INET, 31), 40);
    assert_eq!(icmp_echo_reply_struct_len(AF_INET, 33), 40);
    assert_eq!(icmp_echo_reply_struct_len(AF_INET, 0), 40);
    assert_eq!(icmp_echo_reply_struct_len(AF_INET6, 32), 36);
    assert_eq!(icmp_echo_reply_struct_len(0, 32), 0);
    assert_eq!(icmp_echo_reply_struct_len(1, 32), 0);
    assert_eq!(icmp_echo_reply_struct_len(10, 32), 0);
}

#[test]
fn chksum_edge_cases() {
    // Not oracle-derived, worked by hand. An empty input sums to 0, so the checksum is 0xffff.
    // The single byte 0x01 is an odd final byte, giving ~1 = 0xfffe. Two words of 0xffff sum to
    // 0x1fffe, which folds to 0xffff, giving 0. The odd byte's carry is folded: 0xffff + 0x01
    // gives 0x10000, which folds to 1.
    assert_eq!(chksum(false, &[]), 0xffff);
    assert_eq!(chksum(false, &[0x01]), 0xfffe);
    assert_eq!(chksum(false, &[0xff; 4]), 0x0000);
    assert_eq!(chksum(false, &[0xff, 0xff, 0x01]), 0xfffe);
    assert_eq!(chksum(true, &[]), 0);
}

#[test]
fn chksum_of_data_with_its_checksum_is_zero() {
    // Not oracle-derived: the RFC 1071 verification property. The row 11 bytes followed by that
    // checksum in little-endian order (0x0d22 is stored as 22 0d) sum to 0.
    let data = [0x00, 0x01, 0xf2, 0x03, 0xf4, 0xf5, 0xf6, 0xf7, 0x22, 0x0d];
    assert_eq!(chksum(false, &data), 0);
}

#[test]
fn chksum_carries_out_of_the_32_bit_sum() {
    // Not oracle-derived. 65538 words of 0xffff sum to 2^32 + 0xfffe, so the 32-bit accumulator
    // wraps once and the end-around carry must be added back. The result is then 0xffff, whose
    // complement is 0. Without the carry the result would be 0x0001.
    let data = vec![0xff; 65538 * 2];
    assert_eq!(chksum(false, &data), 0);
}

#[test]
fn ipv6_chksum_covers_the_pseudo_header() {
    // Not oracle-derived: the pseudo-header is the source, the destination, the 32-bit
    // big-endian message length, three zero bytes and next header 58, as in struct
    // ipv6_pseudo_header in icmp_echo.c. The bytes are assembled by hand here.
    let src = v6("2001:db8::1");
    let dst = v6("ff02::1");
    let message = [0x80, 0x00, 0x00, 0x00, 0x12, 0x34, 0x00, 0x01];
    let mut manual = Vec::new();
    manual.extend_from_slice(&src);
    manual.extend_from_slice(&dst);
    manual.extend_from_slice(&[0x00, 0x00, 0x00, 0x08]);
    manual.extend_from_slice(&[0x00, 0x00, 0x00, 0x3a]);
    manual.extend_from_slice(&message);
    assert_eq!(
        ipv6_chksum(false, &src, &dst, &message),
        chksum(false, &manual)
    );
    assert_eq!(ipv6_chksum(true, &src, &dst, &[]), 0);
}

#[test]
fn errno_to_ip_status_defaults_to_general_failure() {
    // Not oracle-derived: every errno other than EHOSTUNREACH maps to IP_GENERAL_FAILURE.
    assert_eq!(errno_to_ip_status(0), IP_GENERAL_FAILURE);
    assert_eq!(errno_to_ip_status(-1), IP_GENERAL_FAILURE);
    assert_eq!(errno_to_ip_status(EHOSTUNREACH + 1), IP_GENERAL_FAILURE);
}

#[test]
fn str_to_in6_addr_stops_at_the_first_bad_pair() {
    // Not oracle-derived: the C loop returns at the first pair where either digit is not a hex
    // digit, or where the string ends. The end offset is the start of that pair, and nothing is
    // written for that pair.
    assert_eq!(str_to_in6_addr(b""), ([0; 16], 0));
    assert_eq!(str_to_in6_addr(b"0"), ([0; 16], 0));
    assert_eq!(str_to_in6_addr(b"g0"), ([0; 16], 0));
    assert_eq!(str_to_in6_addr(b"0g"), ([0; 16], 0));
    let (addr, end) = str_to_in6_addr(b"00112233445566778899AABBCCDDEEgg");
    assert_eq!((addr[14], addr[15], end), (0xee, 0, 30));
}

#[test]
fn str_to_in6_addr_reads_at_most_32_digits() {
    // Not oracle-derived: the loop runs 16 times, so digits after the first 32 are not read.
    let (addr, end) = str_to_in6_addr(b"00112233445566778899AABBCCDDEEFF0123");
    assert_eq!(end, 32);
    assert_eq!(addr, v6("0011:2233:4455:6677:8899:aabb:ccdd:eeff"));
}

#[test]
fn mask_prefix_edges() {
    // Not oracle-derived: popcount at the ends of the range.
    assert_eq!(mask_v4_to_prefix(0), 0);
    assert_eq!(mask_v4_to_prefix(u32::MAX), 32);
    assert_eq!(mask_v6_to_prefix(&[0; 16]), 0);
    assert_eq!(mask_v6_to_prefix(&[0xff; 16]), 128);
    assert_eq!(mask_v6_to_prefix(&v6("8000::")), 1);
}

#[test]
fn unspecified_is_checked_before_the_table() {
    // Not oracle-derived: "::" returns 0 even when the table has an entry for it.
    let table = [Ipv6AddrScope {
        addr: [0; 16],
        scope: 5,
    }];
    assert_eq!(find_ipv6_addr_scope(&[0; 16], Some(&table)), 0);
}

#[test]
fn multicast_ignores_the_table_and_masks_the_scope_nibble() {
    // Not oracle-derived: multicast takes its scope from the low nibble of byte 1 and never reads
    // the table. The htons result puts the scope in the high byte on a little-endian host.
    let table = [Ipv6AddrScope {
        addr: v6("ff02::1"),
        scope: 7,
    }];
    assert_eq!(find_ipv6_addr_scope(&v6("ff02::1"), Some(&table)), 0x0200);
    assert_eq!(find_ipv6_addr_scope(&v6("ff12::1"), None), 0x0200);
    assert_eq!(find_ipv6_addr_scope(&v6("ff05::1"), None), 0x0500);
    assert_eq!(find_ipv6_addr_scope(&v6("ff0e::1"), None), 0x0e00);
}

#[test]
fn table_lookup_edges() {
    // Not oracle-derived: an empty table, a table with no match, and the first of two matching
    // entries.
    let addr = v6("fe80::1");
    let duplicates = [
        Ipv6AddrScope { addr, scope: 0x20 },
        Ipv6AddrScope { addr, scope: 0x30 },
    ];
    assert_eq!(find_ipv6_addr_scope(&addr, Some(&duplicates)), 0x20);
    assert_eq!(find_ipv6_addr_scope(&addr, Some(&[])), u32::MAX);
    let other = [Ipv6AddrScope {
        addr: v6("fe80::2"),
        scope: 0x20,
    }];
    assert_eq!(find_ipv6_addr_scope(&addr, Some(&other)), u32::MAX);
}

#[test]
fn request_length_boundaries() {
    // Not oracle-derived: the data end is 84 + ((opt_size + 3) & ~3) + req_size, and the input
    // length must reach it exactly.
    let mut req = ipv4_request();
    assert_eq!(nsiproxy_icmp_echo_check(84, 28, &req), Ok(()));
    req.req_size = 5;
    assert_eq!(
        nsiproxy_icmp_echo_check(88, 28, &req),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(89, 28, &req), Ok(()));
    req.req_size = 0;
    req.opt_size = 1;
    assert_eq!(
        nsiproxy_icmp_echo_check(87, 28, &req),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(88, 28, &req), Ok(()));
    req.opt_size = 4;
    assert_eq!(
        nsiproxy_icmp_echo_check(87, 28, &req),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(88, 28, &req), Ok(()));
    req.opt_size = 5;
    assert_eq!(
        nsiproxy_icmp_echo_check(91, 28, &req),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(92, 28, &req), Ok(()));
}

#[test]
fn request_length_arithmetic_does_not_wrap_into_range() {
    // Not oracle-derived: the padding sum wraps in 32 bits, as the C UINT arithmetic does, and the
    // data end is then formed in 64 bits. 0xffff_fffe + 3 wraps to 1, which pads to 0, so only
    // req_size counts.
    let mut req = ipv4_request();
    req.opt_size = 0xffff_fffe;
    req.req_size = 5;
    assert_eq!(
        nsiproxy_icmp_echo_check(88, 28, &req),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(89, 28, &req), Ok(()));

    // 0xffff_fffc + 3 does not wrap, so the data end is past every 32-bit input length.
    req.opt_size = 0xffff_fffc;
    req.req_size = 0;
    assert_eq!(
        nsiproxy_icmp_echo_check(u32::MAX, 28, &req),
        Err(STATUS_INVALID_PARAMETER)
    );

    // With req_size = u32::MAX the data end is 84 + u32::MAX, which no input length reaches.
    req.opt_size = 0;
    req.req_size = u32::MAX;
    assert_eq!(
        nsiproxy_icmp_echo_check(u32::MAX, 28, &req),
        Err(STATUS_INVALID_PARAMETER)
    );
}

#[test]
fn reply_buffer_boundaries() {
    // Not oracle-derived: one byte short of each reply struct is INVALID_PARAMETER, and the exact
    // size passes.
    let mut v4_64 = ipv4_request();
    v4_64.bits = 64;
    assert_eq!(
        nsiproxy_icmp_echo_check(84, 27, &ipv4_request()),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(84, 28, &ipv4_request()), Ok(()));
    assert_eq!(
        nsiproxy_icmp_echo_check(84, 39, &v4_64),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(84, 40, &v4_64), Ok(()));
    assert_eq!(
        nsiproxy_icmp_echo_check(84, 35, &ipv6_request()),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(nsiproxy_icmp_echo_check(84, 36, &ipv6_request()), Ok(()));
}

#[test]
fn request_family_and_address_arms() {
    // Not oracle-derived: the family switch runs after the length checks. Each family reads only
    // its own destination field.
    let mut v4_zero = ipv4_request();
    v4_zero.dst_ipv4 = 0;
    assert_eq!(
        nsiproxy_icmp_echo_check(84, 28, &v4_zero),
        Err(STATUS_INVALID_ADDRESS_WILDCARD)
    );
    assert_eq!(nsiproxy_icmp_echo_check(84, 28, &ipv4_request()), Ok(()));

    let mut v6_zero = ipv6_request();
    v6_zero.dst_ipv6 = [0; 16];
    assert_eq!(
        nsiproxy_icmp_echo_check(84, 36, &v6_zero),
        Err(STATUS_INVALID_ADDRESS_WILDCARD)
    );

    for family in [0, 1, 10] {
        let req = IcmpEchoRequest {
            dst_family: family,
            ..ipv4_request()
        };
        assert_eq!(
            nsiproxy_icmp_echo_check(84, 0, &req),
            Err(STATUS_INVALID_PARAMETER)
        );
    }

    // Length failures come before the wildcard test, so a zero address with a short buffer is
    // INVALID_PARAMETER.
    assert_eq!(
        nsiproxy_icmp_echo_check(84, 27, &v4_zero),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(
        nsiproxy_icmp_echo_check(83, 28, &v4_zero),
        Err(STATUS_INVALID_PARAMETER)
    );
}
