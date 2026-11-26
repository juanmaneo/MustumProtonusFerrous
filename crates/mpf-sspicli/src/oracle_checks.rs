//! Oracle-derived vectors for the ported sspicli logic.
//!
//! The vectors come from the verbatim Wine C bodies in `dlls/sspicli/main.c`, compiled with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/sspicli/` and run by `run.sh`.
//! The harness prints 24 rows. Three are excluded because a sanitizer flags them: `m10` (marshal
//! reads 10 bytes from a 6-byte buffer), `u8` (unmarshal EX2 writes 60 bytes into a 48-byte
//! allocation), and `u9` (unmarshal reads 10 bytes past a 50-byte input). The 21 remaining rows
//! are tested below, each with its printed row as a comment.

use super::*;

/// Decodes the hex that the harness prints for a byte buffer.
fn hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

/// UTF-16LE bytes of `s`.
fn wb(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(u16::to_le_bytes).collect()
}

/// UTF-16 code units of `s`.
fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
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

#[test]
fn oracle_m1_marshal_exw_unicode() {
    // row m1 status=0x00000000 len=70 hex=000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000000000061006c0069006300650043004f00520050007000770000000000
    let (user, domain, password) = (wb("alice"), wb("CORP"), wb("pw"));
    let id = AuthIdentity::Exw {
        creds: creds(
            Some(&user[..]),
            5,
            Some(&domain[..]),
            4,
            Some(&password[..]),
            2,
            AUTH_IDENTITY_UNICODE,
        ),
        package_list: None,
        package_list_length: 0,
    };
    assert_eq!(
        marshal_auth_identity(&id),
        Ok(hex("000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000000000061006c0069006300650043004f00520050007000770000000000")),
    );
}

#[test]
fn oracle_m2_marshal_exw_ansi() {
    // row m2 status=0x00000000 len=59 hex=000200002c0000002c0000000500000031000000040000003500000002000000010000000000000000000000616c696365434f5250707700000000
    let (user, domain, password) = (b"alice".to_vec(), b"CORP".to_vec(), b"pw".to_vec());
    let id = AuthIdentity::Exw {
        creds: creds(
            Some(&user[..]),
            5,
            Some(&domain[..]),
            4,
            Some(&password[..]),
            2,
            AUTH_IDENTITY_ANSI,
        ),
        package_list: None,
        package_list_length: 0,
    };
    assert_eq!(
        marshal_auth_identity(&id),
        Ok(hex("000200002c0000002c0000000500000031000000040000003500000002000000010000000000000000000000616c696365434f5250707700000000")),
    );
}

#[test]
fn oracle_m3_marshal_exw_with_package() {
    // row m3 status=0x00000000 len=88 hex=000200002c0000002c0000000500000036000000040000003e0000000200000002000000420000000900000061006c0069006300650043004f0052005000700077004e00650067006f007400690061007400650000000000
    let (user, domain, password) = (wb("alice"), wb("CORP"), wb("pw"));
    let package = wb("Negotiate");
    let id = AuthIdentity::Exw {
        creds: creds(
            Some(&user[..]),
            5,
            Some(&domain[..]),
            4,
            Some(&password[..]),
            2,
            AUTH_IDENTITY_UNICODE,
        ),
        package_list: Some(&package[..]),
        package_list_length: 9,
    };
    assert_eq!(
        marshal_auth_identity(&id),
        Ok(hex("000200002c0000002c0000000500000036000000040000003e0000000200000002000000420000000900000061006c0069006300650043004f0052005000700077004e00650067006f007400690061007400650000000000")),
    );
}

#[test]
fn oracle_m4_marshal_null_user_keeps_declared_length() {
    // row m4 status=0x00000000 len=70 hex=000200002c00000000000000050000002c00000004000000340000000200000002000000000000000000000043004f0052005000700077000000000000000000000000000000
    let (domain, password) = (wb("CORP"), wb("pw"));
    let id = AuthIdentity::Exw {
        creds: creds(
            None,
            5,
            Some(&domain[..]),
            4,
            Some(&password[..]),
            2,
            AUTH_IDENTITY_UNICODE,
        ),
        package_list: None,
        package_list_length: 0,
    };
    assert_eq!(
        marshal_auth_identity(&id),
        Ok(hex("000200002c00000000000000050000002c00000004000000340000000200000002000000000000000000000043004f0052005000700077000000000000000000000000000000")),
    );
}

#[test]
fn oracle_m5_marshal_empty_user_only() {
    // row m5 status=0x00000000 len=48 hex=000200002c0000002c000000000000000000000000000000000000000000000002000000000000000000000000000000
    let id = AuthIdentity::Exw {
        creds: creds(Some(&b""[..]), 0, None, 0, None, 0, AUTH_IDENTITY_UNICODE),
        package_list: None,
        package_list_length: 0,
    };
    assert_eq!(
        marshal_auth_identity(&id),
        Ok(hex("000200002c0000002c000000000000000000000000000000000000000000000002000000000000000000000000000000")),
    );
}

