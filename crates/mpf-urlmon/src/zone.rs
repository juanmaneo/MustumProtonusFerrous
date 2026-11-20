//! `is_known_zone` from Wine's `dlls/urlmon/zone_id.c`, with the `URLZONE`
//! values of `include/urlmon.idl`.

pub const URLZONE_INVALID: i32 = -1;
pub const URLZONE_LOCAL_MACHINE: i32 = 0;
pub const URLZONE_INTRANET: i32 = 1;
pub const URLZONE_TRUSTED: i32 = 2;
pub const URLZONE_INTERNET: i32 = 3;
pub const URLZONE_UNTRUSTED: i32 = 4;

/// Whether `zone` is one of the five predefined zones. User zones (1000 and up)
/// and the invalid zone are not known.
pub fn is_known_zone(zone: i32) -> bool {
    matches!(zone, URLZONE_LOCAL_MACHINE..=URLZONE_UNTRUSTED)
}
