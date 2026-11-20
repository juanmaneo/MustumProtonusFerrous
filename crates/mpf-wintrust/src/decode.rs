//! DER decoders from `dlls/wintrust/asn.c`. Each takes a complete element. Wine's size query and
//! `CRYPT_DECODE_NOCOPY_FLAG` have no counterpart here: octets are borrowed from the input.

use crate::{length, Error, ASN_OCTETSTRING};

/// `CRYPT_AsnDecodeBool`: the value is `input[2] != 0`. The tag is not checked, as in Wine.
pub fn boolean(input: &[u8]) -> Result<bool, Error> {
    if input.len() < 3 || input[1] > 1 {
        return Err(Error::Corrupt);
    }
    Ok(input[2] != 0)
}

/// `CRYPT_AsnDecodeInt`: a big-endian two's-complement INTEGER of at most four content octets,
/// sign-extended to 32 bits. The tag is not checked, as in Wine.
pub fn int(input: &[u8]) -> Result<i32, Error> {
    let len = length::decode(input)?;
    if len > 4 {
        return Err(Error::TooLarge);
    }
    let content = length::content(input, len)?;
    let mut value = match content.first() {
        Some(&first) if (first & 0x80) != 0 => -1,
        _ => 0,
    };
    for &byte in content {
        value = (value << 8) | i32::from(byte);
    }
    Ok(value)
}

/// `CRYPT_AsnDecodeOctets`: the content of an OCTET STRING, borrowed from `input`.
pub fn octets(input: &[u8]) -> Result<&[u8], Error> {
    match input.first() {
        None => Err(Error::Corrupt),
        Some(&tag) if tag != ASN_OCTETSTRING => Err(Error::BadTag),
        Some(_) => {
            let len = length::decode(input)?;
            length::content(input, len)
        }
    }
}
