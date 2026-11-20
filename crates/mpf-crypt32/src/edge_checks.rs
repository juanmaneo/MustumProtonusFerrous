//! Edge cases for flags, capacities, separators and PEM framing. Expectations marked "oracle"
//! were checked against Wine's `base64.c` with `target/canon/crypt32`. The rest follow the
//! port's documented deviations in `lib.rs`.

use super::*;
use crate::decode::Decoded;

fn from_hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex"))
        .collect()
}

fn sample(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i * 37 + 11) as u8).collect()
}

fn encode_text(binary: &[u8], flags: u32) -> Vec<u8> {
    let capacity = encode::required_size(binary, flags).expect("size");
    encode::binary_to_string(binary, flags, capacity).expect("encode")
}

fn decode_text(text: &[u8], flags: u32) -> Decoded {
    let capacity = decode::required_size(text, flags).expect("size");
    decode::string_to_binary(text, flags, capacity).expect("decode")
}

fn decoded(bytes: &[u8], skip: usize, format: u32) -> Decoded {
    Decoded {
        bytes: bytes.to_vec(),
        skip,
        format,
    }
}

fn decode_bytes(text: &[u8], flags: u32) -> Result<Vec<u8>, Error> {
    decode::string_to_binary(text, flags, usize::MAX).map(|decoded| decoded.bytes)
}

#[test]
fn error_codes_match_win32() {
    let codes = [
        Error::InvalidParameter,
        Error::InvalidData,
        Error::InsufficientBuffer,
        Error::MoreData,
    ]
    .map(Error::code);
    assert_eq!(codes, [87, 13, 122, 234]);
}

#[test]
fn errors_and_decoded_are_clone_eq_debug() {
    let errors = [
        Error::InvalidParameter,
        Error::InvalidData,
        Error::InsufficientBuffer,
        Error::MoreData,
    ];
    let names = [
        "InvalidParameter",
        "InvalidData",
        "InsufficientBuffer",
        "MoreData",
    ];
    for (error, name) in errors.into_iter().zip(names) {
        assert_eq!(Clone::clone(&error), error);
        assert_eq!(format!("{error:?}"), name);
    }
    let value = decoded(&[0x41], 3, 2);
    assert_eq!(value.clone(), value);
    assert!(format!("{value:?}").contains("skip: 3"));
}

#[test]
fn unknown_encode_types_are_invalid_parameter() {
    for flags in [
        0x4,
        0x5,
        0x6,
        0x7,
        0x8,
        0xa,
        0xb,
        0xd,
        0xe,
        0xf,
        0x10,
        0x1000_0004,
    ] {
        assert_eq!(
            encode::binary_to_string(b"ABC", flags, 100),
            Err(Error::InvalidParameter),
            "flags {flags:#x}"
        );
        assert_eq!(
            encode::required_size(b"ABC", flags),
            Err(Error::InvalidParameter),
            "flags {flags:#x}"
        );
    }
}

#[test]
fn unknown_decode_types_are_invalid_parameter() {
    for flags in [0x5, 0x8, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf] {
        assert_eq!(
            decode::string_to_binary(b"41", flags, 10),
            Err(Error::InvalidParameter),
            "flags {flags:#x}"
        );
        assert_eq!(
            decode::required_size(b"41", flags),
            Err(Error::InvalidParameter),
            "flags {flags:#x}"
        );
    }
}

#[test]
fn high_flag_bits_are_invalid_data() {
    for flags in [
        0x10,
        0x1000_0001,
        0x1000_0004,
        0x2000_0001,
        0x4000_0001,
        0xc000_000c,
    ] {
        assert_eq!(
            decode::required_size(b"QUJD", flags),
            Err(Error::InvalidData),
            "flags {flags:#x}"
        );
    }
}

#[test]
fn required_sizes_include_the_terminator() {
    assert_eq!(encode::required_size(b"ABC", CRYPT_STRING_BINARY), Ok(3));
    assert_eq!(encode::required_size(b"ABC", CRYPT_STRING_BASE64), Ok(7));
    assert_eq!(encode::required_size(b"ABC", CRYPT_STRING_HEXRAW), Ok(9));
    assert_eq!(
        encode::required_size(
            b"ABC",
            CRYPT_STRING_HEXRAW | CRYPT_STRING_NOCR | CRYPT_STRING_NOCRLF
        ),
        Ok(8)
    );
    assert_eq!(encode::required_size(b"", CRYPT_STRING_BINARY), Ok(0));
    assert_eq!(
        encode::required_size(b"", CRYPT_STRING_BASE64HEADER),
        Ok(57)
    );
}

