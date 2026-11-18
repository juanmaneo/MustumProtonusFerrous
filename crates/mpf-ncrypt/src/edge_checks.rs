//! Edge checks for the argument rules, name tables and property states that the oracle rows do not reach.

use super::*;

const RSA_PUBLIC_MAGIC: u32 = 0x3141_5352;

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

fn persisted(text: &str) -> Option<(&'static str, u32)> {
    from_persisted_name(&utf16(text)).map(|(algid, bits)| (algid.name(), bits))
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

fn generate_unsuccessful(_: AlgId, _: u32) -> NtStatus {
    STATUS_UNSUCCESSFUL
}

fn generate_no_memory(_: AlgId, _: u32) -> NtStatus {
    STATUS_NO_MEMORY
}

fn import_ok(_: AlgId) -> NtStatus {
    STATUS_SUCCESS
}

fn import_unsuccessful(_: AlgId) -> NtStatus {
    STATUS_UNSUCCESSFUL
}

fn import_no_memory(_: AlgId) -> NtStatus {
    STATUS_NO_MEMORY
}

fn enumerate_list(_ops: u32) -> Result<Vec<Vec<u16>>, NtStatus> {
    Ok(vec![utf16("AES"), utf16("ECDSA_P256"), utf16("RSA")])
}

fn enumerate_no_memory(_ops: u32) -> Result<Vec<Vec<u16>>, NtStatus> {
    Err(STATUS_NO_MEMORY)
}

#[test]
fn algorithm_names_and_symmetry() {
    let names = [
        (AlgId::Aes, "AES"),
        (AlgId::Rsa, "RSA"),
        (AlgId::Dsa, "DSA"),
        (AlgId::Ecdsa, "ECDSA"),
        (AlgId::EcdsaP256, "ECDSA_P256"),
        (AlgId::EcdsaP384, "ECDSA_P384"),
        (AlgId::EcdsaP521, "ECDSA_P521"),
    ];
    for (algid, name) in names {
        assert_eq!(algid.name(), name);
    }
    assert!(AlgId::Aes.is_symmetric());
    assert!(!AlgId::Rsa.is_symmetric());
    assert!(!AlgId::Dsa.is_symmetric());
    assert!(!AlgId::EcdsaP521.is_symmetric());
}

#[test]
fn persisted_names_fold_ascii_and_stop_at_nul() {
    assert_eq!(persisted("rsa"), Some(("RSA", 1024)));
    assert_eq!(persisted("Ecdsa_P256"), Some(("ECDSA_P256", 256)));
    assert_eq!(persisted("ecdsa_p384"), Some(("ECDSA_P384", 384)));
    assert_eq!(persisted("ECDSA_P521\0junk"), Some(("ECDSA_P521", 521)));
    assert_eq!(persisted("RSA_SIGN"), None);
    assert_eq!(persisted("ECDSA"), None);
    assert_eq!(persisted("RSB"), None);
    assert_eq!(persisted(""), None);
}

#[test]
fn rsa_key_template_holds_the_wine_properties() {
    let key = Key::new(AlgId::Rsa, 2048).expect("RSA key");
    assert_eq!(key.algid().name(), "RSA");
    assert_eq!(stored(&key, ALGORITHM_PROPERTY), Some(wide_bytes("RSA")));
    assert_eq!(
        stored(&key, ALGORITHM_GROUP_PROPERTY),
        Some(wide_bytes("RSA"))
    );
    let lengths = vec![
        0x00, 0x02, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00,
    ];
    assert_eq!(stored(&key, LENGTHS_PROPERTY), Some(lengths));
    assert_eq!(stored(&key, BLOCK_LENGTH_PROPERTY), Some(le(128)));
    assert_eq!(stored(&key, SIGNATURE_LENGTH_PROPERTY), Some(le(128)));
    assert_eq!(stored(&key, EXPORT_POLICY_PROPERTY), Some(le(0)));
    assert_eq!(stored(&key, KEY_USAGE_PROPERTY), Some(le(0x00ff_ffff)));
    assert_eq!(stored(&key, KEY_TYPE_PROPERTY), Some(le(0)));
    assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(2048)));
    assert_eq!(stored(&key, PUBLIC_KEY_LENGTH_PROPERTY), Some(le(2048)));
    assert_eq!(stored(&key, PROVIDER_HANDLE_PROPERTY), None);
}

