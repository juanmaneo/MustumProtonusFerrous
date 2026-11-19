//! `AVIFILE_CLSIDFromString` (`api.c`), copied from `ole32`'s `CLSIDFromString`. It parses
//! `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` into the 16 bytes a `CLSID` takes in memory.

use crate::{HResult, CO_E_CLASSSTRING};

/// A `CLSID` as its 16 bytes in memory: `Data1`, `Data2` and `Data3` little-endian, then `Data4`.
pub type Clsid = [u8; 16];

/// Length of the braced form, `{` plus 36 characters plus `}`.
const STRING_LEN: usize = 38;

/// Indexes of the four dashes in the braced form.
const DASHES: [usize; 4] = [9, 14, 19, 24];

/// Parses a CLSID string. `None` is a NULL pointer and gives the all-zero CLSID. The string ends
/// at its first NUL byte, as `lstrlenA` measures it.
pub fn clsid_from_str(idstr: Option<&[u8]>) -> Result<Clsid, HResult> {
    let Some(idstr) = idstr else {
        return Ok([0; 16]);
    };
    let len = idstr.iter().position(|&b| b == 0).unwrap_or(idstr.len());
    let s = &idstr[..len];
    if s.len() != STRING_LEN || !is_well_formed(s) {
        return Err(CO_E_CLASSSTRING);
    }
    // Each entry is the byte written by the hex pair that starts at that index.
    Ok([
        // Data1: its eight digits are big-endian, so the bytes come out reversed.
        hex_pair(s, 7),
        hex_pair(s, 5),
        hex_pair(s, 3),
        hex_pair(s, 1),
        // Data2 and Data3, likewise reversed.
        hex_pair(s, 12),
        hex_pair(s, 10),
        hex_pair(s, 17),
        hex_pair(s, 15),
        // Data4 is stored in text order.
        hex_pair(s, 20),
        hex_pair(s, 22),
        hex_pair(s, 25),
        hex_pair(s, 27),
        hex_pair(s, 29),
        hex_pair(s, 31),
        hex_pair(s, 33),
        hex_pair(s, 35),
    ])
}

/// Checks the braces and dashes, and that every other character is a hex digit.
fn is_well_formed(s: &[u8]) -> bool {
    let shape = s[0] == b'{' && s[STRING_LEN - 1] == b'}' && DASHES.iter().all(|&i| s[i] == b'-');
    shape
        && (1..STRING_LEN - 1)
            .filter(|i| !DASHES.contains(i))
            .all(|i| hex_digit(s[i]).is_some())
}

fn hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// The byte spelled by the two hex digits at `at` and `at + 1`. `s` must pass `is_well_formed`.
fn hex_pair(s: &[u8], at: usize) -> u8 {
    let digit = |i: usize| hex_digit(s[i]).expect("checked by is_well_formed");
    (digit(at) << 4) | digit(at + 1)
}
