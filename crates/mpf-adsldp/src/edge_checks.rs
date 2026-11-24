//! Edge tests: inputs that the oracle rows do not reach.
//!
//! Expected values are Wine's output for these inputs, from the same oracle run as
//! `oracle_checks.rs`. All 59 `ldap.c` cases are here, with the pass-through values.

use super::*;
use crate::fixtures::{check_path, lookup, PathRow, SchemaRow, Table};
use crate::schema::AdsType;

#[rustfmt::skip]
const MAP_EDGE: &[(u32, u32)] = &[
    (LDAP_SUCCESS, ERROR_SUCCESS),
    (LDAP_OPERATIONS_ERROR, ERROR_DS_OPERATIONS_ERROR),
    (LDAP_PROTOCOL_ERROR, ERROR_DS_PROTOCOL_ERROR),
    (LDAP_TIMELIMIT_EXCEEDED, ERROR_DS_TIMELIMIT_EXCEEDED),
    (LDAP_SIZELIMIT_EXCEEDED, ERROR_DS_SIZELIMIT_EXCEEDED),
    (LDAP_COMPARE_FALSE, ERROR_DS_COMPARE_FALSE),
    (LDAP_COMPARE_TRUE, ERROR_DS_COMPARE_TRUE),
    (LDAP_AUTH_METHOD_NOT_SUPPORTED, ERROR_DS_AUTH_METHOD_NOT_SUPPORTED),
    (LDAP_STRONG_AUTH_REQUIRED, ERROR_DS_STRONG_AUTH_REQUIRED),
    (LDAP_REFERRAL_V2, ERROR_DS_REFERRAL),
    (LDAP_REFERRAL, ERROR_DS_REFERRAL),
    (LDAP_ADMIN_LIMIT_EXCEEDED, ERROR_DS_ADMIN_LIMIT_EXCEEDED),
    (LDAP_UNAVAILABLE_CRIT_EXTENSION, ERROR_DS_UNAVAILABLE_CRIT_EXTENSION),
    (LDAP_CONFIDENTIALITY_REQUIRED, ERROR_DS_CONFIDENTIALITY_REQUIRED),
    (LDAP_NO_SUCH_ATTRIBUTE, ERROR_DS_NO_ATTRIBUTE_OR_VALUE),
    (LDAP_UNDEFINED_TYPE, ERROR_DS_ATTRIBUTE_TYPE_UNDEFINED),
    (LDAP_INAPPROPRIATE_MATCHING, ERROR_DS_INAPPROPRIATE_MATCHING),
    (LDAP_CONSTRAINT_VIOLATION, ERROR_DS_CONSTRAINT_VIOLATION),
    (LDAP_ATTRIBUTE_OR_VALUE_EXISTS, ERROR_DS_ATTRIBUTE_OR_VALUE_EXISTS),
    (LDAP_INVALID_SYNTAX, ERROR_DS_INVALID_ATTRIBUTE_SYNTAX),
    (LDAP_NO_SUCH_OBJECT, ERROR_DS_NO_SUCH_OBJECT),
    (LDAP_ALIAS_PROBLEM, ERROR_DS_ALIAS_PROBLEM),
    (LDAP_INVALID_DN_SYNTAX, ERROR_DS_INVALID_DN_SYNTAX),
    (LDAP_IS_LEAF, ERROR_DS_IS_LEAF),
    (LDAP_ALIAS_DEREF_PROBLEM, ERROR_DS_ALIAS_DEREF_PROBLEM),
    (LDAP_INAPPROPRIATE_AUTH, ERROR_DS_INAPPROPRIATE_AUTH),
    (LDAP_INVALID_CREDENTIALS, ERROR_DS_SEC_DESC_INVALID),
    (LDAP_INSUFFICIENT_RIGHTS, ERROR_DS_INSUFF_ACCESS_RIGHTS),
    (LDAP_BUSY, ERROR_DS_BUSY),
    (LDAP_UNAVAILABLE, ERROR_DS_UNAVAILABLE),
    (LDAP_UNWILLING_TO_PERFORM, ERROR_DS_UNWILLING_TO_PERFORM),
    (LDAP_LOOP_DETECT, ERROR_DS_LOOP_DETECT),
    (LDAP_SORT_CONTROL_MISSING, ERROR_DS_SORT_CONTROL_MISSING),
    (LDAP_OFFSET_RANGE_ERROR, ERROR_DS_OFFSET_RANGE_ERROR),
    (LDAP_NAMING_VIOLATION, ERROR_DS_NAMING_VIOLATION),
    (LDAP_OBJECT_CLASS_VIOLATION, ERROR_DS_OBJ_CLASS_VIOLATION),
    (LDAP_NOT_ALLOWED_ON_NONLEAF, ERROR_DS_CANT_ON_NON_LEAF),
    (LDAP_NOT_ALLOWED_ON_RDN, ERROR_DS_CANT_ON_RDN),
    (LDAP_ALREADY_EXISTS, ERROR_ALREADY_EXISTS),
    (LDAP_NO_OBJECT_CLASS_MODS, ERROR_DS_CANT_MOD_OBJ_CLASS),
    (LDAP_RESULTS_TOO_LARGE, ERROR_DS_OBJECT_RESULTS_TOO_LARGE),
    (LDAP_AFFECTS_MULTIPLE_DSAS, ERROR_DS_AFFECTS_MULTIPLE_DSAS),
    (LDAP_SERVER_DOWN, ERROR_DS_SERVER_DOWN),
    (LDAP_LOCAL_ERROR, ERROR_DS_LOCAL_ERROR),
    (LDAP_ENCODING_ERROR, ERROR_DS_ENCODING_ERROR),
    (LDAP_DECODING_ERROR, ERROR_DS_DECODING_ERROR),
    (LDAP_TIMEOUT, ERROR_TIMEOUT),
    (LDAP_AUTH_UNKNOWN, ERROR_DS_AUTH_UNKNOWN),
    (LDAP_FILTER_ERROR, ERROR_DS_FILTER_UNKNOWN),
    (LDAP_USER_CANCELLED, ERROR_CANCELLED),
    (LDAP_PARAM_ERROR, ERROR_DS_PARAM_ERROR),
    (LDAP_NO_MEMORY, ERROR_NOT_ENOUGH_MEMORY),
    (LDAP_CONNECT_ERROR, ERROR_CONNECTION_UNAVAIL),
    (LDAP_NOT_SUPPORTED, ERROR_DS_NOT_SUPPORTED),
    (LDAP_CONTROL_NOT_FOUND, ERROR_DS_CONTROL_NOT_FOUND),
    (LDAP_NO_RESULTS_RETURNED, ERROR_DS_NO_RESULTS_RETURNED),
    (LDAP_MORE_RESULTS_TO_RETURN, ERROR_MORE_DATA),
    (LDAP_CLIENT_LOOP, ERROR_DS_CLIENT_LOOP),
    (LDAP_REFERRAL_LIMIT_EXCEEDED, ERROR_DS_REFERRAL_LIMIT_EXCEEDED),
    (0x100, 0x100),
    (0xffffffff, 0xffffffff),
];