#[test]
fn ecdsa_key_template_has_no_rsa_block_or_lengths() {
    let key = Key::new(AlgId::EcdsaP256, 256).expect("P256 key");
    assert_eq!(
        stored(&key, ALGORITHM_PROPERTY),
        Some(wide_bytes("ECDSA_P256"))
    );
    assert_eq!(
        stored(&key, ALGORITHM_GROUP_PROPERTY),
        Some(wide_bytes("ECDSA"))
    );
    assert_eq!(stored(&key, LENGTHS_PROPERTY), None);
    assert_eq!(stored(&key, BLOCK_LENGTH_PROPERTY), None);
    assert_eq!(stored(&key, SIGNATURE_LENGTH_PROPERTY), None);
    assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(256)));
    assert_eq!(stored(&key, PUBLIC_KEY_LENGTH_PROPERTY), Some(le(256)));
}

#[test]
fn only_rsa_and_the_ecdsa_curves_have_key_templates() {
    assert!(Key::new(AlgId::Aes, 128).is_none());
    assert!(Key::new(AlgId::Dsa, 1024).is_none());
    assert!(Key::new(AlgId::Ecdsa, 256).is_none());
    assert!(Key::new(AlgId::EcdsaP521, 521).is_some());
}

#[test]
fn create_checks_provider_then_name_then_algorithm() {
    let rsa = utf16("RSA");
    let dsa = utf16("DSA");
    assert_eq!(
        create_persisted_key(None, None, &mut generate_ok).err(),
        Some(NTE_INVALID_HANDLE)
    );
    let key_provider = Some(Handle::Key(AlgId::Rsa));
    assert_eq!(
        create_persisted_key(key_provider, Some(rsa.as_slice()), &mut generate_ok).err(),
        Some(NTE_INVALID_HANDLE)
    );
    assert_eq!(
        create_persisted_key(Some(Handle::Provider), None, &mut generate_ok).err(),
        Some(NULL_REF_POINTER)
    );
    let result = create_persisted_key(
        Some(Handle::Provider),
        Some(dsa.as_slice()),
        &mut generate_ok,
    );
    assert_eq!(result.err(), Some(NTE_NOT_SUPPORTED));
}

#[test]
fn create_accepts_each_persisted_name_at_its_default_length() {
    let cases: [(&str, &str, u32); 4] = [
        ("RSA", "RSA", 1024),
        ("ECDSA_P256", "ECDSA_P256", 256),
        ("ECDSA_P384", "ECDSA_P384", 384),
        ("ECDSA_P521", "ECDSA_P521", 521),
    ];
    for (text, expected, bits) in cases {
        let name = utf16(text);
        let key = create_persisted_key(
            Some(Handle::Provider),
            Some(name.as_slice()),
            &mut generate_ok,
        );
        let key = key.expect("persisted key");
        assert_eq!(key.algid().name(), expected);
        assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(bits)));
    }
}

#[test]
fn create_maps_generation_failures() {
    let rsa = utf16("RSA");
    let unsuccessful = create_persisted_key(
        Some(Handle::Provider),
        Some(rsa.as_slice()),
        &mut generate_unsuccessful,
    );
    assert_eq!(unsuccessful.err(), Some(NTE_INTERNAL_ERROR));
    let no_memory = create_persisted_key(
        Some(Handle::Provider),
        Some(rsa.as_slice()),
        &mut generate_no_memory,
    );
    assert_eq!(no_memory.err(), Some(NTE_NO_MEMORY));
}

