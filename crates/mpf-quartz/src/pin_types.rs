//! `MatchTypes` in `dlls/quartz/filtermapper.c`: whether a filter's pin types match the media types
//! that a query asks for.

use crate::{Guid, GUID_NULL};

/// A major and a minor media type, as in `REGPINTYPES`.
pub type TypePair = (Guid, Guid);

/// `MatchTypes`. With no wanted types, any filter that has a pin type matches. A `GUID_NULL` in
/// `wanted` matches any pin type. A `GUID_NULL` on a pin matches any wanted type unless `exact`
/// is set.
pub fn match_types(exact: bool, pins: &[TypePair], wanted: &[TypePair]) -> bool {
    if wanted.is_empty() {
        return !pins.is_empty();
    }
    pins.iter().any(|&(pin_major, pin_minor)| {
        wanted.iter().any(|&(major, minor)| {
            ((!exact && pin_major == GUID_NULL) || major == GUID_NULL || pin_major == major)
                && ((!exact && pin_minor == GUID_NULL) || minor == GUID_NULL || pin_minor == minor)
        })
    })
}
