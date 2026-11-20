//! The integer parse of `SHRegGetIntW` in Wine's `dlls/shcore/main.c`.

use crate::str_len;

/// The units of the `WCHAR buff[32]` that `SHRegGetIntW` reads a value into. The value and its
/// terminator must both fit, so a value of 32 units or more falls back to the default.
const VALUE_BUFFER_UNITS: usize = 32;

fn digit_value(c: u16) -> Option<i32> {
    let digit = c.wrapping_sub(u16::from(b'0'));
    (digit < 10).then_some(i32::from(digit))
}

/// `SHRegGetIntW`, given the data of the value. `None` means the value is absent or unreadable.
///
/// The value is parsed only when it fits the buffer and starts with an ASCII digit. Parsing stops
/// at the first non-digit, and the result saturates at `i32::MAX`, as the 32-bit `wcstol` does.
pub fn reg_int_value(value: Option<&[u16]>, default: i32) -> i32 {
    let Some(data) = value else {
        return default;
    };
    let len = str_len(data);
    if len >= VALUE_BUFFER_UNITS {
        return default;
    }
    if data.first().copied().and_then(digit_value).is_none() {
        return default;
    }
    let mut result: i32 = 0;
    for &c in &data[..len] {
        let Some(digit) = digit_value(c) else {
            break;
        };
        result = result.saturating_mul(10).saturating_add(digit);
    }
    result
}
