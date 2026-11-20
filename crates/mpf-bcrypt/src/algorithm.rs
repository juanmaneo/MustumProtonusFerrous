//! The built-in algorithm table and the provider enumeration and open checks,
//! from Wine's `dlls/bcrypt/bcrypt_main.c`.

use crate::wide::wcs_eq;
use crate::Status;

/// A built-in algorithm, in the order of Wine's `builtin_algorithms` table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgId {
    Des,
    TripleDes,
    ChaCha20Poly1305,
    Aes,
    AesGmac,
    Rc4,
    Sha256,
    Sha384,
    Sha512,
    Sha1,
    Md5,
    Md4,
    Md2,
    Rsa,
    Dh,
    Ecdh,
    EcdhP256,
    EcdhP384,
    EcdhP521,
    RsaSign,
    Ecdsa,
    EcdsaP256,
    EcdsaP384,
    EcdsaP521,
    Dsa,
    Rng,
    Pbkdf2,
    Tls11Kdf,
    Tls12Kdf,
    Hkdf,
}

impl AlgId {
    /// Every built-in algorithm, in table order.
    pub const ALL: [AlgId; 30] = [
        AlgId::Des,
        AlgId::TripleDes,
        AlgId::ChaCha20Poly1305,
        AlgId::Aes,
        AlgId::AesGmac,
        AlgId::Rc4,
        AlgId::Sha256,
        AlgId::Sha384,
        AlgId::Sha512,
        AlgId::Sha1,
        AlgId::Md5,
        AlgId::Md4,
        AlgId::Md2,
        AlgId::Rsa,
        AlgId::Dh,
        AlgId::Ecdh,
        AlgId::EcdhP256,
        AlgId::EcdhP384,
        AlgId::EcdhP521,
        AlgId::RsaSign,
        AlgId::Ecdsa,
        AlgId::EcdsaP256,
        AlgId::EcdsaP384,
        AlgId::EcdsaP521,
        AlgId::Dsa,
        AlgId::Rng,
        AlgId::Pbkdf2,
        AlgId::Tls11Kdf,
        AlgId::Tls12Kdf,
        AlgId::Hkdf,
    ];

    /// Whether the algorithm is a cipher or a key derivation function, as Wine's
    /// `is_symmetric_alg` decides.
    pub fn is_symmetric(self) -> bool {
        matches!(
            self.builtin().interface,
            Interface::Cipher | Interface::KeyDerivation
        )
    }

    pub(crate) fn builtin(self) -> &'static Builtin {
        &BUILTIN[self as usize]
    }
}

/// The interface class of an algorithm, numbered as `BCRYPT_*_INTERFACE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interface {
    Cipher = 1,
    Hash = 2,
    AsymmetricEncryption = 3,
    SecretAgreement = 4,
    Signature = 5,
    Rng = 6,
    KeyDerivation = 7,
}

impl Interface {
    /// The `BCRYPT_*_OPERATION` bit that selects this interface in
    /// `BCryptEnumAlgorithms`.
    fn operation(self) -> u32 {
        match self {
            Interface::Cipher => 0x01,
            Interface::Hash => 0x02,
            Interface::AsymmetricEncryption => 0x04,
            Interface::SecretAgreement => 0x08,
            Interface::Signature => 0x10,
            Interface::Rng => 0x20,
            Interface::KeyDerivation => 0x40,
        }
    }
}

/// A chain mode for block ciphers. Wine's `CHAIN_MODE_CCM` is not modelled,
/// because no call path accepts it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainMode {
    Cbc,
    Ecb,
    Cfb,
    Gcm,
}

impl ChainMode {
    /// The `BCRYPT_CHAIN_MODE_*` string for this mode.
    pub fn name(self) -> &'static str {
        match self {
            ChainMode::Cbc => "ChainingModeCBC",
            ChainMode::Ecb => "ChainingModeECB",
            ChainMode::Cfb => "ChainingModeCFB",
            ChainMode::Gcm => "ChainingModeGCM",
        }
    }
}

/// An ECC curve, numbered as Wine's `enum ecc_curve_id`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CurveId {
    Invalid = -1,
    NoCurve = 0,
    Curve25519 = 1,
    BrainpoolP256r1 = 2,
    BrainpoolP384r1 = 3,
    P256r1 = 4,
    P384r1 = 5,
    P521r1 = 6,
}

/// One row of the built-in algorithm table.
pub(crate) struct Builtin {
    pub(crate) name: &'static str,
    pub(crate) interface: Interface,
    pub(crate) object_length: u32,
    pub(crate) hash_length: u32,
    pub(crate) hash_block_length: u32,
    pub(crate) curve: CurveId,
    pub(crate) chain_mode: ChainMode,
}

