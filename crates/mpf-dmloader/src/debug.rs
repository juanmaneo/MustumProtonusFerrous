//! Port of the portable helpers in Wine's `dlls/dmloader/debug.c`.

const DMUS_FOURCC_AUDIOPATH_FORM: u32 = u32::from_le_bytes(*b"DMAP");
const DMUS_FOURCC_BAND_FORM: u32 = u32::from_le_bytes(*b"DMBD");
const DMUS_FOURCC_CHORDMAP_FORM: u32 = u32::from_le_bytes(*b"DMPR");
const DMUS_FOURCC_CONTAINER_FORM: u32 = u32::from_le_bytes(*b"DMCN");
const FOURCC_DLS: u32 = u32::from_le_bytes(*b"DLS ");
const DMUS_FOURCC_SCRIPT_FORM: u32 = u32::from_le_bytes(*b"DMSC");
const DMUS_FOURCC_SEGMENT_FORM: u32 = u32::from_le_bytes(*b"DMSG");
const DMUS_FOURCC_STYLE_FORM: u32 = u32::from_le_bytes(*b"DMST");
const DMUS_FOURCC_TOOLGRAPH_FORM: u32 = u32::from_le_bytes(*b"DMTG");
const DMUS_FOURCC_TRACK_FORM: u32 = u32::from_le_bytes(*b"DMTK");
const FOURCC_WAVE: u32 = u32::from_le_bytes(*b"WAVE");

/// `STREAM_SEEK_*` from `objidlbase.idl`.
const STREAM_SEEK_SET: u32 = 0;
const STREAM_SEEK_CUR: u32 = 1;
const STREAM_SEEK_END: u32 = 2;

/// `IS_VALID_DMFORM`: whether `chunk_id` is a DirectMusic form ID, `DLS `, or `WAVE`.
pub fn is_valid_dmform(chunk_id: u32) -> bool {
    matches!(
        chunk_id,
        DMUS_FOURCC_AUDIOPATH_FORM
            | DMUS_FOURCC_BAND_FORM
            | DMUS_FOURCC_CHORDMAP_FORM
            | DMUS_FOURCC_CONTAINER_FORM
            | FOURCC_DLS
            | DMUS_FOURCC_SCRIPT_FORM
            | DMUS_FOURCC_SEGMENT_FORM
            | DMUS_FOURCC_STYLE_FORM
            | DMUS_FOURCC_TOOLGRAPH_FORM
            | DMUS_FOURCC_TRACK_FORM
            | FOURCC_WAVE
    )
}

/// `resolve_STREAM_SEEK`: the name of a `STREAM_SEEK_*` origin, or `"()"` for any other value.
pub fn resolve_stream_seek(flag: u32) -> &'static str {
    match flag {
        STREAM_SEEK_SET => "STREAM_SEEK_SET",
        STREAM_SEEK_CUR => "STREAM_SEEK_CUR",
        STREAM_SEEK_END => "STREAM_SEEK_END",
        _ => "()",
    }
}
