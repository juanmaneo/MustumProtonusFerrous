/// `mask_v4_to_prefix` from `dlls/nsiproxy.sys/ip.c`. The prefix length of an IPv4 netmask is the
/// number of set bits in `s_addr`; the C `nsi_popcount` is `u32::count_ones`.
pub fn mask_v4_to_prefix(s_addr: u32) -> u32 {
    s_addr.count_ones()
}

/// `mask_v6_to_prefix` from `dlls/nsiproxy.sys/ip.c`. The prefix length of an IPv6 netmask is the
/// total number of set bits in its 16 bytes.
pub fn mask_v6_to_prefix(s6_addr: &[u8; 16]) -> u32 {
    s6_addr.iter().map(|byte| byte.count_ones()).sum()
}

/// `str_to_in6_addr` from `dlls/nsiproxy.sys/ip.c`. The input is 32 hex digits with no separators,
/// read two digits per byte. The slice stands in for the C string, so its end plays the NUL.
///
/// Returns the address and the offset where parsing stopped (the C `endptr`). On an invalid pair
/// the address holds the bytes before that pair. The C code leaves the rest of the result
/// uninitialised; the port returns those bytes as zero.
pub fn str_to_in6_addr(input: &[u8]) -> ([u8; 16], usize) {
    let mut addr = [0u8; 16];
    for pos in (0..32).step_by(2) {
        match (hex_digit_at(input, pos), hex_digit_at(input, pos + 1)) {
            (Some(high), Some(low)) => addr[pos / 2] = (high << 4) | low,
            _ => return (addr, pos),
        }
    }
    (addr, 32)
}

fn hex_digit_at(input: &[u8], pos: usize) -> Option<u8> {
    let digit = (*input.get(pos)? as char).to_digit(16)?;
    Some(digit as u8)
}