#[test]
fn encode_capacity_errors_depend_on_the_type() {
    assert_eq!(
        encode::binary_to_string(b"ABC", CRYPT_STRING_BINARY, 3),
        Ok(b"ABC".to_vec())
    );
    assert_eq!(
        encode::binary_to_string(b"ABC", CRYPT_STRING_BINARY, 2),
        Err(Error::InsufficientBuffer)
    );
    assert_eq!(
        encode::binary_to_string(b"AB", CRYPT_STRING_BASE64, 6),
        Err(Error::MoreData)
    );
    assert_eq!(
        encode::binary_to_string(b"AB", CRYPT_STRING_BASE64, 7),
        Ok(b"QUI=\r\n".to_vec())
    );
}

#[test]
fn line_endings_match_wine() {
    assert_eq!(encode_text(b"AB", 0x4000_0001), b"QUI=");
    assert_eq!(encode_text(b"AB", 0x1), b"QUI=\r\n");
    assert_eq!(encode_text(b"ABC", 0x1), b"QUJD\r\n");
    assert_eq!(encode_text(b"ABC", 0x8000_0001), b"QUJD\n");
    assert_eq!(encode_text(b"ABC", 0x4000_0001), b"QUJD");
    assert_eq!(encode_text(b"ABC", 0xc000_0001), b"QUJD\n");
    assert_eq!(encode_text(&[0xfb, 0xff, 0xbf], 0x1), b"+/+/\r\n");
    assert_eq!(encode_text(&[0x00, 0xff], 0x1), b"AP8=\r\n");
    assert_eq!(
        encode_text(b"", 0x0),
        b"-----BEGIN CERTIFICATE-----\r\n-----END CERTIFICATE-----\r\n"
    );
    assert_eq!(
        encode_text(b"", 0x3),
        b"-----BEGIN NEW CERTIFICATE REQUEST-----\r\n-----END NEW CERTIFICATE REQUEST-----\r\n"
    );
    assert_eq!(
        encode_text(b"ABC", 0x9),
        b"-----BEGIN X509 CRL-----\r\nQUJD\r\n-----END X509 CRL-----\r\n"
    );
    assert_eq!(
        encode_text(b"ABC", 0x4000_0000),
        b"-----BEGIN CERTIFICATE-----QUJD-----END CERTIFICATE-----"
    );
    assert_eq!(encode_text(b"ABC", 0xc), b"414243\r\n");
    assert_eq!(encode_text(b"ABC", 0x8000_000c), b"414243\n");
    assert_eq!(encode_text(b"ABC", 0x4000_000c), b"414243");
    assert_eq!(encode_text(b"", 0xc), b"\r\n");
    assert_eq!(encode_text(b"", 0x4000_000c), b"");
}

#[test]
fn hexraw_with_both_flags_sizes_the_newline_it_writes() {
    assert_eq!(
        encode::binary_to_string(b"ABC", 0xc000_000c, 8),
        Ok(b"414243\n".to_vec())
    );
}

#[test]
fn full_alphabet_round_trips() {
    let bytes = from_hex(
        "00108310518720928b30d38f41149351559761969b71d79f8218a39259a7a29aabb2dbafc31cb3d35db7e39ebbf3dfbf",
    );
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    assert_eq!(bytes.len(), 48);
    assert_eq!(
        encode_text(&bytes, CRYPT_STRING_BASE64),
        format!("{alphabet}\r\n").into_bytes()
    );
    assert_eq!(
        decode_text(alphabet.as_bytes(), CRYPT_STRING_BASE64),
        decoded(&bytes, 0, CRYPT_STRING_BASE64)
    );
}

