//! Boundary and branch tests for the ported sspicli logic.
//!
//! These reach the failure paths, the wrapping lengths, the ANSI and EX2 branches, and the
//! package list that the oracle rows do not cover. None of these expectations come from the
//! harness; each test says so in a comment.

use super::*;

/// UTF-16 code units of `s`.
fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// UTF-16LE bytes of `s`.
fn wb(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

fn creds<B>(
    user: Option<B>,
    user_length: u32,
    domain: Option<B>,
    domain_length: u32,
    password: Option<B>,
    password_length: u32,
    flags: u32,
) -> Credentials<B> {
    Credentials {
        user,
        user_length,
        domain,
        domain_length,
        password,
        password_length,
        flags,
    }
}

/// The identity that `encode_strings_as_auth_identity` returns: UNICODE and MARSHALLED, no package list.
fn encoded(
    user: Option<Vec<u8>>,
    user_length: u32,
    domain: Option<Vec<u8>>,
    domain_length: u32,
    password: Option<Vec<u8>>,
    password_length: u32,
) -> AuthIdentity<Vec<u8>> {
    AuthIdentity::Exw {
        creds: Credentials {
            user,
            user_length,
            domain,
            domain_length,
            password,
            password_length,
            flags: AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
        },
        package_list: None,
        package_list_length: 0,
    }
}

/// A 44-byte EXW header followed by `data`. `fields` holds `(offset, length)` for the user,
/// domain, password and package list, in that order.
fn exw_input(fields: [(u32, u32); 4], flags: u32, data: &[u8]) -> Vec<u8> {
    let [user, domain, password, package] = fields;
    let words = [
        AUTH_IDENTITY_VERSION,
        44,
        user.0,
        user.1,
        domain.0,
        domain.1,
        password.0,
        password.1,
        flags,
        package.0,
        package.1,
    ];
    let mut out: Vec<u8> = words.iter().flat_map(|v| v.to_le_bytes()).collect();
    out.extend_from_slice(data);
    out
}

/// An EX2 buffer of `len` bytes whose `cbStructureLength` is `cb`. Needs `len >= 12`.
fn ex2_input(len: usize, cb: u32) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    bytes[..4].copy_from_slice(&AUTH_IDENTITY_VERSION_2.to_le_bytes());
    bytes[8..12].copy_from_slice(&cb.to_le_bytes());
    bytes
}

#[test]
fn encode_all_absent_is_invalid_token() {
    // Not oracle-derived: with no user, domain or password, the encode fails.
    assert_eq!(
        encode_strings_as_auth_identity(None, None, None).err(),
        Some(SEC_E_INVALID_TOKEN)
    );
}

#[test]
fn encode_empty_string_is_present() {
    // Not oracle-derived: an empty string is a present, zero-length field, not an absent one.
    let empty = w("");
    let id = encode_strings_as_auth_identity(Some(&empty[..]), None, None).unwrap();
    assert!(id == encoded(Some(Vec::new()), 0, None, 0, None, 0));
}

#[test]
fn encode_domain_only_is_ok() {
    // Not oracle-derived: a domain alone is enough for an identity.
    let domain = w("CORP");
    let id = encode_strings_as_auth_identity(None, Some(&domain[..]), None).unwrap();
    assert!(id == encoded(None, 0, Some(wb("CORP")), 4, None, 0));
}

#[test]
fn encode_cuts_each_string_at_nul() {
    // Not oracle-derived: each string is cut at its first NUL, as lstrlenW does.
    let (user, domain, password) = (w("al\0zz"), w("CORP\0x"), w("pw\0"));
    let id =
        encode_strings_as_auth_identity(Some(&user[..]), Some(&domain[..]), Some(&password[..]))
            .unwrap();
    assert!(id == encoded(Some(wb("al")), 2, Some(wb("CORP")), 4, Some(wb("pw")), 2));
}

