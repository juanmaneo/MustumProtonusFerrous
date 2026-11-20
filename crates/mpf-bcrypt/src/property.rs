//! `BCryptGetProperty` and `BCryptSetProperty` for the built-in algorithms, from
//! Wine's `dlls/bcrypt/bcrypt_main.c`.

use crate::algorithm::{AlgId, Algorithm, ChainMode, CurveId};
use crate::key::KeyLengths;
use crate::wide::{to_wide_nul, wcs_eq};
use crate::{Reply, Status};

/// The padding schemes that Wine reports for RSA: PKCS #1 signatures and OAEP.
const PADDING_SCHEMES: u32 = 0x04 | 0x08;

const AES_KEY_LENGTHS: KeyLengths = KeyLengths {
    min: 128,
    max: 256,
    increment: 64,
};

/// The AES-GCM tag lengths, in bytes. Wine stores them in the same layout as the
/// key lengths.
const AUTH_TAG_LENGTHS: KeyLengths = KeyLengths {
    min: 12,
    max: 16,
    increment: 1,
};

const KDF_KEY_LENGTHS: KeyLengths = KeyLengths {
    min: 0,
    max: 16384,
    increment: 8,
};

const NAMED_CURVES: [(&str, CurveId); 5] = [
    ("secP256r1", CurveId::P256r1),
    ("secP384r1", CurveId::P384r1),
    ("secP521r1", CurveId::P521r1),
    ("brainpoolP256r1", CurveId::BrainpoolP256r1),
    ("brainpoolP384r1", CurveId::BrainpoolP384r1),
];

/// Reads property `prop` of an open algorithm, as `BCryptGetProperty` does. When
/// `buf` is given it must hold at least `size` bytes. The reply carries the
/// status and the byte count that Wine writes to `*ret_size`.
pub fn get_alg_property(
    alg: &Algorithm,
    prop: &[u16],
    mut buf: Option<&mut [u8]>,
    size: u32,
) -> Reply {
    if let Some(reply) = generic_property(alg.id, prop, buf.as_deref_mut(), size) {
        return reply;
    }
    match alg.id {
        AlgId::Des => des_property(alg.chain_mode, 64, prop, buf, size),
        AlgId::TripleDes => des_property(alg.chain_mode, 192, prop, buf, size),
        AlgId::ChaCha20Poly1305 | AlgId::Rc4 => stream_property(prop, buf, size),
        AlgId::Aes | AlgId::AesGmac => aes_property(alg.chain_mode, prop, buf, size),
        AlgId::Rsa => rsa_property(prop, buf, size),
        AlgId::Dsa => dsa_property(prop),
        AlgId::Pbkdf2 | AlgId::Tls11Kdf | AlgId::Tls12Kdf | AlgId::Hkdf => {
            kdf_property(prop, buf, size)
        }
        _ => (Status::NotImplemented, 0),
    }
}

/// Properties that every algorithm shares, or `None` when the name is not one of
/// them.
fn generic_property(id: AlgId, prop: &[u16], buf: Option<&mut [u8]>, size: u32) -> Option<Reply> {
    let builtin = id.builtin();
    if wcs_eq(prop, "ObjectLength") {
        Some(nonzero_dword(builtin.object_length, buf, size))
    } else if wcs_eq(prop, "HashDigestLength") {
        Some(nonzero_dword(builtin.hash_length, buf, size))
    } else if wcs_eq(prop, "HashBlockLength") {
        Some(nonzero_dword(builtin.hash_block_length, buf, size))
    } else if wcs_eq(prop, "AlgorithmName") {
        Some(wide_string(builtin.name, 1, buf, size))
    } else {
        None
    }
}

fn des_property(
    mode: ChainMode,
    key_bits: u32,
    prop: &[u16],
    buf: Option<&mut [u8]>,
    size: u32,
) -> Reply {
    if wcs_eq(prop, "BlockLength") {
        dword(8, buf, size)
    } else if wcs_eq(prop, "ChainingMode") {
        match mode {
            ChainMode::Cbc => wide_string(mode.name(), 2, buf, size),
            _ => (Status::NotImplemented, 0),
        }
    } else if wcs_eq(prop, "KeyLengths") {
        let lengths = KeyLengths {
            min: key_bits,
            max: key_bits,
            increment: 0,
        };
        lengths_property(lengths, buf, size)
    } else {
        (Status::NotImplemented, 0)
    }
}

fn stream_property(prop: &[u16], buf: Option<&mut [u8]>, size: u32) -> Reply {
    if wcs_eq(prop, "BlockLength") {
        dword(1, buf, size)
    } else {
        (Status::NotImplemented, 0)
    }
}

fn aes_property(mode: ChainMode, prop: &[u16], buf: Option<&mut [u8]>, size: u32) -> Reply {
    if wcs_eq(prop, "BlockLength") {
        dword(16, buf, size)
    } else if wcs_eq(prop, "MessageBlockLength") {
        dword(1, buf, size)
    } else if wcs_eq(prop, "ChainingMode") {
        wide_string(mode.name(), 2, buf, size)
    } else if wcs_eq(prop, "KeyLengths") {
        lengths_property(AES_KEY_LENGTHS, buf, size)
    } else if wcs_eq(prop, "AuthTagLength") {
        if mode == ChainMode::Gcm {
            lengths_property(AUTH_TAG_LENGTHS, buf, size)
        } else {
            (Status::NotSupported, 0)
        }
    } else {
        (Status::NotImplemented, 0)
    }
}

fn rsa_property(prop: &[u16], buf: Option<&mut [u8]>, size: u32) -> Reply {
    if wcs_eq(prop, "PaddingSchemes") {
        dword(PADDING_SCHEMES, buf, size)
    } else {
        (Status::NotImplemented, 0)
    }
}

