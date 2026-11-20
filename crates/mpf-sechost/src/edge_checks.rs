//! Boundary checks and supported alias behavior not covered by the oracle rows.

use super::{parse_string_sid, Sid};

const WELL_KNOWN_ALIASES: &[(&str, &str)] = &[
    ("WD", "S-1-1-0"),
    ("CO", "S-1-3-0"),
    ("CG", "S-1-3-1"),
    ("OW", "S-1-3-4"),
    ("NU", "S-1-5-2"),
    ("IU", "S-1-5-4"),
    ("SU", "S-1-5-6"),
    ("AN", "S-1-5-7"),
    ("ED", "S-1-5-9"),
    ("PS", "S-1-5-10"),
    ("AU", "S-1-5-11"),
    ("RC", "S-1-5-12"),
    ("SY", "S-1-5-18"),
    ("LS", "S-1-5-19"),
    ("NS", "S-1-5-20"),
    ("BA", "S-1-5-32-544"),
    ("BU", "S-1-5-32-545"),
    ("BG", "S-1-5-32-546"),
    ("PU", "S-1-5-32-547"),
    ("AO", "S-1-5-32-548"),
    ("SO", "S-1-5-32-549"),
    ("PO", "S-1-5-32-550"),
    ("BO", "S-1-5-32-551"),
    ("RE", "S-1-5-32-552"),
    ("RU", "S-1-5-32-554"),
    ("RD", "S-1-5-32-555"),
    ("NO", "S-1-5-32-556"),
    ("LW", "S-1-16-4096"),
    ("ME", "S-1-16-8192"),
    ("HI", "S-1-16-12288"),
    ("SI", "S-1-16-16384"),
    ("AC", "S-1-15-2-1"),
];

const DOMAIN_RELATIVE_ALIASES: &[(&str, u32)] = &[
    ("LA", 500),
    ("LG", 501),
    ("DA", 512),
    ("DU", 513),
    ("DG", 514),
    ("DC", 515),
    ("DD", 516),
    ("CA", 517),
    ("SA", 518),
    ("EA", 519),
    ("PA", 520),
    ("RS", 553),
];

#[test]
fn every_well_known_alias_matches_its_numeric_form() {
    for (alias, numeric) in WELL_KNOWN_ALIASES {
        assert_eq!(
            parse_string_sid(alias),
            Some(expect_sid(numeric)),
            "alias {alias}"
        );
    }
}

#[test]
fn every_domain_relative_alias_uses_the_computer_sid_prefix() {
    for (alias, rid) in DOMAIN_RELATIVE_ALIASES {
        let numeric = format!("S-1-5-21-0-0-0-{rid}");
        assert_eq!(
            parse_string_sid(alias),
            Some(expect_sid(&numeric)),
            "alias {alias}"
        );
    }
}

#[test]
fn aliases_are_case_insensitive() {
    assert_eq!(parse_string_sid("sy"), parse_string_sid("S-1-5-18"));
    assert_eq!(parse_string_sid("Ba"), parse_string_sid("S-1-5-32-544"));
    assert_eq!(parse_string_sid("la"), parse_string_sid("LA"));
}

#[test]
fn aliases_are_exactly_two_ascii_characters() {
    for input in ["", "W", "WDx", "Wé", "é", "ZZ"] {
        assert_eq!(parse_string_sid(input), None, "input {input:?}");
    }
}

#[test]
fn leading_spaces_are_skipped_and_trailing_spaces_are_not() {
    assert_eq!(parse_string_sid("   WD"), parse_string_sid("WD"));
    assert_eq!(parse_string_sid("  S-1-5-18"), parse_string_sid("S-1-5-18"));
    assert_eq!(parse_string_sid("   "), None);
    assert_eq!(parse_string_sid("WD "), None);
}

#[test]
fn numeric_forms_need_an_s_dash_prefix() {
    for input in ["x-1-5-18", "S1-5-18", "1-5-18", "S1", "S", "S-", "s-"] {
        assert_eq!(parse_string_sid(input), None, "input {input:?}");
    }
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
fn malformed_numeric_forms_are_rejected() {
    for input in [
        "S--1-5-18",
        "S-1-5-",
        "S-1-5",
        "S-1-5-18-19x",
        "S-1-5-18a",
        "S-1-5-18x",
        "S-1-5-+5",
        "S-1-+5-18",
        "S-256-5-18",
        "S-2-5-18",
        "S-1-5-18 ",
        "S-1-5-18\0",
        "S-1-5-0x0x",
        "S-1-5-0xx",
        "S-1-5-0x ",
        "S-1-5-0x00x5",
    ] {
        assert_eq!(parse_string_sid(input), None, "input {input:?}");
    }
}

#[test]
fn hexadecimal_and_signed_tokens_follow_wcstoul() {
    assert_eq!(parse_string_sid("S-1-5-0X1F"), Some(sid(5, &[31])));
    assert_eq!(parse_string_sid("S-1-5-0xAbC"), Some(sid(5, &[0xabc])));
    assert_eq!(parse_string_sid("S-1-5-0x0X5"), Some(sid(5, &[5])));
    assert_eq!(parse_string_sid("S-1-5-0x05"), Some(sid(5, &[5])));
    assert_eq!(parse_string_sid("S-1-5-0x-0"), Some(sid(5, &[0])));
    assert_eq!(parse_string_sid("S-1-5-0x+1"), Some(sid(5, &[1])));
    assert_eq!(parse_string_sid("S-1-5-0x\t5"), Some(sid(5, &[5])));
    assert_eq!(parse_string_sid("S-1-5-0x-"), Some(sid(5, &[0])));
}

#[test]
fn overflowing_tokens_saturate_without_negation() {
    for input in [
        "S-1-5-99999999999999999999999999",
        "S-1-5--99999999999999999999",
        "S-1-5-0x100000000",
        "S-1-5-0xFFFFFFFFFFFFFFFFFFFF",
    ] {
        assert_eq!(
            parse_string_sid(input),
            Some(sid(5, &[u32::MAX])),
            "input {input:?}"
        );
    }
}

#[test]
fn negative_tokens_wrap_within_the_dword_width() {
    assert_eq!(parse_string_sid("S-1-5--2"), Some(sid(5, &[u32::MAX - 1])));
    assert_eq!(parse_string_sid("S-1-5--4294967295"), Some(sid(5, &[1])));
}

#[test]
fn revision_is_truncated_to_a_byte_before_the_check() {
    assert_eq!(parse_string_sid("S-257-5-18"), Some(sid(5, &[18])));
    assert_eq!(parse_string_sid("S-513-5-18"), Some(sid(5, &[18])));
    assert_eq!(parse_string_sid("S-258-5-18"), None);
}

#[test]
fn fifteen_subauthorities_are_accepted_and_sixteen_are_rejected() {
    let fifteen = format!("S-1-5{}", "-18".repeat(15));
    assert!(parse_string_sid(&fifteen).is_some());
    let sixteen = format!("S-1-5{}", "-18".repeat(16));
    assert_eq!(parse_string_sid(&sixteen), None);
}

fn expect_sid(input: &str) -> Sid {
    parse_string_sid(input).expect("numeric SID in test table")
}

fn sid(authority: u64, sub_authorities: &[u32]) -> Sid {
    Sid {
        revision: 1,
        identifier_authority: authority,
        sub_authorities: sub_authorities.to_vec(),
    }
}
