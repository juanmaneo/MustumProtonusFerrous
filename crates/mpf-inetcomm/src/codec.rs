//! The body decoders of `mimeole.c`: `decode_base64` and `decode_qp`. Both read a whole body at
//! once. The C code reads it in 1024-byte chunks, but the state carries over between chunks, so
//! output is the same.

pub use crate::decode_base64;

enum QpState {
    Literal,
    Escape,
    Hex(u8),
}

/// `decode_qp`: quoted-printable. `=` starts an escape, which takes two units. `=` followed by a
/// CRLF is a soft line break and writes nothing. An escape whose two units are not both hex digits
/// writes `=`, and drops both units. A partial escape at the end writes nothing.
pub fn decode_quoted_printable(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    let mut state = QpState::Literal;
    for &c in input {
        state = match state {
            QpState::Literal if c == b'=' => QpState::Escape,
            QpState::Literal => {
                out.push(c);
                QpState::Literal
            }
            QpState::Escape => QpState::Hex(c),
            QpState::Hex(prev) => {
                if (prev, c) != (b'\r', b'\n') {
                    match (hex_digit(prev), hex_digit(c)) {
                        (Some(hi), Some(lo)) => out.push((hi << 4) | lo),
                        _ => out.push(b'='),
                    }
                }
                QpState::Literal
            }
        };
    }
    out
}

fn hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 10),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}
