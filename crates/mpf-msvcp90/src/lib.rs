//! Deterministic character-class property lookup from Wine's `msvcp90/misc.c`.
//!
//! Only the pure `wctype` name-to-mask mapping is included. It is case-sensitive
//! and treats the first NUL byte as the end of the C string.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Returns the MSVCP90 class mask for a recognized C-locale property name.
///
/// The property is a byte string so callers need not provide UTF-8. Like Wine's
/// `wctype`, matching is exact and stops at the first NUL; unknown names return 0.
pub fn wctype(property: &[u8]) -> u16 {
    let end = property
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(property.len());

    match &property[..end] {
        b"alnum" => 0x107,
        b"alpha" => 0x103,
        b"cntrl" => 0x020,
        b"digit" => 0x004,
        b"graph" => 0x117,
        b"lower" => 0x002,
        b"print" => 0x157,
        b"punct" => 0x010,
        b"space" => 0x008,
        b"upper" => 0x001,
        b"xdigit" => 0x080,
        _ => 0,
    }
}
