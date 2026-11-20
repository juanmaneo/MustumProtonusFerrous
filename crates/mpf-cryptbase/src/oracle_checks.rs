use super::*;

#[test]
fn wine_des_block_oracle_rows() {
    let encrypted = [
        "8ca64de9c1b123a7",
        "7359b2163e4edc58",
        "a870feb76e129161",
        "ff389ab6a784bf57",
        "2c6a584f43dde4ff",
        "4fa514be58208411",
        "e44cd2f8c84df8e8",
        "eff377ab44847dec",
        "68dc3958c72d5468",
        "028f85fd57e0a3ba",
        "a80e5a3e1ba30239",
        "4a68e40a909b3190",
        "e23f000346a8fcab",
        "8b5976010b35bb02",
        "0c015820e31c0dcf",
        "9a9b44b339907edc",
        "e2a09251487f8645",
        "d7506b7abbfc87ed",
        "56364b6bdd7b18c4",
        "40209fcd52269865",
    ];
    for (index, expected) in encrypted.iter().enumerate() {
        let key: [u8; 7] = std::array::from_fn(|j| match index {
            0 => 0,
            1 => 255,
            _ => (index * 29 + j * 71) as u8,
        });
        let block: [u8; 8] = std::array::from_fn(|j| match index {
            0 => 0,
            1 => 255,
            _ => (index * 47 + j * 19 + 3) as u8,
        });
        let encrypted = encrypt_block(&key, &block);
        assert_eq!(hex(&encrypted), *expected, "oracle row {index}");
        assert_eq!(
            decrypt_block(&key, &encrypted),
            block,
            "decrypt row {index}"
        );
    }
}

#[test]
fn wine_system_function_003_to_005_oracle_rows() {
    let rows = [
        (
            1usize,
            "a7b7d2d89f04ea7d",
            "96591fd9e203c47ce00ed6cd0409448c",
        ),
        (
            8usize,
            "f758de5159adbe6d",
            "a03bb614bc307bf2279745ad07ce9b1a",
        ),
        (
            13usize,
            "5cf7eee7eab7e646",
            "822e30b2dc82d67eea06b5cb8bae49e4b85ecad2cf2b9b16",
        ),
    ];
    for (index, (len, expected_hash, expected_cipher)) in rows.iter().enumerate() {
        let key: Vec<u8> = (0..7).map(|j| (index * 17 + j * 31 + 1) as u8).collect();
        let plain: Vec<u8> = (0..19).map(|j| (index * 23 + j * 13 + 9) as u8).collect();
        let encrypted = encrypt_data(&plain[..*len], &key, 64).expect("oracle encryption");
        assert_eq!(hex(&hash_lm_key(&normalize_key(&key))), *expected_hash);
        assert_eq!(hex(&encrypted), *expected_cipher, "ECB oracle row {index}");
        assert_eq!(
            decrypt_data(&encrypted, &key, 64).expect("oracle decryption"),
            &plain[..*len]
        );
    }
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        result.push(char::from(DIGITS[usize::from(byte >> 4)]));
        result.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    result
}