#[test]
fn import_rejects_blobs_too_short_for_their_magic_or_bit_length() {
    let empty: &[u8] = &[];
    let partial_magic: &[u8] = &[0x45, 0x43];
    let rsa_magic_only: &[u8] = &[0x52, 0x53, 0x41, 0x31];
    let rsa_partial_bits: &[u8] = &[0x52, 0x53, 0x41, 0x31, 0x00, 0x08, 0x00];
    for bytes in [empty, partial_magic, rsa_magic_only, rsa_partial_bits] {
        let result = import_key(
            Some(Handle::Provider),
            false,
            0,
            Some(bytes),
            &mut import_ok,
        );
        assert_eq!(result.err(), Some(NTE_BAD_DATA));
    }
}

#[test]
fn import_reads_an_ecdsa_magic_from_four_bytes() {
    let magic: &[u8] = &0x3153_4345u32.to_le_bytes();
    let key = import_key(
        Some(Handle::Provider),
        false,
        0,
        Some(magic),
        &mut import_ok,
    )
    .expect("P256 import");
    assert_eq!(key.algid().name(), "ECDSA_P256");
    assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(256)));
}

#[test]
fn import_selects_the_algorithm_of_every_magic() {
    let cases: [(u32, &str, u32); 9] = [
        (0x3141_5352, "RSA", 2048),
        (0x3241_5352, "RSA", 1536),
        (0x3341_5352, "RSA", 1024),
        (0x3153_4345, "ECDSA_P256", 256),
        (0x3253_4345, "ECDSA_P256", 256),
        (0x3353_4345, "ECDSA_P384", 384),
        (0x3453_4345, "ECDSA_P384", 384),
        (0x3553_4345, "ECDSA_P521", 521),
        (0x3653_4345, "ECDSA_P521", 521),
    ];
    for (magic, expected, bits) in cases {
        let bytes = blob(magic, bits);
        let key = import_key(
            Some(Handle::Provider),
            false,
            0,
            Some(bytes.as_slice()),
            &mut import_ok,
        );
        let key = key.expect("import");
        assert_eq!(key.algid().name(), expected);
        assert_eq!(stored(&key, LENGTH_PROPERTY), Some(le(bits)));
    }
}

#[test]
fn import_checks_run_in_wine_order() {
    let rsa = blob(RSA_PUBLIC_MAGIC, 2048);
    let unknown = blob(0x1234_5678, 0);
    let short: &[u8] = &[0x52];
    assert_eq!(
        import_key(None, true, 0x41, None, &mut import_ok).err(),
        Some(NTE_NOT_SUPPORTED)
    );
    assert_eq!(
        import_key(None, false, 0x41, None, &mut import_ok).err(),
        Some(NTE_BAD_FLAGS)
    );
    assert_eq!(
        import_key(None, false, 0, None, &mut import_ok).err(),
        Some(NTE_INVALID_PARAMETER)
    );
    assert_eq!(
        import_key(None, false, 0, Some(short), &mut import_ok).err(),
        Some(NTE_BAD_DATA)
    );
    let unknown_result = import_key(None, false, 0, Some(unknown.as_slice()), &mut import_ok);
    assert_eq!(unknown_result.err(), Some(NTE_INVALID_PARAMETER));
    let no_provider = import_key(None, false, 0, Some(rsa.as_slice()), &mut import_ok);
    assert_eq!(no_provider.err(), Some(NTE_INVALID_HANDLE));
    let key_provider = Some(Handle::Key(AlgId::Rsa));
    let key_result = import_key(key_provider, false, 0, Some(rsa.as_slice()), &mut import_ok);
    assert_eq!(key_result.err(), Some(NTE_INVALID_HANDLE));
}

#[test]
fn import_accepts_the_silent_flag() {
    let rsa = blob(RSA_PUBLIC_MAGIC, 2048);
    let key = import_key(
        Some(Handle::Provider),
        false,
        NCRYPT_SILENT_FLAG,
        Some(rsa.as_slice()),
        &mut import_ok,
    );
    assert_eq!(key.expect("silent import").algid().name(), "RSA");
}

