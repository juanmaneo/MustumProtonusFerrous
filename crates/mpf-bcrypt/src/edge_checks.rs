//! Edge cases for the bcrypt port, checked against the output of Wine's
//! `bcrypt_main.c`, compiled verbatim, plus Rust-only checks for behaviour that
//! the oracle cannot show.
//!
//! The expected lines are copied from the C oracle's run in
//! `target/canon/bcrypt/edge.txt`, which is not committed. Do not edit them by
//! hand.

use crate::algorithm::{AlgId, ChainMode, CurveId, Interface};
use crate::blob::{BlobCurve, BlobType};
use crate::key::KeyLengths;
use crate::oracle_checks::{
    curve, derive, join, mk, open_piece, query, query_into, query_null, set, validate, w, Row,
    DERIVED_KEY, PREFILL,
};
use crate::property::get_alg_property;
use crate::wide::wcs_eq;
use crate::Status;

fn e01() -> String {
    open_piece(Some("AES\0junk"), None, 0)
}

fn e02() -> String {
    let mut aes = mk(AlgId::Aes);
    let mut big = [PREFILL; 64];
    join(vec![
        set(&mut aes, "ChainingMode", "ChainingModeECB\0junk"),
        query_into(&aes, "ChainingMode", 64, &mut big),
    ])
}

fn e03() -> String {
    let sha = mk(AlgId::Sha256);
    join(vec![
        query(&sha, "AlgorithmName", 14),
        query(&sha, "AlgorithmName", 13),
    ])
}

fn e04() -> String {
    let sha = mk(AlgId::Sha256);
    join(vec![
        query(&sha, "ObjectLength", 3),
        query(&sha, "ObjectLength", 4),
    ])
}

fn e05() -> String {
    join(vec![validate(128, 256, 0, 40), validate(128, 256, 0, 17)])
}

fn e06() -> String {
    derive(&DERIVED_KEY, Some(0))
}

fn e07() -> String {
    let sha = mk(AlgId::Sha256);
    let des = mk(AlgId::Des);
    join(vec![
        query_null(&sha, "ObjectLength", 2),
        query_null(&des, "KeyLengths", 11),
    ])
}

fn e08() -> String {
    let aes = mk(AlgId::Aes);
    join(vec![
        query_null(&aes, "AlgorithmName", 8),
        query_null(&aes, "AlgorithmName", 7),
    ])
}

fn e09() -> String {
    join(
        [
            AlgId::Des,
            AlgId::ChaCha20Poly1305,
            AlgId::Aes,
            AlgId::Rc4,
            AlgId::Rsa,
            AlgId::Dsa,
            AlgId::Pbkdf2,
        ]
        .into_iter()
        .map(|id| query(&mk(id), "Bogus", 40))
        .collect(),
    )
}

fn e10() -> String {
    let gmac = mk(AlgId::AesGmac);
    join(vec![
        query(&gmac, "BlockLength", 4),
        query(&gmac, "KeyLengths", 12),
    ])
}

fn e11() -> String {
    let mut chacha = mk(AlgId::ChaCha20Poly1305);
    let mut rc4 = mk(AlgId::Rc4);
    let mut aes = mk(AlgId::Aes);
    let mut gmac = mk(AlgId::AesGmac);
    let mut ecdsa = mk(AlgId::Ecdsa);
    join(vec![
        set(&mut chacha, "ChainingMode", "ChainingModeCBC"),
        set(&mut chacha, "BlockLength", "1"),
        set(&mut rc4, "BlockLength", "1"),
        set(&mut aes, "BlockLength", "16"),
        set(&mut gmac, "ChainingMode", "ChainingModeECB"),
        set(&mut ecdsa, "Bogus", "x"),
    ])
}

fn e12() -> String {
    let mut ecdsa = mk(AlgId::Ecdsa);
    let mut ecdh = mk(AlgId::Ecdh);
    join(vec![
        set(&mut ecdsa, "ECCCurveName", "secP256r1"),
        curve(&ecdsa),
        set(&mut ecdsa, "ECCCurveName", "secP521r1"),
        curve(&ecdsa),
        set(&mut ecdh, "ECCCurveName", "brainpoolP384r1"),
        curve(&ecdh),
    ])
}

fn e13() -> String {
    let gmac = mk(AlgId::AesGmac);
    let mut aes = mk(AlgId::Aes);
    let mut big = [PREFILL; 64];
    join(vec![
        query_into(&gmac, "ChainingMode", 64, &mut big),
        set(&mut aes, "ChainingMode", "ChainingModeCFB"),
        query_into(&aes, "ChainingMode", 64, &mut big),
        set(&mut aes, "ChainingMode", "ChainingModeCBC"),
    ])
}

fn e14() -> String {
    let sha = mk(AlgId::Sha256);
    let aes = mk(AlgId::Aes);
    join(vec![
        query_null(&sha, "ObjectLength", 4),
        query_null(&aes, "BlockLength", 4),
    ])
}

