//! `demuxer_check` from `dlls/winedmo/unix_demuxer.c`: the MIME types that FFmpeg has an input
//! format for.

use crate::status::{STATUS_NOT_SUPPORTED, STATUS_SUCCESS};

/// The MIME types that `demuxer_check` accepts, with the FFmpeg input format each one selects.
const INPUT_FORMATS: [(&str, &str); 8] = [
    ("video/mp4", "mp4"),
    ("video/avi", "avi"),
    ("audio/wav", "wav"),
    ("audio/x-ms-wma", "asf"),
    ("video/x-ms-wmv", "asf"),
    ("video/x-ms-asf", "asf"),
    ("video/mpeg", "mpeg"),
    ("audio/mp3", "mp3"),
];

/// The FFmpeg input format for a MIME type. The comparison is exact and case-sensitive.
pub fn input_format(mime_type: &str) -> Option<&'static str> {
    INPUT_FORMATS
        .iter()
        .find(|(mime, _)| *mime == mime_type)
        .map(|&(_, format)| format)
}

/// `demuxer_check`: `STATUS_SUCCESS` when the MIME type has an input format, else
/// `STATUS_NOT_SUPPORTED`.
pub fn demuxer_check(mime_type: &str) -> u32 {
    if input_format(mime_type).is_some() {
        STATUS_SUCCESS
    } else {
        STATUS_NOT_SUPPORTED
    }
}
