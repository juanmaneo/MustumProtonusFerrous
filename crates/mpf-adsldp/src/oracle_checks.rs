//! Tests against Wine's oracle rows.
//!
//! The rows are output of Wine's code, built by `target/canon/adsldp/gen_all.py`: `ldap.c`
//! and `schema.c` unchanged, and `parse_path` included verbatim from `adsldp.c`, all compiled
//! with `gcc -fsanitize=address,undefined` and run through a Win32 shim. No sanitizer report
//! was produced, so no vector was excluded.

use super::*;
use crate::fixtures::{check_path, lookup, PathRow, SchemaRow, Table};
use crate::schema::AdsType;

#[rustfmt::skip]
const MAP_VECTORS: &[(u32, u32)] = &[
    (LDAP_SUCCESS, ERROR_SUCCESS),
    (LDAP_REFERRAL_V2, ERROR_DS_REFERRAL),
    (LDAP_INVALID_CREDENTIALS, ERROR_DS_SEC_DESC_INVALID),
    (LDAP_NO_MEMORY, ERROR_NOT_ENOUGH_MEMORY),
    (LDAP_TIMEOUT, ERROR_TIMEOUT),
    (LDAP_MORE_RESULTS_TO_RETURN, ERROR_MORE_DATA),
    (0x7f, 0x7f),
];

#[rustfmt::skip]
const SCHEMA_VECTORS: &[SchemaRow] = &[
    ("cn", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("isDeleted", Table::Full, 0, 10, 4, AdsType::Boolean),
    ("CN", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("uSNChanged", Table::Full, 0, 10, 4, AdsType::LargeInteger),
    ("missingAttr", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("aa", Table::Unsorted, 0, 3, 0, AdsType::CaseIgnoreString),
];

#[rustfmt::skip]
const PATH_VECTORS: &[PathRow] = &[
    ("LDAP://dc.example.com", Ok((Some("dc.example.com"), 0, None))),
    ("LDAP://dc1:389/CN=Users,DC=example,DC=com", Ok((Some("dc1"), 389, Some("CN=Users,DC=example,DC=com")))),
    ("ldap://h/", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP:", Ok((None, 0, None))),
    ("HTTP://x", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP:///cn=x", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:-1", Ok((Some("h"), 4294967295, None))),
    ("LDAP://h:99999999999", Ok((Some("h"), 2147483647, None))),
    ("LDAP://h:", Ok((Some("h"), 0, None))),
    ("LDAP://h:12x/", Err(E_ADS_BAD_PATHNAME)),
];

#[test]
fn map_rows_match_wine() {
    for &(input, expected) in MAP_VECTORS {
        assert_eq!(map_ldap_error(input), expected, "input {input:#x}");
    }
}

#[test]
fn schema_rows_match_wine() {
    for &(name, table, start, single, multiple, expected) in SCHEMA_VECTORS {
        assert!(
            lookup(table, start, single, multiple, name) == expected,
            "{name}"
        );
    }
}

#[test]
fn path_rows_match_wine() {
    for &(text, expected) in PATH_VECTORS {
        check_path(text, expected);
    }
}
