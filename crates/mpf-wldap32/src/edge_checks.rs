//! Edge cases beyond the oracle rows in `oracle_checks.rs`: integer extremes, table bounds,
//! buffer boundaries and the separator and scheme rules of the host helpers.

use super::*;

#[test]
fn map_error_passes_the_i32_extremes_through_as_two_complement() {
    assert_eq!(map_error(i32::MIN), 0x8000_0000);
    assert_eq!(map_error(i32::MAX), 0x7fff_ffff);
}

#[test]
fn map_error_passes_unlisted_codes_through() {
    assert_eq!(map_error(15), 15);
    assert_eq!(map_error(22), 22);
    assert_eq!(map_error(55), 55);
    assert_eq!(map_error(77), 77);
    assert_eq!(map_error(-18), 0xffff_ffee);
}

#[test]
fn win32_mapping_covers_the_table_bounds() {
    assert_eq!(ldap_map_error_to_win32(0), 0);
    assert_eq!(ldap_map_error_to_win32(61), 8262);
    assert_eq!(ldap_map_error_to_win32(97), 8341);
    assert_eq!(ldap_map_error_to_win32(98), 8341);
}

#[test]
fn escape_error_has_code_89_and_derived_traits() {
    let error = EscapeError::ParamError;
    assert_eq!(error.code(), 89);
    assert_eq!(Clone::clone(&error), error);
    assert_eq!(format!("{error:?}"), "ParamError");
}

#[test]
fn escape_size_is_one_for_alphanumerics_and_three_for_others() {
    for byte in [
        b'/', b'0', b'9', b':', b'@', b'A', b'Z', b'[', b'`', b'a', b'z', b'{', 0x7f, 0x80,
    ] {
        let expected = if matches!(byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z') {
            2
        } else {
            4
        };
        assert_eq!(
            escape_filter_element_size(Some(&[byte])),
            expected,
            "byte {byte:#x}"
        );
    }
}

#[test]
fn escape_matches_a_format_reference_for_all_256_bytes() {
    let mut expected = String::new();
    for byte in 0..=u8::MAX {
        if matches!(byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z') {
            expected.push(char::from(byte));
        } else {
            expected.push_str(&format!("\\{byte:02X}"));
        }
    }
    let src: Vec<u8> = (0..=u8::MAX).collect();
    let size = escape_filter_element_size(Some(&src));
    assert_eq!(size, expected.len() + 1);

    let mut dst = vec![0xAA; size + 1];
    assert_eq!(escape_filter_element(Some(&src), &mut dst), Ok(()));
    assert_eq!(&dst[..size - 1], expected.as_bytes());
    assert_eq!(dst[size - 1], 0);
    assert_eq!(dst[size], 0xAA);
}

#[test]
fn escape_fits_an_exact_destination_including_the_nul() {
    let mut dst = [0xAA; 4];
    assert_eq!(escape_filter_element(Some(b"("), &mut dst), Ok(()));
    assert_eq!(dst, [0x5C, 0x32, 0x38, 0x00]);

    let mut dst = [0xAA; 6];
    assert_eq!(escape_filter_element(Some(b"a\0z"), &mut dst), Ok(()));
    assert_eq!(dst, [0x61, 0x5C, 0x30, 0x30, 0x7A, 0x00]);
}

#[test]
fn escape_fails_without_writing_when_the_destination_is_one_byte_short() {
    let mut dst = [0xAA; 3];
    assert_eq!(
        escape_filter_element(Some(b"("), &mut dst),
        Err(EscapeError::ParamError)
    );
    assert_eq!(dst, [0xAA; 3]);
}

#[test]
fn split_cuts_the_list_at_its_first_nul() {
    assert_eq!(split_hostnames(b"dc1\0dc2"), [&b"dc1"[..]]);
    assert_eq!(
        split_hostnames(b"dc1 dc2\0 dc3"),
        [&b"dc1"[..], &b"dc2"[..]]
    );
    assert!(split_hostnames(b"\0dc1").is_empty());
}

