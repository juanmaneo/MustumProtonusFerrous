//! Edge checks for the cases the oracle rows do not reach. Expected bytes marked as C were printed by
//! `target/canon/rsaenh/edge.c`, which runs the verbatim functions under the same ASan and UBSan build
//! as the oracle. Tests marked "Rust only" pin the deviations listed in `lib.rs`. The C either writes
//! past a buffer, which ASan flags, or accepts an input that the crate rejects.

use super::*;
use crate::blob::{
    private_blob_len, CUR_BLOB_VERSION, PRIVATEKEYBLOB, PUBLICKEYBLOB, RSAENH_MAGIC_RSA1,
    RSAENH_MAGIC_RSA2, SIMPLEBLOB,
};
use crate::oracle_checks::{harness_random, hex, plaintext_blob, rsa_blob, CALG_RC2};

/// `count` bytes of the sequence `base, base + 1, ..`.
fn seq(base: u8, count: usize) -> Vec<u8> {
    (0..count).map(|i| base + i as u8).collect()
}

#[test]
fn copy_param_copies_into_a_larger_buffer_and_keeps_the_tail() {
    let mut buffer = [0xee; 8];
    assert_eq!(copy_param(Some(&mut buffer), &[1, 2, 3, 4]), Ok(4));
    assert_eq!(buffer, [1, 2, 3, 4, 0xee, 0xee, 0xee, 0xee]);
}

#[test]
fn copy_param_fits_exactly_and_accepts_an_empty_param() {
    let mut buffer = [0u8; 4];
    assert_eq!(copy_param(Some(&mut buffer), &[1, 2, 3, 4]), Ok(4));
    assert_eq!(buffer, [1, 2, 3, 4]);
    let mut empty: [u8; 0] = [];
    assert_eq!(copy_param(Some(&mut empty), &[]), Ok(0));
}

#[test]
fn nte_codes_are_the_winerror_values() {
    assert_eq!(Nte::BadLen.code(), 0x8009_0004);
    assert_eq!(Nte::BadData.code(), 0x8009_0005);
    assert_eq!(Nte::BadAlgId.code(), 0x8009_0008);
    assert_eq!(Nte::BadFlags.code(), 0x8009_0009);
    assert_eq!(Nte::BadType.code(), 0x8009_000a);
}

#[test]
fn pad_at_the_minimum_length_has_eight_padding_bytes() {
    let mut out = [0u8; 16];
    pad_data_pkcs1(b"hello", &mut out, 0, harness_random()).unwrap();
    assert_eq!(out.to_vec(), hex("000220212224252627280068656c6c6f"));
}

#[test]
fn pad_ssl2_fallback_at_the_minimum_length() {
    let mut out = [0u8; 16];
    pad_data_pkcs1(b"hello", &mut out, CRYPT_SSL2_FALLBACK, harness_random()).unwrap();
    assert_eq!(out.to_vec(), hex("000203030303030303030068656c6c6f"));
}

#[test]
fn pad_empty_data_into_an_eleven_byte_block() {
    let mut out = [0u8; 11];
    pad_data_pkcs1(&[], &mut out, 0, harness_random()).unwrap();
    assert_eq!(out.to_vec(), hex("0002202122242526272800"));
}

#[test]
fn pad_a_21_byte_payload_into_32_bytes() {
    let data: Vec<u8> = (0x41..0x56).collect();
    let mut out = [0u8; 32];
    pad_data_pkcs1(&data, &mut out, 0, harness_random()).unwrap();
    assert_eq!(
        out.to_vec(),
        hex("00022021222425262728004142434445464748494a4b4c4d4e4f505152535455")
    );
}

