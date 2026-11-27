//! Oracle-derived vectors for the ported `activeds.tlb` enum constants.
//!
//! The rows were captured by copying the twelve `typedef enum` blocks of `include/iads.idl`
//! verbatim into `target/canon/activeds.tlb/iads_enums.h`, compiling `harness.c` with
//! `gcc -fsanitize=address,undefined -g`, and running it. The harness prints the value gcc
//! evaluates for each row. ASan and UBSan reported nothing, so no vector was excluded.

use super::*;

/// `(enum table, member name, value)` rows printed by the oracle harness.
const ORACLE_ROWS: &[(&[Member], &str, u32)] = &[
    (ADS_RIGHTS_ENUM, "ADS_RIGHT_DS_CREATE_CHILD", 1),
    (ADS_RIGHTS_ENUM, "ADS_RIGHT_DS_CONTROL_ACCESS", 256),
    (
        ADS_RIGHTS_ENUM,
        "ADS_RIGHT_ACCESS_SYSTEM_SECURITY",
        2_097_152,
    ),
    (ADS_RIGHTS_ENUM, "ADS_RIGHT_GENERIC_READ", 2_147_483_648),
    (ADS_AUTHENTICATION_ENUM, "ADS_SECURE_AUTHENTICATION", 1),
    (ADS_AUTHENTICATION_ENUM, "ADS_USE_ENCRYPTION", 2),
    (ADS_AUTHENTICATION_ENUM, "ADS_USE_SSL", 2),
    (ADS_AUTHENTICATION_ENUM, "ADS_NO_REFERRAL_CHASING", 1024),
    (ADS_AUTHENTICATION_ENUM, "ADS_AUTH_RESERVED", 2_147_483_648),
    (ADS_SEARCHPREF_ENUM, "ADS_SEARCHPREF_ASYNCHRONOUS", 0),
    (ADS_SEARCHPREF_ENUM, "ADS_SEARCHPREF_EXTENDED_DN", 18),
    (ADS_CHASE_REFERRALS_ENUM, "ADS_CHASE_REFERRALS_NEVER", 0),
    (ADS_CHASE_REFERRALS_ENUM, "ADS_CHASE_REFERRALS_EXTERNAL", 64),
    (ADS_CHASE_REFERRALS_ENUM, "ADS_CHASE_REFERRALS_ALWAYS", 96),
    (ADS_STATUSENUM, "ADS_STATUS_INVALID_SEARCHPREFVALUE", 2),
    (ADSTYPEENUM, "ADSTYPE_INVALID", 0),
    (ADSTYPEENUM, "ADSTYPE_CASE_EXACT_STRING", 2),
    (ADSTYPEENUM, "ADSTYPE_DN_WITH_STRING", 28),
    (ADS_SCOPEENUM, "ADS_SCOPE_SUBTREE", 2),
    (ADS_PASSWORD_ENCODING_ENUM, "ADS_PASSWORD_ENCODE_CLEAR", 1),
    (ADS_OPTION_ENUM, "ADS_OPTION_SKIP_SID_LOOKUP", 9),
    (ADS_SECURITY_INFO_ENUM, "ADS_SECURITY_INFO_SACL", 8),
    (ADS_SETTYPE_ENUM, "ADS_SETTYPE_DN", 4),
    (ADS_FORMAT_ENUM, "ADS_FORMAT_X500_PARENT", 8),
    (ADS_FORMAT_ENUM, "ADS_FORMAT_LEAF", 11),
];

#[test]
fn oracle_enum_rows() {
    for (table, name, value) in ORACLE_ROWS {
        assert_eq!(member_value(table, name), Some(*value), "{name}");
    }
}
