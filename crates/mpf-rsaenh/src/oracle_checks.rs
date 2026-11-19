//! Replays of the oracle rows. `target/canon/rsaenh/harness.c` compiles the verbatim functions of
//! `dlls/rsaenh/rsaenh.c` with `-fsanitize=address,undefined`, and that run reports no sanitizer finding
//! for these 25 rows (`O01` to `O25`). Each test names the row it replays, and its expected bytes are the
//! output that run printed. `RtlGenRandom` is replaced by the fixed sequence of `harness_random`.

use super::*;
use crate::blob::{
    CUR_BLOB_VERSION, PLAINTEXTKEYBLOB, PRIVATEKEYBLOB, PUBLICKEYBLOB, RSAENH_MAGIC_RSA1,
    RSAENH_MAGIC_RSA2,
};

/// `CALG_RSA_KEYX`, the `aiKeyAlg` the harness writes into RSA blobs.
const CALG_RSA_KEYX: u32 = 0x0000_a400;
/// `CALG_RC2`, the `aiKeyAlg` the harness writes into plaintext blobs.
pub(crate) const CALG_RC2: u32 = 0x0000_6602;

/// Decodes the lowercase hex of an oracle row.
pub(crate) fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&text[at..at + 2], 16).unwrap())
        .collect()
}

/// The harness's `RtlGenRandom`: draw `i` is `0x20 + i`, except that draws with `i % 7 == 3` are zero.
/// The sequence repeats every 64 draws, as the C's `rng_seq` does.
pub(crate) fn harness_random() -> impl FnMut() -> u8 {
    let mut draw = 0usize;
    move || {
        let i = draw % 64;
        draw += 1;
        if i % 7 == 3 {
            0
        } else {
            0x20 + i as u8
        }
    }
}

/// The 16-byte hash of the MD5 rows: `10 11 .. 1f`.
fn hash_md5() -> Vec<u8> {
    (0x10..0x20).collect()
}

/// The 32-byte hash of the SHA-256 row: `a0 a1 .. bf`.
fn hash_sha256() -> Vec<u8> {
    (0xa0..0xc0).collect()
}

/// The 36-byte SSL 3.0 hash: `40 41 42` followed by zeros.
fn hash_ssl3() -> Vec<u8> {
    let mut hash = vec![0x40, 0x41, 0x42];
    hash.resize(36, 0);
    hash
}

/// A blob with a `BLOBHEADER` and an `RSAPUBKEY` (pubexp 65537), then `body` filler bytes `01, 02, ..`.
pub(crate) fn rsa_blob(b_type: u8, version: u8, magic: u32, bitlen: u32, body: usize) -> Vec<u8> {
    let mut blob = header(b_type, version, CALG_RSA_KEYX);
    blob.extend(magic.to_le_bytes());
    blob.extend(bitlen.to_le_bytes());
    blob.extend(65537u32.to_le_bytes());
    blob.extend((0..body).map(|i| (i + 1) as u8));
    blob
}

/// A plaintext blob: a `BLOBHEADER`, the key length and the key bytes `keys`.
pub(crate) fn plaintext_blob(alg_id: u32, key_len: u32, keys: &[u8]) -> Vec<u8> {
    let mut blob = header(PLAINTEXTKEYBLOB, CUR_BLOB_VERSION, alg_id);
    blob.extend(key_len.to_le_bytes());
    blob.extend_from_slice(keys);
    blob
}

fn header(b_type: u8, version: u8, alg_id: u32) -> Vec<u8> {
    let mut blob = vec![b_type, version, 0, 0];
    blob.extend(alg_id.to_le_bytes());
    blob
}

#[test]
fn oracle_o01() {
    let mut out = [0u8; 32];
    pad_data_pkcs1(b"hello", &mut out, 0, harness_random()).unwrap();
    assert_eq!(
        out.to_vec(),
        hex("00022021222425262728292b2c2d2e2f30323334353637393a3b0068656c6c6f")
    );
}

#[test]
fn oracle_o02() {
    let mut out = [0u8; 32];
    pad_data_pkcs1(b"hello", &mut out, CRYPT_SSL2_FALLBACK, harness_random()).unwrap();
    assert_eq!(
        out.to_vec(),
        hex("00022021222425262728292b2c2d2e2f303203030303030303030068656c6c6f")
    );
}

#[test]
fn oracle_o03() {
    let mut out = [0u8; 32];
    let err = pad_data_pkcs1(&[0x41; 22], &mut out, 0, harness_random()).unwrap_err();
    assert_eq!(err, Nte::BadLen);
    assert_eq!(err.code(), 0x8009_0004);
}

#[test]
fn oracle_o04() {
    let block = [0x00, 0x02, 0x05, 0x00, b'x', b'y'];
    assert_eq!(unpad_data_pkcs1(&block, 6), Ok(&b"xy"[..]));
}

#[test]
fn oracle_o05() {
    let block = [0x00, 0x02, 0x01, 0x01, 0x01];
    assert_eq!(unpad_data_pkcs1(&block, 5), Err(Nte::BadData));
}

#[test]
fn oracle_o06() {
    let block = [0x00, 0x01, 0x05, 0x00, 0x09];
    assert_eq!(unpad_data_pkcs1(&block, 5), Err(Nte::BadData));
}

#[test]
fn oracle_o07() {
    let block = [0x00, 0x02, 0x05, 0x00, b'x', b'y'];
    assert_eq!(unpad_data_pkcs1(&block, 1), Err(Nte::BadData));
}