#[test]
fn marshal_short_source_is_internal_error() {
    // Not oracle-derived: each field's length asks for 6 bytes, but its source has only 4.
    let src = [0u8; 4];
    let flags = AUTH_IDENTITY_UNICODE;
    let user: AuthIdentity<&[u8]> =
        AuthIdentity::Wide(creds(Some(&src[..]), 3, None, 0, None, 0, flags));
    let domain: AuthIdentity<&[u8]> =
        AuthIdentity::Wide(creds(None, 0, Some(&src[..]), 3, None, 0, flags));
    let password: AuthIdentity<&[u8]> =
        AuthIdentity::Wide(creds(None, 0, None, 0, Some(&src[..]), 3, flags));
    let package: AuthIdentity<&[u8]> = AuthIdentity::Exw {
        creds: creds(None, 0, None, 0, None, 0, flags),
        package_list: Some(&src[..]),
        package_list_length: 3,
    };
    for id in [user, domain, password, package] {
        assert_eq!(marshal_auth_identity(&id).err(), Some(SEC_E_INTERNAL_ERROR));
    }
}

#[test]
fn marshal_wrapped_length_is_internal_error() {
    // Not oracle-derived: 0x7FFF_FFFF units times 2 wraps the buffer to 46 bytes, and the
    // user copy then runs past its end.
    let src = [0u8; 4];
    let id: AuthIdentity<&[u8]> = AuthIdentity::Wide(creds(
        Some(&src[..]),
        0x7FFF_FFFF,
        None,
        0,
        None,
        0,
        AUTH_IDENTITY_UNICODE,
    ));
    assert_eq!(marshal_auth_identity(&id).err(), Some(SEC_E_INTERNAL_ERROR));
}

#[test]
fn marshal_header_that_does_not_fit_is_internal_error() {
    // Not oracle-derived: 0x7FFF_FFE8 units times 2, plus the 48-byte header and trailer, wraps
    // to a 0-byte buffer, which cannot hold the header.
    let id: AuthIdentity<&[u8]> = AuthIdentity::Wide(creds(
        None,
        0x7FFF_FFE8,
        None,
        0,
        None,
        0,
        AUTH_IDENTITY_UNICODE,
    ));
    assert_eq!(marshal_auth_identity(&id).err(), Some(SEC_E_INTERNAL_ERROR));
}

#[test]
fn marshal_other_version_is_internal_error() {
    // Not oracle-derived: an identity of an unknown version is rejected.
    let id: AuthIdentity<&[u8]> = AuthIdentity::Other;
    assert_eq!(marshal_auth_identity(&id).err(), Some(SEC_E_INTERNAL_ERROR));
}

#[test]
fn marshal_empty_exw_is_header_and_trailer() {
    // Not oracle-derived: with no fields the buffer is the 44-byte header plus the 4-byte trailer.
    let id: AuthIdentity<&[u8]> = AuthIdentity::Exw {
        creds: creds(None, 0, None, 0, None, 0, 0),
        package_list: None,
        package_list_length: 0,
    };
    let mut expected = vec![0u8; 48];
    expected[..8].copy_from_slice(&[0x00, 0x02, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00]);
    assert_eq!(marshal_auth_identity(&id), Ok(expected));
}

#[test]
fn marshal_ansi_uses_one_byte_per_character() {
    // Not oracle-derived: ANSI fields take one byte per character, so 5 characters take 5 bytes.
    let user = b"alice";
    let id: AuthIdentity<&[u8]> = AuthIdentity::Wide(creds(
        Some(&user[..]),
        5,
        None,
        0,
        None,
        0,
        AUTH_IDENTITY_ANSI,
    ));
    let bytes = marshal_auth_identity(&id).unwrap();
    assert_eq!(bytes.len(), 53);
    let expected: AuthIdentity<Vec<u8>> = AuthIdentity::Exw {
        creds: Credentials {
            user: Some(b"alice".to_vec()),
            user_length: 5,
            domain: None,
            domain_length: 0,
            password: None,
            password_length: 0,
            flags: AUTH_IDENTITY_ANSI | AUTH_IDENTITY_MARSHALLED,
        },
        package_list: None,
        package_list_length: 0,
    };
    assert!(unmarshal_auth_identity(&bytes).unwrap() == expected);
}

#[test]
fn marshal_ex2_copies_first_cb_bytes() {
    // Not oracle-derived: EX2 marshal copies exactly cbStructureLength bytes and drops the rest.
    let bytes = ex2_input(60, 44);
    let id: AuthIdentity<&[u8]> = AuthIdentity::Ex2(&bytes[..]);
    assert_eq!(marshal_auth_identity(&id), Ok(bytes[..44].to_vec()));
}

