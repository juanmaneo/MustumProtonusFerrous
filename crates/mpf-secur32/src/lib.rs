//! Deterministic parsing logic from Wine's `dlls/secur32`.
//!
//! The ported helper is `parse_alpn_protocol_list` from `schannel_gnutls.c`. It parses the
//! length-prefixed ALPN identifiers that Schannel receives in `SECBUFFER_APPLICATION_PROTOCOLS`
//! and returns borrowed byte slices. Identifier bytes stay opaque; they are not interpreted as
//! text, and no Schannel or SSPI handles are involved.
//!
//! Deviations from Wine, where Wine's C helper returns 0 for both an empty list and malformed
//! input:
//! - An empty input is `Some(vec![])`, and malformed input is `None`, so the two cases stay
//!   distinct.
//! - The C helper is called twice to count and then fill its output array. This port makes one
//!   pass and returns the slices directly.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// Parses a sequence of one-byte-length-prefixed ALPN identifiers.
///
/// Returns `None` if an identifier has length zero or extends past the end of the input. An
/// empty input is a valid empty list. Identifier bytes are opaque and may hold any byte value.
pub fn parse_alpn_protocol_list(mut buffer: &[u8]) -> Option<Vec<&[u8]>> {
    let mut protocols = Vec::new();

    while !buffer.is_empty() {
        let length = usize::from(buffer[0]);
        let contents = &buffer[1..];
        if length == 0 || length > contents.len() {
            return None;
        }

        let (protocol, remaining) = contents.split_at(length);
        protocols.push(protocol);
        buffer = remaining;
    }

    Some(protocols)
}