#[test]
fn encodings_round_trip() {
    for len in (0..=20usize).chain([200]) {
        let binary = sample(len);
        for flags in [
            CRYPT_STRING_BASE64,
            CRYPT_STRING_BASE64 | CRYPT_STRING_NOCRLF,
        ] {
            let text = encode_text(&binary, flags);
            assert_eq!(
                decode_text(&text, CRYPT_STRING_BASE64),
                decoded(&binary, 0, CRYPT_STRING_BASE64),
                "len {len} flags {flags:#x}"
            );
        }
    }
    // An empty body is an underflow in Wine (ERROR_INVALID_DATA), so headers start at 1 byte.
    for len in 1..=20usize {
        let binary = sample(len);
        for flags in [
            CRYPT_STRING_BASE64HEADER,
            CRYPT_STRING_BASE64REQUESTHEADER,
            CRYPT_STRING_BASE64X509CRLHEADER,
        ] {
            let text = encode_text(&binary, flags);
            assert_eq!(
                decode_text(&text, flags),
                decoded(&binary, 0, flags),
                "len {len} flags {flags:#x}"
            );
        }
    }
}

#[test]
fn base64_decode_follows_wine_padding_rules() {
    let cases: &[(&str, Result<&[u8], Error>)] = &[
        ("QUI=", Ok(b"AB")),
        ("Q", Ok(b"\x40")),
        ("QQ==!!!", Ok(b"A")),
        ("QQ==A", Ok(b"A")),
        ("QQ=A", Err(Error::InvalidData)),
        ("QU=", Err(Error::InvalidData)),
        ("Q=", Err(Error::InvalidData)),
        ("QU==", Ok(b"A")),
        ("QUJD=!", Ok(b"ABC")),
        ("QUJ=", Ok(b"AB")),
        ("=", Ok(b"")),
        (" \t\r\n", Ok(b"")),
        ("QU!", Err(Error::InvalidData)),
        ("+/+/", Ok(b"\xfb\xff\xbf")),
        ("YWJj", Ok(b"abc")),
        ("MA==", Ok(b"0")),
        ("QU JD\t\r\n", Ok(b"ABC")),
        ("QUJD ", Ok(b"ABC")),
    ];
    for &(text, expected) in cases {
        assert_eq!(
            decode_bytes(text.as_bytes(), CRYPT_STRING_BASE64),
            expected.map(<[u8]>::to_vec),
            "input {text:?}"
        );
    }
}

#[test]
fn base64_capacity_errors() {
    assert_eq!(
        decode::string_to_binary(b"QUJD", CRYPT_STRING_BASE64, 2),
        Err(Error::InsufficientBuffer)
    );
    // Wine writes past a zero-length buffer here (undefined behaviour); the port reports it.
    assert_eq!(
        decode::string_to_binary(b"QUJD", CRYPT_STRING_BASE64, 0),
        Err(Error::InsufficientBuffer)
    );
    assert_eq!(
        decode::string_to_binary(b"", CRYPT_STRING_BASE64, 0),
        Ok(decoded(b"", 0, CRYPT_STRING_BASE64))
    );
}

#[test]
fn hex_text_accepts_pairs_and_separators() {
    let cases: &[(&str, &[u8])] = &[
        ("AB", &[0xab]),
        ("ab", &[0xab]),
        ("41-42", b"AB"),
        ("-,-41,,-42-", b"AB"),
        (",4142", b"AB"),
        ("  \t\r\n", b""),
        ("", b""),
    ];
    for &(text, expected) in cases {
        assert_eq!(
            decode_bytes(text.as_bytes(), CRYPT_STRING_HEX),
            Ok(expected.to_vec()),
            "input {text:?}"
        );
    }
}

#[test]
fn hex_text_rejects_bad_digits_and_separators() {
    for text in ["41x", "41\0", "4 1", "4", "41 42", "414", "4G"] {
        assert_eq!(
            decode_bytes(text.as_bytes(), CRYPT_STRING_HEX),
            Err(Error::InvalidData),
            "input {text:?}"
        );
    }
}

