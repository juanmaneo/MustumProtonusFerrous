//! NetBIOS name encoding and the name query packet from Wine's `nbt.c`.
//!
//! The name and scope are byte strings. The C code keeps a NetBIOS name in 16 units and a scope
//! in 255 bytes, and both stop at the first NUL.

const NCBNAMSZ: usize = 16;
const MAX_SCOPE_LEN: usize = 255;
const NBNS_CLASS_INTERNET: u16 = 1;

/// The smallest buffer that `name_request` accepts: a 16-byte name and a 255-byte scope.
pub const SIMPLE_NAME_QUERY_PKT_SIZE: usize = 16 + 255;

/// The bytes before the first NUL, and at most `max` of them.
fn c_bytes(s: &[u8], max: usize) -> &[u8] {
    let s = &s[..s.len().min(max)];
    &s[..s.iter().position(|&b| b == 0).unwrap_or(s.len())]
}

/// `NetBTNameEncode`: the length byte, the 32 half-ASCII name characters, the scope, and a
/// terminating NUL. A short name is padded with `CA`, which encodes a space.
pub fn encode_name(name: &[u8], scope: &[u8]) -> Vec<u8> {
    let name = c_bytes(name, NCBNAMSZ);
    let mut out = vec![(2 * NCBNAMSZ) as u8];
    for &byte in name {
        out.push((byte >> 4) + b'A');
        out.push((byte & 0x0f) + b'A');
    }
    for _ in name.len()..NCBNAMSZ {
        out.extend_from_slice(b"CA");
    }
    out.extend_from_slice(c_bytes(scope, MAX_SCOPE_LEN));
    out.push(0);
    out
}

/// `NetBTNameReq`: a name query packet for `name` with transaction id `xid`, question type
/// `qtype`, and the broadcast flag. Returns `None` when `capacity` is below
/// [`SIMPLE_NAME_QUERY_PKT_SIZE`], or when the packet does not fit in `capacity`.
pub fn name_request(
    name: &[u8],
    xid: u16,
    qtype: u16,
    broadcast: bool,
    scope: &[u8],
    capacity: usize,
) -> Option<Vec<u8>> {
    if capacity < SIMPLE_NAME_QUERY_PKT_SIZE {
        return None;
    }
    let flags: u16 = if broadcast { 0x0110 } else { 0x0100 };
    let mut out = Vec::new();
    for word in [xid, flags, 1, 0, 0, 0] {
        out.extend_from_slice(&word.to_be_bytes());
    }
    out.extend(encode_name(name, scope));
    out.extend_from_slice(&qtype.to_be_bytes());
    out.extend_from_slice(&NBNS_CLASS_INTERNET.to_be_bytes());
    (out.len() <= capacity).then_some(out)
}
