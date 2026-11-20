//! NUL-terminated UTF-16 helpers with the semantics of the `WCHAR` functions the C code calls.

/// Returns the units before the first NUL, as `wcslen` sees them.
pub(crate) fn before_nul(s: &[u16]) -> &[u16] {
    match s.iter().position(|&unit| unit == 0) {
        Some(end) => &s[..end],
        None => s,
    }
}

/// Folds ASCII capitals only. Wine's `wcsicmp` also folds non-ASCII letters, which this port does not model.
fn fold(unit: u16) -> u16 {
    if (b'A' as u16..=b'Z' as u16).contains(&unit) {
        unit + 32
    } else {
        unit
    }
}

/// Compares two unit sequences of equal length without regard to ASCII case, as `wcsicmp` does.
pub fn eq_ignore_case(a: &[u16], b: &[u16]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(&x, &y)| fold(x) == fold(y))
}
