//! Port of the deterministic logic of Wine's `dlls/dsdmo/main.c`: the media type checks of the
//! DirectSound effect objects' `IMediaObject::SetInputType` and `SetOutputType`, and the stored
//! `WAVEFORMATEX` they compare against.
//!
//! GUIDs are reduced to the comparisons the C code makes: the major type is audio or not, the
//! format type is `WAVEFORMATEX` or not, and the subtype is PCM, float or neither. The COM
//! interfaces, the critical sections, the parameter copies and the `FIXME` stubs are not modelled.

pub mod effect;
pub mod format;
pub mod media;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The failure the media type setters report, as the `HRESULT` Wine returns for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    TypeNotAccepted,
}

impl Error {
    /// The `HRESULT` Wine returns for this failure: `DMO_E_TYPE_NOT_ACCEPTED`.
    pub fn hresult(self) -> u32 {
        match self {
            Error::TypeNotAccepted => 0x8004_0205,
        }
    }
}
