//! Oracle rows O01-O24 for dlls/ncrypt/main.c. Each test repeats one line of the verbatim Wine C output
//! (target/canon/ncrypt/rows.txt, built with gcc -fsanitize=address,undefined). No row was excluded.

use super::*;
use std::cell::Cell;

const RSA_PUBLIC_MAGIC: u32 = 0x3141_5352;
const ECDSA_P384_PRIVATE_MAGIC: u32 = 0x3453_4345;

/// A 24-byte blob: the magic, then the RSA bit length at offset 4 (ECDSA blobs ignore it).
fn blob(magic: u32, bits: u32) -> Vec<u8> {
    let mut bytes = vec![0u8; 24];
    bytes[..4].copy_from_slice(&magic.to_le_bytes());
    bytes[4..8].copy_from_slice(&bits.to_le_bytes());
    bytes
}

fn le(value: u32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

fn stored(key: &Key, name: &str) -> Option<Vec<u8>> {
    key.properties().get(&utf16(name)).map(<[u8]>::to_vec)
}

fn buffer(size: u32) -> Buffer {
    Buffer {
        is_null: false,
        size,
    }
}

fn null_buffer(size: u32) -> Buffer {
    Buffer {
        is_null: true,
        size,
    }
}

fn generate_ok(_: AlgId, _: u32) -> NtStatus {
    STATUS_SUCCESS
}

fn generate_invalid_parameter(_: AlgId, _: u32) -> NtStatus {
    STATUS_INVALID_PARAMETER
}

fn import_ok(_: AlgId) -> NtStatus {
    STATUS_SUCCESS
}

fn import_bad_signature(_: AlgId) -> NtStatus {
    STATUS_INVALID_SIGNATURE
}

/// The three algorithms of the oracle's enumeration list: AES, ECDSA_P256 and RSA.
fn enumerate_list(_ops: u32) -> Result<Vec<Vec<u16>>, NtStatus> {
    Ok(vec![utf16("AES"), utf16("ECDSA_P256"), utf16("RSA")])
}

fn enumerate_no_memory(_ops: u32) -> Result<Vec<Vec<u16>>, NtStatus> {
    Err(STATUS_NO_MEMORY)
}

#[test]
fn o01_map_ntstatus_handle_signature_and_success() {
    assert_eq!(map_ntstatus(0xC000_0008), 0x8009_0026);
    assert_eq!(map_ntstatus(0xC000_A000), 0x8009_0006);
    assert_eq!(map_ntstatus(0), 0);
}

#[test]
fn o02_map_ntstatus_parameter_memory_and_default() {
    assert_eq!(map_ntstatus(0xC000_000D), 0x8009_0027);
    assert_eq!(map_ntstatus(0xC000_0017), 0x8009_000E);
    assert_eq!(map_ntstatus(0xC000_00BB), 0x8009_0029);
    assert_eq!(map_ntstatus(0xC000_0023), 0x8009_0028);
    assert_eq!(map_ntstatus(0x8009_0005), 0x8009_0005);
    assert_eq!(map_ntstatus(0xC000_0001), 0x8009_002D);
}

#[test]
fn o03_create_checks_provider_then_name() {
    let rsa = utf16("RSA");
    assert_eq!(
        create_persisted_key(None, Some(&rsa), &mut generate_ok).err(),
        Some(0x8009_0026)
    );
    assert_eq!(
        create_persisted_key(Some(Handle::Provider), None, &mut generate_ok).err(),
        Some(0x8007_06F4)
    );
}

#[test]
fn o04_create_refuses_a_name_without_a_persisted_form() {
    let ecdsa = utf16("ECDSA");
    assert_eq!(
        create_persisted_key(Some(Handle::Provider), Some(&ecdsa), &mut generate_ok).err(),
        Some(0x8009_0029)
    );
}

#[test]
fn o05_create_rsa_generates_the_1024_bit_default() {
    let rsa = utf16("RSA");
    let generated = Cell::new(0);
    let key = create_persisted_key(
        Some(Handle::Provider),
        Some(&rsa),
        &mut |_: AlgId, bits: u32| {
            generated.set(bits);
            STATUS_SUCCESS
        },
    )
    .expect("RSA key");
    assert_eq!(generated.get(), 1024);
    assert_eq!(key.algid().name(), "RSA");
    assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(1024)));
    assert_eq!(stored(&key, PUBLIC_KEY_LENGTH_PROPERTY), Some(le(1024)));
}

#[test]
fn o06_create_ecdsa_p384_is_matched_without_case() {
    let name = utf16("ecdsa_p384");
    let key = create_persisted_key(Some(Handle::Provider), Some(&name), &mut generate_ok)
        .expect("P384 key");
    assert_eq!(key.algid().name(), "ECDSA_P384");
    assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(384)));
    assert_eq!(stored(&key, PUBLIC_KEY_LENGTH_PROPERTY), Some(le(384)));
}