#[test]
fn hex_capacity_errors_are_more_data() {
    assert_eq!(
        decode::string_to_binary(b"41", CRYPT_STRING_HEX, 0),
        Err(Error::MoreData)
    );
    assert_eq!(
        decode::string_to_binary(b"41", CRYPT_STRING_HEX, 1),
        Ok(decoded(&[0x41], 0, CRYPT_STRING_HEX))
    );
    assert_eq!(
        decode::string_to_binary(b"414243", CRYPT_STRING_HEX, 2),
        Err(Error::MoreData)
    );
}

#[test]
fn binary_decode_copies_and_checks_capacity() {
    assert_eq!(
        decode::string_to_binary(b"ab", CRYPT_STRING_BINARY, 2),
        Ok(decoded(b"ab", 0, CRYPT_STRING_BINARY))
    );
    assert_eq!(
        decode::string_to_binary(b"ab", CRYPT_STRING_BINARY, 1),
        Err(Error::InsufficientBuffer)
    );
    assert_eq!(
        decode::string_to_binary(b"", CRYPT_STRING_BINARY, 0),
        Ok(decoded(b"", 0, CRYPT_STRING_BINARY))
    );
}

#[test]
fn any_falls_back_from_base64_to_raw_bytes() {
    assert_eq!(
        decode::string_to_binary(b"QUJD", CRYPT_STRING_ANY, 3),
        Ok(decoded(b"ABC", 0, CRYPT_STRING_BASE64))
    );
    // Wine returns InsufficientBuffer for this input (oracle); the port copies the bytes.
    assert_eq!(
        decode::string_to_binary(b"ab!c", CRYPT_STRING_ANY, 4),
        Ok(decoded(b"ab!c", 0, CRYPT_STRING_BINARY))
    );
    assert_eq!(
        decode::string_to_binary(b"ab!c", CRYPT_STRING_ANY, 3),
        Err(Error::InsufficientBuffer)
    );
    assert_eq!(decode::required_size(b"ab!c", CRYPT_STRING_ANY), Ok(4));
}

#[test]
fn base64_any_tries_pem_then_base64() {
    assert_eq!(
        decode::string_to_binary(b"QUJD", CRYPT_STRING_BASE64_ANY, 3),
        Ok(decoded(b"ABC", 0, CRYPT_STRING_BASE64))
    );
    assert_eq!(
        decode_bytes(
            b"-----BEGIN CERTIFICATE-----\nQUJD\n-----END CERTIFICATE-----",
            CRYPT_STRING_BASE64_ANY
        ),
        Ok(b"ABC".to_vec())
    );
    assert_eq!(
        decode::string_to_binary(b"QU!", CRYPT_STRING_BASE64_ANY, 3),
        Err(Error::InvalidData)
    );
}

