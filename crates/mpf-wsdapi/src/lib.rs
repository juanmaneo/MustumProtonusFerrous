//! Port of the deterministic helpers in Wine's `dlls/wsdapi/soap.c`: the decimal
//! parser behind the WS-Discovery message numbers, the trailing-slash trim of a
//! URI, and the space-joined URI list that goes into a SOAP message.
//!
//! Ported:
//! - [`number::parse_uint64`] (`str_to_uint64`, with `read_isspace`).
//! - [`number::parse_wide_uint64`] (`wide_text_to_ulonglong`, the UTF-16 wrapper).
//! - [`uri::trim_trailing_slash`] (`trim_trailing_slash`).
//! - [`uri::build_uri_list`] (`build_uri_list`, with its capacity check).
//!
//! Skipped, because they need a host or a shared structure:
//! - `build_types_list` and `build_types_list_from_string`. The latter assigns
//!   `prev_list->Next` where `prev_list` is never set, so an input with two or
//!   more names dereferences NULL. The former also records namespaces.
//! - `is_name_in_list`, `is_prefix_unique` and `generate_namespace_prefix`. They
//!   compare with `lstrcmpW`, which is a locale collation, so the result depends on
//!   the host.
//! - Everything else in the file: XML and SOAP writing and reading, COM objects,
//!   the networking code, and the linked allocator.
//!
//! Deviations:
//! - `parse_wide_uint64` maps each non-ASCII UTF-16 unit to one byte instead of
//!   converting to UTF-8. Every byte of a non-ASCII character is neither a digit
//!   nor a blank, so the result is the same as Wine's.
//! - On an error Wine may leave a partial value in `*ret`. The port returns only
//!   the error.
//! - `parse_uint64` keeps Wine's unsigned wrap in `max - digit`. For a `max` below
//!   9 a digit can slip past the bound. Wine only calls it with `UINT64_MAX`.
//! - `build_uri_list` with an empty list returns `Error::InvalidArg`. Wine
//!   dereferences NULL there.
//! - `build_uri_list` takes the number Wine passes as `buffer_size` and compares
//!   it against counts of UTF-16 units, as Wine does. Wine's callers pass a byte
//!   count (`WSD_MAX_TEXT_LENGTH * sizeof(WCHAR)`) into a buffer of
//!   `WSD_MAX_TEXT_LENGTH` units, so Wine can write past its buffer. The port
//!   cannot, and a caller must pass a count of units.

pub mod number;
pub mod uri;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// A failed call, with the HRESULT that Wine returns for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The text is empty (`E_FAIL`, from `wide_text_to_ulonglong`).
    Fail,
    /// The URI list is empty, or it does not fit the buffer (`E_INVALIDARG`).
    InvalidArg,
    /// The text has a character that is neither a digit nor a blank
    /// (`WS_E_INVALID_FORMAT`).
    InvalidFormat,
    /// The value is above the bound (`WS_E_NUMERIC_OVERFLOW`).
    NumericOverflow,
}

impl Error {
    /// The HRESULT that Wine returns for this error.
    pub fn hresult(self) -> u32 {
        match self {
            Error::Fail => 0x8000_4005,
            Error::InvalidArg => 0x8007_0057,
            Error::InvalidFormat => 0x803D_0000,
            Error::NumericOverflow => 0x803D_0002,
        }
    }
}

/// The prefix of `s` before its first zero element, as a C string reads.
pub(crate) fn until_nul<T: PartialEq + Default>(s: &[T]) -> &[T] {
    let end = s.iter().position(|c| *c == T::default()).unwrap_or(s.len());
    &s[..end]
}
