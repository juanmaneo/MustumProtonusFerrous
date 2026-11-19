//! `pad_data_pkcs1` and `unpad_data_pkcs1` of `dlls/rsaenh/rsaenh.c`: PKCS #1 v1.5 block type 2.

use crate::{Nte, CRYPT_SSL2_FALLBACK};

/// `RSAENH_PKC_BLOCKTYPE`.
const PKC_BLOCKTYPE: u8 = 0x02;

/// `pad_data_pkcs1`: writes `data` right-aligned in `out`, as `00 02 random.. 00 data`.
///
/// Every padding byte comes from `random`, and a zero draw is repeated. With `CRYPT_SSL2_FALLBACK` the
/// eight padding bytes before the separator are `03`, as SSL 2.0 requires.
pub fn pad_data_pkcs1(
    data: &[u8],
    out: &mut [u8],
    flags: u32,
    mut random: impl FnMut() -> u8,
) -> Result<(), Nte> {
    let len = out.len();
    if data.len() + 11 > len {
        return Err(Nte::BadLen);
    }
    let sep = len - data.len() - 1;
    out[sep + 1..].copy_from_slice(data);
    out[0] = 0x00;
    out[1] = PKC_BLOCKTYPE;
    for byte in &mut out[2..sep] {
        *byte = loop {
            let drawn = random();
            if drawn != 0 {
                break drawn;
            }
        };
    }
    if flags & CRYPT_SSL2_FALLBACK != 0 {
        out[sep - 8..sep].fill(0x03);
    }
    out[sep] = 0x00;
    Ok(())
}

/// `unpad_data_pkcs1`: returns the payload after the separator of a block type 2 block, if the payload
/// fits in `capacity` bytes.
pub fn unpad_data_pkcs1(data: &[u8], capacity: usize) -> Result<&[u8], Nte> {
    if data.len() < 3 {
        return Err(Nte::BadData);
    }
    let sep = data[2..]
        .iter()
        .position(|&byte| byte == 0)
        .ok_or(Nte::BadData)?
        + 2;
    let payload = &data[sep + 1..];
    if capacity < payload.len() || data[0] != 0x00 || data[1] != PKC_BLOCKTYPE {
        return Err(Nte::BadData);
    }
    Ok(payload)
}