#[test]
fn marshal_ex2_short_input_is_internal_error() {
    // Not oracle-derived: an EX2 buffer needs its cbStructureLength field (bytes 8..12).
    let bytes = [0u8; 11];
    let id: AuthIdentity<&[u8]> = AuthIdentity::Ex2(&bytes[..]);
    assert_eq!(marshal_auth_identity(&id).err(), Some(SEC_E_INTERNAL_ERROR));
}

#[test]
fn marshal_ex2_length_past_input_is_internal_error() {
    // Not oracle-derived: cbStructureLength 20 is longer than the 12-byte input.
    let bytes = ex2_input(12, 20);
    let id: AuthIdentity<&[u8]> = AuthIdentity::Ex2(&bytes[..]);
    assert_eq!(marshal_auth_identity(&id).err(), Some(SEC_E_INTERNAL_ERROR));
}

#[test]
fn round_trip_encoded_identity() {
    // Not oracle-derived: unmarshal inverts marshal for an identity that encode produced.
    let (user, domain, password) = (w("alice"), w("CORP"), w("pw"));
    let enc =
        encode_strings_as_auth_identity(Some(&user[..]), Some(&domain[..]), Some(&password[..]))
            .unwrap();
    let (u, d, p) = (wb("alice"), wb("CORP"), wb("pw"));
    let id: AuthIdentity<&[u8]> = AuthIdentity::Exw {
        creds: creds(
            Some(&u[..]),
            5,
            Some(&d[..]),
            4,
            Some(&p[..]),
            2,
            AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
        ),
        package_list: None,
        package_list_length: 0,
    };
    let bytes = marshal_auth_identity(&id).unwrap();
    assert!(unmarshal_auth_identity(&bytes).unwrap() == enc);
}

#[test]
fn round_trip_with_package_list() {
    // Not oracle-derived: a package list is written after the password and read back the same way.
    let (u, pkg) = (wb("alice"), wb("Kerberos"));
    let id: AuthIdentity<&[u8]> = AuthIdentity::Exw {
        creds: creds(Some(&u[..]), 5, None, 0, None, 0, AUTH_IDENTITY_UNICODE),
        package_list: Some(&pkg[..]),
        package_list_length: 8,
    };
    let bytes = marshal_auth_identity(&id).unwrap();
    assert_eq!(bytes.len(), 74);
    let expected: AuthIdentity<Vec<u8>> = AuthIdentity::Exw {
        creds: Credentials {
            user: Some(wb("alice")),
            user_length: 5,
            domain: None,
            domain_length: 0,
            password: None,
            password_length: 0,
            flags: AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
        },
        package_list: Some(wb("Kerberos")),
        package_list_length: 8,
    };
    assert!(unmarshal_auth_identity(&bytes).unwrap() == expected);
}

#[test]
fn unmarshal_short_input_is_invalid_token() {
    // Not oracle-derived: 43 bytes is one byte short of a header.
    assert_eq!(
        unmarshal_auth_identity(&[0u8; 43]).err(),
        Some(SEC_E_INVALID_TOKEN)
    );
}

#[test]
fn unmarshal_unknown_version_is_invalid_token() {
    // Not oracle-derived: version 0x55 is neither EXW nor EX2.
    let mut bytes = exw_input([(0, 0); 4], AUTH_IDENTITY_UNICODE, &[]);
    bytes[0] = 0x55;
    assert_eq!(
        unmarshal_auth_identity(&bytes).err(),
        Some(SEC_E_INVALID_TOKEN)
    );
}

#[test]
fn unmarshal_wrong_header_length_is_invalid_token() {
    // Not oracle-derived: an EXW header must declare its own 44-byte length.
    let mut bytes = exw_input([(0, 0); 4], AUTH_IDENTITY_UNICODE, &[]);
    bytes[4] = 40;
    assert_eq!(
        unmarshal_auth_identity(&bytes).err(),
        Some(SEC_E_INVALID_TOKEN)
    );
}

