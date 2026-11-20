//! Pure address parsing logic from Wine's `dlls/ws2_32/protocol.c`.
//!
//! This crate ports only `inet_addr`; APIs that interact with sockets, OS state, or address
//! buffers are intentionally omitted.

pub mod inet_addr;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
