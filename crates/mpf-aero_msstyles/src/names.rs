/// `COLORNAMES` in `dlls/aero.msstyles/aero.rc`: one name, then the empty name that ends the list.
/// The resource stores UTF-16, so the text is read with [`str::encode_utf16`].
pub const COLOR_NAMES: &str = "Blue\0\0";

/// `SIZENAMES` in `dlls/aero.msstyles/aero.rc`.
pub const SIZE_NAMES: &str = "NormalSize\0\0";

/// `FILERESNAMES` in `dlls/aero.msstyles/aero.rc`.
pub const FILE_RES_NAMES: &str = "BLUE_INI\0\0";

/// Splits a double-NUL-terminated list of UTF-16 names into its entries, as the loop in
/// `dlls/uxtheme/msstyles.c` does: `while (*tmp) { ...; tmp += lstrlenW(tmp) + 1; }`.
/// The list ends at an empty name, or at the end of the slice, which stands in for a missing NUL.
pub fn multi_sz_entries(units: &[u16]) -> Vec<&[u16]> {
    let mut entries = Vec::new();
    let mut rest = units;
    while !rest.is_empty() && rest[0] != 0 {
        let len = rest
            .iter()
            .position(|&unit| unit == 0)
            .unwrap_or(rest.len());
        entries.push(&rest[..len]);
        rest = &rest[(len + 1).min(rest.len())..];
    }
    entries
}
