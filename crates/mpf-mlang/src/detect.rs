//! Port of `DetectJapaneseCode`: guesses the Japanese codepage of a byte string from its escape
//! sequences and byte pairs.

use crate::classes::{is_euc, is_hankata, is_not_euc_pair};

/// Codepage guessed for `input`: `50220` for an ISO-2022-JP escape, `51932` for EUC-JP, `932`
/// for Shift-JIS, and `0` when nothing decides it. Scanning stops at the first decisive
/// sequence other than EUC-JP, and a truncated sequence at the end returns the code found so far.
pub fn japanese_code(input: &[u8]) -> u32 {
    let mut code = 0;
    let mut i = 0;
    while (code == 0 || code == 51932) && i < input.len() {
        let mut c1 = input[i];
        if c1 == 0x1b {
            i += 1;
            if i >= input.len() {
                return code;
            }
            c1 = input[i];
            if c1 == b'$' {
                i += 1;
                if i >= input.len() {
                    return code;
                }
                c1 = input[i];
                if c1 == b'B' || c1 == b'@' {
                    code = 50220;
                }
            }
            if c1 == b'K' {
                code = 50220;
            }
        } else if c1 >= 129 {
            i += 1;
            if i >= input.len() {
                return code;
            }
            let c2 = input[i];
            if is_not_euc_pair(c1, c2) {
                code = 932;
            } else if (is_euc(c1) && is_euc(c2)) || (c1 == 142 && is_hankata(c2)) {
                code = 51932;
            }
        }
        i += 1;
    }
    code
}
