//! Address helpers: the IPv4 mask, the loopback, link-local and unspecified
//! predicates, prefix matching and the IPv4-mapped IPv6 conversion.

use crate::Error;

/// `INADDR_NONE`, the value `ConvertLengthToIpv4Mask` stores when the length is over 32.
pub const INADDR_NONE: u32 = u32::MAX;

const LOOPBACK_V6: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

/// An IP address as its bytes in network order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpAddress {
    V4([u8; 4]),
    V6([u8; 16]),
}

/// An address prefix (`IP_ADDRESS_PREFIX`): the leading `length` bits of `address` count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefix {
    pub address: IpAddress,
    pub length: u8,
}

/// `ConvertLengthToIpv4Mask`: the mask of `mask_len` leading one bits, as the
/// `u32` Wine stores (`htonl` of the host mask).
pub fn length_to_ipv4_mask(mask_len: u32) -> Result<u32, Error> {
    match mask_len {
        0 => Ok(0),
        1..=32 => Ok((u32::MAX << (32 - mask_len)).to_be()),
        _ => Err(Error::InvalidParameter),
    }
}

/// `sockaddr_is_loopback`: `127.0.0.0/8` or `::1`.
pub fn is_loopback(addr: &IpAddress) -> bool {
    match addr {
        IpAddress::V4(a) => a[0] == 127,
        IpAddress::V6(a) => *a == LOOPBACK_V6,
    }
}

/// `sockaddr_is_linklocal`: `fe80::/10`. IPv4 has no link-local test here, so it is false.
pub fn is_linklocal(addr: &IpAddress) -> bool {
    match addr {
        IpAddress::V4(_) => false,
        IpAddress::V6(a) => a[0] == 0xfe && (a[1] & 0xc0) == 0x80,
    }
}

/// `is_addr_unspecified`: `0.0.0.0` or `::`.
pub fn is_unspecified(addr: &IpAddress) -> bool {
    match addr {
        IpAddress::V4(a) => *a == [0; 4],
        IpAddress::V6(a) => *a == [0; 16],
    }
}

/// `match_ip_address_with_prefix`: whether `addr` lies in `prefix`. Addresses of
/// different families never match. A zero length matches every address of the
/// family, and a length past the family's width (32 or 128 bits) matches nothing.
pub fn matches_prefix(addr: &IpAddress, prefix: &Prefix) -> bool {
    let (a, p, max_bits): (&[u8], &[u8], u8) = match (addr, &prefix.address) {
        (IpAddress::V4(a), IpAddress::V4(p)) => (&a[..], &p[..], 32),
        (IpAddress::V6(a), IpAddress::V6(p)) => (&a[..], &p[..], 128),
        _ => return false,
    };
    let len = prefix.length;
    if len == 0 {
        return true;
    }
    if len > max_bits {
        return false;
    }
    let whole = usize::from(len / 8);
    if a[..whole] != p[..whole] {
        return false;
    }
    let rem = len % 8;
    if rem == 0 {
        return true;
    }
    let mask = 0xffu8 << (8 - rem);
    (a[whole] & mask) == (p[whole] & mask)
}

/// `map_address_6to4`: the IPv4 address of an IPv4-mapped IPv6 address
/// (`::ffff:a.b.c.d`), in the byte order Wine produces.
///
/// Wine builds the IPv4 value as `Word[6] << 16 | Word[7]`. On a little-endian
/// host, which is the oracle and every Windows target, the result is
/// `[bytes 14, 15, 12, 13]` of the address. For `::ffff:1.2.3.4` that is
/// `3.4.1.2`. See the crate documentation.
pub fn map_ipv6_to_ipv4(addr: &[u8; 16]) -> Option<[u8; 4]> {
    let mapped = addr[..10].iter().all(|&b| b == 0) && addr[10..12] == [0xff, 0xff];
    if !mapped {
        return None;
    }
    Some([addr[14], addr[15], addr[12], addr[13]])
}
