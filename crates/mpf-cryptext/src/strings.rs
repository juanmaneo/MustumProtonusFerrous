//! `strdupAW` from `dlls/cryptext/cryptext_main.c`.

/// The part of a C string before its first NUL. A slice without a NUL is taken whole.
pub(crate) fn until_nul<T: Default + PartialEq>(s: &[T]) -> &[T] {
    let end = s.iter().position(|c| *c == T::default()).unwrap_or(s.len());
    &s[..end]
}

/// `strdupAW`. `None` stays `None`. Otherwise every byte up to the first NUL becomes one code
/// unit through `ansi_unit`, which stands in for `MultiByteToWideChar(CP_ACP, 0, src, -1, ...)`,
/// and the result ends with a NUL code unit.
pub fn strdup_ansi_to_wide(src: Option<&[u8]>, ansi_unit: impl Fn(u8) -> u16) -> Option<Vec<u16>> {
    let src = until_nul(src?);
    let mut wide: Vec<u16> = src.iter().map(|&b| ansi_unit(b)).collect();
    wide.push(0);
    Some(wide)
}
