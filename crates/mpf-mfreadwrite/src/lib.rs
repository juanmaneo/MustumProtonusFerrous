//! mfreadwrite subset for MPF: source reader stream selection, read-index resolution, transform
//! lookup, video-processor policy and the start-state plan.
//!
//! Reference Wine's `dlls/mfreadwrite` implements the source reader and sink writer on top of COM
//! objects, media sources, transforms, critical sections and worker threads. This crate ports the
//! pure functions of `reader.c` that decide things from stream state. They work on plain integers,
//! booleans and slices, with no COM objects, allocation, threads or handles.
//!
//! Ported (pure logic, verified against the verbatim C by oracle rows and edge tests):
//! - `next_selected_stream`: picks the selected stream with the lowest last sample timestamp,
//!   skipping drained streams, and falls back to the first selected stream.
//! - `stream_read_index`: resolves `MF_SOURCE_READER_FIRST_VIDEO_STREAM`, `..._FIRST_AUDIO_STREAM`
//!   and `..._ANY_STREAM`, and rejects out-of-range and deselected streams.
//! - `transform_at_index`: maps an index over the visible (not hidden) transforms of a stream.
//! - `allow_video_processor`: the advanced and plain video-processing attribute policy.
//! - `is_source_stopped`: whether every stream that has a media stream reported `MEStreamStopped`.
//! - `first_stream_index`: the first stream whose major type matches.
//! - `start_source`: updates the selected and presented flags and decides whether `Start` runs.
//!
//! Deviations from the C:
//! - Descriptor lookups are replaced by the `selected` and `major` values they return. A failed
//!   descriptor lookup becomes a `false` selection or a `None` major type, which is what the C
//!   does with a failed call.
//! - `stream_read_index` and `next_selected_stream` return `Result<u32, Hresult>`, where the C
//!   writes through an out-pointer and returns an `HRESULT`.
//! - `start_source` takes a closure for `IMFMediaSource::Start`. The closure receives the
//!   `VT_EMPTY` flag, which is true when the source is already started. The C passes the
//!   `PROPVARIANT` built from it.
//! - `allow_video_processor` returns `(advanced, allowed)` where the C writes `advanced` through
//!   an out-pointer and returns `allowed`.
//!
//! Skipped (reason per item):
//! - `get_read_result`, `source_reader_flush`, the async command and stream event callbacks,
//!   sample queueing and transform draining: they manipulate queues, samples and transforms.
//! - `got_response_for_stream` and the sink writer's `sink_writer_get_stream`: a list membership
//!   test and a bounds check, too small to be worth a port on their own.
//! - Everything else in `reader.c` and `writer.c`: COM vtables, `IMFSourceReader` and
//!   `IMFSinkWriter` methods, media type conversion, the video processor and the decoder setup,
//!   `MFCreateSourceReader*`, `DllGetClassObject` and the class factory. They need COM objects,
//!   allocation, threads, handles, the registry or the media pipeline.

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `HRESULT`.
pub type Hresult = i32;

/// `S_OK`.
pub const S_OK: Hresult = 0;

/// `E_FAIL`.
pub const E_FAIL: Hresult = 0x8000_4005u32 as i32;

/// `MF_E_INVALIDREQUEST`.
pub const MF_E_INVALIDREQUEST: Hresult = 0xC00D_36B2u32 as i32;

/// `MF_E_INVALIDSTREAMNUMBER`.
pub const MF_E_INVALIDSTREAMNUMBER: Hresult = 0xC00D_36B3u32 as i32;

/// `MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED`.
pub const MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED: Hresult = 0xC00D_3E9Cu32 as i32;

/// `MF_SOURCE_READER_FIRST_VIDEO_STREAM`: the first video stream of the source.
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: u32 = 0xFFFF_FFFC;

/// `MF_SOURCE_READER_FIRST_AUDIO_STREAM`: the first audio stream of the source.
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: u32 = 0xFFFF_FFFD;

/// `MF_SOURCE_READER_ANY_STREAM`: the selected stream with the lowest timestamp.
pub const MF_SOURCE_READER_ANY_STREAM: u32 = 0xFFFF_FFFE;

/// `MF_SOURCE_READER_INVALID_STREAM_INDEX`: no stream matches.
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: u32 = 0xFFFF_FFFF;

