//! The integer scanner of `istream::getint` in Wine's `msvcirt.c`.

use crate::{FLAGS_DEC, FLAGS_HEX, FLAGS_OCT, IOSTATE_EOFBIT, IOSTATE_FAILBIT, IOS_BASEFIELD};

/// The most characters `getint` extracts, a hard limit on Windows.
pub const MAX_TOKEN: usize = 15;

/// The outcome of one `getint` scan of a stream.
///
/// The scan reads `input[..consumed]`, which is the token that Wine copies into
/// its buffer. Wine's buffer then holds `consumed` characters and a NUL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scan {
    /// The radix Wine parses the token with: 0 (autodetect), 8, 10 or 16.
    pub num_base: u32,
    /// The state bits this scan sets: `IOSTATE_EOFBIT` and/or `IOSTATE_FAILBIT`.
    pub state: u32,
    /// The characters extracted from the stream, which the caller advances past.
    pub consumed: usize,
}

/// Scans an integer token from the start of `input`, which holds the rest of the
/// stream, and reports the radix, the state bits and the extracted length.
///
/// The input is treated as ending at its end, where Wine sees `EOF`. `flags` are the
/// stream's `ios` flags; the radix comes from its radix bits, and the scan autodetects
/// the radix when none is set. A token with no valid digit extracts nothing, so
/// `consumed` is 0 and the failbit is set.
pub fn scan_integer(input: &[u8], flags: u32) -> Scan {
    let mut num_base = if flags & FLAGS_DEC != 0 {
        10
    } else if flags & FLAGS_HEX != 0 {
        16
    } else if flags & FLAGS_OCT != 0 {
        8
    } else {
        0
    };
    let mut scan_sign = true;
    let mut scan_prefix = true;
    let mut scan_x = false;
    let mut valid_integer = false;
    let mut state = 0;
    let mut consumed = 0;

    while consumed < MAX_TOKEN {
        let ch = input.get(consumed).copied();
        if matches!(ch, Some(b'+' | b'-')) && scan_sign {
            scan_sign = false;
        } else if matches!(ch, Some(b'x' | b'X')) && scan_x {
            scan_x = false;
            valid_integer = false;
            num_base = 16;
        } else if ch == Some(b'0') && scan_prefix {
            scan_sign = false;
            scan_prefix = false;
            scan_x = num_base == 0 || num_base == 16;
            valid_integer = true;
            if num_base == 0 {
                num_base = 8;
            }
        } else if is_valid_digit(ch, num_base) {
            scan_sign = false;
            scan_prefix = false;
            scan_x = false;
            valid_integer = true;
        } else {
            if !valid_integer {
                return Scan {
                    num_base,
                    state: IOSTATE_FAILBIT,
                    consumed: 0,
                };
            }
            if ch.is_none() {
                state |= IOSTATE_EOFBIT;
                if scan_x && flags & IOS_BASEFIELD == 0 {
                    num_base = 0;
                }
            }
            break;
        }
        consumed += 1;
    }

    Scan {
        num_base,
        state,
        consumed,
    }
}

/// `getint_is_valid_digit`: whether `ch` is a digit of `base`. `None` is `EOF`.
fn is_valid_digit(ch: Option<u8>, base: u32) -> bool {
    match (ch, base) {
        (None, _) => false,
        (Some(c), 8) => (b'0'..=b'7').contains(&c),
        (Some(c), 16) => c.is_ascii_hexdigit(),
        (Some(c), _) => c.is_ascii_digit(),
    }
}
