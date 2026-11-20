//! Pure address parsing logic from Wine's `dlls/ws2_32/protocol.c`.
//!
//! This crate ports only `inet_addr`; APIs that interact with sockets, OS state, or address
//! buffers are intentionally omitted.

pub mod inet_addr;

/// Parses a legacy IPv4 address through the crate's primary API.
pub fn parse_legacy_ipv4(input: Option<&[u8]>) -> Result<u32, inet_addr::InetAddrError> {
    inet_addr::inet_addr(input)
}

#[cfg(test)]
mod root_api_tests {
    #[test]
    fn legacy_ipv4_parser_matches_the_address_module() {
        let input = Some(b"127.0.0.1".as_slice());
        assert_eq!(
            super::parse_legacy_ipv4(input),
            super::inet_addr::inet_addr(input)
        );
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
