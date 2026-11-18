//! Headless subset of Wine's `dlls/mscms` (Image Color Management).
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`icc`]: `icc.c`, ICC header and tag table access over the profile bytes.
//! - [`transform`]: the pure parts of `transform.c`: the `BMFORMAT` and `COLORTYPE` mapping,
//!   the rendering intent and proofing flag of `CreateColorTransformW`, and the argument
//!   checks of `CreateMultiProfileTransform`.
//! - [`profile`]: the pure parts of `profile.c`: `basename`, `GetColorDirectoryW`,
//!   `GetStandardColorSpaceProfileW` and `match_profile`.
//!
//! Skipped: the lcms calls and transform objects, `TranslateBitmapBits` and `TranslateColors`,
//! handle objects (`handle.c`), registry and profile-store code, file access
//! (`header_from_file`, `OpenColorProfileW`), `EnumColorProfilesW` (its exactly sized buffer
//! overflows in Wine), the ANSI wrappers and the WCS stubs.
//!
//! Buffer sizes follow Wine's reporting. One deliberate deviation: Wine accepts a buffer that
//! holds the string but not its NUL terminator and then overflows it; the port reports the
//! size including the terminator instead.

pub mod error;
pub mod icc;
pub mod profile;
pub mod transform;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

pub use error::McsError;

/// Raw ICC profile bytes, as Wine's `struct profile` holds them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    data: Vec<u8>,
}

impl Profile {
    /// Wraps `data`, or returns `None` when it is too short to hold the header and tag count.
    pub fn new(data: Vec<u8>) -> Option<Self> {
        (data.len() >= icc::TAG_TABLE_OFFSET).then_some(Self { data })
    }

    /// The profile bytes as stored.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}