#[rustfmt::skip]
const SCHEMA_EDGE: &[SchemaRow] = &[
    ("distinguishedName", Table::Full, 0, 10, 4, AdsType::DnString),
    ("noSyntax", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("nTSecurityDescriptor", Table::Full, 0, 10, 4, AdsType::NtSecurityDescriptor),
    ("objectSid", Table::Full, 0, 10, 4, AdsType::OctetString),
    ("serverState", Table::Full, 0, 10, 4, AdsType::Integer),
    ("unknownSyntax", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("lastLogon", Table::Full, 0, 10, 4, AdsType::UtcTime),
    ("AAA", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("zzz", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("cn\u{0}zz", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("ou", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("organizationalUnitName", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("bootParameter", Table::Full, 0, 10, 4, AdsType::OctetString),
    ("bp", Table::Full, 0, 10, 4, AdsType::OctetString),
    ("dnBinary", Table::Full, 0, 10, 4, AdsType::DnWithBinary),
    ("DNB", Table::Full, 0, 10, 4, AdsType::DnWithBinary),
    ("lastModified", Table::Full, 0, 10, 4, AdsType::UtcTime),
    ("nope", Table::Full, 0, 10, 4, AdsType::CaseIgnoreString),
    ("bp", Table::Full, 10, 0, 4, AdsType::OctetString),
    ("cn", Table::Full, 0, 0, 14, AdsType::CaseIgnoreString),
    ("lm", Table::Full, 0, 0, 14, AdsType::UtcTime),
    ("bb", Table::Unsorted, 0, 3, 0, AdsType::Integer),
    ("zz", Table::Unsorted, 0, 3, 0, AdsType::CaseIgnoreString),
];

#[rustfmt::skip]
const PATH_EDGE: &[PathRow] = &[
    ("", Err(E_ADS_BAD_PATHNAME)),
    ("LDA", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP:h", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP:/h", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP:/", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:/", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h/a/b", Ok((Some("h"), 0, Some("a/b")))),
    ("LDAP://h/CN=A", Ok((Some("h"), 0, Some("CN=A")))),
    ("lDaP://h", Ok((Some("h"), 0, None))),
    ("LDAP://h:+7", Ok((Some("h"), 7, None))),
    ("LDAP://h:+", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:-0/o", Ok((Some("h"), 0, Some("o")))),
    ("LDAP://h: 42/o", Ok((Some("h"), 42, Some("o")))),
    ("LDAP://h:\u{a0}5", Ok((Some("h"), 5, None))),
    ("LDAP://h:\u{9}5/o", Ok((Some("h"), 5, Some("o")))),
    ("LDAP://h: ", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:5 /o", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:-99999999999", Ok((Some("h"), 2147483648, None))),
    ("LDAP://h:-2147483648", Ok((Some("h"), 2147483648, None))),
    ("LDAP://h:2147483647", Ok((Some("h"), 2147483647, None))),
    ("LDAP://h:2147483648", Ok((Some("h"), 2147483647, None))),
    ("LDAP://h:00389", Ok((Some("h"), 389, None))),
    ("LDAP://h:0x1f", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:1x", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:1", Ok((Some("h"), 1, None))),
    ("LDAP://h\u{0}junk", Ok((Some("h"), 0, None))),
    ("LDAP://:389/x", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP:x/y", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:-", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h:1:2", Err(E_ADS_BAD_PATHNAME)),
    ("LDAP://h\u{e9}st:80/x", Ok((Some("h\u{e9}st"), 80, Some("x")))),
];

#[test]
fn map_edge_matches_wine() {
    for &(input, expected) in MAP_EDGE {
        assert_eq!(map_ldap_error(input), expected, "input {input:#x}");
    }
}

#[test]
fn schema_edge_matches_wine() {
    for &(name, table, start, single, multiple, expected) in SCHEMA_EDGE {
        assert!(
            lookup(table, start, single, multiple, name) == expected,
            "{name}"
        );
    }
}

#[test]
fn path_edge_matches_wine() {
    for &(text, expected) in PATH_EDGE {
        check_path(text, expected);
    }
}
