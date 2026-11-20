//! PEM framing: the header and trailer lines around the base64 body.

use crate::{base64, Error};

pub(crate) struct Frame {
    pub(crate) header: &'static str,
    pub(crate) trailer: &'static str,
}

pub(crate) const CERTIFICATE: Frame = Frame {
    header: "-----BEGIN CERTIFICATE-----",
    trailer: "-----END CERTIFICATE-----",
};

pub(crate) const REQUEST: Frame = Frame {
    header: "-----BEGIN NEW CERTIFICATE REQUEST-----",
    trailer: "-----END NEW CERTIFICATE REQUEST-----",
};

pub(crate) const X509_CRL: Frame = Frame {
    header: "-----BEGIN X509 CRL-----",
    trailer: "-----END X509 CRL-----",
};

const BEGIN: &[u8] = b"-----BEGIN ";
const END: &[u8] = b"-----END ";
const DELIMITER: &[u8] = b"-----";

/// Writes the header, the base64 body and the trailer, each line followed by `line_end`.
pub(crate) fn wrap(binary: &[u8], line_end: &str, frame: &Frame) -> Vec<u8> {
    let mut text = Vec::new();
    text.extend_from_slice(frame.header.as_bytes());
    text.extend_from_slice(line_end.as_bytes());
    text.extend(base64::encode(binary, line_end));
    text.extend_from_slice(frame.trailer.as_bytes());
    text.extend_from_slice(line_end.as_bytes());
    text
}

/// Finds the first PEM block. Returns the offset of its header and the base64 body.
///
/// The body ends before the line break that precedes `-----END `. The search stops at the
/// first NUL, as Wine's `strstr` does.
pub(crate) fn locate(input: &[u8]) -> Result<(usize, &[u8]), Error> {
    if input.len() < BEGIN.len() + END.len() {
        return Err(Error::InvalidData);
    }
    let text = &input[..input.iter().position(|&c| c == 0).unwrap_or(input.len())];

    let begin = find(text, BEGIN, 0).ok_or(Error::InvalidData)?;
    let delimiter = find(text, DELIMITER, begin + BEGIN.len()).ok_or(Error::InvalidData)?;
    let mut data = delimiter + DELIMITER.len();
    if text.get(data) == Some(&b'\r') {
        data += 1;
    }
    if text.get(data) == Some(&b'\n') {
        data += 1;
    }

    let end = find(text, END, data).ok_or(Error::InvalidData)?;
    let mut stop = end;
    if text[stop - 1] == b'\n' {
        stop -= 1;
    }
    if text[stop - 1] == b'\r' {
        stop -= 1;
    }
    if stop < data {
        return Err(Error::InvalidData);
    }
    Ok((begin, &text[data..stop]))
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack[from..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|offset| from + offset)
}