/// Rust only: the C accepts these blocks. Its `dwBufferLen - 11` wraps below 11 bytes, so a 10-byte block
/// comes back as `00 02 20 21 00 'hello'` with two padding bytes, and a 9-byte block as
/// `00 02 20 00 'hello'`. A 5-byte block for 5 bytes of data is ASan `heap-buffer-overflow`. The crate
/// returns `BadLen` for all three.
#[test]
fn pad_rejects_blocks_under_eleven_bytes() {
    for size in [5, 9, 10] {
        let mut out = vec![0u8; size];
        assert_eq!(
            pad_data_pkcs1(b"hello", &mut out, 0, harness_random()),
            Err(Nte::BadLen)
        );
    }
}

/// Rust only: the C returns `00 02 00 42 43` for `ABC` in 5 bytes (edge probe `overlap.c`). The separator
/// overwrites the first data byte, and ASan reports nothing. The crate returns `BadLen`.
#[test]
fn pad_rejects_data_that_the_separator_would_overwrite() {
    let mut out = [0u8; 5];
    assert_eq!(
        pad_data_pkcs1(b"ABC", &mut out, 0, harness_random()),
        Err(Nte::BadLen)
    );
}

#[test]
fn unpad_rejects_a_block_shorter_than_three_bytes() {
    assert_eq!(unpad_data_pkcs1(&[0x00, 0x02], 8), Err(Nte::BadData));
}

#[test]
fn unpad_rejects_a_block_without_the_leading_zero() {
    let block = [0x01, 0x02, 0x05, 0x00, b'x', b'y'];
    assert_eq!(unpad_data_pkcs1(&block, 6), Err(Nte::BadData));
}

#[test]
fn unpad_returns_a_payload_that_fills_the_capacity() {
    let block = [0x00, 0x02, 0x05, 0x00, b'x', b'y'];
    assert_eq!(unpad_data_pkcs1(&block, 2), Ok(&b"xy"[..]));
}

#[test]
fn unpad_returns_an_empty_payload_at_the_end_of_the_block() {
    let block = [0x00, 0x02, 0x01, 0x00];
    assert_eq!(unpad_data_pkcs1(&block, 1), Ok(&[][..]));
}

#[test]
fn unpad_takes_the_bytes_after_a_separator_at_index_two() {
    let block = [0x00, 0x02, 0x00, 0xaa, 0xbb];
    assert_eq!(unpad_data_pkcs1(&block, 8), Ok(&[0xaa, 0xbb][..]));
}

#[test]
fn signature_md2_with_eight_padding_bytes() {
    let mut sig = [0u8; 45];
    build_hash_signature(&mut sig, CALG_MD2, &seq(0x10, 16), 0).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("0001ffffffffffffffff003020300c06082a864886f70d020205000410101112131415161718191a1b1c1d1e1f")
    );
}

#[test]
fn signature_md4_with_eight_padding_bytes() {
    let mut sig = [0u8; 45];
    build_hash_signature(&mut sig, CALG_MD4, &seq(0x10, 16), 0).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("0001ffffffffffffffff003020300c06082a864886f70d020405000410101112131415161718191a1b1c1d1e1f")
    );
}

#[test]
fn signature_sha1_with_eight_padding_bytes() {
    let mut sig = [0u8; 46];
    build_hash_signature(&mut sig, CALG_SHA, &seq(0x30, 20), 0).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("0001ffffffffffffffff003021300906052b0e03021a05000414303132333435363738393a3b3c3d3e3f40414243")
    );
}

#[test]
fn signature_sha384_with_eight_padding_bytes() {
    let mut sig = [0u8; 78];
    build_hash_signature(&mut sig, CALG_SHA_384, &seq(0x60, 48), 0).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("0001ffffffffffffffff003041300d060960864801650304020205000430606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f")
    );
}

#[test]
fn signature_sha512_with_eight_padding_bytes() {
    let mut sig = [0u8; 94];
    build_hash_signature(&mut sig, CALG_SHA_512, &seq(0x80, 64), 0).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("0001ffffffffffffffff003051300d060960864801650304020305000440808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf")
    );
}

#[test]
fn signature_x931_at_the_minimum_length() {
    let mut sig = [0u8; 20];
    build_hash_signature(&mut sig, CALG_MD5, &seq(0x10, 16), CRYPT_X931_FORMAT).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("6bba101112131415161718191a1b1c1d1e1f33cc")
    );
}

