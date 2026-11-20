//! DER encoders from `dlls/wintrust/asn.c`. Each returns the complete element. `write_to` is the
//! buffer protocol that `CryptEncodeObject` applies on top of them.

use crate::{length, Error, ASN_BOOL, ASN_INTEGER, ASN_OCTETSTRING};

/// `CRYPT_AsnEncodeBool`: `01 01 ff` for true and `01 01 00` for false.
pub fn boolean(value: bool) -> Vec<u8> {
    vec![ASN_BOOL, 1, if value { 0xff } else { 0 }]
}

/// `CRYPT_AsnEncodeInteger`: encodes a little-endian blob as the minimal two's-complement
/// INTEGER. An empty blob encodes as `02 00`, as in Wine.
pub fn integer(value: &[u8]) -> Vec<u8> {
    let mut significant = value.len();
    let mut pad = None;
    if let Some(&top) = value.last() {
        if (top & 0x80) != 0 {
            significant = value.iter().rposition(|&b| b != 0xff).map_or(0, |i| i + 1);
            if value[..significant].last().copied().unwrap_or(0) < 0x80 {
                pad = Some(0xff);
            }
        } else {
            significant = value.iter().rposition(|&b| b != 0).map_or(1, |i| i + 1);
            if value[significant - 1] > 0x7f {
                pad = Some(0x00);
            }
        }
    }
    let mut out = vec![ASN_INTEGER];
    out.extend(length::encode(significant + usize::from(pad.is_some())));
    out.extend(pad);
    out.extend(value[..significant].iter().rev());
    out
}

/// `CRYPT_AsnEncodeInt`: the little-endian blob of an `i32`, encoded as an INTEGER.
pub fn int(value: i32) -> Vec<u8> {
    integer(&value.to_le_bytes())
}

/// `CRYPT_AsnEncodeOctets`: an OCTET STRING holding `data`.
pub fn octets(data: &[u8]) -> Vec<u8> {
    let mut out = vec![ASN_OCTETSTRING];
    out.extend(length::encode(data.len()));
    out.extend_from_slice(data);
    out
}

/// The buffer protocol of `CryptEncodeObject`. Without a buffer it reports the length needed.
/// With one, it copies `encoded` and reports the length, or fails with `MoreData` when the
/// buffer is too small.
pub fn write_to(encoded: &[u8], out: Option<&mut [u8]>) -> Result<usize, Error> {
    match out {
        None => Ok(encoded.len()),
        Some(buf) if buf.len() < encoded.len() => Err(Error::MoreData {
            needed: encoded.len(),
        }),
        Some(buf) => {
            buf[..encoded.len()].copy_from_slice(encoded);
            Ok(encoded.len())
        }
    }
}