#[test]
fn import_maps_bcrypt_failures() {
    let rsa = blob(RSA_PUBLIC_MAGIC, 2048);
    let unsuccessful = import_key(
        Some(Handle::Provider),
        false,
        0,
        Some(rsa.as_slice()),
        &mut import_unsuccessful,
    );
    assert_eq!(unsuccessful.err(), Some(NTE_INTERNAL_ERROR));
    let no_memory = import_key(
        Some(Handle::Provider),
        false,
        0,
        Some(rsa.as_slice()),
        &mut import_no_memory,
    );
    assert_eq!(no_memory.err(), Some(NTE_NO_MEMORY));
}

#[test]
fn encrypt_accepts_only_the_wine_flag_mask() {
    for flags in [0, 0x1, 0x2, 0x4, 0x40, 0x47] {
        assert_eq!(encrypt(Some(Handle::Key(AlgId::Rsa)), flags), Ok(()));
    }
    for flags in [0x8, 0x80, 0x8000_0000] {
        assert_eq!(
            encrypt(Some(Handle::Key(AlgId::Rsa)), flags).err(),
            Some(NTE_BAD_FLAGS)
        );
    }
    assert_eq!(encrypt(Some(Handle::Key(AlgId::Aes)), 0), Ok(()));
    assert_eq!(encrypt(None, 0x40).err(), Some(NTE_INVALID_HANDLE));
    assert_eq!(
        encrypt(Some(Handle::Provider), 0).err(),
        Some(NTE_INVALID_HANDLE)
    );
}

#[test]
fn decrypt_needs_a_key_handle_of_any_algorithm() {
    assert_eq!(decrypt(None).err(), Some(NTE_INVALID_HANDLE));
    assert_eq!(
        decrypt(Some(Handle::Provider)).err(),
        Some(NTE_INVALID_HANDLE)
    );
    assert_eq!(decrypt(Some(Handle::Key(AlgId::Aes))), Ok(()));
    assert_eq!(decrypt(Some(Handle::Key(AlgId::Rsa))), Ok(()));
}

#[test]
fn sign_hash_needs_a_key_and_drops_only_the_silent_flag() {
    assert_eq!(sign_hash(None, 0).err(), Some(NTE_INVALID_HANDLE));
    assert_eq!(
        sign_hash(Some(Handle::Provider), NCRYPT_SILENT_FLAG).err(),
        Some(NTE_INVALID_HANDLE)
    );
    assert_eq!(
        sign_hash(Some(Handle::Key(AlgId::EcdsaP256)), 0x7f),
        Ok(0x3f)
    );
    assert_eq!(sign_hash(Some(Handle::Key(AlgId::Aes)), 0), Ok(0));
}

#[test]
fn verify_signature_checks_sizes_then_pointers_then_handle() {
    assert_eq!(
        verify_signature(None, buffer(0), buffer(0)).err(),
        Some(NTE_INVALID_PARAMETER)
    );
    assert_eq!(
        verify_signature(None, null_buffer(0), buffer(64)).err(),
        Some(NTE_INVALID_PARAMETER)
    );
    assert_eq!(
        verify_signature(None, buffer(32), buffer(0)).err(),
        Some(NTE_INVALID_PARAMETER)
    );
    assert_eq!(
        verify_signature(None, null_buffer(32), buffer(64)).err(),
        Some(NULL_REF_POINTER)
    );
    assert_eq!(
        verify_signature(None, buffer(32), null_buffer(64)).err(),
        Some(NULL_REF_POINTER)
    );
    assert_eq!(
        verify_signature(None, buffer(32), buffer(64)).err(),
        Some(NTE_INVALID_HANDLE)
    );
    let provider = Some(Handle::Provider);
    assert_eq!(
        verify_signature(provider, buffer(32), buffer(64)).err(),
        Some(NTE_INVALID_HANDLE)
    );
    let aes = Some(Handle::Key(AlgId::Aes));
    assert_eq!(
        verify_signature(aes, buffer(32), buffer(64)).err(),
        Some(NTE_NOT_SUPPORTED)
    );
    let dsa = Some(Handle::Key(AlgId::Dsa));
    assert_eq!(verify_signature(dsa, buffer(32), buffer(64)), Ok(()));
}

