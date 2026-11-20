//! `CryptStringToBinaryA` for the encodings that are ported.

use crate::{
    base64, hex, pem, Error, CRYPT_STRING_ANY, CRYPT_STRING_BASE64, CRYPT_STRING_BASE64HEADER,
    CRYPT_STRING_BASE64REQUESTHEADER, CRYPT_STRING_BASE64X509CRLHEADER, CRYPT_STRING_BASE64_ANY,
    CRYPT_STRING_BINARY, CRYPT_STRING_HEX,
};

/// The bytes `CryptStringToBinaryA` writes, with the header skip and the format it reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decoded {
    pub bytes: Vec<u8>,
    pub skip: usize,
    pub format: u32,
}

/// The size `CryptStringToBinaryA` reports for `input`.
pub fn required_size(input: &[u8], flags: u32) -> Result<usize, Error> {
    decode(input, flags).map(|decoded| decoded.bytes.len())
}

/// Decodes `input` into a buffer of `capacity` bytes, as `CryptStringToBinaryA` does.
pub fn string_to_binary(input: &[u8], flags: u32, capacity: usize) -> Result<Decoded, Error> {
    let decoded = decode(input, flags)?;
    if decoded.bytes.len() > capacity {
        return Err(if flags == CRYPT_STRING_HEX {
            Error::MoreData
        } else {
            Error::InsufficientBuffer
        });
    }
    Ok(decoded)
}

fn decode(input: &[u8], flags: u32) -> Result<Decoded, Error> {
    if (flags & 0xffff_fff0) != 0 {
        return Err(Error::InvalidData);
    }
    match flags {
        CRYPT_STRING_BASE64 => plain(input),
        CRYPT_STRING_BASE64HEADER
        | CRYPT_STRING_BASE64REQUESTHEADER
        | CRYPT_STRING_BASE64X509CRLHEADER => framed(input, flags),
        CRYPT_STRING_BASE64_ANY => {
            or_invalid(framed(input, CRYPT_STRING_BASE64HEADER), || plain(input))
        }
        CRYPT_STRING_BINARY => Ok(raw(input)),
        CRYPT_STRING_HEX => hex_text(input),
        CRYPT_STRING_ANY => or_invalid(
            or_invalid(framed(input, CRYPT_STRING_BASE64HEADER), || plain(input)),
            || Ok(raw(input)),
        ),
        _ => Err(Error::InvalidParameter),
    }
}

/// Tries `fallback` when `first` fails with `InvalidData`, as the `*_ANY` types do.
fn or_invalid(
    first: Result<Decoded, Error>,
    fallback: impl FnOnce() -> Result<Decoded, Error>,
) -> Result<Decoded, Error> {
    match first {
        Err(Error::InvalidData) => fallback(),
        other => other,
    }
}

fn framed(input: &[u8], format: u32) -> Result<Decoded, Error> {
    let (skip, data) = pem::locate(input)?;
    Ok(Decoded {
        bytes: base64::decode(data)?,
        skip,
        format,
    })
}

fn plain(input: &[u8]) -> Result<Decoded, Error> {
    Ok(Decoded {
        bytes: base64::decode(input)?,
        skip: 0,
        format: CRYPT_STRING_BASE64,
    })
}

fn raw(input: &[u8]) -> Decoded {
    Decoded {
        bytes: input.to_vec(),
        skip: 0,
        format: CRYPT_STRING_BINARY,
    }
}

fn hex_text(input: &[u8]) -> Result<Decoded, Error> {
    Ok(Decoded {
        bytes: hex::decode(input)?,
        skip: 0,
        format: CRYPT_STRING_HEX,
    })
}
