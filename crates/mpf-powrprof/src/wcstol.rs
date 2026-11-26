/// Parses a NUL-terminated UTF-16LE string the way `wcstol(s, NULL, 10)` does
/// with a 32-bit `long`: leading ASCII whitespace, an optional sign, then ASCII
/// digits up to the first other unit. No digits gives 0, and out-of-range
/// values saturate to `i32::MIN` or `i32::MAX`.
///
/// Parsing stops at the first NUL or at the end of `data`, and an odd trailing
/// byte is ignored. The C call reads until NUL, so a value without a NUL inside
/// its 40-byte buffer reads past it. This port stops at the slice end instead.
pub fn wcstol_decimal(data: &[u8]) -> i32 {
    let (pairs, _) = data.as_chunks::<2>();
    let mut units = pairs
        .iter()
        .map(|&pair| u16::from_le_bytes(pair))
        .take_while(|&unit| unit != 0)
        .peekable();
    while units.next_if(|&unit| is_c_space(unit)).is_some() {}
    let negative = match units.peek() {
        Some(&unit) if unit == u16::from(b'-') => {
            units.next();
            true
        }
        Some(&unit) if unit == u16::from(b'+') => {
            units.next();
            false
        }
        _ => false,
    };
    let mut magnitude: i64 = 0;
    for unit in units.take_while(|&unit| is_decimal_digit(unit)) {
        magnitude = (magnitude * 10 + i64::from(unit - u16::from(b'0'))).min(MAGNITUDE_CAP);
    }
    let value = if negative { -magnitude } else { magnitude };
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

/// Caps the accumulator so it cannot overflow. Anything above `i32::MAX`
/// saturates anyway, so the cap does not change the result.
const MAGNITUDE_CAP: i64 = 1 << 32;

fn is_c_space(unit: u16) -> bool {
    matches!(unit, 0x09..=0x0d | 0x20)
}

fn is_decimal_digit(unit: u16) -> bool {
    (u16::from(b'0')..=u16::from(b'9')).contains(&unit)
}
