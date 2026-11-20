//! Deterministic format helpers from Wine's `dlls/qcap`.
//!
//! The capture filters depend on devices, COM, and operating-system services and are not ported.
//! This crate keeps the pure input-media-type check used by the AVI mux input pin.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The media major types inspected by the AVI mux input pin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum MediaMajorType {
    Audio = 0,
    Interleaved = 1,
    Video = 2,
    Other = 3,
}

/// The format-block types inspected by the AVI mux input pin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum FormatType {
    WaveFormatEx = 0,
    DvInfo = 1,
    VideoInfo = 2,
    None = 3,
    Other = 4,
}

/// Returns whether qcap's AVI mux accepts this major-type/format-type pair.
///
/// This mirrors `sink_query_accept` in `dlls/qcap/avimux.c`; it checks only GUID identity, not the
/// contents or size of a media type's format block.
pub const fn accepts_media_type(major: MediaMajorType, format: FormatType) -> bool {
    matches!(
        (major, format),
        (MediaMajorType::Audio, FormatType::WaveFormatEx)
            | (MediaMajorType::Interleaved, FormatType::DvInfo)
            | (MediaMajorType::Video, FormatType::VideoInfo)
            | (MediaMajorType::Video, FormatType::DvInfo)
    )
}