fn dsa_property(prop: &[u16]) -> Reply {
    if wcs_eq(prop, "PaddingSchemes") {
        (Status::NotSupported, 0)
    } else {
        (Status::NotImplemented, 0)
    }
}

fn kdf_property(prop: &[u16], buf: Option<&mut [u8]>, size: u32) -> Reply {
    if wcs_eq(prop, "BlockLength") {
        (Status::NotSupported, 0)
    } else if wcs_eq(prop, "KeyLengths") {
        lengths_property(KDF_KEY_LENGTHS, buf, size)
    } else {
        (Status::NotImplemented, 0)
    }
}

/// Sets property `prop` of an open algorithm to the wide string `value`, as
/// `BCryptSetProperty` does. Wine's size and flags arguments are not needed here.
pub fn set_alg_property(alg: &mut Algorithm, prop: &[u16], value: &[u16]) -> Status {
    match alg.id {
        AlgId::Des | AlgId::TripleDes => des_set(alg, prop, value),
        AlgId::ChaCha20Poly1305 | AlgId::Rc4 => not_applicable_set(prop, value),
        AlgId::Aes => aes_set(alg, prop, value),
        AlgId::Ecdh | AlgId::Ecdsa => ecc_set(alg, prop, value),
        _ => Status::NotImplemented,
    }
}

fn des_set(alg: &mut Algorithm, prop: &[u16], value: &[u16]) -> Status {
    if !wcs_eq(prop, "ChainingMode") {
        return Status::NotImplemented;
    }
    match find_chain_mode(value, &[ChainMode::Cbc, ChainMode::Ecb]) {
        Some(mode) => {
            alg.chain_mode = mode;
            Status::Success
        }
        None => Status::NotSupported,
    }
}

fn aes_set(alg: &mut Algorithm, prop: &[u16], value: &[u16]) -> Status {
    if wcs_eq(prop, "ChainingMode") {
        let allowed = [
            ChainMode::Ecb,
            ChainMode::Cbc,
            ChainMode::Gcm,
            ChainMode::Cfb,
        ];
        match find_chain_mode(value, &allowed) {
            Some(mode) => {
                alg.chain_mode = mode;
                Status::Success
            }
            None => Status::NotImplemented,
        }
    } else if wcs_eq(prop, "MessageBlockLength") {
        Status::InvalidParameter
    } else {
        Status::NotImplemented
    }
}

/// Stream ciphers accept only the "not applicable" chain mode.
fn not_applicable_set(prop: &[u16], value: &[u16]) -> Status {
    if wcs_eq(prop, "ChainingMode") && wcs_eq(value, "ChainingModeN/A") {
        Status::Success
    } else {
        Status::NotImplemented
    }
}

fn ecc_set(alg: &mut Algorithm, prop: &[u16], value: &[u16]) -> Status {
    if !wcs_eq(prop, "ECCCurveName") {
        return Status::NotImplemented;
    }
    if wcs_eq(value, "curve25519") {
        if alg.id != AlgId::Ecdh {
            return Status::NotSupported;
        }
        alg.curve_id = CurveId::Curve25519;
        return Status::Success;
    }
    match NAMED_CURVES.iter().find(|(name, _)| wcs_eq(value, name)) {
        Some(&(_, curve)) => {
            alg.curve_id = curve;
            Status::Success
        }
        None => Status::NotImplemented,
    }
}

fn find_chain_mode(value: &[u16], allowed: &[ChainMode]) -> Option<ChainMode> {
    allowed
        .iter()
        .copied()
        .find(|mode| wcs_eq(value, mode.name()))
}

/// A DWORD property that Wine reports as not supported when it is zero.
fn nonzero_dword(value: u32, buf: Option<&mut [u8]>, size: u32) -> Reply {
    if value == 0 {
        (Status::NotSupported, 0)
    } else {
        dword(value, buf, size)
    }
}

/// Writes a DWORD. Wine checks the size before it looks at the buffer, so the
/// size check also applies when the buffer is NULL.
fn dword(value: u32, buf: Option<&mut [u8]>, size: u32) -> Reply {
    if size < 4 {
        return (Status::BufferTooSmall, 4);
    }
    if let Some(buf) = buf {
        buf[..4].copy_from_slice(&value.to_le_bytes());
    }
    (Status::Success, 4)
}

/// Writes a key length structure: minimum, maximum and increment as DWORDs. Wine
/// checks the size only when a buffer is given.
fn lengths_property(lengths: KeyLengths, buf: Option<&mut [u8]>, size: u32) -> Reply {
    if let Some(buf) = buf {
        if size < 12 {
            return (Status::BufferTooSmall, 12);
        }
        let values = [lengths.min, lengths.max, lengths.increment];
        let (words, _) = buf.as_chunks_mut::<4>();
        for (dst, value) in words.iter_mut().zip(values) {
            *dst = value.to_le_bytes();
        }
    }
    (Status::Success, 12)
}

/// Writes a wide string with its NUL. Wine reports `scale` times the byte length
/// of the string. The scale is 1 for a name and 2 for a chaining mode, which is a
/// quirk that the port keeps.
fn wide_string(text: &str, scale: u32, buf: Option<&mut [u8]>, size: u32) -> Reply {
    let bytes = to_wide_nul(text);
    let ret = bytes.len() as u32 * scale;
    if size < ret {
        return (Status::BufferTooSmall, ret);
    }
    if let Some(buf) = buf {
        buf[..bytes.len()].copy_from_slice(&bytes);
    }
    (Status::Success, ret)
}