#[test]
fn export_refuses_options_before_checking_the_handle() {
    assert_eq!(export_key(None, true, false).err(), Some(NTE_NOT_SUPPORTED));
    assert_eq!(export_key(None, false, true).err(), Some(NTE_NOT_SUPPORTED));
    assert_eq!(
        export_key(None, false, false).err(),
        Some(NTE_INVALID_HANDLE)
    );
    assert_eq!(
        export_key(Some(Handle::Provider), false, false).err(),
        Some(NTE_INVALID_HANDLE)
    );
    assert_eq!(
        export_key(Some(Handle::Key(AlgId::Aes)), false, false),
        Ok(())
    );
}

#[test]
fn is_alg_supported_checks_provider_name_then_flags_then_names() {
    assert_eq!(SUPPORTED_OPERATIONS, 0x1D);
    let rsa = utf16("RSA");
    let provider = Some(Handle::Provider);
    assert_eq!(
        is_alg_supported(None, None, 0, &mut enumerate_list).err(),
        Some(NTE_INVALID_HANDLE)
    );
    assert_eq!(
        is_alg_supported(provider, None, 1, &mut enumerate_list).err(),
        Some(NULL_REF_POINTER)
    );
    let flags = is_alg_supported(
        Some(Handle::Provider),
        Some(rsa.as_slice()),
        0x1,
        &mut enumerate_list,
    );
    assert_eq!(flags.err(), Some(NTE_BAD_FLAGS));
    let silent = is_alg_supported(
        Some(Handle::Provider),
        Some(rsa.as_slice()),
        NCRYPT_SILENT_FLAG,
        &mut enumerate_list,
    );
    assert_eq!(silent, Ok(()));
    let rsa_sign = utf16("RSA_SIGN");
    let signing = is_alg_supported(
        Some(Handle::Provider),
        Some(rsa_sign.as_slice()),
        0,
        &mut enumerate_list,
    );
    assert_eq!(signing.err(), Some(NTE_NOT_SUPPORTED));
    let lower = utf16("aes");
    assert_eq!(
        is_alg_supported(
            Some(Handle::Provider),
            Some(lower.as_slice()),
            0,
            &mut enumerate_list
        ),
        Ok(())
    );
    let with_nul = utf16("RSA\0junk");
    assert_eq!(
        is_alg_supported(
            Some(Handle::Provider),
            Some(with_nul.as_slice()),
            0,
            &mut enumerate_list
        ),
        Ok(())
    );
    let missing = utf16("RSB");
    let missing_result = is_alg_supported(
        Some(Handle::Provider),
        Some(missing.as_slice()),
        0,
        &mut enumerate_list,
    );
    assert_eq!(missing_result.err(), Some(NTE_NOT_SUPPORTED));
    let no_memory = is_alg_supported(
        Some(Handle::Provider),
        Some(rsa.as_slice()),
        0,
        &mut enumerate_no_memory,
    );
    assert_eq!(no_memory.err(), Some(NTE_NO_MEMORY));
}

#[test]
fn property_store_replaces_values_and_compares_names_exactly() {
    let mut store = PropertyStore::default();
    assert_eq!(store.get(&utf16("Name")), None);
    store.set(&utf16("Name"), &[1]);
    store.set(&utf16("Name"), &[2, 3]);
    assert_eq!(store.get(&utf16("Name")), Some(&[2, 3][..]));
    assert_eq!(store.get(&utf16("name")), None);
    store.set(&utf16("Name\0tail"), &[4]);
    assert_eq!(store.get(&utf16("Name")), Some(&[4][..]));
    assert_eq!(store.get(&utf16("Name\0tail")), Some(&[4][..]));
}

