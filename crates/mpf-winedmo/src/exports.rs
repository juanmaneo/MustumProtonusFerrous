//! The export table of `dlls/winedmo/winedmo.spec`, and the port behind each export.

/// The exports of `winedmo.spec`, in spec order. All eight are `@ cdecl`, none is a `@ stub`.
pub const EXPORTS: [&str; 8] = [
    "winedmo_demuxer_check",
    "winedmo_demuxer_create",
    "winedmo_demuxer_destroy",
    "winedmo_demuxer_read",
    "winedmo_demuxer_seek",
    "winedmo_demuxer_stream_lang",
    "winedmo_demuxer_stream_name",
    "winedmo_demuxer_stream_type",
];

/// The Rust function that ports an export's logic. Exports that need FFmpeg or a demuxer handle
/// have no entry and return `None`. `winedmo_demuxer_seek` is partial: only its timestamp scaling
/// is ported.
pub fn ported_as(name: &str) -> Option<&'static str> {
    match name {
        "winedmo_demuxer_check" => Some("demuxer_check"),
        "winedmo_demuxer_seek" => Some("seek_timestamp"),
        "winedmo_demuxer_stream_lang" => Some("stream_language"),
        "winedmo_demuxer_stream_name" => Some("stream_name"),
        "winedmo_demuxer_stream_type" => Some("stream_media_type"),
        _ => None,
    }
}
