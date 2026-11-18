//! The `DPSP_MSG_HEADER` layout constants and the `mixed` word, from `dpwsockx_dll.h`.
//!
//! `mixed` packs the message size into the low 20 bits and a 12-bit token above them.

/// Size of `DPSP_MSG_HEADER` in bytes: the `mixed` word and a 16-byte `SOCKADDR_IN`.
pub const HEADER_SIZE: usize = 20;

/// `AF_INET`, the family every header carries.
pub const AF_INET: u16 = 2;

/// `DPSP_MSG_TOKEN_REMOTE`, the token on headers this service provider builds.
pub const TOKEN_REMOTE: u32 = 0xFAB0_0000;

const SIZE_MASK: u32 = 0x000F_FFFF;
const TOKEN_MASK: u32 = 0xFFF0_0000;

/// `DPSP_MSG_MAKE_MIXED`: a plain OR, as in Wine. A `size` of 2^20 or more sets token bits.
pub fn make_mixed(size: u32, token: u32) -> u32 {
    size | token
}

/// `DPSP_MSG_SIZE`: the message size in bytes.
pub fn msg_size(mixed: u32) -> u32 {
    mixed & SIZE_MASK
}

/// `DPSP_MSG_TOKEN`: the token bits, still in place.
pub fn msg_token(mixed: u32) -> u32 {
    mixed & TOKEN_MASK
}