/// `STREAM_FLAG_SELECTED`: mirrors the stream's selection in the presentation descriptor.
pub const STREAM_FLAG_SELECTED: u32 = 0x2;

/// `STREAM_FLAG_PRESENTED`: set if the stream was selected the last time `Start` succeeded.
pub const STREAM_FLAG_PRESENTED: u32 = 0x4;

/// `STREAM_FLAG_STOPPED`: the stream received `MEStreamStopped`.
pub const STREAM_FLAG_STOPPED: u32 = 0x8;

/// A COM `GUID`, laid out as in the Windows header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `MFMediaType_Video`.
pub const MF_MEDIA_TYPE_VIDEO: Guid = Guid {
    data1: 0x7364_6976,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// `MFMediaType_Audio`.
pub const MF_MEDIA_TYPE_AUDIO: Guid = Guid {
    data1: 0x7364_7561,
    data2: 0x0000,
    data3: 0x0010,
    data4: [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71],
};

/// The per-stream state that stream selection reads: the descriptor selection, the end-of-stream
/// state, the number of queued responses and the timestamp of the last sample.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamState {
    pub selected: bool,
    pub eos: bool,
    pub responses: u32,
    pub last_sample_ts: i64,
}

/// The per-stream state that `is_source_stopped` reads: whether a media stream exists and the
/// `STREAM_FLAG_*` bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamSlot {
    pub has_stream: bool,
    pub flags: u32,
}

const fn succeeded(hr: Hresult) -> bool {
    hr >= 0
}

/// `source_reader_get_next_selected_stream`: the selected stream whose last sample has the lowest
/// timestamp. Drained streams (end of stream with no queued responses) are skipped, and if every
/// selected stream is drained, or none has a timestamp below `i64::MAX`, the first selected stream
/// is used.
///
/// Mirrors Wine's `source_reader_get_next_selected_stream` in `dlls/mfreadwrite/reader.c`.
pub fn next_selected_stream(streams: &[StreamState]) -> Result<u32, Hresult> {
    let mut first_selected = None;
    let mut min_ts = i64::MAX;
    let mut chosen = None;

    for (index, stream) in streams.iter().enumerate() {
        if !stream.selected {
            continue;
        }
        let index = index as u32;
        first_selected.get_or_insert(index);

        let drained = stream.eos && stream.responses == 0;
        if !drained && stream.last_sample_ts < min_ts {
            min_ts = stream.last_sample_ts;
            chosen = Some(index);
        }
    }

    match first_selected {
        None => Err(MF_E_MEDIA_SOURCE_NO_STREAMS_SELECTED),
        Some(first) => Ok(chosen.unwrap_or(first)),
    }
}

/// `source_reader_get_stream_read_index`: resolves a `ReadSample` stream index to a selected
/// stream. The first-video and first-audio indices map to the cached stream indices, and
/// `MF_SOURCE_READER_ANY_STREAM` delegates to `next_selected_stream`. A stream index past the end
/// of the descriptor gives `MF_E_INVALIDSTREAMNUMBER`, and a deselected stream gives
/// `MF_E_INVALIDREQUEST`.
///
/// Mirrors Wine's `source_reader_get_stream_read_index` in `dlls/mfreadwrite/reader.c`.
pub fn stream_read_index(
    index: u32,
    first_video: u32,
    first_audio: u32,
    streams: &[StreamState],
) -> Result<u32, Hresult> {
    let stream_index = match index {
        MF_SOURCE_READER_FIRST_VIDEO_STREAM => first_video,
        MF_SOURCE_READER_FIRST_AUDIO_STREAM => first_audio,
        MF_SOURCE_READER_ANY_STREAM => return next_selected_stream(streams),
        _ => index,
    };

    match streams.get(stream_index as usize) {
        None => Err(MF_E_INVALIDSTREAMNUMBER),
        Some(stream) if !stream.selected => Err(MF_E_INVALIDREQUEST),
        Some(_) => Ok(stream_index),
    }
}

