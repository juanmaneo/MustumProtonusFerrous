//! Edge cases that the oracle rows do not reach: the buffer checks of each message function, the
//! non-extended and zero-signature formats, the hashing of PADDING and STREAM buffers, and the
//! boundaries of the hash and base64 code. Expected values come from the RFCs, from `hashlib`
//! or from the oracle.

use crate::base64;
use crate::crc32::crc32;
use crate::keys::{self, Mode};
use crate::md4::md4;
use crate::md5::{md5, HmacMd5, Md5};
use crate::message::Buffer;
use crate::rc4::Rc4;
use crate::session::Session;
use crate::{
    Error, NEGOTIATE_ALWAYS_SIGN, NEGOTIATE_EXTENDED_SESSIONSECURITY, NEGOTIATE_KEY_EXCH,
    NEGOTIATE_SEAL, NEGOTIATE_SIGN, SECBUFFER_DATA, SECBUFFER_PADDING, SECBUFFER_STREAM,
    SECBUFFER_TOKEN,
};

const KEY: [u8; 16] = [
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
];
const INVALID_TOKEN: u32 = 0x8009_0308;
const BUFFER_TOO_SMALL: u32 = 0x8009_0321;
const UNSUPPORTED_FUNCTION: u32 = 0x8009_0302;
const MESSAGE_ALTERED: u32 = 0x8009_030f;

fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex byte"))
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn pattern(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i * 7 + 3) as u8).collect()
}

/// A buffer of `len` zero bytes, as `(kind, data)` for `buffers_of`.
fn zeros(kind: u32, len: usize) -> (u32, Vec<u8>) {
    (kind, vec![0; len])
}

fn bytes(kind: u32, text: &str) -> (u32, Vec<u8>) {
    (kind, hex(text))
}

fn buffers_of(parts: &mut [(u32, Vec<u8>)]) -> Vec<Buffer<'_>> {
    parts
        .iter_mut()
        .map(|(kind, data)| Buffer {
            kind: *kind,
            data: data.as_mut_slice(),
        })
        .collect()
}

fn code_of(result: Result<(), Error>) -> Result<(), u32> {
    result.map_err(Error::code)
}

/// The NTLM2 stream of `md5_in_pieces` is one `update` per piece, between the cut points.
fn md5_in_pieces(data: &[u8], cuts: &[usize]) -> [u8; 16] {
    let mut hash = Md5::default();
    let mut start = 0;
    for &cut in cuts {
        hash.update(&data[start..cut]);
        start = cut;
    }
    hash.update(&data[start..]);
    hash.finalize()
}

#[test]
fn error_codes_match_security_status() {
    assert_eq!(Error::InvalidToken.code(), INVALID_TOKEN);
    assert_eq!(Error::BufferTooSmall.code(), BUFFER_TOO_SMALL);
    assert_eq!(Error::UnsupportedFunction.code(), UNSUPPORTED_FUNCTION);
    assert_eq!(Error::MessageAltered.code(), MESSAGE_ALTERED);
}

#[test]
fn base64_decodes_padding_and_rejects_bad_input() {
    let decode = |text: &str| base64::decode(text.as_bytes());
    assert_eq!(decode(""), Some(Vec::new()));
    assert_eq!(decode("QUI="), Some(b"AB".to_vec()));
    assert_eq!(decode("QUJD"), Some(b"ABC".to_vec()));
    assert_eq!(decode("QQ=="), Some(b"A".to_vec()));
    assert_eq!(decode("QQ=A"), Some(b"A".to_vec()));
    assert_eq!(decode("QQ="), None);
    assert_eq!(decode("QUJDR"), None);
    assert_eq!(decode("QUJD!!!!"), None);
    assert_eq!(decode("Q!=A"), None);
    assert_eq!(decode("QU!="), None);
    assert_eq!(decode("=QUJ"), None);
}

#[test]
fn base64_encodes_and_round_trips() {
    assert_eq!(base64::encode(b""), "");
    assert_eq!(base64::encode(b"A"), "QQ==");
    assert_eq!(base64::encode(b"AB"), "QUI=");
    assert_eq!(base64::encode(b"ABC"), "QUJD");
    for len in 0..=40 {
        let data = pattern(len);
        assert_eq!(base64::decode(base64::encode(&data).as_bytes()), Some(data));
    }
}

#[test]
fn rc4_matches_known_keystream() {
    let mut data = b"Plaintext".to_vec();
    Rc4::new(b"Key").process(&mut data);
    assert_eq!(to_hex(&data), "bbf316e8d940af0ad3");
}

#[test]
#[should_panic(expected = "RC4 key must not be empty")]
fn rc4_rejects_empty_key() {
    Rc4::new(&[]);
}

