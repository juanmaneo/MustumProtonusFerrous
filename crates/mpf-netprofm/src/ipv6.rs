//! IPv6 address checks from `include/ws2ipdef.h` and `map_address_6to4` from
//! `dlls/netprofm/list.c`. Addresses are the 16 bytes of `IN6_ADDR` in memory order.

use crate::{AF_INET, AF_INET6};

/// Scope of an `AF_INET6` unicast address, as `has_ipv6_address` classifies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipv6Scope {
    /// Link-local or site-local.
    Local,
    /// Neither local, nor unspecified, multicast or loopback.
    Global,
    /// Unspecified, multicast or loopback.
    Other,
}

/// The scope test in `has_ipv6_address`.
pub fn classify_ipv6(addr: &[u8; 16]) -> Ipv6Scope {
    if is_linklocal(addr) || is_sitelocal(addr) {
        Ipv6Scope::Local
    } else if is_unspecified(addr) || is_multicast(addr) || is_loopback(addr) {
        Ipv6Scope::Other
    } else {
        Ipv6Scope::Global
    }
}

fn is_linklocal(a: &[u8; 16]) -> bool {
    a[0] == 0xfe && (a[1] & 0xc0) == 0x80
}

fn is_sitelocal(a: &[u8; 16]) -> bool {
    a[0] == 0xfe && (a[1] & 0xc0) == 0xc0
}

fn is_multicast(a: &[u8; 16]) -> bool {
    a[0] == 0xff
}

fn is_unspecified(a: &[u8; 16]) -> bool {
    a.iter().all(|&b| b == 0)
}

/// `::1`. Wine checks word 7 == 0x0100, which in little-endian memory order is byte 14 == 0 and byte 15 == 1.
fn is_loopback(a: &[u8; 16]) -> bool {
    a[..15].iter().all(|&b| b == 0) && a[15] == 1
}

/// The `SOCKADDR_IN` fields that `map_address_6to4` writes.
#[derive(Debug, PartialEq, Eq)]
pub struct SockAddrIn {
    pub family: u16,
    pub port: u16,
    /// `sin_addr.S_un.S_addr`, computed as `Word[6] << 16 | Word[7]`.
    pub s_addr: u32,
}

/// `map_address_6to4`: converts an IPv4-mapped IPv6 address (`::ffff:a.b.c.d`).
///
/// Words are read little-endian, as on the x86 host the oracle ran on. The
/// combination `Word[6] << 16 | Word[7]` is the big-endian layout, so on a
/// little-endian host the result is word-swapped: `::ffff:1.2.3.4` yields
/// `3.4.1.2`. The port keeps that output so it matches Wine.
pub fn map_address_6to4(family: u16, port: u16, addr: &[u8; 16]) -> Option<SockAddrIn> {
    if family != AF_INET6 {
        return None;
    }
    let word = |i: usize| u16::from_le_bytes([addr[2 * i], addr[2 * i + 1]]);
    if (0..5).any(|i| word(i) != 0) || word(5) != 0xffff {
        return None;
    }
    Some(SockAddrIn {
        family: AF_INET,
        port,
        s_addr: (u32::from(word(6)) << 16) | u32::from(word(7)),
    })
}
