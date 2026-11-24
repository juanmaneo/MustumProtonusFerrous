use crate::c_str;

const CR: u16 = b'\r' as u16;
const LF: u16 = b'\n' as u16;

/// Copies the additional headers and makes sure the last line ends in CRLF. A missing
/// value stays missing, and empty headers stay empty.
pub fn normalize_headers(headers: Option<&[u16]>) -> Option<Vec<u16>> {
    headers.map(|headers| {
        let mut out = c_str(headers).to_vec();
        if out.last().is_some_and(|&last| last != LF && last != CR) {
            out.extend_from_slice(&[CR, LF]);
        }
        out
    })
}
