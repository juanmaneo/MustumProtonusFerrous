use super::*;

#[test]
fn key_sizing_and_capacity_edges() {
    assert_eq!(rounded_len(usize::MAX), Err(STATUS_INVALID_PARAMETER));
    assert_eq!(with_header(usize::MAX), Err(STATUS_INVALID_PARAMETER));
    assert_eq!(
        encrypt_data_with_length(&[], &[1], usize::MAX, usize::MAX),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(
        encrypt_data_with_length(&[], &[1], usize::MAX, usize::MAX - 7),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(
        encrypt_data_with_length(&[], &[1], usize::MAX, u32::MAX as usize + 1),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(
        decrypt_payload(&[], &[1, 2, 3, 4, 5, 6, 7], usize::MAX, usize::MAX),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(
        decrypt_payload(&[], &[1, 2, 3, 4, 5, 6, 7], usize::MAX, usize::MAX - 7),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(wire_len(u32::MAX as usize), Ok(u32::MAX));
    assert_eq!(
        wire_len(u32::MAX as usize + 1),
        Err(STATUS_INVALID_PARAMETER)
    );
    let short_key = [0x31, 0x72];
    let padded_key = [0x31, 0x72, 0, 0, 0, 0, 0];
    let input = [0x91; 8];
    assert_eq!(
        encrypt_block(&padded_key, &input),
        encrypt_block(&normalize_key(&short_key), &input)
    );
    assert_eq!(
        encrypt_block(&normalize_key(&[1, 2, 3, 4, 5, 6, 7, 8]), &input),
        encrypt_block(&[1, 2, 3, 4, 5, 6, 7], &input)
    );
    let empty_cipher = encrypt_data(&[], &short_key, 8).unwrap();
    assert_eq!(empty_cipher.len(), 8);
    assert_eq!(decrypt_data(&empty_cipher, &short_key, 0), Ok(Vec::new()));
    assert_eq!(encrypt_data(&[1], &[], 9), Err(STATUS_INVALID_PARAMETER_2));
    assert_eq!(encrypt_data(&[1], &[1], 8), Err(STATUS_BUFFER_TOO_SMALL));
    assert_eq!(decrypt_data(&[], &[], 0), Err(STATUS_INVALID_PARAMETER_2));
    assert_eq!(
        decrypt_data(&[0; 7], &[1], 0),
        Err(STATUS_INVALID_PARAMETER)
    );
}

#[test]
fn decrypt_header_and_buffer_edges() {
    let key = [0x62, 0x19, 0x27, 0x8a, 0x4c, 0x3d, 0xe1];
    let invalid_version_header = [8, 0, 0, 0, 2, 0, 0, 0];
    let invalid_version_cipher = encrypt_block(&key, &invalid_version_header);
    assert_eq!(
        decrypt_data(&invalid_version_cipher, &key, 8),
        Err(STATUS_UNKNOWN_REVISION)
    );

    let plain = [0x55; 8];
    let cipher = encrypt_data(&plain, &key, 16).unwrap();
    assert_eq!(decrypt_data(&cipher, &key, 7), Err(STATUS_BUFFER_TOO_SMALL));
    assert_eq!(
        decrypt_data(&cipher[..8], &key, 8),
        Err(STATUS_INVALID_PARAMETER)
    );
    assert_eq!(decrypt_data(&cipher, &key, 8), Ok(plain.to_vec()));

    let empty_cipher = encrypt_data(&[], &key, 8).unwrap();
    assert_eq!(decrypt_data(&empty_cipher, &key, 0), Ok(Vec::new()));
    assert_eq!(hash_lm_key(&key), encrypt_block(&key, b"KGS!@#$%"));
}