/// `get_transform_at_index`: the position in `hidden` of the `index`-th entry that is not hidden.
/// Returns `None` when fewer than `index + 1` entries are visible.
///
/// Mirrors Wine's `get_transform_at_index` in `dlls/mfreadwrite/reader.c`.
pub fn transform_at_index(hidden: &[bool], index: u32) -> Option<usize> {
    let mut remaining = index;

    for (position, &is_hidden) in hidden.iter().enumerate() {
        if is_hidden {
            continue;
        }
        if remaining == 0 {
            return Some(position);
        }
        remaining -= 1;
    }

    None
}

/// `source_reader_allow_video_processor`: decides whether the video processor may be used.
/// `has_attributes` is whether the reader has an attribute store. `advanced` and `video` are the
/// values of `MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING` and
/// `MF_SOURCE_READER_ENABLE_VIDEO_PROCESSING`, or `None` when the attribute is absent.
///
/// Returns `(advanced, allowed)`. Without attributes both are `false`. A present plain value
/// decides `allowed` unless advanced processing is enabled, which always allows it.
///
/// Mirrors Wine's `source_reader_allow_video_processor` in `dlls/mfreadwrite/reader.c`.
pub fn allow_video_processor(
    has_attributes: bool,
    advanced: Option<u32>,
    video: Option<u32>,
) -> (bool, bool) {
    if !has_attributes {
        return (false, false);
    }

    let advanced = advanced.is_some_and(|value| value != 0);
    let allowed = match video {
        Some(value) => value != 0 || advanced,
        None => advanced,
    };

    (advanced, allowed)
}

/// `source_reader_is_source_stopped`: whether the source is stopped and every stream that has a
/// media stream has `STREAM_FLAG_STOPPED` set.
///
/// Mirrors Wine's `source_reader_is_source_stopped` in `dlls/mfreadwrite/reader.c`.
pub fn is_source_stopped(source_stopped: bool, streams: &[StreamSlot]) -> bool {
    source_stopped
        && streams
            .iter()
            .all(|slot| !slot.has_stream || (slot.flags & STREAM_FLAG_STOPPED) != 0)
}

/// `reader_get_first_stream_index`: the index of the first stream whose major type equals
/// `major`, or `MF_SOURCE_READER_INVALID_STREAM_INDEX`. A stream whose descriptor, media type
/// handler or major type cannot be read is `None`, and is skipped.
///
/// Mirrors Wine's `reader_get_first_stream_index` in `dlls/mfreadwrite/reader.c`.
pub fn first_stream_index(majors: &[Option<Guid>], major: &Guid) -> u32 {
    majors
        .iter()
        .position(|candidate| *candidate == Some(*major))
        .map_or(MF_SOURCE_READER_INVALID_STREAM_INDEX, |index| index as u32)
}

/// `source_reader_start_source`: refreshes `STREAM_FLAG_SELECTED` from `selected`, then calls
/// `Start` when the source is stopped or a stream's selection differs from its presented state.
/// On success every selected stream gains `STREAM_FLAG_PRESENTED`.
///
/// `start` receives `position_empty`, which is true when the source is already started. The C
/// passes `VT_EMPTY` in that case and `VT_I8` zero otherwise. `selected` and `flags` must hold one
/// entry per stream.
///
/// Mirrors Wine's `source_reader_start_source` in `dlls/mfreadwrite/reader.c`.
pub fn start_source(
    source_started: bool,
    selected: &[bool],
    flags: &mut [u32],
    start: &mut dyn FnMut(bool) -> Hresult,
) -> Hresult {
    for (stream_flags, &is_selected) in flags.iter_mut().zip(selected) {
        if is_selected {
            *stream_flags |= STREAM_FLAG_SELECTED;
        } else {
            *stream_flags &= !STREAM_FLAG_SELECTED;
        }
    }

    let selection_changed = flags.iter().any(|&stream_flags| {
        (stream_flags & STREAM_FLAG_SELECTED != 0) != (stream_flags & STREAM_FLAG_PRESENTED != 0)
    });
    if source_started && !selection_changed {
        return S_OK;
    }

    let hr = start(source_started);
    if succeeded(hr) {
        for stream_flags in flags.iter_mut() {
            if *stream_flags & STREAM_FLAG_SELECTED != 0 {
                *stream_flags |= STREAM_FLAG_PRESENTED;
            }
        }
    }

    hr
}
