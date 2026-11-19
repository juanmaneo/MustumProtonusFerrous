//! `build_hash_signature` of `dlls/rsaenh/rsaenh.c`: the signature encoding of a hash value.

use crate::{Nte, CRYPT_NOHASHOID, CRYPT_X931_FORMAT};

pub const CALG_MD2: u32 = 0x8001;
pub const CALG_MD4: u32 = 0x8002;
pub const CALG_MD5: u32 = 0x8003;
pub const CALG_SHA: u32 = 0x8004;
pub const CALG_SSL3_SHAMD5: u32 = 0x8008;
pub const CALG_SHA_256: u32 = 0x800c;
pub const CALG_SHA_384: u32 = 0x800d;
pub const CALG_SHA_512: u32 = 0x800e;

/// One entry of `aOIDDescriptor`: the DigestInfo prefix that precedes a hash of `alg_id`.
struct DigestInfo {
    alg_id: u32,
    prefix: &'static [u8],
}

/// The DigestInfo prefixes, in the C's order. SSL 3.0's MD5 and SHA-1 concatenation has an empty prefix.
const DIGEST_INFO: [DigestInfo; 8] = [
    DigestInfo {
        alg_id: CALG_MD2,
        prefix: &[
            0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x02,
            0x05, 0x00, 0x04, 0x10,
        ],
    },
    DigestInfo {
        alg_id: CALG_MD4,
        prefix: &[
            0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x04,
            0x05, 0x00, 0x04, 0x10,
        ],
    },
    DigestInfo {
        alg_id: CALG_MD5,
        prefix: &[
            0x30, 0x20, 0x30, 0x0c, 0x06, 0x08, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x02, 0x05,
            0x05, 0x00, 0x04, 0x10,
        ],
    },
    DigestInfo {
        alg_id: CALG_SHA,
        prefix: &[
            0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0e, 0x03, 0x02, 0x1a, 0x05, 0x00, 0x04,
            0x14,
        ],
    },
    DigestInfo {
        alg_id: CALG_SHA_256,
        prefix: &[
            0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
            0x01, 0x05, 0x00, 0x04, 0x20,
        ],
    },
    DigestInfo {
        alg_id: CALG_SHA_384,
        prefix: &[
            0x30, 0x41, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
            0x02, 0x05, 0x00, 0x04, 0x30,
        ],
    },
    DigestInfo {
        alg_id: CALG_SHA_512,
        prefix: &[
            0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02,
            0x03, 0x05, 0x00, 0x04, 0x40,
        ],
    },
    DigestInfo {
        alg_id: CALG_SSL3_SHAMD5,
        prefix: &[],
    },
];

/// `build_hash_signature`: encodes `hash` as the whole of `out`, a signature block of the modulus length.
///
/// With `CRYPT_X931_FORMAT` the block is `6b bb.. ba hash 33 cc`. Otherwise it is PKCS #1 v1.5 type 1,
/// `00 01 ff.. 00 DigestInfo hash`, and `CRYPT_NOHASHOID` leaves out the DigestInfo.
pub fn build_hash_signature(
    out: &mut [u8],
    alg_id: u32,
    hash: &[u8],
    flags: u32,
) -> Result<(), Nte> {
    let info = DIGEST_INFO
        .iter()
        .find(|info| info.alg_id == alg_id)
        .ok_or(Nte::BadAlgId)?;
    let len = out.len();
    if flags & CRYPT_X931_FORMAT != 0 {
        if len < hash.len() + 4 {
            return Err(Nte::BadLen);
        }
        let ba = len - hash.len() - 3;
        out[0] = 0x6b;
        out[1..ba].fill(0xbb);
        out[ba] = 0xba;
        out[ba + 1..len - 2].copy_from_slice(hash);
        out[len - 2] = 0x33;
        out[len - 1] = 0xcc;
    } else {
        let prefix: &[u8] = if flags & CRYPT_NOHASHOID != 0 {
            &[]
        } else {
            info.prefix
        };
        if len < prefix.len() + hash.len() + 3 {
            return Err(Nte::BadLen);
        }
        let sep = len - prefix.len() - hash.len() - 1;
        out[0] = 0x00;
        out[1] = 0x01;
        out[2..sep].fill(0xff);
        out[sep] = 0x00;
        out[sep + 1..sep + 1 + prefix.len()].copy_from_slice(prefix);
        out[sep + 1 + prefix.len()..].copy_from_slice(hash);
    }
    Ok(())
}
