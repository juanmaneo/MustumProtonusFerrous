//! The ECC key blob magic numbers, the curve that a blob names and the blob type
//! resolution, from Wine's `dlls/bcrypt/bcrypt_main.c`.

use crate::algorithm::{AlgId, CurveId};
use crate::wide::wcs_eq;

const RSA_PUBLIC: u32 = 0x3141_5352;
const DSA_PUBLIC: u32 = 0x4250_5344;

const ECDH_PUBLIC_GENERIC: u32 = 0x504b_4345;
const ECDH_PRIVATE_GENERIC: u32 = 0x564b_4345;
const ECDH_P256_PUBLIC: u32 = 0x314b_4345;
const ECDH_P256_PRIVATE: u32 = 0x324b_4345;
const ECDH_P384_PUBLIC: u32 = 0x334b_4345;
const ECDH_P384_PRIVATE: u32 = 0x344b_4345;
const ECDH_P521_PUBLIC: u32 = 0x354b_4345;
const ECDH_P521_PRIVATE: u32 = 0x364b_4345;

const ECDSA_PUBLIC_GENERIC: u32 = 0x5044_4345;
const ECDSA_PRIVATE_GENERIC: u32 = 0x5644_4345;
const ECDSA_P256_PUBLIC: u32 = 0x3153_4345;
const ECDSA_P256_PRIVATE: u32 = 0x3253_4345;
const ECDSA_P384_PUBLIC: u32 = 0x3353_4345;
const ECDSA_P384_PRIVATE: u32 = 0x3453_4345;
const ECDSA_P521_PUBLIC: u32 = 0x3553_4345;
const ECDSA_P521_PRIVATE: u32 = 0x3653_4345;

/// The magic numbers of one ECC algorithm, and the curve that its blobs name.
/// `mismatch` is the curve that Wine reports for a blob whose magic is neither
/// the public nor the private value.
struct EccKind {
    public: u32,
    private: u32,
    curve: CurveId,
    mismatch: CurveId,
}

fn ecc_kind(id: AlgId) -> Option<EccKind> {
    let (public, private, curve, mismatch) = match id {
        AlgId::Ecdh => (
            ECDH_PUBLIC_GENERIC,
            ECDH_PRIVATE_GENERIC,
            CurveId::NoCurve,
            CurveId::NoCurve,
        ),
        AlgId::EcdhP256 => (
            ECDH_P256_PUBLIC,
            ECDH_P256_PRIVATE,
            CurveId::P256r1,
            CurveId::Invalid,
        ),
        AlgId::EcdhP384 => (
            ECDH_P384_PUBLIC,
            ECDH_P384_PRIVATE,
            CurveId::P384r1,
            CurveId::Invalid,
        ),
        AlgId::EcdhP521 => (
            ECDH_P521_PUBLIC,
            ECDH_P521_PRIVATE,
            CurveId::P521r1,
            CurveId::Invalid,
        ),
        AlgId::Ecdsa => (
            ECDSA_PUBLIC_GENERIC,
            ECDSA_PRIVATE_GENERIC,
            CurveId::NoCurve,
            CurveId::NoCurve,
        ),
        AlgId::EcdsaP256 => (
            ECDSA_P256_PUBLIC,
            ECDSA_P256_PRIVATE,
            CurveId::P256r1,
            CurveId::Invalid,
        ),
        AlgId::EcdsaP384 => (
            ECDSA_P384_PUBLIC,
            ECDSA_P384_PRIVATE,
            CurveId::P384r1,
            CurveId::Invalid,
        ),
        AlgId::EcdsaP521 => (
            ECDSA_P521_PUBLIC,
            ECDSA_P521_PRIVATE,
            CurveId::P521r1,
            CurveId::Invalid,
        ),
        _ => return None,
    };
    Some(EccKind {
        public,
        private,
        curve,
        mismatch,
    })
}

/// The magic number of the public or private key blob for `id`, or 0 when `id`
/// is not an ECC algorithm.
pub fn ecc_magic(id: AlgId, private: bool) -> u32 {
    ecc_kind(id).map_or(0, |kind| if private { kind.private } else { kind.public })
}

/// The curve and the private flag that an ECC key blob carries.
#[derive(Debug, Clone, Copy)]
pub struct BlobCurve {
    pub curve: CurveId,
    pub private: bool,
}

/// Maps the magic number of an ECC key blob to its curve, as `get_ecc_blob_curve`
/// does. A magic that matches neither the public nor the private value gives
/// `NoCurve` for the generic algorithms and `Invalid` for the named curves. A
/// non-ECC algorithm gives `Invalid`.
pub fn ecc_blob_curve(id: AlgId, magic: u32) -> BlobCurve {
    let Some(kind) = ecc_kind(id) else {
        return BlobCurve {
            curve: CurveId::Invalid,
            private: false,
        };
    };
    if magic == kind.private {
        BlobCurve {
            curve: kind.curve,
            private: true,
        }
    } else if magic == kind.public {
        BlobCurve {
            curve: kind.curve,
            private: false,
        }
    } else {
        BlobCurve {
            curve: kind.mismatch,
            private: false,
        }
    }
}

/// The blob type that a key import reads: the caller's own type name, or the
/// name that a public key blob resolves to from its magic number.
#[derive(Debug, Clone, Copy)]
pub enum BlobType<'a> {
    Given(&'a [u16]),
    Named(&'static str),
}

/// Resolves the blob type of a key import, as `resolve_blob_type` does. Any type
/// other than `PUBLICBLOB` is returned as given. `PUBLICBLOB` is resolved from
/// the four-byte magic at the start of `input`. Returns `None` when there is no
/// type, when `input` is shorter than four bytes, or when the magic is not a
/// known public key magic.
pub fn resolve_blob_type<'a>(blob_type: Option<&'a [u16]>, input: &[u8]) -> Option<BlobType<'a>> {
    let blob_type = blob_type?;
    if !wcs_eq(blob_type, "PUBLICBLOB") {
        return Some(BlobType::Given(blob_type));
    }
    let magic = u32::from_le_bytes(*input.first_chunk::<4>()?);
    match magic {
        ECDH_P256_PUBLIC | ECDH_P384_PUBLIC | ECDH_P521_PUBLIC | ECDSA_P256_PUBLIC
        | ECDSA_P384_PUBLIC | ECDSA_P521_PUBLIC => Some(BlobType::Named("ECCPUBLICBLOB")),
        RSA_PUBLIC => Some(BlobType::Named("RSAPUBLICBLOB")),
        DSA_PUBLIC => Some(BlobType::Named("DSAPUBLICBLOB")),
        _ => None,
    }
}
