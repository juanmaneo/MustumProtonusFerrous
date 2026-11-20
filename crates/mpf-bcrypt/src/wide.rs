//! UTF-16 helpers for the wide strings that the bcrypt API takes.

/// Compares a wide string with an ASCII name. The wide string ends at its
/// first NUL or at the end of the slice, as `wcscmp` on a NUL-terminated
/// string would see it.
pub(crate) fn wcs_eq(wide: &[u16], name: &str) -> bool {
    let end = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    wide[..end].iter().copied().eq(name.encode_utf16())
}

/// Encodes `text` as UTF-16LE followed by a NUL, the bytes that Wine copies out
/// for a wide string property.
pub(crate) fn to_wide_nul(text: &str) -> Vec<u8> {
    text.encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(u16::to_le_bytes)
        .collect()
}
