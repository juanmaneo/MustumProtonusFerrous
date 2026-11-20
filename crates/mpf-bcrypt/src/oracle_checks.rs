//! Oracle rows for the bcrypt port, checked against the output of Wine's
//! `bcrypt_main.c`, compiled verbatim.
//!
//! The expected lines are copied from the C oracle's run in
//! `target/canon/bcrypt/ok.txt`, which is not committed. Do not edit them by
//! hand. Each row function makes the same calls, with the same buffers, as the
//! matching `rNN` function in `oracle_main.c`, and renders the result in the same
//! text form.

use crate::algorithm::{enum_algorithms, open_algorithm, AlgId, Algorithm};
use crate::blob::{ecc_blob_curve, ecc_magic, resolve_blob_type, BlobCurve, BlobType};
use crate::key::{derive_key_raw, validate_len_aes, KeyLengths};
use crate::property::{get_alg_property, set_alg_property};
use crate::{Reply, Status};

const OUT_MAX: usize = 40;
pub(crate) const PREFILL: u8 = 0xAA;
pub(crate) const DERIVED_KEY: [u8; 5] = [1, 2, 3, 4, 5];
pub(crate) type Row = (&'static str, fn() -> String, &'static str);

const MS_PRIMITIVE_PROVIDER: &str = "Microsoft Primitive Provider";
const CIPHER_OPERATION: u32 = 0x01;
const HASH_OPERATION: u32 = 0x02;
const KEY_DERIVATION_OPERATION: u32 = 0x40;
const HMAC_FLAG: u32 = 0x08;
const HASH_REUSABLE_FLAG: u32 = 0x20;

const RSA_PUBLIC: u32 = 0x3141_5352;
const DSA_PUBLIC: u32 = 0x4250_5344;
const ECDH_PUBLIC_P256: u32 = 0x314b_4345;
const ECDH_PRIVATE_P256: u32 = 0x324b_4345;
const ECDH_PUBLIC_GENERIC: u32 = 0x504b_4345;
const ECDSA_PUBLIC_P521: u32 = 0x3553_4345;
const UNKNOWN_MAGIC: u32 = 0x1234_5678;

