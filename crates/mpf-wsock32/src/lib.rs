//! Deterministic mappings from Wine's `dlls/wsock32`.
//!
//! Socket calls and protocol enumeration are OS-dependent and are not modeled
//! here. This crate provides only the pure service-flag and socket-option
//! translations used by those APIs.

/// Legacy Winsock 1 IP protocol level.
pub const IPPROTO_IP: i32 = 0;

pub const XP1_CONNECTIONLESS: u32 = 0x0000_0001;
pub const XP1_GUARANTEED_DELIVERY: u32 = 0x0000_0002;
pub const XP1_GUARANTEED_ORDER: u32 = 0x0000_0004;
pub const XP1_MESSAGE_ORIENTED: u32 = 0x0000_0008;
pub const XP1_PSEUDO_STREAM: u32 = 0x0000_0010;
pub const XP1_GRACEFUL_CLOSE: u32 = 0x0000_0020;
pub const XP1_EXPEDITED_DATA: u32 = 0x0000_0040;
pub const XP1_CONNECT_DATA: u32 = 0x0000_0080;
pub const XP1_DISCONNECT_DATA: u32 = 0x0000_0100;
pub const XP1_SUPPORT_BROADCAST: u32 = 0x0000_0200;
pub const XP1_SUPPORT_MULTIPOINT: u32 = 0x0000_0400;
pub const XP1_QOS_SUPPORTED: u32 = 0x0000_2000;
pub const XP1_PARTIAL_MESSAGE: u32 = 0x0004_0000;

pub const XP_CONNECTIONLESS: u32 = 0x0000_0001;
pub const XP_GUARANTEED_DELIVERY: u32 = 0x0000_0002;
pub const XP_GUARANTEED_ORDER: u32 = 0x0000_0004;
pub const XP_MESSAGE_ORIENTED: u32 = 0x0000_0008;
pub const XP_PSEUDO_STREAM: u32 = 0x0000_0010;
pub const XP_GRACEFUL_CLOSE: u32 = 0x0000_0020;
pub const XP_EXPEDITED_DATA: u32 = 0x0000_0040;
pub const XP_CONNECT_DATA: u32 = 0x0000_0080;
pub const XP_DISCONNECT_DATA: u32 = 0x0000_0100;
pub const XP_SUPPORTS_BROADCAST: u32 = 0x0000_0200;
pub const XP_SUPPORTS_MULTICAST: u32 = 0x0000_0400;
pub const XP_BANDWIDTH_ALLOCATION: u32 = 0x0000_0800;
pub const XP_FRAGMENTATION: u32 = 0x0000_1000;

/// Translates WSA protocol service flags to the legacy `PROTOCOL_INFO` flags.
pub fn map_service_flags(wsaflags: u32) -> u32 {
    let mut flags = 0;

    if wsaflags & XP1_CONNECTIONLESS != 0 {
        flags |= XP_CONNECTIONLESS;
    }
    if wsaflags & XP1_GUARANTEED_DELIVERY != 0 {
        flags |= XP_GUARANTEED_DELIVERY;
    }
    if wsaflags & XP1_GUARANTEED_ORDER != 0 {
        flags |= XP_GUARANTEED_ORDER;
    }
    if wsaflags & XP1_MESSAGE_ORIENTED != 0 {
        flags |= XP_MESSAGE_ORIENTED;
    }
    if wsaflags & XP1_PSEUDO_STREAM != 0 {
        flags |= XP_PSEUDO_STREAM;
    }
    if wsaflags & XP1_GRACEFUL_CLOSE != 0 {
        flags |= XP_GRACEFUL_CLOSE;
    }
    if wsaflags & XP1_EXPEDITED_DATA != 0 {
        flags |= XP_EXPEDITED_DATA;
    }
    if wsaflags & XP1_CONNECT_DATA != 0 {
        flags |= XP_CONNECT_DATA;
    }
    if wsaflags & XP1_DISCONNECT_DATA != 0 {
        flags |= XP_DISCONNECT_DATA;
    }
    if wsaflags & XP1_SUPPORT_BROADCAST != 0 {
        flags |= XP_SUPPORTS_BROADCAST;
    }
    if wsaflags & XP1_SUPPORT_MULTIPOINT != 0 {
        flags |= XP_SUPPORTS_MULTICAST;
    }
    if wsaflags & XP1_QOS_SUPPORTED != 0 {
        flags |= XP_BANDWIDTH_ALLOCATION;
    }
    if wsaflags & XP1_PARTIAL_MESSAGE != 0 {
        flags |= XP_FRAGMENTATION;
    }

    flags
}

/// Remaps a Winsock 1 IP socket option to its Winsock 2 option number.
/// Other protocol levels and unknown option numbers are returned unchanged.
pub fn remap_socket_option(level: i32, optname: i32) -> i32 {
    if level != IPPROTO_IP {
        return optname;
    }

    match optname {
        2 => 9,
        3 => 10,
        4 => 11,
        5 => 12,
        6 => 13,
        7 => 4,
        8 => 3,
        9 => 14,
        _ => optname,
    }
}

#[cfg(test)]
#[path = "oracle_checks.rs"]
mod oracle_checks;

#[cfg(test)]
#[path = "edge_checks.rs"]
mod edge_checks;
