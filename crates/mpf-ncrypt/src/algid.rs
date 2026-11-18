//! Algorithm identifiers from the enum algid table of Wine's dlls/ncrypt/main.c.

use crate::wide::{fold_eq, utf16};

/// The algorithm of a key, in the order of the C enum.
#[derive(Clone, Copy)]
pub enum AlgId {
    Aes,
    Rsa,
    Dsa,
    Ecdsa,
    EcdsaP256,
    EcdsaP384,
    EcdsaP521,
}

impl AlgId {
    /// The algorithm name that the C enum maps to, as stored in the Algorithm Name property.
    pub fn name(self) -> &'static str {
        match self {
            Self::Aes => "AES",
            Self::Rsa => "RSA",
            Self::Dsa => "DSA",
            Self::Ecdsa => "ECDSA",
            Self::EcdsaP256 => "ECDSA_P256",
            Self::EcdsaP384 => "ECDSA_P384",
            Self::EcdsaP521 => "ECDSA_P521",
        }
    }

    /// Whether the algorithm is symmetric, which NCryptVerifySignature rejects.
    pub fn is_symmetric(self) -> bool {
        matches!(self, Self::Aes)
    }
}

/// The persisted key names that NCryptCreatePersistedKey accepts, with their default bit lengths.
const PERSISTED: [(&str, AlgId, u32); 4] = [
    ("RSA", AlgId::Rsa, 1024),
    ("ECDSA_P256", AlgId::EcdsaP256, 256),
    ("ECDSA_P384", AlgId::EcdsaP384, 384),
    ("ECDSA_P521", AlgId::EcdsaP521, 521),
];

/// The algorithm and default bit length of a persisted key name, compared without case.
pub fn from_persisted_name(name: &[u16]) -> Option<(AlgId, u32)> {
    PERSISTED
        .iter()
        .find(|(text, _, _)| fold_eq(name, &utf16(text)))
        .map(|&(_, alg, bits)| (alg, bits))
}