const EDGE_ROWS: &[Row] = &[
    ("E01", e01, "status=00000000 id=3 flags=0"),
    ("E02", e02, "status=00000000 ; status=00000000 ret=64 out=43006800610069006e0069006e0067004d006f00640065004500430042000000aaaaaaaaaaaaaaaa"),
    ("E03", e03, "status=00000000 ret=14 out=5300480041003200350036000000 ; status=c0000023 ret=14 out=aaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
    ("E04", e04, "status=c0000023 ret=4 out=aaaaaaaa ; status=00000000 ret=4 out=1e010000"),
    ("E05", e05, "status=00000000 ret=32 ; status=00000000 ret=17"),
    ("E06", e06, "status=00000000 ret=0 out="),
    ("E07", e07, "status=c0000023 ret=4 ; status=00000000 ret=12"),
    ("E08", e08, "status=00000000 ret=8 ; status=c0000023 ret=8"),
    ("E09", e09, "status=c0000002 ret=0 out= ; status=c0000002 ret=0 out= ; status=c0000002 ret=0 out= ; status=c0000002 ret=0 out= ; status=c0000002 ret=0 out= ; status=c0000002 ret=0 out= ; status=c0000002 ret=0 out="),
    ("E10", e10, "status=00000000 ret=4 out=10000000 ; status=00000000 ret=12 out=800000000001000040000000"),
    ("E11", e11, "status=c0000002 ; status=c0000002 ; status=c0000002 ; status=c0000002 ; status=c0000002 ; status=c0000002"),
    ("E12", e12, "status=00000000 ; curve=4 ; status=00000000 ; curve=6 ; status=00000000 ; curve=3"),
    ("E13", e13, "status=00000000 ret=64 out=43006800610069006e0069006e0067004d006f0064006500470043004d000000aaaaaaaaaaaaaaaa ; status=00000000 ; status=00000000 ret=64 out=43006800610069006e0069006e0067004d006f00640065004300460042000000aaaaaaaaaaaaaaaa ; status=00000000"),
    ("E14", e14, "status=00000000 ret=4 ; status=00000000 ret=4"),
];

#[test]
fn edge_rows_match_the_c_driver() {
    for (label, row, expected) in EDGE_ROWS {
        assert_eq!(row(), *expected, "{label}");
    }
}

#[test]
fn null_chaining_mode_reports_the_size_without_a_buffer() {
    let aes = mk(AlgId::Aes);
    let prop = w("ChainingMode");
    assert_eq!(
        get_alg_property(&aes, &prop, None, 64),
        (Status::Success, 64)
    );
    assert_eq!(
        get_alg_property(&aes, &prop, None, 63),
        (Status::BufferTooSmall, 64)
    );
}

#[test]
fn wcs_eq_stops_at_the_first_nul() {
    assert!(wcs_eq(&w("AES"), "AES"));
    assert!(wcs_eq(&w("AES\0junk"), "AES"));
    assert!(!wcs_eq(&w("AE"), "AES"));
    assert!(!wcs_eq(&w("AESX"), "AES"));
}

fn cloned<T: Clone>(value: &T) -> T {
    value.clone()
}

#[test]
fn derived_impls_are_exercised() {
    for id in AlgId::ALL {
        assert_eq!(cloned(&id), id);
        assert!(!format!("{id:?}").is_empty());
    }
    for interface in [
        Interface::Cipher,
        Interface::Hash,
        Interface::AsymmetricEncryption,
        Interface::SecretAgreement,
        Interface::Signature,
        Interface::Rng,
        Interface::KeyDerivation,
    ] {
        assert_eq!(cloned(&interface), interface);
        assert!(!format!("{interface:?}").is_empty());
    }
    for mode in [
        ChainMode::Cbc,
        ChainMode::Ecb,
        ChainMode::Cfb,
        ChainMode::Gcm,
    ] {
        assert_eq!(cloned(&mode), mode);
        assert!(!format!("{mode:?}").is_empty());
    }
    for curve_id in [
        CurveId::Invalid,
        CurveId::NoCurve,
        CurveId::Curve25519,
        CurveId::BrainpoolP256r1,
        CurveId::BrainpoolP384r1,
        CurveId::P256r1,
        CurveId::P384r1,
        CurveId::P521r1,
    ] {
        assert_eq!(cloned(&curve_id), curve_id);
        assert!(!format!("{curve_id:?}").is_empty());
    }
    for status in [
        Status::Success,
        Status::NotImplemented,
        Status::InvalidParameter,
        Status::BufferTooSmall,
        Status::NotSupported,
    ] {
        assert_eq!(cloned(&status), status);
        assert!(!format!("{status:?}").is_empty());
    }
    let alg = mk(AlgId::Aes);
    let alg_copy = cloned(&alg);
    assert_eq!(alg_copy.id, AlgId::Aes);
    assert!(!format!("{alg_copy:?}").is_empty());
    let lengths = KeyLengths {
        min: 128,
        max: 256,
        increment: 64,
    };
    assert!(!format!("{:?}", cloned(&lengths)).is_empty());
    let blob_curve = BlobCurve {
        curve: CurveId::P256r1,
        private: true,
    };
    assert!(!format!("{:?}", cloned(&blob_curve)).is_empty());
    let given = w("ECCPRIVATEBLOB");
    for blob_type in [BlobType::Given(&given), BlobType::Named("ECCPUBLICBLOB")] {
        assert!(!format!("{:?}", cloned(&blob_type)).is_empty());
    }
}
