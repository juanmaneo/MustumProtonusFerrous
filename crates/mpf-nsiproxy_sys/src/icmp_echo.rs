/// `IPPROTO_ICMPV6` from `include/ws2def.h`.
pub const IPPROTO_ICMPV6: u8 = 58;

/// `IP_STATUS_BASE` from `include/ipexport.h`.
pub const IP_STATUS_BASE: u32 = 11000;

/// `IP_DEST_HOST_UNREACHABLE` from `include/ipexport.h`.
pub const IP_DEST_HOST_UNREACHABLE: u32 = IP_STATUS_BASE + 3;

/// `IP_GENERAL_FAILURE` from `include/ipexport.h`.
pub const IP_GENERAL_FAILURE: u32 = IP_STATUS_BASE + 50;

/// `EHOSTUNREACH` from the host `errno.h`. The C code compares against the host value, so this is
/// host-dependent; 113 is the Linux value.
pub const EHOSTUNREACH: i32 = 113;

/// `chksum` from `dlls/nsiproxy.sys/icmp_echo.c`. This is the RFC 1071 Internet checksum over
/// `data`, read as little-endian 16-bit words, with an odd final byte added as is. A ping socket
/// computes no checksum and returns 0.
pub fn chksum(ping_socket: bool, data: &[u8]) -> u16 {
    if ping_socket {
        return 0;
    }
    let mut sum: u32 = 0;
    let mut carry: u32 = 0;
    let (words, remainder) = data.as_chunks::<2>();
    for &[low, high] in words {
        let s = u32::from(u16::from_le_bytes([low, high]));
        sum = sum.wrapping_add(carry).wrapping_add(s);
        carry = u32::from(s > sum);
    }
    sum = sum.wrapping_add(carry);
    sum = (sum & 0xffff) + (sum >> 16);
    if let Some(&byte) = remainder.first() {
        sum += u32::from(byte);
    }
    sum = (sum & 0xffff) + (sum >> 16);
    sum = (sum & 0xffff) + (sum >> 16);
    !(sum as u16)
}

/// `ipv6_chksum` from `dlls/nsiproxy.sys/icmp_echo.c`, with the source address passed in. The C
/// code finds the source with `getsockname` on a probe socket, which is not ported. The checksum
/// covers the IPv6 pseudo-header (source, destination, 32-bit big-endian length, three zero bytes
/// and next header 58) followed by `message`. A ping socket computes no checksum and returns 0.
pub fn ipv6_chksum(ping_socket: bool, src: &[u8; 16], dst: &[u8; 16], message: &[u8]) -> u16 {
    if ping_socket {
        return 0;
    }
    let mut pseudo_header = Vec::with_capacity(40 + message.len());
    pseudo_header.extend_from_slice(src);
    pseudo_header.extend_from_slice(dst);
    pseudo_header.extend_from_slice(&(message.len() as u32).to_be_bytes());
    pseudo_header.extend_from_slice(&[0, 0, 0, IPPROTO_ICMPV6]);
    pseudo_header.extend_from_slice(message);
    chksum(false, &pseudo_header)
}

/// `errno_to_ip_status` from `dlls/nsiproxy.sys/icmp_echo.c`.
pub fn errno_to_ip_status(err: i32) -> u32 {
    match err {
        EHOSTUNREACH => IP_DEST_HOST_UNREACHABLE,
        _ => IP_GENERAL_FAILURE,
    }
}
