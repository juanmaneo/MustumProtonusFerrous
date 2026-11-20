//! Pure ASPI support-info bit-packing from Wine's `dlls/wnaspi32/winaspi32.c`.
//!
//! The Wine export `GetASPI32SupportInfo` queries the registry for the number
//! of host adapters and packs the result into a DWORD. This crate ports only
//! the dependency-free packing step; the registry query is excluded.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// ASPI status: command completed successfully.
pub const SS_COMP: u32 = 0x01;

/// ASPI status: no host adapters were found.
pub const SS_NO_ADAPTERS: u32 = 0xE8;

/// Packs the host-adapter count into the `GetASPI32SupportInfo` return value.
///
/// The low word holds the status in its high byte and the adapter count in its
/// low byte. A zero count reports `SS_NO_ADAPTERS`; otherwise the status is
/// `SS_COMP` and the count is OR-ed in. Counts above 255 overlap the status
/// byte, matching Wine's raw `(SS_COMP << 8) | controllers` expression.
#[must_use]
pub const fn support_info(controllers: u32) -> u32 {
    if controllers == 0 {
        SS_NO_ADAPTERS << 8
    } else {
        (SS_COMP << 8) | controllers
    }
}
