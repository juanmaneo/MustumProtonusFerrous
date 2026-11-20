//! Port of the deterministic logic of Wine's `dlls/dmsynth`: the `IDirectMusicSynth8::Open`
//! parameter negotiation, the `GetFormat` and sink format rules, the sample and reference time
//! conversions, the DLS connection to FluidSynth generator and modulator mapping, and the region
//! helpers (loop settings, attenuation, instrument matching and the drum fallback).
//!
//! The COM interfaces, the critical sections, the FluidSynth calls, the download and playback
//! paths, the KS and debug code, and the `pow()` gain value are not modelled.

pub mod dls;
pub mod format;
pub mod generator;
pub mod modulator;
pub mod port;
pub mod region;
pub mod sink;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The failures the ported calls report, as the `HRESULT` Wine returns for each.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidArg,
    AlreadyOpen,
    NotConfigured,
    Unexpected,
}

impl Error {
    /// The `HRESULT` Wine returns for this failure.
    pub fn hresult(self) -> u32 {
        match self {
            Error::InvalidArg => 0x8007_0057,
            Error::AlreadyOpen => 0x8878_112F,
            Error::NotConfigured => 0x8878_1131,
            Error::Unexpected => 0x8000_FFFF,
        }
    }
}