#[test]
fn oracle_o08() {
    let block = [0x00, 0x02, 0x00];
    assert_eq!(unpad_data_pkcs1(&block, 6), Ok(&[][..]));
}

#[test]
fn oracle_o09() {
    let mut sig = [0u8; 64];
    build_hash_signature(&mut sig, CALG_MD5, &hash_md5(), 0).unwrap();
    assert_eq!(sig.to_vec(), hex("0001ffffffffffffffffffffffffffffffffffffffffffffffffffffff003020300c06082a864886f70d020505000410101112131415161718191a1b1c1d1e1f"));
}

#[test]
fn oracle_o10() {
    let mut sig = [0u8; 128];
    build_hash_signature(&mut sig, CALG_SHA_256, &hash_sha256(), CRYPT_NOHASHOID).unwrap();
    assert_eq!(sig.to_vec(), hex("0001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00a0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf"));
}

#[test]
fn oracle_o11() {
    let mut sig = [0u8; 64];
    build_hash_signature(&mut sig, CALG_MD5, &hash_md5(), CRYPT_X931_FORMAT).unwrap();
    assert_eq!(sig.to_vec(), hex("6bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba101112131415161718191a1b1c1d1e1f33cc"));
}

#[test]
fn oracle_o12() {
    let mut sig = [0u8; 64];
    let err = build_hash_signature(&mut sig, 0x8005, &hash_md5(), 0).unwrap_err();
    assert_eq!(err, Nte::BadAlgId);
    assert_eq!(err.code(), 0x8009_0008);
}

#[test]
fn oracle_o13() {
    let mut sig = [0u8; 64];
    build_hash_signature(&mut sig, CALG_SSL3_SHAMD5, &hash_ssl3(), 0).unwrap();
    assert_eq!(sig.to_vec(), hex("0001ffffffffffffffffffffffffffffffffffffffffffffffffff00404142000000000000000000000000000000000000000000000000000000000000000000"));
}

#[test]
fn oracle_o14() {
    assert_eq!(copy_param(None, &[0u8; 20]), Ok(20));
}

#[test]
fn oracle_o15() {
    assert_eq!(copy_param(Some(&mut [0u8; 4]), &[0u8; 8]), Err(8));
}

#[test]
fn oracle_o16() {
    assert_eq!(classify_blob(&[0u8; 7]), Err(Nte::BadData));
}

#[test]
fn oracle_o17() {
    let blob = rsa_blob(PRIVATEKEYBLOB, 1, RSAENH_MAGIC_RSA2, 512, 224);
    assert_eq!(classify_blob(&blob), Err(Nte::BadData));
}

#[test]
fn oracle_o18() {
    let blob = rsa_blob(9, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA2, 512, 224);
    let err = classify_blob(&blob).unwrap_err();
    assert_eq!(err, Nte::BadType);
    assert_eq!(err.code(), 0x8009_000a);
}

#[test]
fn oracle_o19() {
    let blob = rsa_blob(
        PRIVATEKEYBLOB,
        CUR_BLOB_VERSION,
        RSAENH_MAGIC_RSA2,
        512,
        224,
    );
    assert_eq!(classify_blob(&blob), Ok(BlobKind::PrivateKey));
    assert_eq!(
        check_private_key_blob(&blob, 0),
        Ok(RsaPubKey {
            magic: RSAENH_MAGIC_RSA2,
            bitlen: 512,
            pubexp: 65537,
        })
    );
}

#[test]
fn oracle_o20() {
    let blob = rsa_blob(
        PRIVATEKEYBLOB,
        CUR_BLOB_VERSION,
        RSAENH_MAGIC_RSA2,
        512,
        223,
    );
    assert_eq!(check_private_key_blob(&blob, 0), Err(Nte::BadData));
}

#[test]
fn oracle_o21() {
    let blob = rsa_blob(
        PRIVATEKEYBLOB,
        CUR_BLOB_VERSION,
        RSAENH_MAGIC_RSA2,
        512,
        224,
    );
    let err = check_private_key_blob(&blob, CRYPT_IPSEC_HMAC_KEY).unwrap_err();
    assert_eq!(err, Nte::BadFlags);
    assert_eq!(err.code(), 0x8009_0009);
}

#[test]
fn oracle_o22() {
    let blob = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA1, 512, 64);
    assert_eq!(classify_blob(&blob), Ok(BlobKind::PublicKey));
    assert_eq!(
        check_public_key_blob(&blob, 0),
        Ok(RsaPubKey {
            magic: RSAENH_MAGIC_RSA1,
            bitlen: 512,
            pubexp: 65537,
        })
    );
}

#[test]
fn oracle_o23() {
    let mut blob = rsa_blob(PUBLICKEYBLOB, CUR_BLOB_VERSION, RSAENH_MAGIC_RSA1, 512, 64);
    blob.pop();
    assert_eq!(check_public_key_blob(&blob, 0), Err(Nte::BadData));
}

#[test]
fn oracle_o24() {
    let blob = plaintext_blob(CALG_RC2, 5, &[1, 2, 3, 4, 5]);
    assert_eq!(classify_blob(&blob), Ok(BlobKind::PlaintextKey));
    assert_eq!(plaintext_key(&blob), Ok(&[1, 2, 3, 4, 5][..]));
}

#[test]
fn oracle_o25() {
    let blob = plaintext_blob(CALG_RC2, 5, &[1, 2, 3]);
    assert_eq!(plaintext_key(&blob), Err(Nte::BadData));
}
