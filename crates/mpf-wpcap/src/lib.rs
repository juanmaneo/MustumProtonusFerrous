//! Port of the deterministic logic in Wine's `dlls/wpcap/wpcap.c` and `unixlib.c`.
//!
//! The DLL forwards almost every `pcap_*` export to libpcap on the Unix side.
//! The port keeps the pure parts: libpcap status strings, the Win32 device name
//! and description builders, the IPv6 netmask construction and the
//! `pcap_dispatch` / `pcap_loop` loop control.
//!
//! Ported:
//! - `pcap_statustostr`: [`statustostr`]
//! - `build_win32_name`: [`build_win32_name`]
//! - `build_win32_description`: [`build_win32_description`] (the offset decoding of the Unix device block stays on the caller's side)
//! - `convert_length_to_ipv6_mask` as used by `get_netmask`: [`ipv6_prefix_mask`]
//! - `pcap_dispatch` and `pcap_loop` loop control: [`dispatch`] and [`packet_loop`]
//!
//! Skipped: the libpcap forwarders, the adapter enumeration (`pcap_findalldevs`,
//! `get_adapters`), the IPv4 netmask and broadcast (`ConvertLengthToIpv4Mask`),
//! the datalink and time stamp name caches (they call the Unix side), `pcap_dump`
//! and the other handle-based exports, and the `FIXME` stubs.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Status codes of `wpcap.c`.
pub const PCAP_WARNING: i32 = 1;
pub const PCAP_WARNING_PROMISC_NOTSUP: i32 = 2;
pub const PCAP_WARNING_TSTAMP_TYPE_NOTSUP: i32 = 3;
pub const PCAP_ERROR: i32 = -1;
pub const PCAP_ERROR_BREAK: i32 = -2;
pub const PCAP_ERROR_NOT_ACTIVATED: i32 = -3;
pub const PCAP_ERROR_ACTIVATED: i32 = -4;
pub const PCAP_ERROR_NO_SUCH_DEVICE: i32 = -5;

/// The text `pcap_statustostr` returns for each status code.
const STATUS_TEXT: &[(i32, &str)] = &[
    (PCAP_WARNING, "Generic warning"),
    (
        PCAP_WARNING_PROMISC_NOTSUP,
        "That device doesn't support promiscuous mode",
    ),
    (
        PCAP_WARNING_TSTAMP_TYPE_NOTSUP,
        "That type of time stamp is not supported by that device",
    ),
    (PCAP_ERROR, "Generic error"),
    (PCAP_ERROR_BREAK, "Loop terminated by pcap_breakloop"),
    (
        PCAP_ERROR_NOT_ACTIVATED,
        "The pcap_t has not been activated",
    ),
    (
        PCAP_ERROR_ACTIVATED,
        "The setting can't be changed after the pcap_t is activated",
    ),
    (PCAP_ERROR_NO_SUCH_DEVICE, "No such device exists"),
];

/// Text for a libpcap status code, as `pcap_statustostr` returns it.
pub fn statustostr(status: i32) -> String {
    match STATUS_TEXT.iter().find(|(code, _)| *code == status) {
        Some((_, text)) => text.to_string(),
        None => format!("Unknown error: {status}"),
    }
}

/// The `\Device\NPF_` device name for an adapter, prefixed by `source` when given.
pub fn build_win32_name(source: Option<&str>, adapter_name: &str) -> String {
    format!("{}\\Device\\NPF_{adapter_name}", source.unwrap_or(""))
}

/// The display name of a device: `description name` when it has a description, else `name`.
pub fn build_win32_description(name: &str, description: Option<&str>) -> String {
    match description {
        Some(description) => format!("{description} {name}"),
        None => name.to_owned(),
    }
}

/// The 16-byte IPv6 netmask for a prefix length, or `None` outside 8..=135.
///
/// The C code writes past its 16-byte mask for lengths outside that range
/// (ASan reports a stack-buffer-overflow), so the port refuses them instead.
pub fn ipv6_prefix_mask(length: u32) -> Option<[u8; 16]> {
    if !(8..=135).contains(&length) {
        return None;
    }
    let mut mask = [0u8; 16];
    let full = (length / 8 - 1) as usize;
    mask[..full].fill(0xff);
    mask[full] = (0xff_u32 << (8 - length % 8)) as u8;
    Some(mask)
}

/// Drives `pcap_dispatch`: `next` yields a packet (`Ok`) or a libpcap return code
/// (`Err`), and `count <= 0` means no limit. A timeout (`Err(0)`) ends the loop.
pub fn dispatch<P>(
    count: i32,
    mut next: impl FnMut() -> Result<P, i32>,
    mut callback: impl FnMut(P),
) -> i32 {
    let mut processed = 0;
    while count <= 0 || processed < count {
        match next() {
            Ok(packet) => {
                processed += 1;
                callback(packet);
            }
            Err(0) => break,
            Err(PCAP_ERROR_BREAK) if processed == 0 => return PCAP_ERROR_BREAK,
            Err(PCAP_ERROR_BREAK) => break,
            Err(ret) => return ret,
        }
    }
    processed
}

/// Drives `pcap_loop`: as [`dispatch`], but a timeout (`Err(0)`) keeps polling.
pub fn packet_loop<P>(
    count: i32,
    mut next: impl FnMut() -> Result<P, i32>,
    mut callback: impl FnMut(P),
) -> i32 {
    let mut processed = 0;
    while count <= 0 || processed < count {
        match next() {
            Ok(packet) => {
                processed += 1;
                callback(packet);
            }
            Err(0) => continue,
            Err(PCAP_ERROR_BREAK) if processed == 0 => return PCAP_ERROR_BREAK,
            Err(PCAP_ERROR_BREAK) => break,
            Err(ret) => return ret,
        }
    }
    processed
}
