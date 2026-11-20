//! Port of `dlls/webservices/url.c`: `WsDecodeUrl` and `WsEncodeUrl` over UTF-16 text.
//!
//! Wine stores the parts of a URL as slices of one decoded buffer. This port copies each part
//! into its own `Vec`. The scanning follows the C loops position by position.

use crate::Error;

/// The URL schemes of `WS_URL_SCHEME_TYPE`, with the values of the C enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    Http = 0,
    Https = 1,
    NetTcp = 2,
    SoapUdp = 3,
    NetPipe = 4,
}

impl Scheme {
    const ALL: [Scheme; 5] = [
        Scheme::Http,
        Scheme::Https,
        Scheme::NetTcp,
        Scheme::SoapUdp,
        Scheme::NetPipe,
    ];

    const fn name(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::NetTcp => "net.tcp",
            Self::SoapUdp => "soap.udp",
            Self::NetPipe => "net.pipe",
        }
    }

    /// The port that a URL of this scheme uses when it does not name one.
    pub const fn default_port(self) -> u16 {
        match self {
            Self::Http => 80,
            Self::Https => 443,
            Self::NetTcp => 808,
            Self::SoapUdp | Self::NetPipe => 65535,
        }
    }

    fn from_units(units: &[u16]) -> Option<Scheme> {
        let text = String::from_utf16_lossy(units);
        Self::ALL
            .into_iter()
            .find(|scheme| scheme.name().eq_ignore_ascii_case(&text))
    }
}

/// The parts of a parsed URL, as `WS_URL` holds them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpUrl {
    pub scheme: Scheme,
    pub host: Vec<u16>,
    pub port: u16,
    /// The digits of the port as written. Empty when the URL names no port.
    pub port_as_string: Vec<u16>,
    pub path: Vec<u16>,
    pub query: Vec<u16>,
    pub fragment: Vec<u16>,
}

/// `WsDecodeUrl`: percent-decodes `text` and then parses it as an HTTP-style URL.
pub fn decode_url(text: &[u16], flags: u32) -> Result<HttpUrl, Error> {
    if text.is_empty() {
        return Err(Error::InvalidFormat);
    }
    if flags != 0 {
        return Err(Error::NotImplemented);
    }
    parse_url(&percent_decode(text)?)
}

/// `WsEncodeUrl`: writes `url` as UTF-16 text, escaping the parts that need it.
pub fn encode_url(url: &HttpUrl, flags: u32) -> Result<Vec<u16>, Error> {
    if flags != 0 {
        return Err(Error::NotImplemented);
    }
    let port = encoded_port(url)?;
    let mut out = String::from(url.scheme.name());
    out += "://";
    out += &escape_component(&url.host, "");
    if port != 0 {
        out += &format!(":{port}");
    }
    out += &escape_component(&url.path, "/");
    if !url.query.is_empty() {
        out.push('?');
        out += &escape_component(&url.query, "/?");
    }
    if !url.fragment.is_empty() {
        out.push('#');
        out += &escape_component(&url.fragment, "/?");
    }
    Ok(out.encode_utf16().collect())
}

/// Wine decodes the escapes byte by byte when every escape is below 0xA0. Otherwise it decodes
/// the UTF-8 form of the text, and the result must then be valid UTF-8.
fn percent_decode(units: &[u16]) -> Result<Vec<u16>, Error> {
    let convert = (0..units.len()).any(|at| escape_at(units, at).is_some_and(|v| v > 159));
    if convert {
        convert_decode(units)
    } else {
        Ok(unescape(units))
    }
}

fn convert_decode(units: &[u16]) -> Result<Vec<u16>, Error> {
    let wide: Vec<u16> = String::from_utf16_lossy(units)
        .bytes()
        .map(u16::from)
        .collect();
    let bytes: Vec<u8> = unescape(&wide).into_iter().map(|unit| unit as u8).collect();
    let text = String::from_utf8(bytes).map_err(|_| Error::QuotaExceeded)?;
    Ok(text.encode_utf16().collect())
}

fn unescape(units: &[u16]) -> Vec<u16> {
    let mut out = Vec::with_capacity(units.len());
    let mut at = 0;
    while at < units.len() {
        match escape_at(units, at) {
            Some(value) => {
                out.push(u16::from(value));
                at += 3;
            }
            None => {
                out.push(units[at]);
                at += 1;
            }
        }
    }
    out
}

fn escape_at(units: &[u16], at: usize) -> Option<u8> {
    let window = units.get(at..at + 3)?;
    if window[0] != u16::from(b'%') {
        return None;
    }
    Some(hex_value(window[1])? * 16 + hex_value(window[2])?)
}

fn hex_value(unit: u16) -> Option<u8> {
    match u8::try_from(unit).ok()? {
        digit @ b'0'..=b'9' => Some(digit - b'0'),
        digit @ b'a'..=b'f' => Some(digit - b'a' + 10),
        digit @ b'A'..=b'F' => Some(digit - b'A' + 10),
        _ => None,
    }
}