#[test]
fn oracle_m6_marshal_null_package_keeps_declared_length() {
    // row m6 status=0x00000000 len=76 hex=000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000300000061006c0069006300650043004f00520050007000770000000000000000000000
    let (user, domain, password) = (wb("alice"), wb("CORP"), wb("pw"));
    let id = AuthIdentity::Exw {
        creds: creds(
            Some(&user[..]),
            5,
            Some(&domain[..]),
            4,
            Some(&password[..]),
            2,
            AUTH_IDENTITY_UNICODE,
        ),
        package_list: None,
        package_list_length: 3,
    };
    assert_eq!(
        marshal_auth_identity(&id),
        Ok(hex("000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000300000061006c0069006300650043004f00520050007000770000000000000000000000")),
    );
}

#[test]
fn oracle_m7_marshal_wide_matches_exw() {
    // row m7 status=0x00000000 len=70 hex=000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000000000061006c0069006300650043004f00520050007000770000000000
    let (user, domain, password) = (wb("alice"), wb("CORP"), wb("pw"));
    let id = AuthIdentity::Wide(creds(
        Some(&user[..]),
        5,
        Some(&domain[..]),
        4,
        Some(&password[..]),
        2,
        AUTH_IDENTITY_UNICODE,
    ));
    assert_eq!(
        marshal_auth_identity(&id),
        Ok(hex("000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000000000061006c0069006300650043004f00520050007000770000000000")),
    );
}

#[test]
fn oracle_m8_marshal_ex2_copies_raw_bytes() {
    // row m8 status=0x00000000 len=56 hex=010200000000000038000000000000000000000000000000000000000000000000000000020000000000000000000000a0a1a2a3a4a5a6a7
    let raw = hex("010200000000000038000000000000000000000000000000000000000000000000000000020000000000000000000000a0a1a2a3a4a5a6a7");
    assert_eq!(
        marshal_auth_identity(&AuthIdentity::Ex2(&raw[..])),
        Ok(hex("010200000000000038000000000000000000000000000000000000000000000000000000020000000000000000000000a0a1a2a3a4a5a6a7")),
    );
}

#[test]
fn oracle_m9_marshal_unknown_version_is_internal_error() {
    // row m9 status=0x80090304
    assert_eq!(
        marshal_auth_identity(&AuthIdentity::Other),
        Err(SEC_E_INTERNAL_ERROR)
    );
}

#[test]
fn oracle_u1_unmarshal_exw_unicode() {
    // row u1 in=70:000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000000000061006c0069006300650043004f00520050007000770000000000 status=0x00000000 flags=0x6 user=5:61006c00690063006500 domain=4:43004f0052005000 password=2:70007700 package=0:-
    assert!(
        unmarshal_auth_identity(&hex("000200002c0000002c0000000500000036000000040000003e0000000200000002000000000000000000000061006c0069006300650043004f00520050007000770000000000"))
            == Ok(AuthIdentity::Exw {
                creds: creds(
                    Some(wb("alice")),
                    5,
                    Some(wb("CORP")),
                    4,
                    Some(wb("pw")),
                    2,
                    AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
                ),
                package_list: None,
                package_list_length: 0,
            })
    );
}

#[test]
fn oracle_u2_unmarshal_exw_ansi() {
    // row u2 in=59:000200002c0000002c0000000500000031000000040000003500000002000000010000000000000000000000616c696387434f5250707700000000 status=0x00000000 flags=0x5 user=5:616c696365 domain=4:434f5250 password=2:7077 package=0:-
    assert!(
        unmarshal_auth_identity(&hex("000200002c0000002c0000000500000031000000040000003500000002000000010000000000000000000000616c696365434f5250707700000000"))
            == Ok(AuthIdentity::Exw {
                creds: creds(
                    Some(b"alice".to_vec()),
                    5,
                    Some(b"CORP".to_vec()),
                    4,
                    Some(b"pw".to_vec()),
                    2,
                    AUTH_IDENTITY_ANSI | AUTH_IDENTITY_MARSHALLED,
                ),
                package_list: None,
                package_list_length: 0,
            })
    );
}

#[test]
fn oracle_u3_unmarshal_null_user_and_empty_domain() {
    // row u3 in=48:000200002c00000000000000050000002c00000000000000000000000000000002000000000000000000000000000000 status=0x00000000 flags=0x6 user=5:- domain=0: password=0:- package=0:-
    assert!(
        unmarshal_auth_identity(&hex("000200002c00000000000000050000002c00000000000000000000000000000002000000000000000000000000000000"))
            == Ok(AuthIdentity::Exw {
                creds: creds(
                    None,
                    5,
                    Some(Vec::new()),
                    0,
                    None,
                    0,
                    AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
                ),
                package_list: None,
                package_list_length: 0,
            })
    );
}

