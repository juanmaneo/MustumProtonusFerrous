//! Headless subset of Wine's `dlls/amstream`: the media-type and format checks of the audio and
//! video stream pins and of the audio data format setter.
//!
//! Ported, checked against the verbatim C source by `oracle_checks.rs`:
//! - [`audio`]: the PCM media-type check of `audio_sink_ReceiveConnection` and the argument
//!   check of `IAudioData::SetFormat`.
//! - [`video`]: the pixel format to subtype mapping `subtype_from_pf`, the stream format
//!   compatibility check `is_format_compatible`, and the decision of `ddraw_sink_QueryAccept`.
//!
//! Skipped: the COM objects and their reference counts, the critical sections, the filter graph
//! and its state, the DirectDraw surfaces and palettes, the sample queues and the format
//! negotiation through `memcmp` against the connected pin.

pub mod audio;
pub mod video;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// An `HRESULT`.
pub type Hresult = u32;

pub const S_OK: Hresult = 0x0000_0000;
pub const E_POINTER: Hresult = 0x8000_4003;
pub const E_INVALIDARG: Hresult = 0x8007_0057;
pub const VFW_E_TYPE_NOT_ACCEPTED: Hresult = 0x8004_022A;

/// A `GUID`, with the fields of Wine's `guiddef.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const MEDIATYPE_AUDIO: Guid = Guid {
    data1: 0x7364_7561,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

pub const MEDIATYPE_VIDEO: Guid = Guid {
    data1: 0x7364_6976,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71],
};

pub const FORMAT_VIDEOINFO: Guid = Guid {
    data1: 0x0558_9f80,
    data2: 0xc356,
    data3: 0x11ce,
    data4: [0xbf, 0x01, 0x00, 0xaa, 0x00, 0x55, 0x59, 0x5a],
};

pub const FORMAT_WAVEFORMATEX: Guid = Guid {
    data1: 0x0558_9f81,
    data2: 0xc356,
    data3: 0x11ce,
    data4: [0xbf, 0x01, 0x00, 0xaa, 0x00, 0x55, 0x59, 0x5a],
};

pub const MEDIASUBTYPE_RGB8: Guid = Guid {
    data1: 0xe436_eb7a,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

pub const MEDIASUBTYPE_RGB555: Guid = Guid {
    data1: 0xe436_eb7c,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

pub const MEDIASUBTYPE_RGB565: Guid = Guid {
    data1: 0xe436_eb7b,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

pub const MEDIASUBTYPE_RGB24: Guid = Guid {
    data1: 0xe436_eb7d,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

pub const MEDIASUBTYPE_RGB32: Guid = Guid {
    data1: 0xe436_eb7e,
    data2: 0x524f,
    data3: 0x11ce,
    data4: [0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70],
};

/// Validates a PCM format tag through the crate's primary audio API.
pub fn validate_audio_format(format_tag: Option<u16>) -> Hresult {
    audio::check_set_format(format_tag)
}

#[cfg(test)]
mod root_api_checks {
    #[test]
    fn audio_format_validation_matches_audio_module() {
        assert_eq!(
            super::validate_audio_format(Some(1)),
            super::audio::check_set_format(Some(1))
        );
    }
}
