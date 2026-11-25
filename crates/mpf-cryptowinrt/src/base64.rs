//! `encodeBase64W` from Wine's `dlls/crypt32/base64.c`, for the
//! `CRYPT_STRING_BASE64 | CRYPT_STRING_NOCRLF` form that `EncodeToBase64String` uses.
//! The separator is empty, so the output never contains line breaks.

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Standard base64 with `=` padding. Empty input gives an empty string.
pub fn encode_base64(data: &[u8]) -> String {
    let mut out = String::new();
    let (chunks, remainder) = data.as_chunks::<3>();
    for &[a, b, c] in chunks {
        out.extend([
            sextet(a >> 2),
            sextet(((a << 4) & 0x30) | (b >> 4)),
            sextet(((b << 2) & 0x3c) | (c >> 6)),
            sextet(c & 0x3f),
        ]);
    }
    match *remainder {
        [a] => out.extend([sextet(a >> 2), sextet((a << 4) & 0x30), '=', '=']),
        [a, b] => out.extend([
            sextet(a >> 2),
            sextet(((a << 4) & 0x30) | (b >> 4)),
            sextet((b << 2) & 0x3c),
            '=',
        ]),
        _ => {}
    }
    out
}

fn sextet(value: u8) -> char {
    char::from(ALPHABET[usize::from(value)])
}