#[test]
fn signature_nohashoid_at_the_minimum_length() {
    let mut sig = [0u8; 35];
    build_hash_signature(&mut sig, CALG_SHA_256, &seq(0xa0, 32), CRYPT_NOHASHOID).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("000100a0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf")
    );
}

#[test]
fn signature_x931_takes_precedence_over_nohashoid() {
    let mut sig = [0u8; 64];
    let flags = CRYPT_X931_FORMAT | CRYPT_NOHASHOID;
    build_hash_signature(&mut sig, CALG_MD5, &seq(0x10, 16), flags).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("6bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba101112131415161718191a1b1c1d1e1f33cc")
    );
}

#[test]
fn signature_ssl3_pair_at_the_minimum_length() {
    let mut sig = [0u8; 39];
    build_hash_signature(&mut sig, CALG_SSL3_SHAMD5, &seq(0x40, 36), 0).unwrap();
    assert_eq!(
        sig.to_vec(),
        hex("000100404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f60616263")
    );
}

/// Rust only: ASan flags both blocks in the C with `heap-buffer-overflow`. The crate returns `BadLen`.
#[test]
fn signature_rejects_blocks_shorter_than_the_encoding() {
    let mut sig = [0u8; 36];
    assert_eq!(
        build_hash_signature(&mut sig, CALG_MD5, &seq(0x10, 16), 0),
        Err(Nte::BadLen)
    );
    let mut sig = [0u8; 19];
    assert_eq!(
        build_hash_signature(&mut sig, CALG_MD5, &seq(0x10, 16), CRYPT_X931_FORMAT),
        Err(Nte::BadLen)
    );
}

#[test]
fn plaintext_key_of_zero_length_is_empty() {
    let blob = plaintext_blob(CALG_RC2, 0, &[]);
    assert_eq!(plaintext_key(&blob), Ok(&[][..]));
}

#[test]
fn plaintext_key_returns_only_the_declared_length() {
    let blob = plaintext_blob(CALG_RC2, 2, &[1, 2, 3, 4, 5]);
    assert_eq!(plaintext_key(&blob), Ok(&[1, 2][..]));
}

#[test]
fn plaintext_key_rejects_a_length_past_the_end() {
    let blob = plaintext_blob(CALG_RC2, 0xffff_ffff, &[]);
    assert_eq!(plaintext_key(&blob), Err(Nte::BadData));
}

/// Rust only: the C reads the key length from bytes 8 to 11, which ASan flags for blobs of 8 and 11
/// bytes. The crate returns `BadData` for any blob shorter than 12 bytes.
#[test]
fn plaintext_key_rejects_blobs_without_the_length_field() {
    let mut blob = plaintext_blob(CALG_RC2, 0, &[]);
    blob.truncate(11);
    assert_eq!(plaintext_key(&blob), Err(Nte::BadData));
    blob.truncate(8);
    assert_eq!(plaintext_key(&blob), Err(Nte::BadData));
}

#[test]
fn classify_recognises_a_simple_blob() {
    let blob = [SIMPLEBLOB, CUR_BLOB_VERSION, 0, 0, 0, 0, 0, 0];
    assert_eq!(classify_blob(&blob), Ok(BlobKind::SimpleKey));
}

#[test]
fn classify_rejects_a_header_shorter_than_eight_bytes() {
    let blob = [PRIVATEKEYBLOB, CUR_BLOB_VERSION, 0, 0, 0, 0, 0];
    assert_eq!(classify_blob(&blob), Err(Nte::BadData));
}

#[test]
fn private_blob_shorter_than_the_rsapubkey_is_rejected() {
    let blob = rsa_blob(PRIVATEKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA2, 512, 0);
    assert_eq!(check_private_key_blob(&blob[..19], 0), Err(Nte::BadData));
}

