//! Port of the deterministic helpers in Wine's `dlls/hnetcfg`.
//!
//! Ported:
//! - `ssdp::search_location` (`port.c`, `parse_search_response`): the `LOCATION:` URL of an SSDP reply.
//! - `upnp::wanip_service_version` (`port.c`, `parse_desc_xml`): recognizes a WANIPConnection service type.
//! - `upnp::request_envelope` and `upnp::request_headers` (`port.c`, `build_request`): the SOAP body and
//!   SOAPAction header of a WANIPConnection action.
//! - `is_valid_protocol`, `is_valid_port` and `is_valid_firewall_protocol` (`port.c` argument checks): the
//!   `S_OK` or `E_INVALIDARG` decision as a boolean.
//!
//! Skipped: the XML reader, WinHTTP and Winsock calls, the gateway and port mapping state, the COM objects
//! and the registry. Each one needs an OS, network, registry or COM runtime. `atoi` and `_wtoi` are ported
//! only as far as the two parsers above use them.

pub mod ssdp;
pub mod upnp;

#[cfg(test)]
pub mod edge_checks;
#[cfg(test)]
pub mod oracle_checks;

/// `IPPROTO_TCP` as used by `NET_FW_IP_PROTOCOL_TCP` in `icftypes.idl`.
pub const NET_FW_IP_PROTOCOL_TCP: i32 = 6;
/// `IPPROTO_UDP` as used by `NET_FW_IP_PROTOCOL_UDP` in `icftypes.idl`.
pub const NET_FW_IP_PROTOCOL_UDP: i32 = 17;

/// `is_valid_protocol` (`port.c`): accepts `UDP` and `TCP`, compared case-sensitively up to the first NUL.
pub fn is_valid_protocol(protocol: Option<&[u16]>) -> bool {
    let Some(protocol) = protocol else {
        return false;
    };
    wcs_eq(protocol, "UDP") || wcs_eq(protocol, "TCP")
}

/// The port range check of `static_ports_get_Item` (`port.c`): 0 through 65535.
pub fn is_valid_port(port: i32) -> bool {
    (0..=65535).contains(&port)
}

/// The protocol check of `fw_port_put_Protocol` (`port.c`): only TCP and UDP.
pub fn is_valid_firewall_protocol(protocol: i32) -> bool {
    matches!(protocol, NET_FW_IP_PROTOCOL_TCP | NET_FW_IP_PROTOCOL_UDP)
}

/// Compares the NUL-terminated part of `text` with an ASCII `want`, like `wcscmp` against a literal.
fn wcs_eq(text: &[u16], want: &str) -> bool {
    let len = text
        .iter()
        .position(|&unit| unit == 0)
        .unwrap_or(text.len());
    text[..len].iter().copied().eq(want.encode_utf16())
}

/// Parses a leading decimal integer the way `atoi` and `_wtoi` do in Wine's msvcrt: white space, an
/// optional sign and digits, stopping at the first other unit and clamped to the `int` range.
pub(crate) fn decimal_int(units: impl IntoIterator<Item = u32>) -> i32 {
    let mut units = units.into_iter().peekable();
    while units
        .next_if(|&unit| matches!(unit, 0x09..=0x0d | 0x20))
        .is_some()
    {}
    let sign = units.next_if(|&unit| unit == u32::from(b'-') || unit == u32::from(b'+'));
    let negative = sign == Some(u32::from(b'-'));
    let mut value: i64 = 0;
    while let Some(unit) = units.next_if(|unit| (u32::from(b'0')..=u32::from(b'9')).contains(unit))
    {
        let digit = i64::from(unit - u32::from(b'0'));
        let step = if negative { -digit } else { digit };
        value = value.saturating_mul(10).saturating_add(step);
    }
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}