#[test]
fn oracle_u4_unmarshal_wrong_header_length_is_invalid_token() {
    // row u4 in=48:000200002800000000000000000000000000000000000000000000000000000002000000000000000000000000000000 status=0x80090308
    assert_eq!(
        unmarshal_auth_identity(&hex("000200002800000000000000000000000000000000000000000000000000000002000000000000000000000000000000")).err(),
        Some(SEC_E_INVALID_TOKEN),
    );
}

#[test]
fn oracle_u5_unmarshal_unknown_version_is_invalid_token() {
    // row u5 in=48:000300002c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000 status=0x80090308
    assert_eq!(
        unmarshal_auth_identity(&hex("000300002c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000")).err(),
        Some(SEC_E_INVALID_TOKEN),
    );
}

#[test]
fn oracle_u6_unmarshal_ex2_keeps_bytes() {
    // row u6 in=48:010200000000000030000000000000000000000000000000000000000000000000000000020000000000000000000000 status=0x00000000 ex2=010200000000000030000000000000000000000000000000000000000000000000000000020000000000000000000000
    assert!(
        unmarshal_auth_identity(&hex("010200000000000030000000000000000000000000000000000000000000000000000000020000000000000000000000"))
            == Ok(AuthIdentity::Ex2(hex("010200000000000030000000000000000000000000000000000000000000000000000000020000000000000000000000")))
    );
}

#[test]
fn oracle_u7_unmarshal_ex2_longer_than_input_is_invalid_token() {
    // row u7 in=50:01020000000000003c0000000000000000000000000000000000000000000000000000000000000000000000000000000000 status=0x80090308
    assert_eq!(
        unmarshal_auth_identity(&hex("01020000000000003c0000000000000000000000000000000000000000000000000000000000000000000000000000000000")).err(),
        Some(SEC_E_INVALID_TOKEN),
    );
}

#[test]
fn oracle_e1_encode_alice_corp_pw() {
    // row e1 status=0x00000000 flags=0x6 user=5:61006c00690063006500 domain=4:43004f0052005000 password=2:70007700 package=0:-
    assert!(
        encode_strings_as_auth_identity(Some(&w("alice")), Some(&w("CORP")), Some(&w("pw")))
            == Ok(AuthIdentity::Exw {
                creds: creds(
                    Some(wb("alice")),
                    5,
                    Some(wb("CORP")),
                    4,
                    Some(wb("pw")),
                    2,
                    AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
                ),
                package_list: None,
                package_list_length: 0,
            })
    );
}

#[test]
fn oracle_e2_encode_empty_user_null_domain() {
    // row e2 flags=0x6 user=0: domain=0:- password=2:70007700 package=0:-
    let empty: Vec<u16> = Vec::new();
    assert!(
        encode_strings_as_auth_identity(Some(&empty), None, Some(&w("pw")))
            == Ok(AuthIdentity::Exw {
                creds: creds(
                    Some(Vec::new()),
                    0,
                    None,
                    0,
                    Some(wb("pw")),
                    2,
                    AUTH_IDENTITY_UNICODE | AUTH_IDENTITY_MARSHALLED,
                ),
                package_list: None,
                package_list_length: 0,
            })
    );
}

#[test]
fn oracle_c1_cred_write_with_domain_and_target() {
    // row c1 status=0x00000000 type=2 username=CORP\alice target=TERM/host blob=4:70007700
    let cw = prepare_for_cred_write(
        &w("alice"),
        Some(&w("CORP")),
        Some(&w("TERM/host")),
        &w("pw"),
    );
    assert_eq!(cw.cred_type, 2);
    assert_eq!(cw.username, w("CORP\\alice"));
    assert_eq!(cw.target_name, w("TERM/host"));
    assert_eq!(cw.blob, hex("70007700"));
}

#[test]
fn oracle_c2_cred_write_empty_domain_and_no_target() {
    // row c2 status=0x00000000 type=2 username=alice target=alice blob=4:70007700
    let cw = prepare_for_cred_write(&w("alice"), Some(&[]), None, &w("pw"));
    assert_eq!(cw.cred_type, 2);
    assert_eq!(cw.username, w("alice"));
    assert_eq!(cw.target_name, w("alice"));
    assert_eq!(cw.blob, hex("70007700"));
}

#[test]
fn oracle_c3_cred_write_cuts_each_string_at_nul() {
    // row c3 status=0x00000000 type=2 username=CO\al target=ab blob=2:7000
    let cw = prepare_for_cred_write(&w("al\0zz"), Some(&w("CO")), Some(&w("ab")), &w("p\0w"));
    assert_eq!(cw.cred_type, 2);
    assert_eq!(cw.username, w("CO\\al"));
    assert_eq!(cw.target_name, w("ab"));
    assert_eq!(cw.blob, hex("7000"));
}