#[test]
fn unmarshal_field_offset_past_input_is_invalid_token() {
    // Not oracle-derived: a field offset past the end of the input fails in each field slot.
    let slots = [
        [(0x100, 1), (0, 0), (0, 0), (0, 0)],
        [(0, 0), (0x100, 1), (0, 0), (0, 0)],
        [(0, 0), (0, 0), (0x100, 1), (0, 0)],
        [(0, 0), (0, 0), (0, 0), (0x100, 1)],
    ];
    for fields in slots {
        let bytes = exw_input(fields, AUTH_IDENTITY_UNICODE, &[]);
        assert_eq!(
            unmarshal_auth_identity(&bytes).err(),
            Some(SEC_E_INVALID_TOKEN)
        );
    }
}

#[test]
fn unmarshal_field_longer_than_input_is_invalid_token() {
    // Not oracle-derived: the user field starts inside the input but asks for 8 bytes, and only
    // 4 follow the header.
    let bytes = exw_input(
        [(44, 4), (0, 0), (0, 0), (0, 0)],
        AUTH_IDENTITY_UNICODE,
        &[1, 2, 3, 4],
    );
    assert_eq!(
        unmarshal_auth_identity(&bytes).err(),
        Some(SEC_E_INVALID_TOKEN)
    );
}

#[test]
fn unmarshal_zero_length_field_past_input_is_empty() {
    // Not oracle-derived: an empty range is valid wherever it starts, as in the port's `take`.
    let bytes = exw_input(
        [(0x100, 0), (0, 0), (0, 0), (0, 0)],
        AUTH_IDENTITY_UNICODE,
        &[],
    );
    let expected = encoded(Some(Vec::new()), 0, None, 0, None, 0);
    assert!(unmarshal_auth_identity(&bytes).unwrap() == expected);
}

#[test]
fn unmarshal_ex2_pads_short_structure_to_sizeof_ex2() {
    // Not oracle-derived: a 44-byte EX2 is copied and zero-padded to sizeof(EX2) = 48.
    let bytes = ex2_input(44, 44);
    let mut expected = bytes.clone();
    expected.resize(48, 0);
    assert!(unmarshal_auth_identity(&bytes).unwrap() == AuthIdentity::Ex2(expected));
}

#[test]
fn unmarshal_ex2_keeps_long_structure() {
    // Not oracle-derived: a 60-byte EX2 keeps all 60 bytes, so the output is not truncated to 48.
    let bytes = ex2_input(60, 60);
    assert!(unmarshal_auth_identity(&bytes).unwrap() == AuthIdentity::Ex2(bytes.clone()));
}

#[test]
fn unmarshal_ex2_length_past_input_is_invalid_token() {
    // Not oracle-derived: cbStructureLength 45 is longer than the 44-byte input.
    let bytes = ex2_input(44, 45);
    assert_eq!(
        unmarshal_auth_identity(&bytes).err(),
        Some(SEC_E_INVALID_TOKEN)
    );
}

#[test]
fn cred_write_cuts_every_string_at_nul() {
    // Not oracle-derived: each string is cut at its first NUL. The domain is kept up to its NUL.
    let out = prepare_for_cred_write(
        &w("al\0ice"),
        Some(&w("CORP\0x")[..]),
        Some(&w("host\0y")[..]),
        &w("pw\0zz"),
    );
    assert_eq!(out.cred_type, CRED_TYPE_DOMAIN_PASSWORD);
    assert_eq!(out.username, w("CORP\\al"));
    assert_eq!(out.target_name, w("host"));
    assert_eq!(out.blob, wb("pw"));
}

#[test]
fn cred_write_empty_domain_is_absent() {
    // Not oracle-derived: a zero-length domain is absent, as in dup_auth_str, so the name is the
    // user alone, and the target falls back to it.
    let out = prepare_for_cred_write(&w("alice"), Some(&w("")[..]), None, &w("pw"));
    assert_eq!(out.username, w("alice"));
    assert_eq!(out.target_name, w("alice"));
}

#[test]
fn cred_write_nul_domain_is_empty_prefix() {
    // Not oracle-derived: a domain that starts with NUL has an empty prefix, so the name is
    // "\alice".
    let domain = [0u16, 0x41];
    let out = prepare_for_cred_write(&w("alice"), Some(&domain[..]), None, &w("pw"));
    assert_eq!(out.username, w("\\alice"));
}
