//! `CryptBinaryToStringA` for the encodings that are ported.

use crate::{
    base64, hex, pem, Error, CRYPT_STRING_BASE64, CRYPT_STRING_BASE64HEADER,
    CRYPT_STRING_BASE64REQUESTHEADER, CRYPT_STRING_BASE64X509CRLHEADER, CRYPT_STRING_BINARY,
    CRYPT_STRING_HEXRAW, CRYPT_STRING_NOCR, CRYPT_STRING_NOCRLF,
};

const TYPE_MASK: u32 = 0x0fff_ffff;

/// The size `CryptBinaryToStringA` reports for `binary`, including the terminator.
pub fn required_size(binary: &[u8], flags: u32) -> Result<usize, Error> {
    encode(binary, flags).map(|text| text.len() + terminator(flags))
}

/// Encodes `binary` into a buffer of `capacity` bytes, as `CryptBinaryToStringA` does.
///
/// The result has no terminator. The capacity must also hold the terminator, which is a NUL
/// for text and nothing for binary.
pub fn binary_to_string(binary: &[u8], flags: u32, capacity: usize) -> Result<Vec<u8>, Error> {
    let text = encode(binary, flags)?;
    if capacity < text.len() + terminator(flags) {
        return Err(if is_binary(flags) {
            Error::InsufficientBuffer
        } else {
            Error::MoreData
        });
    }
    Ok(text)
}

fn terminator(flags: u32) -> usize {
    usize::from(!is_binary(flags))
}

fn is_binary(flags: u32) -> bool {
    flags & TYPE_MASK == CRYPT_STRING_BINARY
}

fn encode(binary: &[u8], flags: u32) -> Result<Vec<u8>, Error> {
    let line_end = line_ending(flags);
    match flags & TYPE_MASK {
        CRYPT_STRING_BINARY => Ok(binary.to_vec()),
        CRYPT_STRING_BASE64 => Ok(base64::encode(binary, line_end)),
        CRYPT_STRING_BASE64HEADER => Ok(pem::wrap(binary, line_end, &pem::CERTIFICATE)),
        CRYPT_STRING_BASE64REQUESTHEADER => Ok(pem::wrap(binary, line_end, &pem::REQUEST)),
        CRYPT_STRING_BASE64X509CRLHEADER => Ok(pem::wrap(binary, line_end, &pem::X509_CRL)),
        CRYPT_STRING_HEXRAW => Ok(hex::encode(binary, line_end)),
        _ => Err(Error::InvalidParameter),
    }
}

fn line_ending(flags: u32) -> &'static str {
    if flags & CRYPT_STRING_NOCR != 0 {
        "\n"
    } else if flags & CRYPT_STRING_NOCRLF != 0 {
        ""
    } else {
        "\r\n"
    }
}
