//! The audio format checks of `dlls/amstream`: `audio_sink_ReceiveConnection` in `audiostream.c`
//! and `IAudioData::SetFormat` in `audiodata.c`.

use crate::{
    Guid, Hresult, E_INVALIDARG, E_POINTER, FORMAT_WAVEFORMATEX, MEDIATYPE_AUDIO, S_OK,
    VFW_E_TYPE_NOT_ACCEPTED,
};

/// `WAVE_FORMAT_PCM`.
pub const WAVE_FORMAT_PCM: u16 = 1;

/// `sizeof(WAVEFORMATEX)`.
pub const WAVEFORMATEX_SIZE: u32 = 18;

/// The fields of an `AM_MEDIA_TYPE` that [`check_receive_media_type`] reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaType {
    pub majortype: Guid,
    pub formattype: Guid,
    pub cb_format: u32,
    /// The `wFormatTag` of `pbFormat`. It is ignored when `cb_format` is too short to hold a
    /// `WAVEFORMATEX`, where Wine does not read it either.
    pub format_tag: u16,
}

/// Accepts a PCM `WAVEFORMATEX` on the audio media type, as `audio_sink_ReceiveConnection` does
/// before it looks at the connection. A media type that is not audio gives
/// `VFW_E_TYPE_NOT_ACCEPTED`, and a non-PCM format tag gives `E_INVALIDARG`.
pub fn check_receive_media_type(mt: &MediaType) -> Hresult {
    if mt.majortype != MEDIATYPE_AUDIO
        || mt.formattype != FORMAT_WAVEFORMATEX
        || mt.cb_format < WAVEFORMATEX_SIZE
    {
        return VFW_E_TYPE_NOT_ACCEPTED;
    }
    if mt.format_tag != WAVE_FORMAT_PCM {
        return E_INVALIDARG;
    }
    S_OK
}

/// The argument check shared by `IAudioData::SetFormat` and `IAudioMediaStream::SetFormat`.
/// `None` is a null pointer, which gives `E_POINTER`.
pub fn check_set_format(format_tag: Option<u16>) -> Hresult {
    match format_tag {
        None => E_POINTER,
        Some(WAVE_FORMAT_PCM) => S_OK,
        Some(_) => E_INVALIDARG,
    }
}