#[test]
fn md4_matches_rfc_1320_vectors() {
    assert_eq!(to_hex(&md4(b"")), "31d6cfe0d16ae931b73c59d7e0c089c0");
    assert_eq!(to_hex(&md4(b"abc")), "a448017aaf21d8525fc10ae87aa6729d");
    assert_eq!(
        to_hex(&md4(&b"1234567890".repeat(8))),
        "e33b4ddc9c38f2199c3e7b164fcc0536"
    );
}

#[test]
fn md5_matches_rfc_1321_vectors() {
    assert_eq!(to_hex(&md5(b"")), "d41d8cd98f00b204e9800998ecf8427e");
    assert_eq!(to_hex(&md5(b"abc")), "900150983cd24fb0d6963f7d28e17f72");
    assert_eq!(
        to_hex(&md5(&b"1234567890".repeat(8))),
        "57edf4a22be3c955ac49da2e2107b67a"
    );
}

/// Lengths on both sides of the 56-byte padding limit and the 64-byte block size.
#[test]
fn md5_padding_boundaries_match_reference() {
    for (len, expected) in [
        (55, "52c0e574e1198de5fe3f8f11440dcb1b"),
        (56, "46c9907fc908ee68b1e7b8e71286a518"),
        (63, "a62f6d59e837867693f042f5b8f5a236"),
        (64, "7160b8fb5e9e4023d549c3971fbaeead"),
        (65, "70bd662e7aefbda85a0f7244167b7897"),
        (200, "4c79b81ac94bad7a875519ce6b964c66"),
    ] {
        assert_eq!(to_hex(&md5(&pattern(len))), expected, "length {len}");
    }
}

#[test]
fn md5_streaming_matches_one_shot() {
    let data = pattern(200);
    let one_shot = md5(&data);
    assert_eq!(
        md5_in_pieces(&data, &[0, 1, 10, 55, 63, 64, 65, 130]),
        one_shot
    );
    assert_eq!(md5_in_pieces(&data, &[10, 20]), one_shot);
    let mut hash = Md5::default();
    for byte in &data {
        hash.update(std::slice::from_ref(byte));
    }
    assert_eq!(hash.finalize(), one_shot);
}

#[test]
fn hmac_md5_matches_rfc_2202_and_key_boundary() {
    let mut hmac = HmacMd5::new(&[0x0b; 16]);
    hmac.update(b"Hi There");
    assert_eq!(to_hex(&hmac.finalize()), "9294727a3638bb1c13f48ef8158bfc9d");

    let key: Vec<u8> = (0..64).map(|i| (i * 3 + 1) as u8).collect();
    let mut hmac = HmacMd5::new(&key);
    hmac.update(b"Hi");
    hmac.update(b" Ther");
    assert_eq!(to_hex(&hmac.finalize()), "755649bb6d0263f5af3d098b0daa9b43");
}

#[test]
fn crc32_matches_check_value_and_chains() {
    assert_eq!(crc32(0, b"123456789"), 0xcbf4_3926);
    assert_eq!(crc32(crc32(0, b"12"), b"3456789"), 0xcbf4_3926);
    assert_eq!(crc32(0, b""), 0);
}

#[test]
fn ntlm1_session_key_of_empty_secret_matches_oracle() {
    assert_eq!(
        to_hex(&keys::ntlm1_session_key(b"")),
        "be6bc64c94bbc062bcebfb40b4f93304"
    );
}

#[test]
fn server_subkeys_mirror_the_client_ones() {
    let client = keys::ntlm2_subkeys(Mode::Client, &KEY);
    let server = keys::ntlm2_subkeys(Mode::Server, &KEY);
    assert_eq!(client.send_sign, server.recv_sign);
    assert_eq!(client.send_seal, server.recv_seal);
    assert_eq!(client.recv_sign, server.send_sign);
    assert_eq!(client.recv_seal, server.send_seal);
    let both = keys::ntlm2_subkeys(Mode::Both, &KEY);
    assert_eq!(both.send_seal, server.send_seal);
}

