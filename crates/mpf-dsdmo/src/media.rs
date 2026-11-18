//! The `DMO_MEDIA_TYPE` checks of `effect_SetInputType` and `effect_SetOutputType`, once their
//! `DMO_SET_TYPEF_CLEAR` branch is taken out.

use crate::format::{WaveFormat, WAVE_FORMAT_IEEE_FLOAT, WAVE_FORMAT_PCM};
use crate::Error;

/// The major type: `MEDIATYPE_Audio` or another GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Major {
    Audio,
    Other,
}

/// The format type: `FORMAT_WaveFormatEx` or another GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatKind {
    WaveFormatEx,
    Other,
}

/// The subtype: `MEDIASUBTYPE_PCM`, `MEDIASUBTYPE_IEEE_FLOAT` or another GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subtype {
    Pcm,
    Float,
    Other,
}

/// A `DMO_MEDIA_TYPE` whose `pbFormat` buffer is `format`. The C code reads only the first 18
/// bytes of the buffer, whatever `cbFormat` says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MediaType<'a> {
    pub major: Major,
    pub subtype: Subtype,
    pub format_kind: FormatKind,
    pub format: &'a [u8],
}

/// The checks both setters share: audio, PCM or float, and `WAVEFORMATEX`.
fn check_common(media: &MediaType) -> Result<(), Error> {
    if media.major != Major::Audio
        || media.subtype == Subtype::Other
        || media.format_kind != FormatKind::WaveFormatEx
    {
        return Err(Error::TypeNotAccepted);
    }
    Ok(())
}

/// `effect_SetInputType` past its clear branch. Accepts PCM with 8 or 16 bits and float with 32
/// bits, each with 1 or 2 channels, and returns the header to store. The subtype is not compared
/// with the format tag, as in the C code.
///
/// Deviation: a buffer shorter than the 18-byte header is refused. Wine reads past its end.
pub fn check_input_type(media: &MediaType) -> Result<WaveFormat, Error> {
    check_common(media)?;
    let format = WaveFormat::parse(media.format).ok_or(Error::TypeNotAccepted)?;
    let depth_ok = match format.tag {
        WAVE_FORMAT_PCM => matches!(format.bits_per_sample, 8 | 16),
        WAVE_FORMAT_IEEE_FLOAT => format.bits_per_sample == 32,
        _ => false,
    };
    if !depth_ok || !matches!(format.channels, 1 | 2) {
        return Err(Error::TypeNotAccepted);
    }
    Ok(format)
}

/// `effect_SetOutputType` past its clear branch. Accepts only a type whose header is the stored
/// one, byte for byte. A short buffer is refused, as in [`check_input_type`].
pub fn check_output_type(media: &MediaType, stored: WaveFormat) -> Result<(), Error> {
    check_common(media)?;
    let format = WaveFormat::parse(media.format).ok_or(Error::TypeNotAccepted)?;
    if format != stored {
        return Err(Error::TypeNotAccepted);
    }
    Ok(())
}
