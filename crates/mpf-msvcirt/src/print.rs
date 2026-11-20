//! `ostream::internal_print_integer` in Wine's `msvcirt.c`: the `printf` conversion
//! and the radix or sign prefix of an integer insertion.

use crate::pad::writepad;
use crate::{FLAGS_HEX, FLAGS_OCT, FLAGS_SHOWBASE, FLAGS_SHOWPOS, FLAGS_UPPERCASE};

/// Formats `n` as the `printf` conversion that Wine picks for the stream flags.
///
/// Returns the prefix and the digits, which the padding places around each other.
/// Hex and octal print the bit pattern unsigned. Decimal prints it unsigned when
/// `unsig` is set and signed otherwise. `shrt` narrows the value to 16 bits first.
/// A `+` prefix is written for a non-zero value under `showpos`, and it applies only
/// to decimal output.
pub fn format_integer(n: i32, flags: u32, unsig: bool, shrt: bool) -> (Vec<u8>, Vec<u8>) {
    let upper = (flags & FLAGS_UPPERCASE) != 0;
    let showbase = (flags & FLAGS_SHOWBASE) != 0;
    let mut prefix = Vec::new();
    let digits = if (flags & FLAGS_HEX) != 0 {
        if showbase {
            prefix.push(b'0');
            prefix.push(if upper { b'X' } else { b'x' });
        }
        let bits = unsigned_bits(n, shrt);
        if upper {
            format!("{bits:X}")
        } else {
            format!("{bits:x}")
        }
    } else if (flags & FLAGS_OCT) != 0 {
        if showbase {
            prefix.push(b'0');
        }
        format!("{:o}", unsigned_bits(n, shrt))
    } else {
        if (flags & FLAGS_SHOWPOS) != 0 && n != 0 && (unsig || n > 0) {
            prefix.push(b'+');
        }
        if unsig {
            unsigned_bits(n, shrt).to_string()
        } else if shrt {
            (n as i16).to_string()
        } else {
            n.to_string()
        }
    };
    (prefix, digits.into_bytes())
}

/// Formats `n` as [`format_integer`] does, then lays the prefix and digits out in a
/// field of `width` characters.
pub fn print_integer(n: i32, flags: u32, unsig: bool, shrt: bool, width: i32, fill: u8) -> Vec<u8> {
    let (prefix, digits) = format_integer(n, flags, unsig, shrt);
    writepad(flags, width, fill, &prefix, &digits)
}

/// The bit pattern of `n` that the unsigned conversions print: 16 bits with `shrt`,
/// otherwise 32 bits.
fn unsigned_bits(n: i32, shrt: bool) -> u32 {
    if shrt {
        u32::from(n as u16)
    } else {
        n as u32
    }
}
