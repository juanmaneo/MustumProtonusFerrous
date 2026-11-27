//! The `CP_ACP` conversions that the C bodies call as `WideCharToMultiByte`, modelled for ASCII.

/// Converts one UTF-16 unit to one `CP_ACP` byte. Only ASCII is modelled; any other unit becomes
/// `?`, the default character.
pub fn wide_to_acp_byte(unit: u16) -> u8 {
    match u8::try_from(unit) {
        Ok(byte) if byte.is_ascii() => byte,
        _ => b'?',
    }
}

/// Converts `src` up to and including its first NUL into `dst`, as `WideCharToMultiByte` does with
/// a length of -1. Stops at the end of `dst`, and at the end of `src` when there is no NUL.
pub fn wide_to_acp_z(src: &[u16], dst: &mut [u8]) {
    for (out, &unit) in dst.iter_mut().zip(src) {
        *out = wide_to_acp_byte(unit);
        if unit == 0 {
            break;
        }
    }
}
