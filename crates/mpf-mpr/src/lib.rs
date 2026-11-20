//! Deterministic helpers from Wine's `dlls/mpr` implementation.

mod pwcache;

pub use pwcache::{ctox, decode_value_name, hex_digit, select_resource, value_name, value_prefix};

/// Decodes two ASCII hexadecimal digits into one byte.
pub fn decode_hex_pair(high: u8, low: u8) -> Option<u8> {
    let high = pwcache::ctox(high)?;
    let low = pwcache::ctox(low)?;
    Some((high << 4) | low)
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