#[test]
fn split_separates_only_on_c_isspace_bytes() {
    assert_eq!(
        split_hostnames(b"a\x0bbc\x0cde"),
        [&b"a"[..], &b"bc"[..], &b"de"[..]]
    );
    assert_eq!(split_hostnames(b"a\xc2\xa0bc"), [&b"a\xc2\xa0bc"[..]]);
    assert_eq!(split_hostnames(b"ab\x85cd"), [&b"ab\x85cd"[..]]);
}

#[test]
fn split_ends_the_last_name_before_trailing_whitespace() {
    assert_eq!(split_hostnames(b"a\t"), [&b"a"[..]]);
    assert_eq!(split_hostnames(b"dc1 dc2 "), [&b"dc1"[..], &b"dc2"[..]]);
    assert_eq!(split_hostnames(b"dc1 dc2\n"), [&b"dc1"[..], &b"dc2"[..]]);
    assert_eq!(split_hostnames(b"a  "), [&b"a"[..]]);
}

#[test]
fn split_handles_single_and_blank_lists() {
    assert_eq!(split_hostnames(b"a"), [&b"a"[..]]);
    assert_eq!(split_hostnames(b"  ab  "), [&b"ab"[..]]);
    assert_eq!(split_hostnames(b"a  bc"), [&b"a"[..], &b"bc"[..]]);
    assert!(split_hostnames(b" \t\n").is_empty());
}

#[test]
fn has_ldap_scheme_matches_the_four_schemes_ignoring_case() {
    assert!(has_ldap_scheme(b"ldap://dc1"));
    assert!(has_ldap_scheme(b"LDAPS://dc1"));
    assert!(has_ldap_scheme(b"ldapi://%2Fsock"));
    assert!(has_ldap_scheme(b"CLDAP://dc1"));
    assert!(has_ldap_scheme(b"ldap://"));
}

#[test]
fn has_ldap_scheme_rejects_other_prefixes_and_short_input() {
    assert!(!has_ldap_scheme(b""));
    assert!(!has_ldap_scheme(b"ldap"));
    assert!(!has_ldap_scheme(b"ldap:"));
    assert!(!has_ldap_scheme(b"ldap:/"));
    assert!(!has_ldap_scheme(b"ldaps:/"));
    assert!(!has_ldap_scheme(b"xldap://dc1"));
    assert!(!has_ldap_scheme(b"ldapx://dc1"));
    assert!(!has_ldap_scheme(b"http://dc1"));
}

#[test]
fn join_mixes_bare_and_schemed_hosts() {
    let hosts: [&[u8]; 3] = [b"dc1", b"ldaps://dc2", b"dc3:1234"];
    assert_eq!(
        join_hostnames(b"ldap://", &hosts, 389),
        b"ldap://dc1:389 ldaps://dc2:389 ldap://dc3:1234"
    );
}

#[test]
fn join_prints_the_port_in_full() {
    let hosts: [&[u8]; 1] = [b"dc1"];
    assert_eq!(join_hostnames(b"ldap://", &hosts, 0), b"ldap://dc1:0");
    assert_eq!(
        join_hostnames(b"ldap://", &hosts, 100_000),
        b"ldap://dc1:100000"
    );
    assert_eq!(
        join_hostnames(b"ldap://", &hosts, u32::MAX),
        b"ldap://dc1:4294967295"
    );
}

#[test]
fn join_checks_a_schemed_host_for_a_port_after_its_first_slash() {
    let hosts: [&[u8]; 2] = [b"ldap://dc1/base", b"ldap://dc2/x:9"];
    assert_eq!(
        join_hostnames(b"ldap://", &hosts, 389),
        b"ldap://dc1/base:389 ldap://dc2/x:9"
    );
}

#[test]
fn join_checks_a_bare_host_for_a_port_in_the_whole_host() {
    let hosts: [&[u8]; 1] = [b"dc1/x"];
    assert_eq!(join_hostnames(b"ldap://", &hosts, 389), b"ldap://dc1/x:389");
}

#[test]
fn join_of_no_hosts_is_empty() {
    assert!(join_hostnames(b"ldap://", &[], 389).is_empty());
}

#[test]
fn urlify_of_a_blank_list_is_empty() {
    assert!(urlify_hostnames(b"ldap://", b" \t ", 389).is_empty());
}