pub(crate) fn w(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

pub(crate) fn mk(id: AlgId) -> Algorithm {
    Algorithm::new(id, 0)
}

pub(crate) fn join(pieces: Vec<String>) -> String {
    pieces.join(" ; ")
}

fn status_text(status: Status) -> String {
    format!("status={:08x}", status.code())
}

fn reply_text((status, ret): Reply) -> String {
    format!("status={:08x} ret={ret}", status.code())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The reply, then the first `min(ret, 40)` bytes of `buf`.
fn with_out(reply: Reply, buf: &[u8]) -> String {
    let n = (reply.1 as usize).min(OUT_MAX);
    format!("{} out={}", reply_text(reply), hex(&buf[..n]))
}

pub(crate) fn query_into(alg: &Algorithm, prop: &str, size: u32, buf: &mut [u8]) -> String {
    let reply = get_alg_property(alg, &w(prop), Some(&mut *buf), size);
    with_out(reply, buf)
}

pub(crate) fn query(alg: &Algorithm, prop: &str, size: u32) -> String {
    query_into(alg, prop, size, &mut [PREFILL; OUT_MAX])
}

pub(crate) fn query_null(alg: &Algorithm, prop: &str, size: u32) -> String {
    reply_text(get_alg_property(alg, &w(prop), None, size))
}

pub(crate) fn set(alg: &mut Algorithm, prop: &str, value: &str) -> String {
    status_text(set_alg_property(alg, &w(prop), &w(value)))
}

pub(crate) fn curve(alg: &Algorithm) -> String {
    format!("curve={}", alg.curve_id as i32)
}

pub(crate) fn open_piece(name: Option<&str>, implementation: Option<&str>, flags: u32) -> String {
    let name = name.map(w);
    let implementation = implementation.map(w);
    match open_algorithm(name.as_deref(), implementation.as_deref(), flags) {
        Ok(alg) => format!("status=00000000 id={} flags={}", alg.id as u32, alg.flags),
        Err(status) => status_text(status),
    }
}

pub(crate) fn validate(min: u32, max: u32, increment: u32, secret_len: u32) -> String {
    let lengths = KeyLengths {
        min,
        max,
        increment,
    };
    match validate_len_aes(lengths, secret_len) {
        Ok(ret) => reply_text((Status::Success, ret)),
        Err(status) => reply_text((status, 0)),
    }
}

pub(crate) fn derive(key: &[u8], out_len: Option<usize>) -> String {
    match out_len {
        None => reply_text(derive_key_raw(key, None)),
        Some(len) => {
            let mut out = [PREFILL; OUT_MAX];
            let reply = derive_key_raw(key, Some(&mut out[..len]));
            with_out(reply, &out)
        }
    }
}

fn enum_piece(operations: u32) -> String {
    match enum_algorithms(operations) {
        Ok(list) => {
            let count = list.len();
            let entries: String = list
                .into_iter()
                .map(|(name, class)| format!(" {name}:{}", class as u32))
                .collect();
            format!("status=00000000 count={count}{entries}")
        }
        Err(status) => status_text(status),
    }
}

fn ecc_magic_piece(id: AlgId) -> String {
    format!(
        "{}:{:08x}:{:08x}",
        id as u32,
        ecc_magic(id, false),
        ecc_magic(id, true)
    )
}

fn blob_piece(id: AlgId, magic: u32) -> String {
    let BlobCurve { curve, private } = ecc_blob_curve(id, magic);
    format!("curve={} private={}", curve as i32, u32::from(private))
}

fn resolve_piece(blob_type: Option<&str>, magic: u32, len: usize) -> String {
    let blob_type = blob_type.map(w);
    let input = magic.to_le_bytes();
    match resolve_blob_type(blob_type.as_deref(), &input[..len]) {
        None => "null".to_string(),
        Some(BlobType::Named(name)) => name.to_string(),
        Some(BlobType::Given(text)) => String::from_utf16_lossy(text),
    }
}

fn r01() -> String {
    enum_piece(0)
}

fn r02() -> String {
    join(vec![
        enum_piece(CIPHER_OPERATION),
        enum_piece(CIPHER_OPERATION | HASH_OPERATION),
        enum_piece(KEY_DERIVATION_OPERATION),
        enum_piece(0x80),
    ])
}

fn r03() -> String {
    join(vec![
        open_piece(Some("AES"), None, 0),
        open_piece(
            Some("SHA256"),
            Some(MS_PRIMITIVE_PROVIDER),
            HASH_REUSABLE_FLAG,
        ),
        open_piece(Some("PBKDF2"), None, HMAC_FLAG),
    ])
}

fn r04() -> String {
    join(vec![
        open_piece(Some("AES"), None, 0x1),
        open_piece(Some("MD9"), None, 0),
        open_piece(None, None, 0),
        open_piece(Some("AES"), Some("Microsoft Platform Crypto Provider"), 0),
    ])
}

fn r05() -> String {
    let sha = mk(AlgId::Sha256);
    let rng = mk(AlgId::Rng);
    join(vec![
        query(&sha, "ObjectLength", 4),
        query(&sha, "HashDigestLength", 4),
        query(&sha, "HashBlockLength", 4),
        query(&rng, "ObjectLength", 4),
        query(&sha, "ObjectLength", 2),
        query(&sha, "Bogus", 4),
    ])
}

fn r06() -> String {
    let aes = mk(AlgId::Aes);
    join(vec![
        query(&aes, "AlgorithmName", 40),
        query(&aes, "AlgorithmName", 7),
    ])
}

fn r07() -> String {
    let des = mk(AlgId::Des);
    join(vec![
        query(&des, "BlockLength", 4),
        query(&des, "KeyLengths", 12),
        query(&des, "KeyLengths", 11),
        query_null(&des, "KeyLengths", 12),
    ])
}

fn r08() -> String {
    let mut tdes = mk(AlgId::TripleDes);
    join(vec![
        query(&tdes, "KeyLengths", 12),
        set(&mut tdes, "ChainingMode", "ChainingModeECB"),
        set(&mut tdes, "ChainingMode", "ChainingModeCFB"),
        set(&mut tdes, "BlockLength", "8"),
    ])
}

fn r09() -> String {
    let mut des = mk(AlgId::Des);
    join(vec![
        set(&mut des, "ChainingMode", "ChainingModeECB"),
        query(&des, "ChainingMode", 40),
        set(&mut des, "ChainingMode", "ChainingModeCBC"),
        query(&des, "ChainingMode", 40),
    ])
}

fn r10() -> String {
    let aes = mk(AlgId::Aes);
    let mut big = [PREFILL; 64];
    join(vec![
        query_into(&aes, "ChainingMode", 64, &mut big),
        query(&aes, "ChainingMode", 40),
    ])
}

fn r11() -> String {
    let mut aes = mk(AlgId::Aes);
    join(vec![
        set(&mut aes, "ChainingMode", "ChainingModeGCM"),
        set(&mut aes, "MessageBlockLength", "1"),
        set(&mut aes, "ChainingMode", "ChainingModeCCM"),
    ])
}

fn r12() -> String {
    let mut gcm = mk(AlgId::Aes);
    let cbc = mk(AlgId::Aes);
    join(vec![
        set(&mut gcm, "ChainingMode", "ChainingModeGCM"),
        query(&gcm, "AuthTagLength", 12),
        query(&cbc, "AuthTagLength", 12),
    ])
}

fn r13() -> String {
    let mut aes = mk(AlgId::Aes);
    let mut big = [PREFILL; 64];
    join(vec![
        query(&aes, "KeyLengths", 12),
        query(&aes, "BlockLength", 4),
        query(&aes, "MessageBlockLength", 4),
        set(&mut aes, "ChainingMode", "ChainingModeECB"),
        query_into(&aes, "ChainingMode", 64, &mut big),
        set(&mut aes, "ChainingMode", "ChainingModeCFB"),
    ])
}

fn r14() -> String {
    let mut rc4 = mk(AlgId::Rc4);
    let mut chacha = mk(AlgId::ChaCha20Poly1305);
    join(vec![
        query(&rc4, "BlockLength", 4),
        set(&mut rc4, "ChainingMode", "ChainingModeN/A"),
        set(&mut rc4, "ChainingMode", "ChainingModeCBC"),
        query(&chacha, "BlockLength", 4),
        query(&chacha, "ObjectLength", 4),
        set(&mut chacha, "ChainingMode", "ChainingModeN/A"),
    ])
}

fn r15() -> String {
    let rsa = mk(AlgId::Rsa);
    let dsa = mk(AlgId::Dsa);
    join(vec![
        query(&rsa, "PaddingSchemes", 4),
        query(&rsa, "ObjectLength", 4),
        query(&dsa, "PaddingSchemes", 4),
        query(&dsa, "BlockLength", 4),
    ])
}

fn r16() -> String {
    let pbkdf2 = mk(AlgId::Pbkdf2);
    let tls11 = mk(AlgId::Tls11Kdf);
    let tls12 = mk(AlgId::Tls12Kdf);
    let hkdf = mk(AlgId::Hkdf);
    join(vec![
        query(&pbkdf2, "KeyLengths", 12),
        query(&tls12, "KeyLengths", 12),
        query(&hkdf, "KeyLengths", 12),
        query(&pbkdf2, "ObjectLength", 4),
        query(&tls11, "ObjectLength", 4),
        query(&pbkdf2, "BlockLength", 4),
    ])
}

fn r17() -> String {
    let mut ecdsa = mk(AlgId::Ecdsa);
    let mut ecdh = mk(AlgId::Ecdh);
    let mut ecdsa2 = mk(AlgId::Ecdsa);
    let mut ecdsa256 = mk(AlgId::EcdsaP256);
    let mut sha = mk(AlgId::Sha256);
    join(vec![
        set(&mut ecdsa, "ECCCurveName", "secP384r1"),
        curve(&ecdsa),
        set(&mut ecdh, "ECCCurveName", "curve25519"),
        curve(&ecdh),
        set(&mut ecdsa2, "ECCCurveName", "curve25519"),
        set(&mut ecdsa2, "ECCCurveName", "brainpoolP256r1"),
        curve(&ecdsa2),
        set(&mut ecdsa256, "ECCCurveName", "secP256r1"),
        set(&mut ecdsa, "ECCCurveName", "bogus"),
        set(&mut sha, "ChainingMode", "ChainingModeCBC"),
        query(&sha, "ChainingMode", 40),
        query(&ecdsa256, "ObjectLength", 4),
    ])
}

fn r18() -> String {
    join(vec![
        validate(128, 256, 64, 16),
        validate(128, 256, 64, 24),
        validate(128, 256, 64, 32),
        validate(128, 256, 64, 20),
        validate(128, 256, 64, 15),
        validate(128, 256, 64, 40),
    ])
}

fn r19() -> String {
    join(vec![validate(130, 256, 8, 16), validate(129, 256, 3, 16)])
}

fn r20() -> String {
    join(vec![
        derive(&DERIVED_KEY, None),
        derive(&DERIVED_KEY, Some(3)),
        derive(&DERIVED_KEY, Some(40)),
    ])
}

fn r21() -> String {
    join(vec![
        ecc_magic_piece(AlgId::Des),
        ecc_magic_piece(AlgId::Ecdh),
        ecc_magic_piece(AlgId::EcdhP256),
        ecc_magic_piece(AlgId::EcdhP384),
        ecc_magic_piece(AlgId::EcdhP521),
        ecc_magic_piece(AlgId::Ecdsa),
        ecc_magic_piece(AlgId::EcdsaP256),
        ecc_magic_piece(AlgId::EcdsaP384),
        ecc_magic_piece(AlgId::EcdsaP521),
    ])
}

fn r22() -> String {
    let mut pieces = vec![blob_piece(AlgId::Des, ECDH_PUBLIC_GENERIC)];
    for id in [
        AlgId::Ecdh,
        AlgId::EcdhP256,
        AlgId::EcdhP384,
        AlgId::EcdhP521,
        AlgId::Ecdsa,
        AlgId::EcdsaP256,
        AlgId::EcdsaP384,
        AlgId::EcdsaP521,
    ] {
        pieces.push(blob_piece(id, ecc_magic(id, false)));
        pieces.push(blob_piece(id, ecc_magic(id, true)));
        pieces.push(blob_piece(id, UNKNOWN_MAGIC));
    }
    join(pieces)
}

fn r23() -> String {
    join(vec![
        resolve_piece(None, RSA_PUBLIC, 4),
        resolve_piece(Some("ECCPRIVATEBLOB"), RSA_PUBLIC, 4),
        resolve_piece(Some("PUBLICBLOB"), ECDH_PUBLIC_P256, 4),
        resolve_piece(Some("PUBLICBLOB"), ECDSA_PUBLIC_P521, 4),
        resolve_piece(Some("PUBLICBLOB"), RSA_PUBLIC, 4),
        resolve_piece(Some("PUBLICBLOB"), DSA_PUBLIC, 4),
        resolve_piece(Some("PUBLICBLOB"), RSA_PUBLIC, 3),
        resolve_piece(Some("PUBLICBLOB"), ECDH_PUBLIC_GENERIC, 4),
        resolve_piece(Some("PUBLICBLOB"), ECDH_PRIVATE_P256, 4),
    ])
}

fn r24() -> String {
    AlgId::ALL
        .into_iter()
        .map(|id| if id.is_symmetric() { "1" } else { "0" })
        .collect::<Vec<_>>()
        .join(" ")
}

const ROWS: &[Row] = &[
    ("R01", r01, "status=00000000 count=30 DES:1 3DES:1 CHACHA20_POLY1305:1 AES:1 AES-GMAC:1 RC4:1 SHA256:2 SHA384:2 SHA512:2 SHA1:2 MD5:2 MD4:2 MD2:2 RSA:3 DH:4 ECDH:4 ECDH_P256:4 ECDH_P384:4 ECDH_P521:4 RSA_SIGN:5 ECDSA:5 ECDSA_P256:5 ECDSA_P384:5 ECDSA_P521:5 DSA:5 RNG:6 PBKDF2:7 TLS1_1_KDF:7 TLS1_2_KDF:7 HKDF:7"),
    ("R02", r02, "status=00000000 count=6 DES:1 3DES:1 CHACHA20_POLY1305:1 AES:1 AES-GMAC:1 RC4:1 ; status=00000000 count=13 DES:1 3DES:1 CHACHA20_POLY1305:1 AES:1 AES-GMAC:1 RC4:1 SHA256:2 SHA384:2 SHA512:2 SHA1:2 MD5:2 MD4:2 MD2:2 ; status=00000000 count=4 PBKDF2:7 TLS1_1_KDF:7 TLS1_2_KDF:7 HKDF:7 ; status=c000000d"),
    ("R03", r03, "status=00000000 id=3 flags=0 ; status=00000000 id=6 flags=32 ; status=00000000 id=26 flags=8"),
    ("R04", r04, "status=c0000002 ; status=c0000002 ; status=c000000d ; status=c0000002"),
    ("R05", r05, "status=00000000 ret=4 out=1e010000 ; status=00000000 ret=4 out=20000000 ; status=00000000 ret=4 out=40000000 ; status=c00000bb ret=0 out= ; status=c0000023 ret=4 out=aaaaaaaa ; status=c0000002 ret=0 out="),
    ("R06", r06, "status=00000000 ret=8 out=4100450053000000 ; status=c0000023 ret=8 out=aaaaaaaaaaaaaaaa"),
    ("R07", r07, "status=00000000 ret=4 out=08000000 ; status=00000000 ret=12 out=400000004000000000000000 ; status=c0000023 ret=12 out=aaaaaaaaaaaaaaaaaaaaaaaa ; status=00000000 ret=12"),
    ("R08", r08, "status=00000000 ret=12 out=c0000000c000000000000000 ; status=00000000 ; status=c00000bb ; status=c0000002"),
    ("R09", r09, "status=00000000 ; status=c0000002 ret=0 out= ; status=00000000 ; status=c0000023 ret=64 out=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
    ("R10", r10, "status=00000000 ret=64 out=43006800610069006e0069006e0067004d006f00640065004300420043000000aaaaaaaaaaaaaaaa ; status=c0000023 ret=64 out=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
    ("R11", r11, "status=00000000 ; status=c000000d ; status=c0000002"),
    ("R12", r12, "status=00000000 ; status=00000000 ret=12 out=0c0000001000000001000000 ; status=c00000bb ret=0 out="),
    ("R13", r13, "status=00000000 ret=12 out=800000000001000040000000 ; status=00000000 ret=4 out=10000000 ; status=00000000 ret=4 out=01000000 ; status=00000000 ; status=00000000 ret=64 out=43006800610069006e0069006e0067004d006f00640065004500430042000000aaaaaaaaaaaaaaaa ; status=00000000"),
    ("R14", r14, "status=00000000 ret=4 out=01000000 ; status=00000000 ; status=c0000002 ; status=00000000 ret=4 out=01000000 ; status=00000000 ret=4 out=a6000000 ; status=00000000"),
    ("R15", r15, "status=00000000 ret=4 out=0c000000 ; status=c00000bb ret=0 out= ; status=c00000bb ret=0 out= ; status=c0000002 ret=0 out="),
    ("R16", r16, "status=00000000 ret=12 out=000000000040000008000000 ; status=00000000 ret=12 out=000000000040000008000000 ; status=00000000 ret=12 out=000000000040000008000000 ; status=00000000 ret=4 out=6a020000 ; status=00000000 ret=4 out=be020000 ; status=c00000bb ret=0 out="),
    ("R17", r17, "status=00000000 ; curve=5 ; status=00000000 ; curve=1 ; status=c00000bb ; status=00000000 ; curve=2 ; status=c0000002 ; status=c0000002 ; status=c0000002 ; status=c0000002 ret=0 out= ; status=c00000bb ret=0 out="),
    ("R18", r18, "status=00000000 ret=16 ; status=00000000 ret=24 ; status=00000000 ret=32 ; status=c000000d ret=0 ; status=c000000d ret=0 ; status=00000000 ret=32"),
    ("R19", r19, "status=c000000d ret=0 ; status=00000000 ret=16"),
    ("R20", r20, "status=00000000 ret=5 ; status=00000000 ret=3 out=030201 ; status=00000000 ret=5 out=0504030201"),
    ("R21", r21, "0:00000000:00000000 ; 15:504b4345:564b4345 ; 16:314b4345:324b4345 ; 17:334b4345:344b4345 ; 18:354b4345:364b4345 ; 20:50444345:56444345 ; 21:31534345:32534345 ; 22:33534345:34534345 ; 23:35534345:36534345"),
    ("R22", r22, "curve=-1 private=0 ; curve=0 private=0 ; curve=0 private=1 ; curve=0 private=0 ; curve=4 private=0 ; curve=4 private=1 ; curve=-1 private=0 ; curve=5 private=0 ; curve=5 private=1 ; curve=-1 private=0 ; curve=6 private=0 ; curve=6 private=1 ; curve=-1 private=0 ; curve=0 private=0 ; curve=0 private=1 ; curve=0 private=0 ; curve=4 private=0 ; curve=4 private=1 ; curve=-1 private=0 ; curve=5 private=0 ; curve=5 private=1 ; curve=-1 private=0 ; curve=6 private=0 ; curve=6 private=1 ; curve=-1 private=0"),
    ("R23", r23, "null ; ECCPRIVATEBLOB ; ECCPUBLICBLOB ; ECCPUBLICBLOB ; RSAPUBLICBLOB ; DSAPUBLICBLOB ; null ; null ; null"),
    ("R24", r24, "1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 1"),
];

#[test]
fn oracle_rows_match_the_c_driver() {
    for (label, row, expected) in ROWS {
        assert_eq!(row(), *expected, "{label}");
    }
}