#[test]
fn make_signature_checks_the_buffers() {
    let mut session = Session::new(Mode::Client, NEGOTIATE_SIGN, &KEY);
    let mut make =
        |parts: &mut [(u32, Vec<u8>)]| code_of(session.make_signature(&mut buffers_of(parts)));
    assert_eq!(make(&mut [zeros(SECBUFFER_TOKEN, 16)]), Err(INVALID_TOKEN));
    assert_eq!(
        make(&mut [zeros(SECBUFFER_PADDING, 1), bytes(SECBUFFER_DATA, "414243")]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        make(&mut [zeros(SECBUFFER_TOKEN, 15), bytes(SECBUFFER_DATA, "414243")]),
        Err(BUFFER_TOO_SMALL)
    );
}

#[test]
fn make_signature_rejects_flags_without_a_signature() {
    let mut session = Session::new(Mode::Client, NEGOTIATE_EXTENDED_SESSIONSECURITY, &KEY);
    let mut parts = [zeros(SECBUFFER_TOKEN, 16), bytes(SECBUFFER_DATA, "414243")];
    assert_eq!(
        code_of(session.make_signature(&mut buffers_of(&mut parts))),
        Err(UNSUPPORTED_FUNCTION)
    );
}

/// With no signing flags, `make_signature` writes the dummy token.
#[test]
fn make_signature_writes_the_dummy_token() {
    for flags in [0, NEGOTIATE_ALWAYS_SIGN] {
        let mut session = Session::new(Mode::Client, flags, &KEY);
        let mut parts = [zeros(SECBUFFER_TOKEN, 16), bytes(SECBUFFER_DATA, "414243")];
        assert_eq!(
            code_of(session.make_signature(&mut buffers_of(&mut parts))),
            Ok(())
        );
        assert_eq!(to_hex(&parts[0].1), "01000000000000000000000000000000");
        assert_eq!(to_hex(&parts[1].1), "414243");
    }
}

/// The values are the oracle's `mks` replies for candidate lines 59 and 60. The STREAM buffer
/// is not hashed, and the PADDING buffer is.
#[test]
fn make_signature_hashes_data_and_padding_but_not_stream() {
    let flags = NEGOTIATE_EXTENDED_SESSIONSECURITY | NEGOTIATE_SIGN;
    let mut session = Session::new(Mode::Client, flags, &KEY);
    let mut parts = [
        zeros(SECBUFFER_TOKEN, 16),
        bytes(SECBUFFER_STREAM, "0102"),
        bytes(SECBUFFER_DATA, "414243"),
    ];
    assert_eq!(
        code_of(session.make_signature(&mut buffers_of(&mut parts))),
        Ok(())
    );
    assert_eq!(to_hex(&parts[0].1), "01000000bc5fd3f1d3210e0c00000000");

    let mut session = Session::new(Mode::Client, flags, &KEY);
    let mut parts = [
        zeros(SECBUFFER_TOKEN, 16),
        bytes(SECBUFFER_DATA, "414243"),
        bytes(SECBUFFER_PADDING, "ff"),
    ];
    assert_eq!(
        code_of(session.make_signature(&mut buffers_of(&mut parts))),
        Ok(())
    );
    assert_eq!(to_hex(&parts[0].1), "01000000dc77dde71d9be8c100000000");
}

#[test]
fn verify_checks_the_buffers_and_the_token() {
    let flags = NEGOTIATE_EXTENDED_SESSIONSECURITY | NEGOTIATE_SIGN;
    let mut session = Session::new(Mode::Server, flags, &KEY);
    let mut verify =
        |parts: &mut [(u32, Vec<u8>)]| code_of(session.verify_signature(&buffers_of(parts)));
    assert_eq!(
        verify(&mut [zeros(SECBUFFER_TOKEN, 16)]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        verify(&mut [zeros(SECBUFFER_PADDING, 1), bytes(SECBUFFER_DATA, "414243")]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        verify(&mut [zeros(SECBUFFER_TOKEN, 15), bytes(SECBUFFER_DATA, "414243")]),
        Err(BUFFER_TOO_SMALL)
    );

    let mut session = Session::new(Mode::Server, flags, &KEY);
    let mut parts = [
        bytes(SECBUFFER_TOKEN, "01000000bc5fd3f1d3210e0c00000000"),
        bytes(SECBUFFER_DATA, "414243"),
    ];
    assert_eq!(
        code_of(session.verify_signature(&buffers_of(&mut parts))),
        Ok(())
    );

    let mut session = Session::new(Mode::Server, flags, &KEY);
    let mut parts = [
        bytes(SECBUFFER_TOKEN, "01000000bc5fd3f1d3210e0c00000001"),
        bytes(SECBUFFER_DATA, "414243"),
    ];
    assert_eq!(
        code_of(session.verify_signature(&buffers_of(&mut parts))),
        Err(MESSAGE_ALTERED)
    );
}

#[test]
fn verify_rejects_flags_without_a_signature() {
    let mut session = Session::new(Mode::Server, NEGOTIATE_EXTENDED_SESSIONSECURITY, &KEY);
    let mut parts = [zeros(SECBUFFER_TOKEN, 16), bytes(SECBUFFER_DATA, "414243")];
    assert_eq!(
        code_of(session.verify_signature(&buffers_of(&mut parts))),
        Err(UNSUPPORTED_FUNCTION)
    );
}

#[test]
fn seal_checks_the_buffers() {
    let flags = NEGOTIATE_EXTENDED_SESSIONSECURITY | NEGOTIATE_SEAL | NEGOTIATE_SIGN;
    let mut session = Session::new(Mode::Client, flags, &KEY);
    let mut seal =
        |parts: &mut [(u32, Vec<u8>)]| code_of(session.seal_message(&mut buffers_of(parts)));
    assert_eq!(
        seal(&mut [bytes(SECBUFFER_DATA, "414243")]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        seal(&mut [zeros(SECBUFFER_TOKEN, 16), zeros(SECBUFFER_PADDING, 1)]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        seal(&mut [zeros(SECBUFFER_PADDING, 1), bytes(SECBUFFER_DATA, "414243")]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        seal(&mut [zeros(SECBUFFER_TOKEN, 15), bytes(SECBUFFER_DATA, "414243")]),
        Err(BUFFER_TOO_SMALL)
    );
}

/// The token is written and the data is sealed, so the other side can unseal the message.
#[test]
fn sealed_message_unseals_on_the_other_side() {
    let flags =
        NEGOTIATE_EXTENDED_SESSIONSECURITY | NEGOTIATE_SEAL | NEGOTIATE_SIGN | NEGOTIATE_KEY_EXCH;
    let mut sender = Session::new(Mode::Client, flags, &KEY);
    let mut sealed = [zeros(SECBUFFER_TOKEN, 16), bytes(SECBUFFER_DATA, "414243")];
    assert_eq!(
        code_of(sender.seal_message(&mut buffers_of(&mut sealed))),
        Ok(())
    );
    assert_ne!(to_hex(&sealed[1].1), "414243");

    let mut receiver = Session::new(Mode::Server, flags, &KEY);
    assert_eq!(
        code_of(receiver.unseal_message(&mut buffers_of(&mut sealed))),
        Ok(())
    );
    assert_eq!(to_hex(&sealed[1].1), "414243");
}

/// Without extended security, the data and the token share the NTLM1 stream.
#[test]
fn unseal_without_extended_security_uses_the_ntlm1_stream() {
    let mut session = Session::new(Mode::Server, NEGOTIATE_SIGN, &KEY);
    let mut parts = [
        bytes(SECBUFFER_DATA, "c419ef"),
        bytes(SECBUFFER_TOKEN, "0100000057922761cdfbda1a3b95457d"),
    ];
    assert_eq!(
        code_of(session.unseal_message(&mut buffers_of(&mut parts))),
        Ok(())
    );
    assert_eq!(to_hex(&parts[0].1), "414243");
}

/// With `ALWAYS_SIGN`, the checksum of the token is zeroed after the seal.
#[test]
fn seal_with_always_sign_zeroes_the_checksum() {
    let mut session = Session::new(Mode::Client, NEGOTIATE_ALWAYS_SIGN, &KEY);
    let mut parts = [zeros(SECBUFFER_TOKEN, 16), bytes(SECBUFFER_DATA, "414243")];
    assert_eq!(
        code_of(session.seal_message(&mut buffers_of(&mut parts))),
        Ok(())
    );
    assert_eq!(to_hex(&parts[0].1), "0100000000000000cdfbda1a3b95457d");
    assert_eq!(to_hex(&parts[1].1), "c419ef");
}

#[test]
fn unseal_checks_the_buffers() {
    let mut session = Session::new(Mode::Server, NEGOTIATE_SIGN, &KEY);
    let mut unseal =
        |parts: &mut [(u32, Vec<u8>)]| code_of(session.unseal_message(&mut buffers_of(parts)));
    assert_eq!(
        unseal(&mut [bytes(SECBUFFER_DATA, "c419ef")]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        unseal(&mut [zeros(SECBUFFER_PADDING, 1), zeros(SECBUFFER_TOKEN, 16)]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        unseal(&mut [
            bytes(SECBUFFER_DATA, "c419ef"),
            zeros(SECBUFFER_TOKEN, 16),
            zeros(SECBUFFER_STREAM, 16),
        ]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        unseal(&mut [bytes(SECBUFFER_DATA, "c419ef"), zeros(SECBUFFER_PADDING, 1)]),
        Err(INVALID_TOKEN)
    );
    assert_eq!(
        unseal(&mut [bytes(SECBUFFER_DATA, "c419ef"), zeros(SECBUFFER_TOKEN, 15)]),
        Err(BUFFER_TOO_SMALL)
    );
    assert_eq!(
        unseal(&mut [bytes(SECBUFFER_DATA, "c419ef"), zeros(SECBUFFER_STREAM, 15)]),
        Err(BUFFER_TOO_SMALL)
    );
}
