//! UPnP WANIPConnection service types and SOAP requests (`port.c`, `parse_desc_xml` and `build_request`).

use crate::decimal_int;

const WANIP_SERVICE: &str = "urn:schemas-upnp-org:service:WANIPConnection:";

/// The version of a WANIPConnection service type (`parse_desc_xml`). The prefix is compared with ASCII
/// case folding, and the version is the `_wtoi` of the text after it. Like Wine, `-1` gives
/// `Some(u32::MAX)`. Returns `None` for another service or for a version below 1.
pub fn wanip_service_version(service_type: &[u16]) -> Option<u32> {
    let rest = strip_prefix_ascii_nocase(service_type, WANIP_SERVICE)?;
    let version = decimal_int(rest.iter().map(|&unit| u32::from(unit))) as u32;
    (version >= 1).then_some(version)
}

/// The SOAP envelope of a WANIPConnection action (`build_request`). `params` are the `(name, value)` pairs
/// in order. Wine writes the text in the ANSI code page, which matches UTF-8 for ASCII names and values.
pub fn request_envelope(function: &str, version: u32, params: &[(&str, &str)]) -> String {
    let mut body = String::from(
        "<?xml version=\"1.0\"?>\r\n\
         <SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\" \
         SOAP-ENV:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">\r\n\
         \x20 <SOAP-ENV:Body>\r\n",
    );
    body.push_str(&format!(
        "    <m:{function} xmlns:m=\"urn:schemas-upnp-org:service:WANIPConnection:{version}\">\r\n"
    ));
    for (name, value) in params {
        body.push_str(&format!("      <{name}>{value}</{name}>\r\n"));
    }
    body.push_str(&format!(
        "    </m:{function}>\r\n  </SOAP-ENV:Body>\r\n</SOAP-ENV:Envelope>\r\n"
    ));
    body
}

/// The SOAPAction and Content-Type headers of a WANIPConnection action (`build_request`).
pub fn request_headers(function: &str, version: u32) -> String {
    format!(
        "SOAPAction: \"urn:schemas-upnp-org:service:WANIPConnection:{version}#{function}\"\r\n\
         Content-Type: text/xml"
    )
}

/// Strips an ASCII `prefix` from `text`, matching letters without regard to case (`wcsnicmp`).
fn strip_prefix_ascii_nocase<'a>(text: &'a [u16], prefix: &str) -> Option<&'a [u16]> {
    let head = text.get(..prefix.len())?;
    head.iter()
        .zip(prefix.bytes())
        .all(|(&unit, byte)| fold_ascii_case(unit) == fold_ascii_case(u16::from(byte)))
        .then_some(&text[prefix.len()..])
}

fn fold_ascii_case(unit: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&unit) {
        unit + u16::from(b'a' - b'A')
    } else {
        unit
    }
}
