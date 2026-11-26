/// One entry of the table that `get_ipv6_addr_scope_table` builds: `struct ipv6_addr_scope` from
/// `dlls/nsiproxy.sys/unix_private.h`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ipv6AddrScope {
    pub addr: [u8; 16],
    pub scope: u32,
}

/// `find_ipv6_addr_scope` from `dlls/nsiproxy.sys/tcp.c`. The C `-1` is `u32::MAX`.
///
/// Unspecified addresses have scope 0 and are checked before anything else. A multicast address
/// (first byte `0xff`) takes its scope from the low nibble of the second byte, and the C code
/// returns that value through `htons`, so the result is in network byte order: on a little-endian
/// host a scope of 2 comes back as `0x0200`. The table is searched only for other addresses. A
/// `None` table (NULL in C) gives `u32::MAX`, as does an address that is not in the table.
pub fn find_ipv6_addr_scope(addr: &[u8; 16], table: Option<&[Ipv6AddrScope]>) -> u32 {
    const MULTICAST_SCOPE_MASK: u8 = 0x0f;

    if addr.iter().all(|&byte| byte == 0) {
        return 0;
    }
    if addr[0] == 0xff {
        return u32::from(u16::from(addr[1] & MULTICAST_SCOPE_MASK).to_be());
    }
    match table {
        None => u32::MAX,
        Some(table) => table
            .iter()
            .find(|entry| entry.addr == *addr)
            .map_or(u32::MAX, |entry| entry.scope),
    }
}