// The rows follow Wine's `builtin_algorithms` in dlls/bcrypt/bcrypt_main.c.
#[rustfmt::skip]
static BUILTIN: [Builtin; 30] = [
    Builtin { name: "DES", interface: Interface::Cipher, object_length: 522, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "3DES", interface: Interface::Cipher, object_length: 522, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "CHACHA20_POLY1305", interface: Interface::Cipher, object_length: 166, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "AES", interface: Interface::Cipher, object_length: 654, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "AES-GMAC", interface: Interface::Cipher, object_length: 654, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Gcm },
    Builtin { name: "RC4", interface: Interface::Cipher, object_length: 654, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "SHA256", interface: Interface::Hash, object_length: 286, hash_length: 32, hash_block_length: 64, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "SHA384", interface: Interface::Hash, object_length: 382, hash_length: 48, hash_block_length: 128, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "SHA512", interface: Interface::Hash, object_length: 382, hash_length: 64, hash_block_length: 128, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "SHA1", interface: Interface::Hash, object_length: 278, hash_length: 20, hash_block_length: 64, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "MD5", interface: Interface::Hash, object_length: 274, hash_length: 16, hash_block_length: 64, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "MD4", interface: Interface::Hash, object_length: 270, hash_length: 16, hash_block_length: 64, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "MD2", interface: Interface::Hash, object_length: 270, hash_length: 16, hash_block_length: 16, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "RSA", interface: Interface::AsymmetricEncryption, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "DH", interface: Interface::SecretAgreement, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDH", interface: Interface::SecretAgreement, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDH_P256", interface: Interface::SecretAgreement, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::P256r1, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDH_P384", interface: Interface::SecretAgreement, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::P384r1, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDH_P521", interface: Interface::SecretAgreement, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::P521r1, chain_mode: ChainMode::Cbc },
    Builtin { name: "RSA_SIGN", interface: Interface::Signature, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDSA", interface: Interface::Signature, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDSA_P256", interface: Interface::Signature, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::P256r1, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDSA_P384", interface: Interface::Signature, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::P384r1, chain_mode: ChainMode::Cbc },
    Builtin { name: "ECDSA_P521", interface: Interface::Signature, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::P521r1, chain_mode: ChainMode::Cbc },
    Builtin { name: "DSA", interface: Interface::Signature, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "RNG", interface: Interface::Rng, object_length: 0, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "PBKDF2", interface: Interface::KeyDerivation, object_length: 618, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "TLS1_1_KDF", interface: Interface::KeyDerivation, object_length: 702, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "TLS1_2_KDF", interface: Interface::KeyDerivation, object_length: 702, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
    Builtin { name: "HKDF", interface: Interface::KeyDerivation, object_length: 702, hash_length: 0, hash_block_length: 0, curve: CurveId::NoCurve, chain_mode: ChainMode::Cbc },
];

/// The operation bits that `BCryptEnumAlgorithms` accepts.
const ALL_OPERATIONS: u32 = 0x7F;

/// The `BCRYPT_ALG_HANDLE_HMAC_FLAG` and `BCRYPT_HASH_REUSABLE_FLAG` bits.
const SUPPORTED_OPEN_FLAGS: u32 = 0x08 | 0x20;

const MS_PRIMITIVE_PROVIDER: &str = "Microsoft Primitive Provider";

/// An open algorithm provider, the state that Wine keeps in its handle.
#[derive(Debug, Clone, Copy)]
pub struct Algorithm {
    pub id: AlgId,
    pub chain_mode: ChainMode,
    pub flags: u32,
    pub curve_id: CurveId,
}

impl Algorithm {
    /// An open provider for `id`, with the table's default chain mode and curve.
    pub fn new(id: AlgId, flags: u32) -> Self {
        let builtin = id.builtin();
        Algorithm {
            id,
            chain_mode: builtin.chain_mode,
            flags,
            curve_id: builtin.curve,
        }
    }
}

/// Lists the algorithms whose interface matches `operations`, as
/// `BCryptEnumAlgorithms` does. Zero matches every algorithm. Each entry is the
/// algorithm name and its interface class.
pub fn enum_algorithms(operations: u32) -> Result<Vec<(&'static str, Interface)>, Status> {
    if operations & !ALL_OPERATIONS != 0 {
        return Err(Status::InvalidParameter);
    }
    Ok(AlgId::ALL
        .into_iter()
        .map(AlgId::builtin)
        .filter(|b| operations == 0 || operations & b.interface.operation() != 0)
        .map(|b| (b.name, b.interface))
        .collect())
}

/// Checks the arguments of `BCryptOpenAlgorithmProvider` and returns the provider
/// for the named algorithm. Wine returns a handle; the port returns the state.
pub fn open_algorithm(
    name: Option<&[u16]>,
    implementation: Option<&[u16]>,
    flags: u32,
) -> Result<Algorithm, Status> {
    let name = name.ok_or(Status::InvalidParameter)?;
    if flags & !SUPPORTED_OPEN_FLAGS != 0 {
        return Err(Status::NotImplemented);
    }
    let id = AlgId::ALL
        .into_iter()
        .find(|id| wcs_eq(name, id.builtin().name))
        .ok_or(Status::NotImplemented)?;
    if let Some(implementation) = implementation {
        if !wcs_eq(implementation, MS_PRIMITIVE_PROVIDER) {
            return Err(Status::NotImplemented);
        }
    }
    Ok(Algorithm::new(id, flags))
}
