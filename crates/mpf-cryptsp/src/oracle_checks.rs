//! Oracle-derived vectors for the ported cryptsp logic.
//!
//! The rows were printed by `target/canon/cryptsp/harness`, which includes the verbatim Wine
//! `arc4.c`, `lmhash.c` and `crypt.c` and was built with `gcc -fsanitize=address,undefined -g`.
//! The DES block (`SystemFunction001`/`002`) is a fixed XOR stand-in in the harness, and
//! `StubDes` below is the same stand-in. These vectors therefore check the composition, not DES
//! itself. ASan and UBSan flagged no row, so none is excluded: 21 rows.

use super::*;

fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn arr<const N: usize>(s: &str) -> [u8; N] {
    hex(s).try_into().unwrap()
}

/// The 300-byte key `gen(i*7+3)[300]` from the harness.
fn gen_key() -> Vec<u8> {
    (0..300usize).map(|i| (i * 7 + 3) as u8).collect()
}

fn rc4(key: &[u8], input: &[u8]) -> Vec<u8> {
    let mut data = input.to_vec();
    assert_eq!(system_function_032(&mut data, key), STATUS_SUCCESS);
    data
}

/// Same stand-in as `SystemFunction001` and `SystemFunction002` in the harness.
pub(crate) struct StubDes;

impl Des for StubDes {
    fn encrypt_block(&self, key: &[u8; 7], data: &[u8; 8]) -> [u8; 8] {
        std::array::from_fn(|i| data[i] ^ key[i % 7] ^ 0x5a_u8.wrapping_add(i as u8))
    }

    fn decrypt_block(&self, key: &[u8; 7], data: &[u8; 8]) -> [u8; 8] {
        std::array::from_fn(|i| data[i] ^ key[i % 7] ^ 0xa5_u8.wrapping_add(i as u8))
    }
}

#[test]
fn oracle_rc4_rows() {
    // rc4_key_plaintext status=0 key=4b6579 in=506c61696e74657874 out=bbf316e8d940af0ad3
    assert_eq!(
        rc4(&hex("4b6579"), &hex("506c61696e74657874")),
        hex("bbf316e8d940af0ad3")
    );
    // rc4_wiki_pedia status=0 key=57696b69 in=7065646961 out=1021bf0420
    assert_eq!(rc4(&hex("57696b69"), &hex("7065646961")), hex("1021bf0420"));
    // rc4_secret_attack status=0 key=536563726574 in=41747461636b206174206461776e out=45a01f645fc35b383552544b9bf5
    assert_eq!(
        rc4(&hex("536563726574"), &hex("41747461636b206174206461776e")),
        hex("45a01f645fc35b383552544b9bf5")
    );
    // rc4_1byte_key_zeros8 status=0 key=01 in=0000000000000000 out=06080e0e18202929
    assert_eq!(rc4(&hex("01"), &[0u8; 8]), hex("06080e0e18202929"));
    // rc4_16byte_key_zeros16 status=0 key=000102030405060708090a0b0c0d0e0f in=00..00 out=e99c40f947e219cc06db97c60edd2a4f
    assert_eq!(
        rc4(&hex("000102030405060708090a0b0c0d0e0f"), &[0u8; 16]),
        hex("e99c40f947e219cc06db97c60edd2a4f")
    );
    // rc4_300byte_key_zeros16 status=0 key=gen(i*7+3)[300] in=00..00 out=61f388f9b9d4ce6a0c58c9197e721247
    assert_eq!(
        rc4(&gen_key(), &[0u8; 16]),
        hex("61f388f9b9d4ce6a0c58c9197e721247")
    );
    // rc4_3byte_key_zeros300 status=0 key=4b6579 in=zeros[300]; the state index wraps past 256
    assert_eq!(
        rc4(&hex("4b6579"), &[0u8; 300]),
        hex(concat!(
            "eb9f7781b734ca72a7194a2867b642950d5d4c2652177b9e7a8215ec54ab503",
            "e80328ac213693189d556897e79bccdefcb267d544b7455dd966a55f41a910e6",
            "c7027fed709a616a30f636fbe3164653046266fe1c65a8844cd4cd312a0f8b18",
            "d7f8231b4ad8c336bc8b1ffae6d11f5c09e8d7d2e753698bdb1a6186aee0a852",
            "7f1ecb7a8d4b5e28a5e8b5bf340ffa7a78167b385398570424346d2ee8808676",
            "c7fd3cef6fdd6ff587b659dc71361826f76504c41d42c40a30a2bf9e9ab512ea",
            "3c69fc135951c3b152b75eff81bf81cb48ea2b9e8781c8db1ac22edd3c9f7914",
            "7961822478291dd41ff3c2a347950a5ce14a04f1b63d74e42bd3c7bf2d31cc49",
            "092fdd9b6e404ef4fa075f1a344ed816bd0ef77a788f345a7352874ffccdb3b3",
            "ba2a96e53e48ff9e1b7f370ee",
        ))
    );
    // rc4_empty_data status=0 key=4b6579 in= out=
    assert_eq!(rc4(&hex("4b6579"), &[]), hex(""));
}

