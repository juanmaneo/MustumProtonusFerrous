//! Deterministic UUID parsing from Wine's `dlls/winebth.sys/dbus.c`.
//!
//! [`parse_uuid`] ports the component's `parse_uuid` helper: the 36-byte length check after the
//! string's first NUL, then the `sscanf` field conversions. Those conversions accept signs, leading
//! whitespace and a partially parsed final hex pair, as Wine's do. The rest of `dbus.c` works with
//! BlueZ over D-Bus and keeps Bluetooth device and adapter state, so it is not ported.

mod uuid;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use uuid::{parse_uuid, Uuid};

/// Parses a Bluetooth UUID through the crate's primary API.
pub fn parse_bluetooth_uuid(text: &[u8]) -> Option<Uuid> {
    parse_uuid(text)
}

#[cfg(test)]
mod root_api_tests {
    #[test]
    fn bluetooth_uuid_parser_matches_the_low_level_api() {
        let text = b"00112233-4455-6677-8899-aabbccddeeff";
        assert_eq!(super::parse_bluetooth_uuid(text), super::parse_uuid(text));
    }
}
