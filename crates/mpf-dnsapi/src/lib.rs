//! Port of the deterministic logic in Wine's `dlls/dnsapi`: the name checks of
//! `name.c` and the DNS message record parser of `record.c`.
//!
//! Ported:
//! - [`name::validate_name`]: `DnsValidateName_W`, with its per-format rules.
//! - [`name::name_compare`]: `DnsNameCompare_W`, a case-insensitive compare that
//!   ignores trailing dots.
//! - [`wire::extract_records`]: `DnsExtractRecordsFromMessage_UTF8`, the walk over
//!   the question, answer, authority and additional sections, with name
//!   decompression and the rdata decoders for the types Wine supports
//!   (`get_name`, `skip_name`, `skip_record`, `extract_record`, `extract_rdata`).
//!
//! Skipped: `DnsRecordListFree`, the charset conversion (`strdupX`), the record
//! sizing and list bookkeeping, since records here are owned Rust values; the
//! query, resolver and libresolv exports, which need sockets, the registry or
//! resolver configuration; and the allocation failure paths.
//!
//! Deviations:
//! - Names are byte strings cut at the first NUL, as Wine's C strings are. Label
//!   bytes are not checked for UTF-8.
//! - Header counts are read little-endian, as Wine reads them on its hosts. RFC
//!   1035 puts them in big-endian order, so a caller with such a message must
//!   swap the four count fields first.
//! - A message longer than 65535 bytes is `InvalidParameter`, because Wine takes
//!   the length as a `WORD`.
//! - `name_compare` folds case one UTF-16 code unit at a time. Wine folds with
//!   the simple (one-to-one) uppercase table, so U+1F80 folds to U+1F88 and
//!   compares equal to it. This port uses Rust's full uppercase mapping and keeps
//!   a unit whose mapping has several characters, so U+1F80 and U+1F88 differ.
//! - A SIG record whose signer name runs past the end of its rdata is `BadPacket`.
//!   Wine computes a negative signature length and reads outside the buffer
//!   (undefined behaviour), so there is no defined result to copy.

pub mod name;
pub mod wire;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A failed query, with the code Wine returns for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The message is shorter than 13 bytes (`sizeof(DNS_MESSAGE_BUFFER)`) or
    /// longer than 65535 bytes (`ERROR_INVALID_PARAMETER`).
    InvalidParameter,
    /// The name is empty, longer than 255 units, has `..`, starts with a dot that
    /// is not the whole name, or has a label over 62 units (63 for the first label)
    /// (`ERROR_INVALID_NAME`).
    InvalidName,
    /// The record type has no decoder (`DNS_ERROR_RCODE_NOT_IMPLEMENTED`).
    NotImplemented,
    /// The message or a record does not parse (`DNS_ERROR_BAD_PACKET`).
    BadPacket,
    /// The name is well formed but not an RFC name (`DNS_ERROR_NON_RFC_NAME`).
    NonRfcName,
    /// The name has a character that the format does not allow
    /// (`DNS_ERROR_INVALID_NAME_CHAR`).
    InvalidNameChar,
}

impl Error {
    /// The Win32 error code that Wine returns for this error.
    pub fn code(self) -> u32 {
        match self {
            Error::InvalidParameter => 87,
            Error::InvalidName => 123,
            Error::NotImplemented => 9004,
            Error::BadPacket => 9502,
            Error::NonRfcName => 9556,
            Error::InvalidNameChar => 9560,
        }
    }
}

/// The prefix of `s` before its first zero element, as a C string reads.
pub(crate) fn until_nul<T: PartialEq + Default>(s: &[T]) -> &[T] {
    let end = s.iter().position(|c| *c == T::default()).unwrap_or(s.len());
    &s[..end]
}
