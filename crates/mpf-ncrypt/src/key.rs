//! The key object: the property template that NCryptCreatePersistedKey and NCryptImportKey build for each algorithm.

use crate::algid::AlgId;
use crate::property::{
    PropertyStore, ALGORITHM_GROUP_PROPERTY, ALGORITHM_PROPERTY, BLOCK_LENGTH_PROPERTY,
    EXPORT_POLICY_PROPERTY, KEY_TYPE_PROPERTY, KEY_USAGE_PROPERTY, LENGTHS_PROPERTY,
    LENGTH_PROPERTY, PUBLIC_KEY_LENGTH_PROPERTY, SIGNATURE_LENGTH_PROPERTY,
};
use crate::wide::{utf16, wide_bytes};
use crate::NCRYPT_ALLOW_ALL_USAGES;

/// A key object: its algorithm and its properties.
pub struct Key {
    algid: AlgId,
    properties: PropertyStore,
}

impl Key {
    /// The template of a key of bit_length bits, or None for an algorithm that has no key objects here.
    /// The Length and PublicKeyLength properties are written here, where Wine writes them after BCrypt succeeds. The caller sees the same result.
    pub fn new(algid: AlgId, bit_length: u32) -> Option<Key> {
        let group = match algid {
            AlgId::Rsa => "RSA",
            AlgId::EcdsaP256 | AlgId::EcdsaP384 | AlgId::EcdsaP521 => "ECDSA",
            _ => return None,
        };
        let mut properties = PropertyStore::default();
        put(
            &mut properties,
            ALGORITHM_PROPERTY,
            &wide_bytes(algid.name()),
        );
        put(
            &mut properties,
            ALGORITHM_GROUP_PROPERTY,
            &wide_bytes(group),
        );
        if matches!(algid, AlgId::Rsa) {
            let lengths: Vec<u8> = [512, 16384, 8, 1024]
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect();
            put(&mut properties, LENGTHS_PROPERTY, &lengths);
            put(
                &mut properties,
                BLOCK_LENGTH_PROPERTY,
                &128u32.to_le_bytes(),
            );
            put(
                &mut properties,
                SIGNATURE_LENGTH_PROPERTY,
                &128u32.to_le_bytes(),
            );
        }
        put(&mut properties, EXPORT_POLICY_PROPERTY, &0u32.to_le_bytes());
        put(
            &mut properties,
            KEY_USAGE_PROPERTY,
            &NCRYPT_ALLOW_ALL_USAGES.to_le_bytes(),
        );
        put(&mut properties, KEY_TYPE_PROPERTY, &0u32.to_le_bytes());
        put(&mut properties, LENGTH_PROPERTY, &bit_length.to_le_bytes());
        put(
            &mut properties,
            PUBLIC_KEY_LENGTH_PROPERTY,
            &bit_length.to_le_bytes(),
        );
        Some(Key { algid, properties })
    }

    pub fn algid(&self) -> AlgId {
        self.algid
    }

    pub fn properties(&self) -> &PropertyStore {
        &self.properties
    }

    pub fn properties_mut(&mut self) -> &mut PropertyStore {
        &mut self.properties
    }
}

fn put(properties: &mut PropertyStore, name: &str, value: &[u8]) {
    properties.set(&utf16(name), value);
}
