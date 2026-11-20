//! Filter element escaping from `dlls/wldap32/misc.c`.

use crate::EscapeError;

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

/// Returns the buffer size that `ldap_escape_filter_elementA` needs for `src`, including the
/// terminating NUL. A NULL source needs only the terminator.
pub fn escape_filter_element_size(src: Option<&[u8]>) -> usize {
    let escaped: usize = src
        .unwrap_or_default()
        .iter()
        .map(|&byte| if byte.is_ascii_alphanumeric() { 1 } else { 3 })
        .sum();
    escaped + 1
}

/// Escapes `src` into `dst` as `ldap_escape_filter_elementA` does. ASCII letters and digits are
/// copied, and every other byte becomes `\XX` with two uppercase hex digits. The output is
/// NUL-terminated.
///
/// Writes exactly [`escape_filter_element_size`] bytes and leaves the rest of `dst` alone. It
/// fails without writing when `src` is `None` or `dst` is shorter than the escaped size.
pub fn escape_filter_element(src: Option<&[u8]>, dst: &mut [u8]) -> Result<(), EscapeError> {
    let size = escape_filter_element_size(src);
    let src = src.ok_or(EscapeError::ParamError)?;
    if dst.len() < size {
        return Err(EscapeError::ParamError);
    }

    let mut out = 0;
    for &byte in src {
        if byte.is_ascii_alphanumeric() {
            dst[out] = byte;
            out += 1;
        } else {
            dst[out] = b'\\';
            dst[out + 1] = HEX_DIGITS[usize::from(byte >> 4)];
            dst[out + 2] = HEX_DIGITS[usize::from(byte & 0x0f)];
            out += 3;
        }
    }
    dst[out] = 0;
    Ok(())
}
