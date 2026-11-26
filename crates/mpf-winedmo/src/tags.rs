//! `demuxer_stream_lang` and `demuxer_stream_name` from `dlls/winedmo/unix_demuxer.c`: the metadata
//! lookup, and the `lstrcpynA` copy into the caller's fixed-size buffer.

use crate::status::{STATUS_NOT_FOUND, STATUS_SUCCESS};

/// `av_dict_get(metadata, key, NULL, AV_DICT_IGNORE_SUFFIX)`: the first entry whose key starts with
/// `key`, compared case-insensitively. Entries are in insertion order.
pub fn find_tag<'a>(metadata: &[(&str, &'a str)], key: &str) -> Option<&'a str> {
    metadata
        .iter()
        .find(|(entry_key, _)| {
            entry_key.len() >= key.len()
                && entry_key.as_bytes()[..key.len()].eq_ignore_ascii_case(key.as_bytes())
        })
        .map(|&(_, value)| value)
}

/// `lstrcpynA(dst, src, dst.len())`: copies at most `dst.len() - 1` bytes and always writes the NUL
/// that ends the copy. An empty `dst` is left alone.
pub fn lstrcpyn_a(dst: &mut [u8], src: &str) {
    if dst.is_empty() {
        return;
    }
    let copied = src.len().min(dst.len() - 1);
    dst[..copied].copy_from_slice(&src.as_bytes()[..copied]);
    dst[copied] = 0;
}

/// `demuxer_stream_lang`: the `language` tag of a stream, copied into `buffer`.
pub fn stream_language(metadata: &[(&str, &str)], buffer: &mut [u8]) -> u32 {
    copy_tag(metadata, "language", buffer)
}

/// `demuxer_stream_name`: the `title` tag of a stream, copied into `buffer`.
pub fn stream_name(metadata: &[(&str, &str)], buffer: &mut [u8]) -> u32 {
    copy_tag(metadata, "title", buffer)
}

/// The shared body of the two lookups. A missing tag leaves `buffer` untouched.
fn copy_tag(metadata: &[(&str, &str)], key: &str, buffer: &mut [u8]) -> u32 {
    match find_tag(metadata, key) {
        None => STATUS_NOT_FOUND,
        Some(value) => {
            lstrcpyn_a(buffer, value);
            STATUS_SUCCESS
        }
    }
}