#[test]
fn o07_create_maps_a_generation_failure() {
    let name = utf16("ECDSA_P521");
    let result = create_persisted_key(
        Some(Handle::Provider),
        Some(&name),
        &mut generate_invalid_parameter,
    );
    assert_eq!(result.err(), Some(0x8009_0027));
}

#[test]
fn o08_algorithm_name_size_query_and_short_buffer() {
    let key = Key::new(AlgId::Rsa, 1024).expect("RSA key");
    let name = utf16(ALGORITHM_PROPERTY);
    let query = get_property(Some(key.properties()), &name, None);
    assert_eq!((query.size, query.status), (Some(8), 0));
    let short = get_property(Some(key.properties()), &name, Some(4));
    assert_eq!((short.size, short.status), (Some(8), 0x8009_0028));
}

#[test]
fn o09_algorithm_name_and_lengths_bytes() {
    let key = Key::new(AlgId::Rsa, 1024).expect("RSA key");
    let name = get_property(Some(key.properties()), &utf16(ALGORITHM_PROPERTY), Some(8));
    assert_eq!((name.size, name.status), (Some(8), 0));
    assert_eq!(name.value, b"R\0S\0A\0\0\0");
    let lengths = get_property(Some(key.properties()), &utf16(LENGTHS_PROPERTY), Some(16));
    assert_eq!((lengths.size, lengths.status), (Some(16), 0));
    assert_eq!(
        lengths.value,
        [
            0x00, 0x02, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x04,
            0x00, 0x00
        ]
    );
}

#[test]
fn o10_ecdsa_has_no_block_length_and_a_store_is_required() {
    let key = Key::new(AlgId::EcdsaP256, 256).expect("P256 key");
    let name = utf16(BLOCK_LENGTH_PROPERTY);
    assert_eq!(
        get_property(Some(key.properties()), &name, None).status,
        0x8009_0027
    );
    assert_eq!(get_property(None, &name, None).status, 0x8009_0026);
}

#[test]
fn o11_provider_handle_is_refused_and_key_type_round_trips() {
    let mut key = Key::new(AlgId::Rsa, 1024).expect("RSA key");
    let provider = utf16(PROVIDER_HANDLE_PROPERTY);
    assert_eq!(
        set_property(Some(key.properties_mut()), &provider, &[0u8; 8]),
        0x8009_0029
    );
    let key_type = utf16(KEY_TYPE_PROPERTY);
    assert_eq!(
        set_property(
            Some(key.properties_mut()),
            &key_type,
            &[0x78, 0x56, 0x34, 0x12]
        ),
        0
    );
    let read = get_property(Some(key.properties()), &key_type, Some(4));
    assert_eq!((read.size, read.status), (Some(4), 0));
    assert_eq!(read.value, [0x78, 0x56, 0x34, 0x12]);
}

#[test]
fn o12_custom_property_round_trips() {
    let mut key = Key::new(AlgId::Rsa, 1024).expect("RSA key");
    let custom = utf16("Custom");
    assert_eq!(
        set_property(Some(key.properties_mut()), &custom, &[0xaa, 0xbb, 0xcc]),
        0
    );
    let read = get_property(Some(key.properties()), &custom, Some(3));
    assert_eq!((read.size, read.status), (Some(3), 0));
    assert_eq!(read.value, [0xaa, 0xbb, 0xcc]);
}

#[test]
fn o13_import_checks_decrypt_then_flags_then_blob() {
    let rsa = blob(RSA_PUBLIC_MAGIC, 2048);
    let decrypt = import_key(
        Some(Handle::Provider),
        true,
        0,
        Some(rsa.as_slice()),
        &mut import_ok,
    );
    assert_eq!(decrypt.err(), Some(0x8009_0029));
    let flags = import_key(
        Some(Handle::Provider),
        false,
        0x41,
        Some(rsa.as_slice()),
        &mut import_ok,
    );
    assert_eq!(flags.err(), Some(0x8009_0009));
    let no_blob = import_key(Some(Handle::Provider), false, 0x40, None, &mut import_ok);
    assert_eq!(no_blob.err(), Some(0x8009_0027));
}

#[test]
fn o14_import_rsa_public_blob_takes_its_bit_length() {
    let rsa = blob(RSA_PUBLIC_MAGIC, 2048);
    let key = import_key(
        Some(Handle::Provider),
        false,
        0,
        Some(rsa.as_slice()),
        &mut import_ok,
    )
    .expect("RSA import");
    assert_eq!(key.algid().name(), "RSA");
    assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(2048)));
    assert_eq!(stored(&key, PUBLIC_KEY_LENGTH_PROPERTY), Some(le(2048)));
}

#[test]
fn o15_import_ecdsa_private_blob_selects_the_curve() {
    let ecdsa = blob(ECDSA_P384_PRIVATE_MAGIC, 0);
    let key = import_key(
        Some(Handle::Provider),
        false,
        0,
        Some(ecdsa.as_slice()),
        &mut import_ok,
    )
    .expect("P384 import");
    assert_eq!(key.algid().name(), "ECDSA_P384");
    assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(384)));
    assert_eq!(stored(&key, PUBLIC_KEY_LENGTH_PROPERTY), Some(le(384)));
}

