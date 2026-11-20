//! Replays of rows from the sanitizer oracle in `target/canon/wldap32/oracle.txt`. The oracle
//! compiles the verbatim `error.c`, `misc.c` and `init.c` fragments with
//! `-fsanitize=address,undefined`. Each test names the row it replays. Expected bytes are the
//! `out` fields of the oracle, in hex.

use super::*;

fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

/// oracle.txt: `map -16 96`
#[test]
fn oracle_map_client_loop() {
    assert_eq!(map_error(-16), 96);
}

/// oracle.txt: `map -17 97`
#[test]
fn oracle_map_referral_limit() {
    assert_eq!(map_error(-17), 97);
}

/// oracle.txt: `map -20 4294967276`
#[test]
fn oracle_map_unlisted_negative_passes_through() {
    assert_eq!(map_error(-20), 4_294_967_276);
}

/// oracle.txt: `map 60 60`
#[test]
fn oracle_map_unlisted_positive_passes_through() {
    assert_eq!(map_error(60), 60);
}

/// oracle.txt: `win32 60 8261`
#[test]
fn oracle_win32_sort_control_missing() {
    assert_eq!(ldap_map_error_to_win32(60), 8261);
}

/// oracle.txt: `win32 85 1053`
#[test]
fn oracle_win32_service_request_timeout() {
    assert_eq!(ldap_map_error_to_win32(85), 1053);
}

/// oracle.txt: `win32 4294967295 8341`
#[test]
fn oracle_win32_out_of_range() {
    assert_eq!(ldap_map_error_to_win32(u32::MAX), 8341);
}

/// oracle.txt: `esc null_src size 1 status 89`
#[test]
fn oracle_escape_null_source() {
    assert_eq!(escape_filter_element_size(None), 1);
    let mut dst = [0xAA; 8];
    assert_eq!(
        escape_filter_element(None, &mut dst).map_err(EscapeError::code),
        Err(89)
    );
    assert_eq!(dst, [0xAA; 8]);
}

/// oracle.txt: `esc empty size 1 status 0 out 00 next AA`
#[test]
fn oracle_escape_empty_source() {
    assert_eq!(escape_filter_element_size(Some(b"")), 1);
    let mut dst = [0xAA; 2];
    assert_eq!(escape_filter_element(Some(b""), &mut dst), Ok(()));
    assert_eq!(&dst[..1], hex("00").as_slice());
    assert_eq!(dst[1], 0xAA);
}

/// oracle.txt: `esc abc size 4 status 0 out 61626300 next AA`
#[test]
fn oracle_escape_alphanumeric_source() {
    assert_eq!(escape_filter_element_size(Some(b"abc")), 4);
    let mut dst = [0xAA; 5];
    assert_eq!(escape_filter_element(Some(b"abc"), &mut dst), Ok(()));
    assert_eq!(&dst[..4], hex("61626300").as_slice());
    assert_eq!(dst[4], 0xAA);
}

/// oracle.txt: `esc abc_short size 4 status 89`
#[test]
fn oracle_escape_short_destination() {
    assert_eq!(escape_filter_element_size(Some(b"abc")), 4);
    let mut dst = [0xAA; 3];
    assert_eq!(
        escape_filter_element(Some(b"abc"), &mut dst).map_err(EscapeError::code),
        Err(89)
    );
    assert_eq!(dst, [0xAA; 3]);
}

/// oracle.txt: `esc specials size 14 status 0 out 5C3238615C32415C32395C354300 next AA`
#[test]
fn oracle_escape_specials() {
    let src = b"(a*)\\";
    assert_eq!(escape_filter_element_size(Some(src)), 14);
    let mut dst = [0xAA; 15];
    assert_eq!(escape_filter_element(Some(src), &mut dst), Ok(()));
    assert_eq!(&dst[..14], hex("5C3238615C32415C32395C354300").as_slice());
    assert_eq!(dst[14], 0xAA);
}

/// oracle.txt: `esc high_and_nul size 7 status 0 out 5C30305C464600 next AA`
#[test]
fn oracle_escape_high_bytes_and_nul() {
    let src = [0x00, 0xff];
    assert_eq!(escape_filter_element_size(Some(&src)), 7);
    let mut dst = [0xAA; 8];
    assert_eq!(escape_filter_element(Some(&src), &mut dst), Ok(()));
    assert_eq!(&dst[..7], hex("5C30305C464600").as_slice());
    assert_eq!(dst[7], 0xAA);
}

/// oracle.txt: `split empty`
#[test]
fn oracle_split_empty() {
    assert!(split_hostnames(b"").is_empty());
}

/// oracle.txt: `split tabs_lf [dc1] [dc2]`
#[test]
fn oracle_split_tabs_and_newlines() {
    assert_eq!(split_hostnames(b" \tdc1\n dc2"), [&b"dc1"[..], &b"dc2"[..]]);
}

/// oracle.txt: `split single_last [a] [b]`
#[test]
fn oracle_split_drops_a_one_character_last_name() {
    assert_eq!(split_hostnames(b"a b c"), [&b"a"[..], &b"b"[..]]);
}

/// oracle.txt: `split two_last [a] [bc]`
#[test]
fn oracle_split_keeps_a_two_character_last_name() {
    assert_eq!(split_hostnames(b"a bc"), [&b"a"[..], &b"bc"[..]]);
}

/// oracle.txt: `urlify plain ldap://dc1:389`
#[test]
fn oracle_urlify_plain() {
    assert_eq!(urlify_hostnames(b"ldap://", b"dc1", 389), b"ldap://dc1:389");
}

/// oracle.txt: `urlify mixed_case LDAP://DC1:389 cldap://dc2:3268`
#[test]
fn oracle_urlify_mixed_case_schemes() {
    assert_eq!(
        urlify_hostnames(b"cldap://", b"LDAP://DC1 dc2:3268", 389),
        b"LDAP://DC1:389 cldap://dc2:3268"
    );
}

/// oracle.txt: `urlify ipv6_no_port ldaps://dc1:636 ldap://[fe80::1]`
#[test]
fn oracle_urlify_ipv6_without_port() {
    assert_eq!(
        urlify_hostnames(b"ldap://", b"ldaps://dc1 [fe80::1]", 636),
        b"ldaps://dc1:636 ldap://[fe80::1]"
    );
}
