//! Headless port of the byte paths in Wine's `dlls/crypt32/base64.c`, the codecs behind
//! `CryptBinaryToStringA` and `CryptStringToBinaryA`.
//!
//! The functions work on byte slices. They keep Wine's flag checks, capacity checks and error
//! codes, and return the bytes Wine would write.
//!
//! Ported:
//! - Base64 text (`CRYPT_STRING_BASE64`), in 64-character lines: [encode::binary_to_string] and
//!   [decode::string_to_binary].
//! - PEM framing for certificates, requests and X509 CRLs (`CRYPT_STRING_BASE64HEADER`,
//!   `CRYPT_STRING_BASE64REQUESTHEADER` and `CRYPT_STRING_BASE64X509CRLHEADER`).
//! - `CRYPT_STRING_BINARY`, which copies the bytes.
//! - `CRYPT_STRING_HEXRAW`, which writes lowercase hex with no header.
//! - `CRYPT_STRING_HEX`, which reads hex text with `-`, `,` and whitespace separators.
//! - The fallback order of `CRYPT_STRING_BASE64_ANY` and `CRYPT_STRING_ANY`: PEM, then base64,
//!   then the raw bytes.
//!
//! Skipped:
//! - The wide-character entry points, `CryptBinaryToStringW` and `CryptStringToBinaryW`.
//! - The encoders for `CRYPT_STRING_HEX`, `HEXASCII`, `HEXADDR` and `HEXASCIIADDR`, and the
//!   decoders for `HEXASCII`, `HEXADDR` and `HEXASCIIADDR`. These types return
//!   [Error::InvalidParameter], the code Wine returns for them.
//! - The checks on NULL pointer arguments, which a slice cannot express.
//! - The ASN.1 functions in `encode.c` and `decode.c`.
//!
//! Deviations from Wine, where Wine has undefined behaviour or reports a size it does not write:
//! - (a) `CRYPT_STRING_ANY` with a buffer: when the base64 parse fails, Wine returns
//!   [Error::InsufficientBuffer] even if the data fits, because it leaves the output length at
//!   the bytes written so far. The port returns the raw bytes when they fit.
//! - (b) `CRYPT_STRING_HEXRAW` with `CRYPT_STRING_NOCR | CRYPT_STRING_NOCRLF`: Wine sizes the
//!   output as hex only, but writes a `\n` after it. The port uses that `\n` and sizes it.
//! - (c) `CRYPT_STRING_BINARY` and the raw fallback: Wine leaves `pdwSkip` and `pdwFlags`
//!   untouched. The port reports a skip of 0 and the format [CRYPT_STRING_BINARY].
//! - (d) Base64 decoding into a zero-capacity buffer that needs output: Wine's capacity check is
//!   skipped and it writes past the buffer. The port returns [Error::InsufficientBuffer].
//! - (e) An error returns no bytes. Wine's hex decoder may leave a partial prefix in the buffer.
//!
//! Wine measures the input with `strlen` when `cchString` is 0. The port takes the slice as
//! given, so the caller passes the text up to its length. The PEM search still stops at the
//! first NUL, as Wine's `strstr` does.

pub mod decode;
pub mod encode;

mod base64;
mod hex;
mod pem;

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;

// Encoding types and flags, with the values from `wincrypt.h`.
pub const CRYPT_STRING_BASE64HEADER: u32 = 0;
pub const CRYPT_STRING_BASE64: u32 = 1;
pub const CRYPT_STRING_BINARY: u32 = 2;
pub const CRYPT_STRING_BASE64REQUESTHEADER: u32 = 3;
pub const CRYPT_STRING_HEX: u32 = 4;
pub const CRYPT_STRING_BASE64_ANY: u32 = 6;
pub const CRYPT_STRING_ANY: u32 = 7;
pub const CRYPT_STRING_BASE64X509CRLHEADER: u32 = 9;
pub const CRYPT_STRING_HEXRAW: u32 = 0xc;
pub const CRYPT_STRING_NOCRLF: u32 = 0x4000_0000;
pub const CRYPT_STRING_NOCR: u32 = 0x8000_0000;

/// A failure, with the Win32 code Wine sets for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidParameter,
    InvalidData,
    InsufficientBuffer,
    MoreData,
}

impl Error {
    /// The Win32 error code: `ERROR_INVALID_PARAMETER`, `ERROR_INVALID_DATA`,
    /// `ERROR_INSUFFICIENT_BUFFER` or `ERROR_MORE_DATA`.
    pub fn code(self) -> u32 {
        match self {
            Self::InvalidParameter => 87,
            Self::InvalidData => 13,
            Self::InsufficientBuffer => 122,
            Self::MoreData => 234,
        }
    }
}