#[test]
fn get_property_reports_size_then_checks_the_buffer() {
    let key = Key::new(AlgId::EcdsaP256, 256).expect("P256 key");
    let store = key.properties();
    let length = utf16(LENGTH_PROPERTY);
    let expected = le(256);
    let sized = get_property(Some(store), &length, None);
    assert_eq!(
        (sized.size, sized.status, sized.value.len()),
        (Some(4), ERROR_SUCCESS, 0)
    );
    let short = get_property(Some(store), &length, Some(3));
    assert_eq!(
        (short.size, short.status, short.value.len()),
        (Some(4), NTE_BUFFER_TOO_SMALL, 0)
    );
    let exact = get_property(Some(store), &length, Some(4));
    assert_eq!(
        (exact.size, exact.status, exact.value),
        (Some(4), ERROR_SUCCESS, expected.as_slice())
    );
    let larger = get_property(Some(store), &length, Some(8));
    assert_eq!(
        (larger.size, larger.status, larger.value),
        (Some(4), ERROR_SUCCESS, expected.as_slice())
    );
    let missing = get_property(Some(store), &utf16("Nope"), Some(4));
    assert_eq!(
        (missing.size, missing.status),
        (None, NTE_INVALID_PARAMETER)
    );
    let no_store = get_property(None, &length, None);
    assert_eq!((no_store.size, no_store.status), (None, NTE_INVALID_HANDLE));
}

#[test]
fn set_property_needs_an_object_and_refuses_the_provider_handle() {
    let mut key = Key::new(AlgId::Rsa, 1024).expect("RSA key");
    assert_eq!(
        set_property(None, &utf16("Custom"), &[1]),
        NTE_INVALID_HANDLE
    );
    let provider = utf16(PROVIDER_HANDLE_PROPERTY);
    assert_eq!(
        set_property(Some(key.properties_mut()), &provider, &[1]),
        NTE_NOT_SUPPORTED
    );
    let provider_nul = utf16("Provider Handle\0junk");
    assert_eq!(
        set_property(Some(key.properties_mut()), &provider_nul, &[1]),
        NTE_NOT_SUPPORTED
    );
    let lower = utf16("provider handle");
    assert_eq!(
        set_property(Some(key.properties_mut()), &lower, &[1]),
        ERROR_SUCCESS
    );
    assert_eq!(stored(&key, "provider handle"), Some(vec![1]));
    assert_eq!(stored(&key, PROVIDER_HANDLE_PROPERTY), None);
}

#[test]
fn wide_helpers_stop_at_nul_and_fold_ascii_letters_only() {
    assert_eq!(utf16("a\u{e9}"), [0x61, 0xE9]);
    assert_eq!(wide_str(&[0x52, 0x53]), &[0x52, 0x53][..]);
    assert_eq!(wide_str(&[0x52, 0x00, 0x53]), &[0x52][..]);
    assert!(wide_str(&[]).is_empty());
    assert!(fold_eq(&utf16("RSA"), &utf16("rsa")));
    assert!(fold_eq(&[0x41, 0x00, 0x42], &[0x61]));
    assert!(!fold_eq(&utf16("RSA"), &utf16("RS")));
    assert!(!fold_eq(&utf16("RSA"), &utf16("RSB")));
    assert!(!fold_eq(&[0x40], &[0x60]));
    assert!(!fold_eq(&[0x5B], &[0x7B]));
    assert!(!fold_eq(&[], &[0x41]));
    assert!(wide_eq(
        &utf16("Provider Handle\0junk"),
        PROVIDER_HANDLE_PROPERTY
    ));
    assert!(!wide_eq(
        &utf16("provider handle"),
        PROVIDER_HANDLE_PROPERTY
    ));
    assert!(!wide_eq(&[0x41], "AB"));
    assert_eq!(wide_bytes("RSA"), b"R\0S\0A\0\0\0".to_vec());
}
