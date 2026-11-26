use crate::env::expand_environment_strings;

/// The environment block `BrandingFormatString` expands against, from `dlls/winbrand/main.c`.
pub(crate) const BRANDING_ENVIRONMENT: &str =
    "WINDOWS_SHORT=Windows 10\0WINDOWS_LONG=Windows 10 Pro\0";

/// `BrandingFormatString` from `dlls/winbrand/main.c`. `format` is read up to its first NUL,
/// like `wcslen`. `alloc` stands in for `HeapAlloc(GetProcessHeap(), 0, len * sizeof(WCHAR))`,
/// takes the character count `len`, and must return a buffer of at least that length. The result
/// is the expansion followed by its NUL, or `None` when `alloc` fails.
pub fn branding_format_string(
    format: &[u16],
    alloc: impl FnOnce(usize) -> Option<Vec<u16>>,
) -> Option<Vec<u16>> {
    let env: Vec<u16> = BRANDING_ENVIRONMENT.encode_utf16().collect();
    let format = &format[..format.iter().position(|&c| c == 0).unwrap_or(format.len())];
    // The length query reports only STATUS_BUFFER_TOO_SMALL, and the fill into a buffer of
    // that length always reports STATUS_SUCCESS, so neither status is checked.
    let (_, len) = expand_environment_strings(&env, format, &mut []);
    let mut result = alloc(len)?;
    expand_environment_strings(&env, format, &mut result[..len]);
    result.truncate(len);
    Some(result)
}
