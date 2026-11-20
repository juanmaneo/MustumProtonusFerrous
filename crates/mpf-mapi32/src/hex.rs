//! Hex conversions from `dlls/mapi32/util.c`: `FBinFromHex`, `HexFromBin` and `UlFromSzHex`.

use crate::c_str;

/// The `digitsToHex` table of `util.c`: the value of an ASCII hex digit, or `None`.
fn hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 10),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

/// `FBinFromHex`. Decodes pairs of hex digits. Returns `None` for an odd length or for any
/// character that is not a hex digit.
pub fn bin_from_hex(text: &[u8]) -> Option<Vec<u8>> {
    c_str(text)
        .chunks(2)
        .map(|pair| match pair {
            [hi, lo] => Some((hex_digit(*hi)? << 4) | hex_digit(*lo)?),
            _ => None,
        })
        .collect()
}

/// `HexFromBin`. Writes two uppercase digits per byte. The C code also writes a NUL terminator,
/// which the `String` does not carry.
pub fn hex_from_bin(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789ABCDEF";
    bytes
        .iter()
        .flat_map(|&b| [DIGITS[usize::from(b >> 4)], DIGITS[usize::from(b & 0xf)]])
        .map(char::from)
        .collect()
}

/// `UlFromSzHex`. Reads pairs of hex digits and stops at the first pair that is not valid. Each
/// pair updates the result as `result * 16 + pair`, as the C code does, so `"1f2e"` gives
/// `31 * 16 + 0x2e`. The arithmetic wraps at 32 bits, like the C `ULONG`.
pub fn ul_from_sz_hex(text: &[u8]) -> u32 {
    let mut ret: u32 = 0;
    for pair in c_str(text).as_chunks::<2>().0 {
        let (Some(hi), Some(lo)) = (hex_digit(pair[0]), hex_digit(pair[1])) else {
            break;
        };
        ret = ret.wrapping_mul(16).wrapping_add(u32::from((hi << 4) | lo));
    }
    ret
}
