//! `PROPVAR_ConvertNumber` of `propvar.c`, which every integer conversion goes through, and the
//! `strtoll` and `wcstol` parsing of text values that it uses.

use crate::{
    hex_value, unit, widen, HResult, PropValue, DISP_E_TYPEMISMATCH, ERROR_ARITHMETIC_OVERFLOW_HR,
    E_NOTIMPL, S_OK,
};

const TWO_POW_63: f64 = 9_223_372_036_854_775_808.0;

/// Converts `value` to a `dest_bits` integer, signed or not. The number is the one the C code
/// stores in its `LONGLONG`, and it is only meaningful when the result is `S_OK`.
pub(crate) fn convert_number(
    value: &PropValue,
    dest_bits: u32,
    dest_signed: bool,
) -> (HResult, i64) {
    let (res, src_signed) = match value {
        PropValue::I1(v) => (i64::from(*v), true),
        PropValue::Ui1(v) => (i64::from(*v), false),
        PropValue::I2(v) => (i64::from(*v), true),
        PropValue::Ui2(v) => (i64::from(*v), false),
        PropValue::I4(v) => (i64::from(*v), true),
        PropValue::Ui4(v) => (i64::from(*v), false),
        PropValue::I8(v) => (*v, true),
        PropValue::Ui8(v) => (*v as i64, false),
        PropValue::Empty => (0, false),
        PropValue::LpStr(text) => {
            let Some(res) = parse_integer(&widen(text.as_deref().unwrap_or_default()), 63) else {
                return (DISP_E_TYPEMISMATCH, 0);
            };
            (res, res < 0)
        }
        PropValue::LpWStr(text) | PropValue::Bstr(text) => {
            let Some(res) = parse_integer(text.as_deref().unwrap_or_default(), 31) else {
                return (DISP_E_TYPEMISMATCH, 0);
            };
            (res, res < 0)
        }
        PropValue::R8(v) => (trunc_to_i64(*v), true),
        _ => return (E_NOTIMPL, 0),
    };
    let overflow = (res < 0 && src_signed != dest_signed) || exceeds(res, dest_bits, dest_signed);
    (
        if overflow {
            ERROR_ARITHMETIC_OVERFLOW_HR
        } else {
            S_OK
        },
        res,
    )
}

fn exceeds(res: i64, bits: u32, signed: bool) -> bool {
    match (signed, bits) {
        (_, 64) => false,
        (true, _) => res >= (1_i64 << (bits - 1)) || res < (-1_i64 << (bits - 1)),
        (false, _) => (res as u64) >= (1_u64 << bits),
    }
}

/// A C `(LONGLONG)` cast of a double. Values outside `-2^63..2^63` and NaN are undefined in C,
/// and convert to `i64::MIN` here.
fn trunc_to_i64(value: f64) -> i64 {
    if (-TWO_POW_63..TWO_POW_63).contains(&value) {
        value.trunc() as i64
    } else {
        i64::MIN
    }
}

/// `strtoll` with base 0, or `wcstol` when `bits` is 31. `bits` is the magnitude width: the
/// text clamps to `2^bits - 1` in the positive direction and to `-2^bits` in the negative one.
fn parse_integer(text: &[u16], bits: u32) -> Option<i64> {
    let (negative, magnitude) = c_integer(text)?;
    let limit = (1_u64 << bits) - u64::from(!negative);
    let magnitude = magnitude.min(limit);
    Some(if negative {
        magnitude.wrapping_neg() as i64
    } else {
        magnitude as i64
    })
}

/// The digits that `strtol` reads with base 0, after the C locale's white space and a sign.
/// Returns `None` when there are no digits, which is when the C code reports no conversion.
fn c_integer(text: &[u16]) -> Option<(bool, u64)> {
    let mut at = 0;
    while matches!(unit(text, at), 0x09..=0x0D | 0x20) {
        at += 1;
    }
    let sign = unit(text, at);
    let negative = sign == u32::from(b'-');
    if negative || sign == u32::from(b'+') {
        at += 1;
    }
    let zero = unit(text, at) == u32::from(b'0');
    let hex_prefix = zero
        && matches!(unit(text, at + 1), 0x58 | 0x78)
        && hex_value(unit(text, at + 2)).is_some();
    let (base, prefix) = if hex_prefix {
        (16, 2)
    } else if zero {
        (8, 0)
    } else {
        (10, 0)
    };
    at += prefix;
    let mut magnitude = 0_u64;
    let mut digits = false;
    while let Some(digit) = hex_value(unit(text, at)) {
        if digit >= base {
            break;
        }
        magnitude = magnitude
            .saturating_mul(u64::from(base))
            .saturating_add(u64::from(digit));
        digits = true;
        at += 1;
    }
    digits.then_some((negative, magnitude))
}