#[test]
fn private_blob_with_the_public_magic_is_rejected() {
    let blob = rsa_blob(
        PRIVATEKEYBLOB,
        CUR_BLOB_VERSION,
        RSAENH_MAGIC_RSA1,
        512,
        224,
    );
    assert_eq!(check_private_key_blob(&blob, 0), Err(Nte::BadData));
}

/// 520 bits is 65 bytes of modulus. `(520 + 8) >> 4` is 33, so the five CRT components take 165 bytes,
/// and the blob needs 250 bytes in all.
#[test]
fn private_blob_length_rounds_the_crt_components_up() {
    let short = rsa_blob(
        PRIVATEKEYBLOB,
        CUR_BLOB_VERSION,
        RSAENH_MAGIC_RSA2,
        520,
        229,
    );
    assert_eq!(check_private_key_blob(&short, 0), Err(Nte::BadData));
    let exact = rsa_blob(
        PRIVATEKEYBLOB,
        CUR_BLOB_VERSION,
        RSAENH_MAGIC_RSA2,
        520,
        230,
    );
    assert_eq!(
        check_private_key_blob(&exact, 0),
        Ok(RsaPubKey {
            magic: RSAENH_MAGIC_RSA2,
            bitlen: 520,
            pubexp: 65537,
        })
    );
}

#[test]
fn private_blob_with_zero_bitlen_is_only_the_header() {
    let blob = rsa_blob(PRIVATEKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA2, 0, 0);
    assert_eq!(
        check_private_key_blob(&blob, 0),
        Ok(RsaPubKey {
            magic: RSAENH_MAGIC_RSA2,
            bitlen: 0,
            pubexp: 65537,
        })
    );
}

/// Rust only: for `bitlen >= 0xfffffff8` the C's `bitlen + 8` wraps in 32 bits, so the C requires only
/// the modulus. A 512 MiB blob with that header is accepted by the C at exactly `0x2000_0013` bytes
/// (edge probe, route `R`). The crate requires the CRT components as well, `0x7000_0013` bytes.
#[test]
fn private_blob_length_does_not_wrap_at_the_top_of_bitlen() {
    assert_eq!(private_blob_len(0xffff_fff8), 0x7000_0013);
    let blob = rsa_blob(
        PRIVATEKEYBLOB,
        CUR_BLOB_VERSION,
        RSAENH_MAGIC_RSA2,
        0xffff_fff8,
        0,
    );
    assert_eq!(check_private_key_blob(&blob, 0), Err(Nte::BadData));
}

#[test]
fn public_blob_length_rounds_the_modulus_down() {
    let exact = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA1, 515, 64);
    assert_eq!(
        check_public_key_blob(&exact, 0),
        Ok(RsaPubKey {
            magic: RSAENH_MAGIC_RSA1,
            bitlen: 515,
            pubexp: 65537,
        })
    );
    let short = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA1, 515, 63);
    assert_eq!(check_public_key_blob(&short, 0), Err(Nte::BadData));
}

#[test]
fn public_blob_rejects_the_rsa2_magic_and_the_ipsec_flag() {
    let rsa2 = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA2, 512, 64);
    assert_eq!(check_public_key_blob(&rsa2, 0), Err(Nte::BadData));
    let rsa1 = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA1, 512, 64);
    assert_eq!(
        check_public_key_blob(&rsa1, CRYPT_IPSEC_HMAC_KEY),
        Err(Nte::BadFlags)
    );
}

#[test]
fn public_blob_shorter_than_the_rsapubkey_is_rejected() {
    let blob = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA1, 512, 64);
    assert_eq!(check_public_key_blob(&blob[..19], 0), Err(Nte::BadData));
}

#[test]
fn public_blob_with_zero_bitlen_is_only_the_header() {
    let blob = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA1, 0, 0);
    assert_eq!(
        check_public_key_blob(&blob, 0),
        Ok(RsaPubKey {
            magic: RSAENH_MAGIC_RSA1,
            bitlen: 0,
            pubexp: 65537,
        })
    );
}
