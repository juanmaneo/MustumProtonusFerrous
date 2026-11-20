//! The body decoders of `mimeole.c`: `decode_base64` and `decode_qp`. Both read a whole body at
//! once. The C code reads it in 1024-byte chunks, but the state carries over between chunks, so
//! the output is the same.

/// `decode_base64`: decodes four units into three bytes. Units outside the base64 alphabet are
/// skipped, `=` included. A partial group at the end is dropped.
pub fn decode_base64(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() / 4 * 3);
    let mut group = [0u8; 4];
    let mut n = 0;
    for &c in input {
        let Some(value) = base64_value(c) else {
            continue;
        };
        group[n] = value;
        n += 1;
        match n {
            2 => out.push((group[0] << 2) | (group[1] >> 4)),
            3 => out.push((group[1] << 4) | (group[2] >> 2)),
            4 => {
                out.push((group[2] << 6) | group[3]);
                n = 0;
            }
            _ => {}
        }
    }
    out
}

fn base64_value(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

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
