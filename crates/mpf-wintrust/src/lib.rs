//! wintrust subset for MPF: the deterministic logic of `dlls/wintrust`.
//!
//! Ported (pure functions over bytes and values, checked against Wine's verbatim source):
//! - `length`: `CRYPT_GetLen` and `CRYPT_EncodeLen` from `asn.c`, the DER length octets.
//! - `encode`: `CRYPT_AsnEncodeBool`, `CRYPT_AsnEncodeInteger`, `CRYPT_AsnEncodeInt`, and
//!   `CRYPT_AsnEncodeOctets`, plus the buffer protocol of `CryptEncodeObject` (`write_to`).
//! - `decode`: `CRYPT_AsnDecodeBool`, `CRYPT_AsnDecodeInt`, and `CRYPT_AsnDecodeOctets`.
//!   Octets are borrowed from the input, so Wine's size query and `CRYPT_DECODE_NOCOPY_FLAG`
//!   have no counterpart.
//! - `trust`: `WINTRUST_TrustStatusToConfidence` and `WINTRUST_TrustStatusToError` from `softpub.c`.
//!
//! Skipped (they need a DER tree walker, structure types, or OS state):
//! - OID, BMP string, bit string, and sequence codecs, and the `WVTAsn1*` structure
//!   encoders and decoders built on them (SPC link, PE image data, indirect data content,
//!   catalog member and name-value, financial criteria, Opus info).
//! - `CRYPT_AsnDecodeSPCLinkInternal` and the sequence item walker that drives those codecs.
//! - `SoftpubCheckCert`, which walks a `CRYPT_PROVIDER_DATA` chain.
//! - Registry, crypt32 store, file, and handle code, and the `FIXME` and `TRACE` logging.
//!
//! Deviations from the reference:
//! (a) `content` (used by the integer and octets decoders) returns `EndOfData` when the content
//!     runs past the input. Wine's `CRYPT_GetLen` accepts `01 01` (one content octet short), and
//!     its decoders then read past the input.
//! (b) Encoding an integer whose octets are all `0xff` pads it as `02 01 ff`. Wine reads
//!     `pbData[-1]` to decide on the pad.
//! (c) Decoding an integer sign-extends with two's-complement shifts. Wine's `val <<= 8` on a
//!     negative value is undefined behaviour.
//! (d) Encoded lengths are `usize`, not `DWORD`. Lengths above `u32::MAX` encode with five or
//!     more length octets, which `length::decode` rejects as `TooLarge`.

pub mod decode;
pub mod encode;
pub mod length;
pub mod trust;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

/// `CRYPT_E_ASN1_EOD`: the content runs past the end of the input.
pub const CRYPT_E_ASN1_EOD: u32 = 0x8009_3102;
/// `CRYPT_E_ASN1_CORRUPT`: a malformed length, or a value out of range for its type.
pub const CRYPT_E_ASN1_CORRUPT: u32 = 0x8009_3103;
/// `CRYPT_E_ASN1_LARGE`: the length takes more octets than the decoder supports.
pub const CRYPT_E_ASN1_LARGE: u32 = 0x8009_3104;
/// `CRYPT_E_ASN1_BADTAG`: the tag octet is not the one the decoder expects.
pub const CRYPT_E_ASN1_BADTAG: u32 = 0x8009_310b;
/// `ERROR_MORE_DATA`: the output buffer is smaller than the encoding.
pub const ERROR_MORE_DATA: u32 = 234;

const ASN_BOOL: u8 = 0x01;
const ASN_INTEGER: u8 = 0x02;
const ASN_OCTETSTRING: u8 = 0x04;

/// A failed codec call, with the Win32 code that Wine reports for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// `CRYPT_E_ASN1_CORRUPT`.
    Corrupt,
    /// `CRYPT_E_ASN1_BADTAG`.
    BadTag,
    /// `CRYPT_E_ASN1_EOD`.
    EndOfData,
    /// `CRYPT_E_ASN1_LARGE`.
    TooLarge,
    /// `ERROR_MORE_DATA`: the output buffer holds fewer than `needed` octets.
    MoreData { needed: usize },
}

impl Error {
    /// The Win32 error code that Wine reports for this failure.
    pub fn code(self) -> u32 {
        match self {
            Error::Corrupt => CRYPT_E_ASN1_CORRUPT,
            Error::BadTag => CRYPT_E_ASN1_BADTAG,
            Error::EndOfData => CRYPT_E_ASN1_EOD,
            Error::TooLarge => CRYPT_E_ASN1_LARGE,
            Error::MoreData { .. } => ERROR_MORE_DATA,
        }
    }
}
