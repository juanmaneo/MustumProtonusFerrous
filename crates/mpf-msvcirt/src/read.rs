//! `istream::internal_read_integer` and `internal_read_unsigned_integer` in Wine's
//! `msvcirt.c`: parse the scanned integer, then clamp it to the target range.

use crate::scan::scan_integer;
use crate::IOSTATE_FAILBIT;

/// The outcome of reading one integer from a stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Read<T> {
    /// The value to store in the target, after clamping.
    pub value: T,
    /// The state bits this read sets, which the caller ORs into the stream state.
    pub state: u32,
    /// The characters extracted from the stream, which the caller advances past.
    pub consumed: usize,
}

/// Reads a signed integer of the range `min..=max` from the start of `input`.
///
/// `set_flag` selects the overflow rule of `read_long`: with it, a token that
/// overflows the 32-bit `long` sets the failbit and keeps `strtol`'s saturated value.
/// Without it, the value is clamped like any other out-of-range value.
pub fn read_integer(input: &[u8], flags: u32, min: i32, max: i32, set_flag: bool) -> Read<i32> {
    let scan = scan_integer(input, flags);
    let (mut value, erange) = strtol(&input[..scan.consumed], scan.num_base);
    let mut state = scan.state;
    if set_flag && erange {
        state |= IOSTATE_FAILBIT;
    } else if value > max {
        state |= IOSTATE_FAILBIT;
        value = max;
    } else if value < min {
        state |= IOSTATE_FAILBIT;
        value = min;
    }
    Read {
        value,
        state,
        consumed: scan.consumed,
    }
}

/// Reads an unsigned integer from the start of `input`, clamping it to `max`.
///
/// `min` is the signed bound that Wine passes as `LONG` and casts to `ULONG`, so a
/// negative `min` becomes a large unsigned bound. Wine's check then clamps only the
/// values between `max` and that bound. A value outside `0..=max` that is not clamped
/// is returned as read.
pub fn read_unsigned_integer(input: &[u8], flags: u32, min: i32, max: u32) -> Read<u32> {
    let scan = scan_integer(input, flags);
    let (mut value, erange) = strtoul(&input[..scan.consumed], scan.num_base);
    let mut state = scan.state;
    if (value == u32::MAX && erange) || (value > max && value < min as u32) {
        state |= IOSTATE_FAILBIT;
        value = max;
    }
    Read {
        value,
        state,
        consumed: scan.consumed,
    }
}

/// `strtol` on a scanner token, for a 32-bit `long`. Returns the value and whether
/// it overflowed (`ERANGE`), in which case the value saturates.
fn strtol(token: &[u8], base: u32) -> (i32, bool) {
    let (negative, magnitude) = magnitude(token, base);
    let limit = if negative {
        1u64 << 31
    } else {
        (1u64 << 31) - 1
    };
    if magnitude > limit {
        let saturated = if negative { i32::MIN } else { i32::MAX };
        return (saturated, true);
    }
    let value = if negative {
        (-(magnitude as i64)) as i32
    } else {
        magnitude as i32
    };
    (value, false)
}

/// `strtoul` on a scanner token, for a 32-bit `unsigned long`. A negative token
/// wraps, and an overflow returns `ULONG_MAX` with `ERANGE` whatever the sign.
fn strtoul(token: &[u8], base: u32) -> (u32, bool) {
    let (negative, magnitude) = magnitude(token, base);
    if magnitude > u64::from(u32::MAX) {
        return (u32::MAX, true);
    }
    let value = if negative {
        (magnitude as u32).wrapping_neg()
    } else {
        magnitude as u32
    };
    (value, false)
}

/// Splits off the sign and any radix prefix from `token`, then accumulates the digits.
///
/// The scanner has already picked the radix. A leading `0` makes it octal and a `0x`
/// makes it hex, so a base of 0 only meets a decimal token. Scanner tokens have no
/// leading white space, so none is skipped.
fn magnitude(token: &[u8], base: u32) -> (bool, u64) {
    let (negative, rest) = match token.first() {
        Some(b'-') => (true, &token[1..]),
        Some(b'+') => (false, &token[1..]),
        _ => (false, token),
    };
    let hex_prefix = matches!(rest, [b'0', b'x' | b'X', ..]);
    let (base, digits) = match base {
        16 if hex_prefix => (16, &rest[2..]),
        0 => (10, rest),
        _ => (base, rest),
    };
    let value = digits
        .iter()
        .map_while(|&c| char::from(c).to_digit(base))
        .fold(0u64, |value, d| {
            value
                .saturating_mul(u64::from(base))
                .saturating_add(u64::from(d))
        });
    (negative, value)
}