#[test]
fn pem_headers_match_wine() {
    let certificate = b"-----BEGIN CERTIFICATE-----\nQUJD\n-----END CERTIFICATE-----";
    assert_eq!(
        decode::string_to_binary(certificate, CRYPT_STRING_BASE64HEADER, 3),
        Ok(decoded(b"ABC", 0, CRYPT_STRING_BASE64HEADER))
    );
    assert_eq!(
        decode::string_to_binary(
            b"xx-----BEGIN CERTIFICATE-----\r\nQUJD\r\n-----END CERTIFICATE-----",
            CRYPT_STRING_ANY,
            3
        ),
        Ok(decoded(b"ABC", 2, CRYPT_STRING_BASE64HEADER))
    );
    assert_eq!(
        decode::string_to_binary(
            b"-----BEGIN NEW CERTIFICATE REQUEST-----\nQUJD\n-----END NEW CERTIFICATE REQUEST-----",
            CRYPT_STRING_BASE64REQUESTHEADER,
            3
        ),
        Ok(decoded(b"ABC", 0, CRYPT_STRING_BASE64REQUESTHEADER))
    );
    assert_eq!(
        decode::string_to_binary(
            b"-----BEGIN X509 CRL-----\nQUJD\n-----END X509 CRL-----",
            CRYPT_STRING_BASE64X509CRLHEADER,
            3
        ),
        Ok(decoded(b"ABC", 0, CRYPT_STRING_BASE64X509CRLHEADER))
    );
    assert_eq!(
        decode::string_to_binary(
            b"-----BEGIN NEW CERTIFICATE REQUEST-----\nQUJD\n-----END NEW CERTIFICATE REQUEST-----",
            CRYPT_STRING_BASE64HEADER,
            3
        ),
        Ok(decoded(b"ABC", 0, CRYPT_STRING_BASE64HEADER))
    );
    assert_eq!(
        decode_bytes(
            b"-----BEGIN CERTIFICATE-----\rQUJD\r-----END CERTIFICATE-----",
            0
        ),
        Ok(b"ABC".to_vec())
    );
    assert_eq!(
        decode_bytes(
            b" -----BEGIN CERTIFICATE-----QUJD-----END CERTIFICATE-----",
            0
        ),
        Ok(b"ABC".to_vec())
    );
    assert_eq!(
        decode_bytes(
            b"-----BEGIN CERTIFICATE-----QUJD-----END CERTIFICATE-----\0junk",
            0
        ),
        Ok(b"ABC".to_vec())
    );
    assert_eq!(
        decode_bytes(
            b"\0-----BEGIN CERTIFICATE-----QUJD-----END CERTIFICATE-----",
            0
        ),
        Err(Error::InvalidData)
    );
    assert_eq!(
        decode_bytes(
            b"-----BEGIN CERTIFICATE-----QUJD=-----END CERTIFICATE-----",
            0
        ),
        Ok(b"ABC".to_vec())
    );
    assert_eq!(
        decode_bytes(
            b"-----BEGIN CERTIFICATE-----\nQU!\n-----END CERTIFICATE-----",
            0
        ),
        Err(Error::InvalidData)
    );
    assert_eq!(
        decode_bytes(b"-----BEGIN CERTIFICATE-----\nQUJD\n", 0),
        Err(Error::InvalidData)
    );
    assert_eq!(
        decode_bytes(b"QUJDQUJDQUJDQUJDQUJDQUJD", 0),
        Err(Error::InvalidData)
    );
    assert_eq!(
        decode_bytes(b"-----BEGIN CERTIFICATE", 0),
        Err(Error::InvalidData)
    );
    assert_eq!(decode_bytes(b"QUJD", 0), Err(Error::InvalidData));
    assert_eq!(
        decode_bytes(
            b"-----BEGIN NEW CERTIFICATE REQUEST-----\nQUJD\n-----END NEW CERTIFICATE REQUEST-----",
            0
        ),
        Ok(b"ABC".to_vec())
    );
}

#[test]
fn empty_pem_bodies_follow_wine() {
    let ok_empty = |text: &[u8]| decode::string_to_binary(text, CRYPT_STRING_BASE64HEADER, 0);
    assert_eq!(
        ok_empty(b"-----BEGIN CERTIFICATE-----\n\n-----END CERTIFICATE-----"),
        Ok(decoded(b"", 0, CRYPT_STRING_BASE64HEADER))
    );
    assert_eq!(
        ok_empty(b"-----BEGIN CERTIFICATE-----\r\n\r-----END CERTIFICATE-----"),
        Ok(decoded(b"", 0, CRYPT_STRING_BASE64HEADER))
    );
    assert_eq!(
        ok_empty(b"-----BEGIN CERTIFICATE----------END CERTIFICATE-----"),
        Ok(decoded(b"", 0, CRYPT_STRING_BASE64HEADER))
    );
    // With no body, the line break before -----END is the underflow Wine reports.
    assert_eq!(
        ok_empty(b"-----BEGIN CERTIFICATE-----\r\n-----END CERTIFICATE-----"),
        Err(Error::InvalidData)
    );
}

#[test]
fn decode_sizes_follow_the_type() {
    assert_eq!(decode::required_size(b"41-42", CRYPT_STRING_HEX), Ok(2));
    assert_eq!(decode::required_size(b"ab", CRYPT_STRING_BINARY), Ok(2));
    assert_eq!(
        decode::required_size(b"QUJD", CRYPT_STRING_BASE64_ANY),
        Ok(3)
    );
    let certificate_tail = b"-----BEGIN CERTIFICATE-----QUJD";
    assert_eq!(
        decode::required_size(certificate_tail, CRYPT_STRING_ANY),
        Ok(31)
    );
    assert_eq!(
        decode::required_size(certificate_tail, CRYPT_STRING_BASE64_ANY),
        Err(Error::InvalidData)
    );
}