#[test]
fn o16_import_rejects_an_unknown_magic_and_maps_a_bad_signature() {
    let unknown = blob(0x1234_5678, 0);
    let result = import_key(
        Some(Handle::Provider),
        false,
        0,
        Some(unknown.as_slice()),
        &mut import_ok,
    );
    assert_eq!(result.err(), Some(0x8009_0027));
    let rsa = blob(RSA_PUBLIC_MAGIC, 2048);
    let result = import_key(
        Some(Handle::Provider),
        false,
        0,
        Some(rsa.as_slice()),
        &mut import_bad_signature,
    );
    assert_eq!(result.err(), Some(0x8009_0006));
}

#[test]
fn o17_encrypt_checks_flags_then_handle_kind() {
    assert_eq!(encrypt(None, 0x8).err(), Some(0x8009_0009));
    assert_eq!(
        encrypt(Some(Handle::Provider), 0x2).err(),
        Some(0x8009_0026)
    );
    assert_eq!(encrypt(Some(Handle::Key(AlgId::Rsa)), 0x2), Ok(()));
}

#[test]
fn o18_sign_hash_forwards_flags_without_silent() {
    assert_eq!(sign_hash(Some(Handle::Key(AlgId::Rsa)), 0x40), Ok(0));
    assert_eq!(sign_hash(Some(Handle::Key(AlgId::Rsa)), 0x50), Ok(0x10));
}

#[test]
fn o19_verify_signature_checks_symmetry_sizes_and_pointers() {
    let aes = verify_signature(Some(Handle::Key(AlgId::Aes)), buffer(32), buffer(64));
    assert_eq!(aes.err(), Some(0x8009_0029));
    let empty = verify_signature(Some(Handle::Key(AlgId::Rsa)), buffer(0), buffer(64));
    assert_eq!(empty.err(), Some(0x8009_0027));
    let null = verify_signature(Some(Handle::Key(AlgId::Rsa)), null_buffer(32), buffer(64));
    assert_eq!(null.err(), Some(0x8007_06F4));
    assert_eq!(
        verify_signature(Some(Handle::Key(AlgId::Rsa)), buffer(32), buffer(64)),
        Ok(())
    );
}

#[test]
fn o20_export_refuses_an_encryption_key_or_parameters() {
    assert_eq!(
        export_key(Some(Handle::Key(AlgId::Rsa)), true, false).err(),
        Some(0x8009_0029)
    );
    assert_eq!(
        export_key(Some(Handle::Key(AlgId::Rsa)), false, true).err(),
        Some(0x8009_0029)
    );
    assert_eq!(
        export_key(Some(Handle::Key(AlgId::Rsa)), false, false),
        Ok(())
    );
}

#[test]
fn o21_is_alg_supported_checks_provider_name_then_flags() {
    let rsa = utf16("RSA");
    let rsa_sign = utf16("rsa_sign");
    let none = is_alg_supported(None, Some(rsa.as_slice()), 0, &mut enumerate_list);
    assert_eq!(none.err(), Some(0x8009_0026));
    let signing = is_alg_supported(
        Some(Handle::Provider),
        Some(rsa_sign.as_slice()),
        0,
        &mut enumerate_list,
    );
    assert_eq!(signing.err(), Some(0x8009_0029));
    let flags = is_alg_supported(
        Some(Handle::Provider),
        Some(rsa.as_slice()),
        2,
        &mut enumerate_list,
    );
    assert_eq!(flags.err(), Some(0x8009_0009));
}

#[test]
fn o22_is_alg_supported_enumerates_the_supported_operations() {
    let ecdsa = utf16("ecdsa_p256");
    let mask = Cell::new(0);
    let result = is_alg_supported(
        Some(Handle::Provider),
        Some(ecdsa.as_slice()),
        0,
        &mut |ops: u32| {
            mask.set(ops);
            enumerate_list(ops)
        },
    );
    assert_eq!(result, Ok(()));
    assert_eq!(mask.get(), 0x1D);
}

#[test]
fn o23_is_alg_supported_refuses_a_name_outside_the_list() {
    let dsa = utf16("DSA");
    let mask = Cell::new(0);
    let result = is_alg_supported(
        Some(Handle::Provider),
        Some(dsa.as_slice()),
        0,
        &mut |ops: u32| {
            mask.set(ops);
            enumerate_list(ops)
        },
    );
    assert_eq!(result.err(), Some(0x8009_0029));
    assert_eq!(mask.get(), 0x1D);
}

#[test]
fn o24_is_alg_supported_maps_an_enumeration_failure() {
    let rsa = utf16("RSA");
    let result = is_alg_supported(
        Some(Handle::Provider),
        Some(rsa.as_slice()),
        0,
        &mut enumerate_no_memory,
    );
    assert_eq!(result.err(), Some(0x8009_000E));
}
