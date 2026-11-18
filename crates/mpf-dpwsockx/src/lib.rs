//! Headless subset of Wine's `dlls/dpwsockx`, the TCP/IP service provider for DirectPlay.
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`header`]: the `mixed` word of `DPSP_MSG_HEADER` and its size and token fields.
//! - [`connection`]: the ordering of outgoing connections (`DPWS_CompareConnections`) and the
//!   sender address kept by `DPWS_MessageBodyReceiveCompleted`.
//! - [`tcp`]: the stream receive decisions of `DPWS_TcpReceiveCompleted` and
//!   `DPWS_HeaderReceiveCompleted`, and the buffer growth rule.
//! - [`udp`]: the datagram framing of `DPWS_UdpReceiveCompleted`.
//! - [`caps`]: the capability block of `DPWSCB_GetCaps`.
//!
//! Skipped: socket setup and I/O (`socket`, `bind` with its port search, `accept`, `WSARecv`,
//! `sendto`), the receive thread and its events, the send path (`DPWSCB_SendEx`,
//! `DPWS_SendImpl`), player and group bookkeeping (connection lookup, `CreatePlayer`,
//! `AddPlayerToGroup`), `Open`, `Start` and `Stop`, the COM entry point `SPInit`, and the
//! callbacks that return `DPERR_UNSUPPORTED`.
//!
//! Wine's `DPWS_UdpReceiveCompleted` stops receiving after a malformed framed datagram,
//! because it returns without re-arming. The port reports that case as an error and leaves
//! the re-arm to the caller.

pub mod caps;
pub mod connection;
pub mod header;
pub mod tcp;
pub mod udp;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

use header::{make_mixed, AF_INET, HEADER_SIZE, TOKEN_REMOTE};

/// Wire header of a DirectPlay message: Wine's packed `DPSP_MSG_HEADER`, 20 bytes.
///
/// Fields hold the values as the little-endian x86 layout stores them. `sin_port` and
/// `sin_addr` stay in network byte order, as Wine copies them. The `sin_zero` padding is not
/// modelled: [`MsgHeader::to_bytes`] writes zeros and [`MsgHeader::from_bytes`] ignores it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsgHeader {
    pub mixed: u32,
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
}

impl MsgHeader {
    /// The header that `EnumSessions` and `SendImpl` build for a message of `size` bytes sent
    /// from the local TCP port `tcp_port`.
    pub fn remote(size: u32, tcp_port: u16) -> Self {
        Self {
            mixed: make_mixed(size, TOKEN_REMOTE),
            sin_family: AF_INET,
            sin_port: tcp_port,
            sin_addr: 0,
        }
    }

    /// Reads the header from the start of `bytes`, or returns `None` when there are fewer than
    /// 20 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let raw: &[u8; HEADER_SIZE] = bytes.first_chunk()?;
        Some(Self {
            mixed: u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]),
            sin_family: u16::from_le_bytes([raw[4], raw[5]]),
            sin_port: u16::from_le_bytes([raw[6], raw[7]]),
            sin_addr: u32::from_le_bytes([raw[8], raw[9], raw[10], raw[11]]),
        })
    }

    /// The 20 bytes as they sit in memory, with `sin_zero` cleared.
    pub fn to_bytes(self) -> [u8; HEADER_SIZE] {
        let mut out = [0; HEADER_SIZE];
        out[0..4].copy_from_slice(&self.mixed.to_le_bytes());
        out[4..6].copy_from_slice(&self.sin_family.to_le_bytes());
        out[6..8].copy_from_slice(&self.sin_port.to_le_bytes());
        out[8..12].copy_from_slice(&self.sin_addr.to_le_bytes());
        out
    }
}
