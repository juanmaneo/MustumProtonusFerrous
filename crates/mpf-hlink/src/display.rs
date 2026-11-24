use crate::{c_str, eq_ascii_ci, HResult, E_INVALIDARG, S_FALSE, S_OK};

/// Drops a leading `file:` and the slashes after it, ASCII case-insensitively, and
/// returns the rest of the name.
pub fn strip_file_scheme(name: &[u16]) -> &[u16] {
    match name.get(..5) {
        Some(prefix) if eq_ascii_ci(prefix, b"file:") => {
            let rest = &name[5..];
            let slashes = rest
                .iter()
                .take_while(|&&unit| unit == u16::from(b'/'))
                .count();
            &rest[slashes..]
        }
        _ => name,
    }
}

/// A display name is absolute when it contains a `:`.
pub fn is_absolute_display_name(display_name: Option<&[u16]>) -> bool {
    display_name.is_some_and(|name| c_str(name).contains(&u16::from(b':')))
}

/// `HlinkIsShortcut`: `S_OK` for a name that ends in `.url`, `S_FALSE` for any other
/// name, and `E_INVALIDARG` for a missing name.
pub fn is_shortcut(file_name: Option<&[u16]>) -> HResult {
    let Some(file_name) = file_name else {
        return E_INVALIDARG;
    };
    let file_name = c_str(file_name);
    match file_name.len().checked_sub(4) {
        Some(start) if eq_ascii_ci(&file_name[start..], b".url") => S_OK,
        _ => S_FALSE,
    }
}
