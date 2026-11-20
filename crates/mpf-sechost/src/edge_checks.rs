//! Boundary checks and supported alias behavior not covered by the numeric SID oracle rows.

use super::{parse_string_sid, Sid};

#[test]
fn common_well_known_aliases_match_their_numeric_forms() {
    assert_eq!(parse_string_sid("WD"), parse_string_sid("S-1-1-0"));
    assert_eq!(parse_string_sid("sy"), parse_string_sid("S-1-5-18"));
    assert_eq!(parse_string_sid("Ba"), parse_string_sid("S-1-5-32-544"));
    assert_eq!(parse_string_sid("ME"), parse_string_sid("S-1-16-8192"));
    assert_eq!(parse_string_sid("AC"), parse_string_sid("S-1-15-2-1"));
}

#[test]
fn every_supported_well_known_sid_alias_resolves() {
    for alias in [
        "WD", "CO", "CG", "OW", "NU", "IU", "SU", "AN", "ED", "PS", "AU", "RC", "SY", "LS", "NS",
        "BA", "BU", "BG", "PU", "AO", "SO", "PO", "BO", "RE", "RU", "RD", "NO", "LW", "ME", "HI",
        "SI", "AC",
    ] {
        assert!(parse_string_sid(alias).is_some(), "alias {alias}");
    }
}

#[test]
fn every_supported_domain_relative_rid_alias_resolves() {
    for alias in [
        "LA", "LG", "DA", "DU", "DG", "DC", "DD", "CA", "SA", "EA", "PA", "RS",
    ] {
        assert!(parse_string_sid(alias).is_some(), "alias {alias}");
    }
}

#[test]
fn domain_relative_aliases_use_wines_computer_sid_prefix() {
    assert_eq!(
        parse_string_sid("LA"),
        parse_string_sid("S-1-5-21-0-0-0-500")
    );
    assert_eq!(
        parse_string_sid("RS"),
        parse_string_sid("S-1-5-21-0-0-0-553")
    );
}

#[test]
fn aliases_are_exactly_two_ascii_characters() {
    assert_eq!(parse_string_sid("W"), None);
    assert_eq!(parse_string_sid("WDx"), None);
    assert_eq!(parse_string_sid("é"), None);
    assert_eq!(parse_string_sid("ZZ"), None);
}

#[test]
fn numeric_authority_and_subauthorities_serialize_with_sid_byte_order() {
    let parsed = parse_string_sid("S-1-5-32-544").unwrap();
    assert_eq!(
        parsed.to_bytes(),
        vec![1, 2, 0, 0, 0, 0, 0, 5, 32, 0, 0, 0, 32, 2, 0, 0]
    );
    assert_eq!(
        Sid {
            revision: 1,
            identifier_authority: 0,
            sub_authorities: vec![],
        }
        .to_bytes(),
        vec![1, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn malformed_or_unsupported_numeric_forms_are_rejected() {
    for input in [
        "",
        "S",
        "S--1-5-18",
        "S-1-5-",
        "S-1-5-18--",
        "S-1-5-0x",
        "S-1-5-18-19x",
        "S-1-5-18a",
        "S-256-5-18",
    ] {
        assert_eq!(parse_string_sid(input), None, "input {input:?}");
    }
}

#[test]
fn token_values_wrap_to_the_windows_dword_width() {
    assert_eq!(
        parse_string_sid("S-1-5-0x100000001"),
        Some(Sid {
            revision: 1,
            identifier_authority: 5,
            sub_authorities: vec![1],
        })
    );
    assert_eq!(
        parse_string_sid("S-1--5-18"),
        Some(Sid {
            revision: 1,
            identifier_authority: u64::from(u32::MAX - 4),
            sub_authorities: vec![18],
        })
    );
    assert_eq!(
        parse_string_sid("S-1-5--1"),
        Some(Sid {
            revision: 1,
            identifier_authority: 5,
            sub_authorities: vec![u32::MAX],
        })
    );
    assert_eq!(
        parse_string_sid("S-1-0x+5-18"),
        Some(Sid {
            revision: 1,
            identifier_authority: 5,
            sub_authorities: vec![18],
        })
    );
    assert_eq!(
        parse_string_sid("S-1-0XAF-0xB"),
        Some(Sid {
            revision: 1,
            identifier_authority: 175,
            sub_authorities: vec![11],
        })
    );
    assert_eq!(
        parse_string_sid("S-1-5-18446744073709551616"),
        Some(Sid {
            revision: 1,
            identifier_authority: 5,
            sub_authorities: vec![u32::MAX],
        })
    );
}

#[test]
fn more_than_fifteen_subauthorities_are_rejected_without_storage() {
    let too_many = format!("S-1-5{}", "-1".repeat(16));
    assert_eq!(parse_string_sid(&too_many), None);
}
