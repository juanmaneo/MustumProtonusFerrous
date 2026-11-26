//! `DWORD_cmp`, the IP address and UDP row comparators, and `oidToUdpRow` from `main.c`.

use crate::oid::oid_to_ip_addr;

/// `DWORD_cmp`: three-way comparison of two `DWORD`s.
pub fn dword_cmp(a: u32, b: u32) -> i32 {
    a.cmp(&b) as i32
}

/// `compareIpAddrRow`, which `compareIpForwardRow` shares: orders rows by `ntohl(dwAddr)`,
/// that is, the octets read as a big-endian number.
pub fn compare_ip_addr_row(a: &[u8; 4], b: &[u8; 4]) -> i32 {
    dword_cmp(u32::from_be_bytes(*a), u32::from_be_bytes(*b))
}

/// The address and port fields of `MIB_UDPROW`. The port is in host order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UdpRow {
    pub local_addr: [u8; 4],
    pub local_port: u16,
}

/// `oidToUdpRow`: the local address is the first four arcs and the port is arc 5.
pub fn oid_to_udp_row(ids: &[u32]) -> UdpRow {
    assert!(ids.len() >= 5, "oidToUdpRow needs at least 5 arcs");
    UdpRow {
        local_addr: oid_to_ip_addr(ids),
        local_port: ids[4] as u16,
    }
}

/// `compareUdpRow`, bug-compatible: the C code is `DWORD_cmp(...) || port difference`, so the
/// result is only ever 0 or 1 and never negative. It is not a total order.
pub fn compare_udp_row(a: &UdpRow, b: &UdpRow) -> i32 {
    i32::from(
        compare_ip_addr_row(&a.local_addr, &b.local_addr) != 0 || a.local_port != b.local_port,
    )
}
