//! Port of the deterministic logic of Wine's `dlls/resampledmo/resampler.c`: the DMO audio
//! resampler's media type checks and enumeration, buffer sizing, timestamp conversion and stream
//! state.
//!
//! Wine converts samples with libswresample. Here that is the [`resampler::Converter`] trait, so
//! the logic around it runs against a scripted stub in the tests. COM interfaces, IMediaBuffer,
//! IMFSample and the registry are not modelled.

pub mod format;
pub mod media;
pub mod resampler;
pub mod time;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
#[cfg(test)]
mod stub;

/// The failures the resampler reports, as the `DMO_E_*`, `MF_E_*` and `E_*` codes Wine returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidStreamIndex,
    InvalidType,
    TypeNotSet,
    NotAccepting,
    NoMoreItems,
    Fail,
    InvalidArg,
}

impl Error {
    /// The `HRESULT` Wine returns for this failure.
    pub fn hresult(self) -> u32 {
        match self {
            Error::InvalidStreamIndex => 0x8004_0201,
            Error::InvalidType => 0x8004_0202,
            Error::TypeNotSet => 0x8004_0203,
            Error::NotAccepting => 0x8004_0204,
            Error::NoMoreItems => 0x8004_0206,
            Error::Fail => 0x8000_4005,
            Error::InvalidArg => 0x8007_0057,
        }
    }
}
