//! Base64 as in Wine's `encode_base64` and `decode_base64` (`main.c`), with the standard
//! alphabet and `=` padding.

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// `encode_base64`: each group of up to three bytes becomes four characters, padded with `=`.
pub fn encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for group in data.chunks(3) {
        let b0 = group[0];
        let b1 = group.get(1).copied().unwrap_or(0);
        let b2 = group.get(2).copied().unwrap_or(0);
        let sextets = [b0 >> 2, (b0 << 4) | (b1 >> 4), (b1 << 2) | (b2 >> 6), b2];
        for (i, sextet) in sextets.into_iter().enumerate() {
            if i > group.len() {
                out.push('=');
            } else {
                out.push(char::from(ALPHABET[usize::from(sextet & 0x3f)]));
            }
        }
    }
    out
}

/// `decode_base64`. Returns `None` when the length is not a multiple of four, when a character
/// is outside the alphabet, or when `=` appears before the final quartet. In the final quartet,
/// `=` in the third position ends the data after one byte, and `=` in the fourth ends it after
/// two; the characters after the padding are not checked. Empty input decodes to nothing.
pub fn decode(text: &[u8]) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(4) {
        return None;
    }
    let Some((body, last)) = text.split_last_chunk::<4>() else {
        return Some(Vec::new());
    };
    let used = match last[2] {
        b'=' => 2,
        _ if last[3] == b'=' => 3,
        _ => 4,
    };
    let sextets = body
        .iter()
        .chain(&last[..used])
        .map(|&c| sextet(c))
        .collect::<Option<Vec<u8>>>()?;

    let mut out = Vec::new();
    for group in sextets.chunks(4) {
        out.push((group[0] << 2) | (group[1] >> 4));
        if group.len() > 2 {
            out.push((group[1] << 4) | (group[2] >> 2));
        }
        if group.len() > 3 {
            out.push((group[2] << 6) | group[3]);
        }
    }
    Some(out)
}

fn sextet(c: u8) -> Option<u8> {
    ALPHABET.iter().position(|&a| a == c).map(|i| i as u8)
}
