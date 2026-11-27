//! Edge-case tests for the ported `activeds.tlb` enum constants.
//!
//! These cover the found and missing branches of the two lookups, the first-match rule for
//! shared values, the C numbering of enumerators without `=`, and the member counts. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn member_value_finds_by_exact_name_and_misses_others() {
    // Not oracle-derived: the found and missing branches of `member_value`.
    assert_eq!(member_value(ADS_SCOPEENUM, "ADS_SCOPE_BASE"), Some(0));
    assert_eq!(member_value(ADS_SCOPEENUM, "ADS_SCOPE_NOPE"), None);
    assert_eq!(member_value(ADS_SCOPEENUM, "ads_scope_base"), None);
    assert_eq!(member_value(ADS_SCOPEENUM, "ADS_SCOPEENUM"), None);
    assert_eq!(member_value(&[], "ADS_SCOPE_BASE"), None);
}

#[test]
fn member_name_returns_the_first_member_with_a_shared_value() {
    // Not oracle-derived: the first-match rule. Rows 06 and 07 show that ADS_USE_ENCRYPTION and
    // ADS_USE_SSL share the value 2.
    assert_eq!(
        member_name(ADS_AUTHENTICATION_ENUM, 2),
        Some("ADS_USE_ENCRYPTION")
    );

    assert_eq!(
        member_name(ADS_CHASE_REFERRALS_ENUM, 0x60),
        Some("ADS_CHASE_REFERRALS_ALWAYS")
    );
}

#[test]
fn member_name_misses_values_that_no_member_has() {
    // Not oracle-derived: the missing branch of `member_name`. Value 3 is not a member of
    // ADS_SCOPEENUM, which has 0, 1 and 2.
    assert_eq!(member_name(ADS_SCOPEENUM, 3), None);
    assert_eq!(member_name(ADS_SCOPEENUM, u32::MAX), None);
    assert_eq!(member_name(&[], 0), None);
}

#[test]
fn implicit_enumerators_count_up_from_zero() {
    // Not oracle-derived: C numbers enumerators without `=` from 0 in IDL order. Rows 10, 12,
    // 16 and 19 check the first and last members of some of these enums.
    for table in [
        ADSTYPEENUM,
        ADS_SEARCHPREF_ENUM,
        ADS_SCOPEENUM,
        ADS_STATUSENUM,
        ADS_PASSWORD_ENCODING_ENUM,
    ] {
        for (index, member) in table.iter().enumerate() {
            assert_eq!(member.value, index as u32, "{}", member.name);
        }
    }
}

#[test]
fn member_counts_match_the_idl() {
    // Not oracle-derived: the member counts of the twelve `typedef enum` blocks in
    // include/iads.idl.
    let counts = [
        (ADS_RIGHTS_ENUM, 19),
        (ADS_AUTHENTICATION_ENUM, 13),
        (ADS_SEARCHPREF_ENUM, 19),
        (ADS_CHASE_REFERRALS_ENUM, 4),
        (ADS_STATUSENUM, 3),
        (ADSTYPEENUM, 29),
        (ADS_SCOPEENUM, 3),
        (ADS_PASSWORD_ENCODING_ENUM, 2),
        (ADS_OPTION_ENUM, 10),
        (ADS_SECURITY_INFO_ENUM, 4),
        (ADS_SETTYPE_ENUM, 4),
        (ADS_FORMAT_ENUM, 11),
    ];
    for (table, count) in counts {
        assert_eq!(table.len(), count);
    }
}
