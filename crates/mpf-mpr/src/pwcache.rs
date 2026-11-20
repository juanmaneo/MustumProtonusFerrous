//! The value-name codec of Wine's `dlls/mpr/pwcache.c`.
//!
//! The password cache stores each entry as a registry value named `X-TT-` followed
//! by the resource bytes in upper-case hex, where `TT` is the entry type. The
//! registry calls are not ported; the name codec and the filter that
//! `WNetEnumCachedPasswords` applies to each value name are.

/// `hex` in `pwcache.c`: the upper-case hex digit for the low four bits of `nibble`.
///
/// Only the low four bits are used. Wine only passes nibbles, so the mask
/// changes nothing for valid input.
pub fn hex_digit(nibble: u8) -> u8 {
    let n = nibble & 0x0f;
    if n <= 9 {
        n + b'0'
    } else {
        n + b'A' - 10
    }
}

/// `ctox` in `pwcache.c`: the value of an ASCII hex digit, or `None` where Wine
/// returns -1.
pub fn ctox(ch: u8) -> Option<u8> {
    match ch {
        b'0'..=b'9' => Some(ch - b'0'),
        b'A'..=b'F' => Some(ch - b'A' + 10),
        b'a'..=b'f' => Some(ch - b'a' + 10),
        _ => None,
    }
}

/// The five-byte prefix `X-TT-` that every value name of entry type `ntype` starts with.
pub fn value_prefix(ntype: u8) -> String {
    format!("X-{:02X}-", ntype)
}

/// `MPR_GetValueName` in `pwcache.c`: the registry value name for `resource` and `ntype`.
///
/// The name has no terminating NUL, which the Wine version adds in its
/// `malloc`'d buffer.
pub fn value_name(resource: &[u8], ntype: u8) -> String {
    let mut name = value_prefix(ntype);
    for &byte in resource {
        name.push(char::from(hex_digit(byte >> 4)));
        name.push(char::from(hex_digit(byte)));
    }
    name
}

/// Decodes a value name the way `WNetEnumCachedPasswords` does.
///
/// Returns `None` if the name is shorter than six bytes or does not start with
/// [`value_prefix`] for `ntype`. Otherwise decoding reads hex pairs after the
/// prefix and stops at the first pair that is not two hex digits. A trailing
/// odd digit is ignored.
pub fn decode_value_name(name: &[u8], ntype: u8) -> Option<Vec<u8>> {
    if name.len() < 6 || !name.starts_with(value_prefix(ntype).as_bytes()) {
        return None;
    }
    Some(
        name[5..]
            .as_chunks::<2>()
            .0
            .iter()
            .map_while(|pair| super::decode_hex_pair(pair[0], pair[1]))
            .collect(),
    )
}

/// The resource bytes of a cache entry that `WNetEnumCachedPasswords` reports for
/// `prefix`: [`decode_value_name`] followed by the check that the resource starts with `prefix`.
pub fn select_resource(name: &[u8], ntype: u8, prefix: &[u8]) -> Option<Vec<u8>> {
    decode_value_name(name, ntype).filter(|resource| resource.starts_with(prefix))
}