#[test]
fn oracle_lm_hash_rows() {
    // lm_empty pwd=00 out=111c0f7c1e7c4444111c0f7c1e7c4444
    assert_eq!(
        system_function_006(&StubDes, &hex("00")).to_vec(),
        hex("111c0f7c1e7c4444111c0f7c1e7c4444")
    );
    // lm_a pwd=6100 out=701c0f7c1e7c4425111c0f7c1e7c4444
    assert_eq!(
        system_function_006(&StubDes, &hex("6100")).to_vec(),
        hex("701c0f7c1e7c4425111c0f7c1e7c4444")
    );
    // lm_14chars pwd=4142434445464748494a4b4c4d4e00 out=505e4c385b3a03055955453752310a0c
    assert_eq!(
        system_function_006(&StubDes, &hex("4142434445464748494a4b4c4d4e00")).to_vec(),
        hex("505e4c385b3a03055955453752310a0c")
    );
    // lm_16chars pwd=4142434445464748494a4b4c4d4e4f5000 out=505e4c385b3a03055955453752310a0c
    assert_eq!(
        system_function_006(&StubDes, &hex("4142434445464748494a4b4c4d4e4f5000")).to_vec(),
        hex("505e4c385b3a03055955453752310a0c")
    );
    // lm_embedded_nul pwd=414200434400 out=505e0f7c1e7c4405111c0f7c1e7c4444
    assert_eq!(
        system_function_006(&StubDes, &hex("414200434400")).to_vec(),
        hex("505e0f7c1e7c4405111c0f7c1e7c4444")
    );
}

#[test]
fn oracle_lm_response_rows() {
    // lmresp_seq status=0 challenge=0102030405060708 hash=000102030405060708090a0b0c0d0e0f
    assert_eq!(
        system_function_008(
            &StubDes,
            &arr::<8>("0102030405060708"),
            &arr::<16>("000102030405060708090a0b0c0d0e0f"),
        )
        .to_vec(),
        hex("5b585d5a5f5c61695c51565350556a6e55565f595b596767")
    );
    // lmresp_ff_aa status=0 challenge=ffffffffffffffff hash=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    assert_eq!(
        system_function_008(
            &StubDes,
            &arr::<8>("ffffffffffffffff"),
            &arr::<16>("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        )
        .to_vec(),
        hex("0f0e09080b0a35340f0e09080b0a35340f0ea3a2a1a09f34")
    );
}

#[test]
fn oracle_des_pair_rows() {
    let data = arr::<16>("000102030405060708090a0b0c0d0e0f");
    let key = arr::<14>("101112131415161718191a1b1c1d");
    // des12_seq_key14 status=0 in=000102030405060708090a0b0c0d0e0f key=101112131415161718191a1b1c1d
    assert_eq!(
        system_function_012(&StubDes, &data, &key).to_vec(),
        hex("4a4b4c4d4e4f7076454a4f4c494e7379")
    );
    // des13_seq_key14 status=0 same in and key
    assert_eq!(
        system_function_013(&StubDes, &data, &key).to_vec(),
        hex("b5b6b7b8b9babbbbbab7b4b9bebbb8b4")
    );
}

#[test]
fn oracle_des_repeated_key_rows() {
    let data = arr::<16>("000102030405060708090a0b0c0d0e0f");
    let key = arr::<4>("deadbeef");
    // des24_seq_key4 status=0 in=000102030405060708090a0b0c0d0e0f key=deadbeef
    assert_eq!(
        system_function_024(&StubDes, &data, &key).to_vec(),
        hex("84f7e0b184f7d8b8bd8cfbe8bd8cc381")
    );
    // des25_seq_key4 status=0 same in and key
    assert_eq!(
        system_function_025(&StubDes, &data, &key).to_vec(),
        hex("7b0a1b44730213754271001d4a79084c")
    );
}

#[test]
fn oracle_equal16_rows() {
    // eq16_equal a=b=000102030405060708090a0b0c0d0e0f out=1
    let a = arr::<16>("000102030405060708090a0b0c0d0e0f");
    assert!(system_function_030(&a, &a));
    // eq16_last_byte_differs b=000102030405060708090a0b0c0d0eff out=0
    let b = arr::<16>("000102030405060708090a0b0c0d0eff");
    assert!(!system_function_030(&a, &b));
}
