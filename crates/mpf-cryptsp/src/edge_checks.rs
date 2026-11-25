//! Edge-case tests for the ported cryptsp logic.
//!
//! These cover the empty-key and empty-data paths, the 14-byte password window, the NUL
//! terminator, the key split points, and the DES direction. Expectations marked "not
//! oracle-derived" come from reading the C code or from properties of the port; the harness did
//! not print them.

use super::oracle_checks::StubDes;
use super::*;

#[test]
fn rc4_empty_key_is_rejected_and_data_untouched() {
    // Not oracle-derived: the C code reads key[0] from a zero-length buffer, so no output exists.
    let mut data = *b"unchanged";
    assert_eq!(system_function_032(&mut data, &[]), STATUS_UNSUCCESSFUL);
    assert_eq!(&data, b"unchanged");
}

#[test]
fn rc4_empty_data_returns_success() {
    // Not oracle-derived: no bytes are processed.
    assert_eq!(system_function_032(&mut [], b"Key"), STATUS_SUCCESS);
}

#[test]
fn rc4_twice_restores_input_across_state_wrap() {
    // Not oracle-derived: XOR with the same keystream twice is the identity. The 600 bytes pass
    // the 256-entry state several times, so the x and y counters wrap.
    let plain: Vec<u8> = (0..600u32).map(|i| (i * 13) as u8).collect();
    let mut data = plain.clone();
    assert_eq!(system_function_032(&mut data, b"Secret"), STATUS_SUCCESS);
    assert_ne!(data, plain);
    assert_eq!(system_function_032(&mut data, b"Secret"), STATUS_SUCCESS);
    assert_eq!(data, plain);
}

#[test]
fn lm_hash_uses_only_first_fourteen_bytes() {
    // Not oracle-derived: only tmp[0..14] feeds the DES keys, so bytes past 14 are ignored.
    assert_eq!(
        system_function_006(&StubDes, b"ABCDEFGHIJKLMN\0"),
        system_function_006(&StubDes, b"ABCDEFGHIJKLMNxyz\0")
    );
    // Not oracle-derived: the 14th byte is in the second key, so it does matter.
    assert_ne!(
        system_function_006(&StubDes, b"ABCDEFGHIJKLMN\0"),
        system_function_006(&StubDes, b"ABCDEFGHIJKLM\0")
    );
}

#[test]
fn lm_hash_reads_c_string_semantics() {
    // Not oracle-derived: the first NUL ends the password, and later bytes are ignored.
    assert_eq!(
        system_function_006(&StubDes, b"AB\0CD\0"),
        system_function_006(&StubDes, b"AB\0")
    );
    // Not oracle-derived: a slice without NUL is used whole, as if it ended at its length.
    assert_eq!(
        system_function_006(&StubDes, b"ABC"),
        system_function_006(&StubDes, b"ABC\0")
    );
    // Not oracle-derived: an empty password is the same as a lone NUL.
    assert_eq!(
        system_function_006(&StubDes, b""),
        system_function_006(&StubDes, b"\0")
    );
}

#[test]
fn lm_response_block_depends_only_on_its_key_slice() {
    // Not oracle-derived: response block i is encrypted under key[7i..7i+7], which is hash
    // bytes 7i..7i+7 (zero-padded past 16). Changing hash byte j therefore changes only block j/7.
    let challenge = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let hash = [0x11u8; 16];
    let base = system_function_008(&StubDes, &challenge, &hash);
    for j in 0..16 {
        let mut changed = hash;
        changed[j] ^= 0x80;
        let out = system_function_008(&StubDes, &challenge, &changed);
        for block in 0..3 {
            let same = out[block * 8..block * 8 + 8] == base[block * 8..block * 8 + 8];
            assert_eq!(same, block != j / 7, "hash byte {j}, block {block}");
        }
    }
}

#[test]
fn des_pair_block_zero_uses_key_head_and_data_head() {
    // Not oracle-derived: block 0 reads key[0..7] and data[0..8] only.
    let data = [0x01u8; 16];
    let key = [0x02u8; 14];
    let base = system_function_012(&StubDes, &data, &key);

    let mut data2 = data;
    data2[8..].fill(0xee);
    let mut key2 = key;
    key2[7..].fill(0x77);
    let out = system_function_012(&StubDes, &data2, &key2);
    assert_eq!(out[..8], base[..8]);
    assert_ne!(out[8..], base[8..]);
}

#[test]
fn des_pair_decrypt_differs_from_encrypt() {
    // Not oracle-derived: the decrypt path must call the decrypt primitive. The stand-in gives
    // different results for the two directions, so equal outputs would mean a wrong call.
    let data = [0u8; 16];
    let key = [0u8; 14];
    assert_ne!(
        system_function_012(&StubDes, &data, &key),
        system_function_013(&StubDes, &data, &key)
    );
    let key4 = [0u8; 4];
    assert_ne!(
        system_function_024(&StubDes, &data, &key4),
        system_function_025(&StubDes, &data, &key4)
    );
}

#[test]
fn des_repeated_key_matches_expanded_pair() {
    // Not oracle-derived: SystemFunction024 copies the 4-byte key four times into 16 bytes, and
    // only the first 14 of those bytes are used by the two DES blocks.
    let data: [u8; 16] = std::array::from_fn(|i| i as u8);
    let key = [0xde, 0xad, 0xbe, 0xef];
    let expanded: [u8; 14] = std::array::from_fn(|i| key[i % 4]);
    assert_eq!(
        system_function_024(&StubDes, &data, &key),
        system_function_012(&StubDes, &data, &expanded)
    );
    assert_eq!(
        system_function_025(&StubDes, &data, &key),
        system_function_013(&StubDes, &data, &expanded)
    );
}

#[test]
fn equal16_detects_first_and_last_byte_changes() {
    // Not oracle-derived: a change in the first or last byte alone makes the blocks differ.
    let a = [7u8; 16];
    assert!(system_function_030(&a, &a));
    let mut first = a;
    first[0] = 0;
    assert!(!system_function_030(&a, &first));
    let mut last = a;
    last[15] = 0;
    assert!(!system_function_030(&a, &last));
}
