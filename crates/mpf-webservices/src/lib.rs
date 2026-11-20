//! Headless port of the deterministic parts of Wine's `dlls/webservices`: the URL parser and
//! encoder in `url.c`, and the pure text and date helpers in `reader.c`.
//!
//! Ported:
//! - `WsDecodeUrl` and `WsEncodeUrl`, with the scheme table and the escaping rules: [url].
//! - `str_to_bool`, `str_to_int64`, `str_to_uint64`, `str_to_bytes` (base64) and `decode_text`
//!   (the named and numeric character entities): [text].
//! - `str_to_datetime`, `datetime_to_filetime` and `filetime_to_datetime`: [datetime].
//!
//! Skipped:
//! - `str_to_double` and `str_to_float`. Their hand-written scanners accept NaN and Infinity and
//!   are left for a later pass.
//! - `str_to_string`, `str_to_unique_id`, `str_to_qname` and `str_to_xml_string`. They allocate on
//!   the WS heap or use the XML dictionary.
//! - The `read_*` and `text_to_*` reader state machine, which works on `WS_XML_READER` and
//!   `WS_HEAP`. The heap and `WS_ERROR` parameters, and their allocation-failure paths, are
//!   dropped.
//!
//! Deviations from Wine:
//! - In [url::decode_url], Wine keeps only the low byte of each UTF-16 unit when it reads an
//!   escape, so a non-ASCII unit in either digit position whose low byte is a hex digit decodes
//!   as that digit. `%` followed by U+0130 and `0` decodes to U+0000 there, and `%4` followed by
//!   U+0130 decodes to `@`. This port reads only ASCII hex digits, so the `%` stays literal.
//! - In [text::str_to_uint64], Wine computes `max - digit` without a sign check. A digit above
//!   `max` therefore passes when `max < 9`. This port returns [Error::NumericOverflow] instead.
//!   Wine's own callers pass maximums of 255 and up, so only direct calls can tell the two apart.

pub mod datetime;
pub mod text;
pub mod url;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// The failures of the ported functions, each with the HRESULT that Wine returns for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// `E_INVALIDARG`: two arguments disagree, such as a port number and its text.
    InvalidArgument,
    /// `E_NOTIMPL`: the flags are not zero. Wine defines no flags for these calls.
    NotImplemented,
    /// `WS_E_INVALID_FORMAT`: the text does not have the required form.
    InvalidFormat,
    /// `WS_E_NUMERIC_OVERFLOW`: the value does not fit its type or its allowed range.
    NumericOverflow,
    /// `WS_E_QUOTA_EXCEEDED`: the percent-decoded bytes are not valid UTF-8.
    QuotaExceeded,
}

impl Error {
    /// The HRESULT that Wine returns for this error.
    pub const fn hresult(self) -> i32 {
        match self {
            Self::InvalidArgument => 0x8007_0057_u32 as i32,
            Self::NotImplemented => 0x8000_4001_u32 as i32,
            Self::InvalidFormat => 0x803d_0000_u32 as i32,
            Self::NumericOverflow => 0x803d_0002_u32 as i32,
            Self::QuotaExceeded => 0x803d_0008_u32 as i32,
        }
    }
}