fn decimal_digit(unit: u16) -> Option<u32> {
    let value = unit.checked_sub(u16::from(b'0'))?;
    (value < 10).then_some(u32::from(value))
}

/// Reads the digits at the start of `units`. Returns the port and the number of digits read.
fn leading_port(units: &[u16]) -> Result<(u16, usize), Error> {
    let mut port: u32 = 0;
    let mut count = 0;
    for &unit in units {
        let Some(digit) = decimal_digit(unit) else {
            break;
        };
        port = port * 10 + digit;
        if port > 65535 {
            return Err(Error::InvalidFormat);
        }
        count += 1;
    }
    Ok((port as u16, count))
}

/// The first position from `from` whose unit is one of `stops`, or the end of `units`.
fn scan(units: &[u16], from: usize, stops: &[u8]) -> usize {
    units[from..]
        .iter()
        .position(|&unit| stops.iter().any(|&stop| unit == u16::from(stop)))
        .map_or(units.len(), |offset| from + offset)
}

fn is_at(units: &[u16], pos: usize, byte: u8) -> bool {
    units.get(pos) == Some(&u16::from(byte))
}

/// Reads the part that follows `marker` at `pos`, up to the first of `stops`. Returns the part
/// and the position where it ends. Without the marker, the part is empty and nothing is read.
fn after_marker(units: &[u16], pos: usize, marker: u8, stops: &[u8]) -> (Vec<u16>, usize) {
    if !is_at(units, pos, marker) {
        return (Vec::new(), pos);
    }
    let start = pos + 1;
    let end = scan(units, start, stops);
    (units[start..end].to_vec(), end)
}

fn parse_url(text: &[u16]) -> Result<HttpUrl, Error> {
    let colon = scan(text, 0, b":");
    if !is_at(text, colon, b':') {
        return Err(Error::InvalidFormat);
    }
    let scheme = Scheme::from_units(&text[..colon]).ok_or(Error::InvalidFormat)?;
    if !(is_at(text, colon + 1, b'/') && is_at(text, colon + 2, b'/')) {
        return Err(Error::InvalidFormat);
    }

    let start = colon + 3;
    let host_end = if is_at(text, start, b'[') {
        let close = scan(text, start, b"]");
        if !is_at(text, close, b']') {
            return Err(Error::InvalidFormat);
        }
        close + 1
    } else {
        scan(text, start, b"/:?#")
    };
    if host_end == start {
        return Err(Error::InvalidFormat);
    }
    let host = text[start..host_end].to_vec();

    let mut pos = host_end;
    let mut port = 0;
    let mut port_as_string = Vec::new();
    if is_at(text, pos, b':') {
        let digits = pos + 1;
        let (value, count) = leading_port(&text[digits..])?;
        port = value;
        port_as_string = text[digits..digits + count].to_vec();
        pos = digits + count;
    }
    if port == 0 {
        port = scheme.default_port();
        port_as_string.clear();
    }

    let path_end = if is_at(text, pos, b'/') {
        scan(text, pos, b"?")
    } else {
        pos
    };
    let path = text[pos..path_end].to_vec();
    let (query, after_query) = after_marker(text, path_end, b'?', b"#");
    let (fragment, _) = after_marker(text, after_query, b'#', b"");

    Ok(HttpUrl {
        scheme,
        host,
        port,
        port_as_string,
        path,
        query,
        fragment,
    })
}

/// The port to write: zero when the URL names none or names the scheme's default.
fn encoded_port(url: &HttpUrl) -> Result<u16, Error> {
    let port = if url.port_as_string.is_empty() {
        url.port
    } else {
        let (parsed, _) = leading_port(&url.port_as_string)?;
        if url.port != 0 && parsed != url.port {
            return Err(Error::InvalidArgument);
        }
        parsed
    };
    Ok(if port == url.scheme.default_port() {
        0
    } else {
        port
    })
}

/// Escapes `units` as UTF-8 when any unit is at least 0xA0, and byte by byte otherwise.
fn escape_component(units: &[u16], except: &str) -> String {
    let bytes: Vec<u8> = if units.iter().any(|&unit| unit > 159) {
        String::from_utf16_lossy(units).into_bytes()
    } else {
        units.iter().map(|&unit| unit as u8).collect()
    };
    bytes
        .into_iter()
        .map(|byte| escape_byte(byte, except))
        .collect()
}

fn escape_byte(byte: u8, except: &str) -> String {
    if needs_escape(byte, except) {
        format!("%{byte:02X}")
    } else {
        char::from(byte).to_string()
    }
}

fn needs_escape(byte: u8, except: &str) -> bool {
    if except.as_bytes().contains(&byte) || byte.is_ascii_alphanumeric() {
        return false;
    }
    !(33..=126).contains(&byte) || b"/?\"#%<>\\[]^`{|}".contains(&byte)
}
