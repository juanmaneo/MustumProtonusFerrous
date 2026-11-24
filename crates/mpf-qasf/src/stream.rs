//! The stream setup of `WMT_OPENED` in Wine's `dlls/qasf/asfreader.c`: how many output streams the
//! reader keeps, and the names the source pins are given.

/// The number of streams the filter has room for: `ARRAY_SIZE(filter->streams)`.
pub const MAX_STREAMS: u32 = 16;

/// The stream count kept after `GetOutputCount`. A failed call (`None`) keeps no streams, and a
/// count above `MAX_STREAMS` is cut down to it.
pub fn open_stream_count(output_count: Option<u32>) -> u32 {
    match output_count {
        None => 0,
        Some(count) => count.min(MAX_STREAMS),
    }
}

/// The major type of a stream's first output format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Major {
    Video,
    Other,
}

/// The name of stream `index`. Only video gets the video name. A stream whose media type could not
/// be read has the zeroed major type, which is not video, so the caller passes `Major::Other`.
pub fn raw_stream_name(major: Major, index: u32) -> String {
    match major {
        Major::Video => format!("Raw Video {index}"),
        Major::Other => format!("Raw Audio {index}"),
    }
}
