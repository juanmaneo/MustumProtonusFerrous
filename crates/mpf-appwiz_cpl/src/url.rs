use crate::{c_str, Addon};

/// `INTERNET_MAX_URL_LENGTH`: the URL buffer holds this many WCHARs, NUL included.
pub const INTERNET_MAX_URL_LENGTH: usize = 2048;
/// `REG_SZ`: the only value type `get_url` takes from the registry.
pub const REG_SZ: u32 = 1;

const HTTP: [u16; 4] = [b'h' as u16, b't' as u16, b't' as u16, b'p' as u16];
/// `sizeof(httpW)` in bytes.
const HTTP_BYTES: usize = 8;
/// The size `get_url` starts with. It stays as the returned size when the registry has no value.
const BUFFER_BYTES: usize = INTERNET_MAX_URL_LENGTH * 2;

/// `append_url_params`: appends `?arch=..&v=..&winev=..` to `url`. With no Wine version string
/// (`winev` is `None`), `winev=` stays empty. Returns `None` when the result and its NUL would
/// not fit in the URL buffer, where the C code writes past the end.
pub fn append_url_params(
    url: &[u16],
    arch: &str,
    version: &str,
    winev: Option<&str>,
) -> Option<Vec<u16>> {
    let mut out = c_str(url).to_vec();
    out.extend("?arch=".encode_utf16());
    out.extend(arch.encode_utf16());
    out.extend("&v=".encode_utf16());
    out.extend(version.encode_utf16());
    out.extend("&winev=".encode_utf16());
    if let Some(winev) = winev {
        out.extend(winev.encode_utf16());
    }
    (out.len() < INTERNET_MAX_URL_LENGTH).then_some(out)
}

/// `get_url`. `reg` is the value read from the registry: `Some((ty, data))` when
/// `RegQueryValueExW` succeeded, with `data` holding the bytes it wrote as WCHARs. A `REG_SZ`
/// value is the URL. Any other type gives the default URL, but the returned size is still the
/// value's own size. The parameters are appended when that size is over 8 bytes and the URL
/// starts with `http`.
pub fn get_url(addon: Addon, reg: Option<(u32, &[u16])>, winev: Option<&str>) -> Option<Vec<u16>> {
    let default: Vec<u16> = addon.url_default().encode_utf16().collect();
    let (url, returned_bytes) = match reg {
        Some((REG_SZ, data)) => (data, data.len() * 2),
        Some((_, data)) => (default.as_slice(), data.len() * 2),
        None => (default.as_slice(), BUFFER_BYTES),
    };
    let text = c_str(url);
    if returned_bytes > HTTP_BYTES && url.starts_with(&HTTP) {
        append_url_params(text, addon.arch(), addon.version(), winev)
    } else {
        Some(text.to_vec())
    }
}
