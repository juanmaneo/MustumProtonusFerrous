//! Outgoing connections and the sender address of an incoming message.

use std::cmp::Ordering;

const INADDR_ANY: u32 = 0;

/// `DPWS_CONNECTION_KEY`: the remote address and the guaranteed flag of an outgoing connection.
///
/// The field order is the comparison order of `DPWS_CompareConnections`: port, then address,
/// then `guaranteed`. Wine compares its `BOOL` as an integer, so only 0 and 1 are meaningful.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionKey {
    pub sin_port: u16,
    pub sin_addr: u32,
    pub guaranteed: bool,
}

/// `DPWS_CompareConnections`: `-1`, `0` or `1` as `key` sorts before, equal to or after `entry`.
pub fn compare_connections(key: &ConnectionKey, entry: &ConnectionKey) -> i32 {
    match key.cmp(entry) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// The sender address that `DPWS_MessageBodyReceiveCompleted` keeps: an `INADDR_ANY` in the
/// header is replaced by the address the connection was accepted from.
pub fn resolve_sender_addr(header_addr: u32, connection_addr: u32) -> u32 {
    if header_addr == INADDR_ANY {
        connection_addr
    } else {
        header_addr
    }
}
