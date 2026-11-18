//! UTF-16 helpers with the NUL-terminated view that the Wine wide string calls take.

/// The UTF-16 units of text, without a terminator.
pub fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// The units of units before the first NUL, or all of them when there is none.
pub fn wide_str(units: &[u16]) -> &[u16] {
    let end = units
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(units.len());
    &units[..end]
}

/// Compares two wide strings up to their first NUL, folding ASCII letters only.
pub fn fold_eq(a: &[u16], b: &[u16]) -> bool {
    let (a, b) = (wide_str(a), wide_str(b));
    a.len() == b.len() && a.iter().zip(b).all(|(&x, &y)| fold(x) == fold(y))
}

fn fold(unit: u16) -> u16 {
    if matches!(unit, 0x41..=0x5A) {
        unit + 0x20
    } else {
        unit
    }
}

/// Whether the wide string name, up to its first NUL, is exactly text.
pub fn wide_eq(name: &[u16], text: &str) -> bool {
    wide_str(name).iter().copied().eq(text.encode_utf16())
}

/// The little-endian bytes of text with its NUL terminator, as stored in a property value.
pub fn wide_bytes(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .chain([0])
        .flat_map(u16::to_le_bytes)
        .collect()
}
