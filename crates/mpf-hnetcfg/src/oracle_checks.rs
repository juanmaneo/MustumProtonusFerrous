//! Tests against Wine's oracle rows.
//!
//! The rows are output of Wine's verbatim helpers from `dlls/hnetcfg/port.c`, `msvcrt/wcs.c` and
//! `msvcrt/string.c`,
//! compiled with `gcc -fsanitize=address,undefined` by `target/canon/hnetcfg/main.c` through the Win32
//! shim `shim.h`. The sanitizers reported nothing for the run, so no vector is excluded.

use crate::{is_valid_firewall_protocol, is_valid_port, is_valid_protocol, ssdp, upnp};

type SearchRow = (&'static str, &'static str, usize, &'static str);
type ServiceRow = (&'static str, &'static str, &'static str);
type SoapRow = (
    &'static str,
    &'static str,
    u32,
    &'static [(&'static str, &'static str)],
    &'static str,
    &'static str,
);

const SIZE: usize = 512;

/// The C driver's text of a UTF-16 string: `\r` and `\n` escaped, other units outside printable ASCII
/// as `\uXXXX`.
pub(crate) fn esc(units: &[u16]) -> String {
    let mut out = String::new();
    for &unit in units {
        match unit {
            0x0d => out.push_str("\\r"),
            0x0a => out.push_str("\\n"),
            0x20..=0x7e => out.push(char::from(unit as u8)),
            _ => out.push_str(&format!("\\u{unit:04x}")),
        }
    }
    out
}

pub(crate) fn esc_text(text: &str) -> String {
    esc(&w(text))
}

pub(crate) fn w(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// The driver's text of a location result: `none`, or `loc:` followed by the escaped URL.
pub(crate) fn show_loc(location: Option<Vec<u16>>) -> String {
    match location {
        Some(units) => format!("loc:{}", esc(&units)),
        None => "none".to_owned(),
    }
}

/// The driver's text of a service version: `none`, or `ver:` followed by the number.
pub(crate) fn show_version(version: Option<u32>) -> String {
    version.map_or_else(|| "none".to_owned(), |version| format!("ver:{version}"))
}

/// The driver's text of a list of checks: `1` or `0` for each, separated by spaces.
pub(crate) fn bits(flags: &[bool]) -> String {
    flags
        .iter()
        .map(|&flag| if flag { "1" } else { "0" })
        .collect::<Vec<_>>()
        .join(" ")
}

#[rustfmt::skip]
const SEARCH_ROWS: &[SearchRow] = &[
    ("R01", "HTTP/1.1 200 OK\r\nLOCATION: http://192.168.1.1:5000/rootDesc.xml\r\nST: upnp:rootdevice\r\n\r\n", SIZE, "loc:http://192.168.1.1:5000/rootDesc.xml"),
    ("R02", "HTTP/1.1 200 OK\r\nlocation:http://192.168.1.1:5000/rootDesc.xml\r\n\r\n", SIZE, "none"),
    ("R03", "HTTP/1.1 404 Not Found\r\nLOCATION: http://192.168.1.1:5000/rootDesc.xml\r\n\r\n", SIZE, "none"),
    ("R04", "HTTP/1.1 200 OK\r\nST: upnp:rootdevice\r\n\r\n", SIZE, "none"),
    ("R05", "HTTP/1.1 200 OK\r\nLOCATION: \r\n\r\n", SIZE, "none"),
    ("R06", "HTTP/1.1 200 OK\r\nLOCATION: http://a/\r\nLOCATION: http://b/\r\n\r\n", SIZE, "loc:http://a/"),
    ("R07", "HTTP/1.1 200 OK\r\nLOCATION: \r\nLOCATION: http://b/\r\n\r\n", SIZE, "none"),
    ("R08", "HTTP/1.1 200\r\nLOCATION: http://x/\r\n\r\n", SIZE, "loc:http://x/"),
    ("R09", "HTTP/1.1 200 OK\r\nLOCATION: http://x/ extra\r\n\r\n", SIZE, "loc:http://x/"),
];

#[rustfmt::skip]
const SERVICE_ROWS: &[ServiceRow] = &[
    ("S01", "urn:schemas-upnp-org:service:WANIPConnection:1", "ver:1"),
    ("S02", "urn:schemas-upnp-org:service:WANIPConnection:2", "ver:2"),
    ("S03", "urn:schemas-upnp-org:service:WANIPConnection:0", "none"),
    ("S04", "URN:SCHEMAS-UPNP-ORG:SERVICE:WANIPCONNECTION:1", "ver:1"),
    ("S05", "urn:schemas-upnp-org:service:WANPPPConnection:1", "none"),
    ("S06", "urn:schemas-upnp-org:service:WANIPConnection:-1", "ver:4294967295"),
];

#[rustfmt::skip]
const SOAP_ROWS: &[SoapRow] = &[
    (
        "P01",
        "AddPortMapping",
        1,
        &[("NewExternalPort", "8080"), ("NewProtocol", "TCP")],
        r#"<?xml version="1.0"?>\r\n<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" SOAP-ENV:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">\r\n  <SOAP-ENV:Body>\r\n    <m:AddPortMapping xmlns:m="urn:schemas-upnp-org:service:WANIPConnection:1">\r\n      <NewExternalPort>8080</NewExternalPort>\r\n      <NewProtocol>TCP</NewProtocol>\r\n    </m:AddPortMapping>\r\n  </SOAP-ENV:Body>\r\n</SOAP-ENV:Envelope>\r\n"#,
        r#"SOAPAction: "urn:schemas-upnp-org:service:WANIPConnection:1#AddPortMapping"\r\nContent-Type: text/xml"#,
    ),
    (
        "P02",
        "GetExternalIPAddress",
        2,
        &[],
        r#"<?xml version="1.0"?>\r\n<SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" SOAP-ENV:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">\r\n  <SOAP-ENV:Body>\r\n    <m:GetExternalIPAddress xmlns:m="urn:schemas-upnp-org:service:WANIPConnection:2">\r\n    </m:GetExternalIPAddress>\r\n  </SOAP-ENV:Body>\r\n</SOAP-ENV:Envelope>\r\n"#,
        r#"SOAPAction: "urn:schemas-upnp-org:service:WANIPConnection:2#GetExternalIPAddress"\r\nContent-Type: text/xml"#,
    ),
];

#[test]
fn search_location_matches_wine() {
    for (label, response, size, want) in SEARCH_ROWS {
        let got = show_loc(ssdp::search_location(response.as_bytes(), *size));
        assert_eq!(got, *want, "row={label}");
    }
}

#[test]
fn wanip_service_version_matches_wine() {
    for (label, value, want) in SERVICE_ROWS {
        let got = show_version(upnp::wanip_service_version(&w(value)));
        assert_eq!(got, *want, "row={label}");
    }
}

#[test]
fn request_text_matches_wine() {
    for (label, function, version, params, env, hdr) in SOAP_ROWS {
        let envelope = upnp::request_envelope(function, *version, params);
        assert_eq!(esc_text(&envelope), *env, "row={label}: envelope");
        let headers = upnp::request_headers(function, *version);
        assert_eq!(esc_text(&headers), *hdr, "row={label}: headers");
    }
}

#[test]
fn argument_checks_match_wine() {
    let protocols = [
        Some("UDP"),
        Some("TCP"),
        Some("tcp"),
        Some("UDPX"),
        Some(""),
        None,
    ];
    let protocol_bits: Vec<bool> = protocols
        .iter()
        .copied()
        .map(|protocol| is_valid_protocol(protocol.map(w).as_deref()))
        .collect();
    assert_eq!(bits(&protocol_bits), "1 1 0 0 0 0", "A01");

    let ports = [-1, 0, 65535, 65536].map(is_valid_port);
    assert_eq!(bits(&ports), "0 1 1 0", "A02");

    let firewall = [0, 6, 7, 17].map(is_valid_firewall_protocol);
    assert_eq!(bits(&firewall), "0 1 0 1", "A03");
}
