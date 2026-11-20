//! Edge rows from the same Wine harness, and checks on the port that the oracle
//! rows do not reach: a NUL ends the text, and the derived trait impls.

use crate::oracle_checks::replay;
use crate::{parse_criteria, CompareOp, Criteria};

#[rustfmt::skip]
const EDGE_VECTORS: &[(&str, &str)] = &[
    ("[a", "ERR"),
    ("[a]x=1", "ERR"),
    ("[]=1", "ERR"),
    ("[a b] = 1", "ok|2|a b|1"),
    ("a<", "ERR"),
    ("a<>", "ERR"),
    ("a>=", "ERR"),
    ("a>", "ERR"),
    ("a<=", "ERR"),
    ("a< 5", "ERR"),
    ("a=#", "ok|2|a|"),
    ("a=#x#y", "ERR"),
    ("a=#x*#", "ok|2|a|x*"),
    ("a like #x*#", "ok|5|a|x"),
    ("a like #*#", "ERR"),
    ("a like #*x*#", "ok|6|a|x"),
    ("a='x'y", "ERR"),
    ("a=''", "ok|2|a|"),
    ("a=''''", "ok|2|a|'"),
    ("a like ''''", "ok|2|a|'"),
    ("a=  ", "ERR"),
    ("a = 'b' c", "ERR"),
    ("a=b\tc", "ERR"),
    ("a\t=1", "ok|2|a|1"),
    ("a LIKE", "ERR"),
    ("a lik", "ERR"),
    ("a LIKEx", "ok|2|a|x"),
    ("a like ''", "ok|2|a|"),
    ("a !=1", "ERR"),
    ("[", "ERR"),
    ("=", "ERR"),
    ("a=x*", "ok|2|a|x*"),
    ("a=*", "ok|2|a|*"),
    ("a like *", "ERR"),
    ("a like **", "ok|6|a|"),
    ("a like *x", "ERR"),
    ("a like x*", "ok|5|a|x"),
    ("a like *x*", "ok|6|a|x"),
    ("a like '*x*'", "ok|6|a|x"),
    ("a like '*x'", "ERR"),
    ("a like '*'", "ERR"),
    ("a like '*", "ERR"),
    ("a like '*x", "ERR"),
    ("a like 'x*", "ok|5|a|x"),
    ("a like '*''*'", "ok|6|a|'"),
    ("a like '''*'", "ok|5|a|'"),
    ("name like  ab", "ok|2|name|ab"),
    ("a <10", "ok|0|a|10"),
    ("a <>7", "ok|7|a|7"),
    ("a >5", "ok|4|a|5"),
    ("a >=1", "ok|3|a|1"),
    ("a <=1", "ok|1|a|1"),
    ("a >1", "ok|4|a|1"),
    ("   ", "ERR"),
    ("a>=1", "ok|2|a>|1"),
    ("a == 1", "ERR"),
];

#[test]
fn edge_rows_match_wine() {
    for (input, expected) in EDGE_VECTORS {
        assert_eq!(replay(input), *expected, "criteria {input:?}");
    }
}

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn nul_ends_the_text() {
    let mut text = units("a=1");
    text.extend([0, u16::from(b'j')]);
    assert_eq!(
        parse_criteria(&text),
        Some(Criteria {
            column: units("a"),
            op: CompareOp::Eq,
            value: units("1"),
        })
    );
    assert_eq!(parse_criteria(&[0, u16::from(b'a')]), None);
}

#[test]
fn empty_text_is_refused() {
    assert_eq!(parse_criteria(&[]), None);
}

#[test]
fn compare_op_codes_match_dbcompareops() {
    let codes: Vec<u32> = [
        CompareOp::Lt,
        CompareOp::Le,
        CompareOp::Eq,
        CompareOp::Ge,
        CompareOp::Gt,
        CompareOp::BeginsWith,
        CompareOp::Contains,
        CompareOp::Ne,
    ]
    .iter()
    .map(|op| op.code())
    .collect();
    assert_eq!(codes, [0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn derived_impls() {
    let criteria = parse_criteria(&units("[x y]<>'z'")).unwrap();
    assert_eq!(criteria.clone(), criteria);
    assert_eq!(criteria.op, CompareOp::Ne);
    assert_eq!(criteria.column, units("x y"));
    assert_eq!(criteria.value, units("z"));
    assert!(format!("{criteria:?}").contains("Ne"));
    let op = criteria.op;
    assert_eq!(op.clone(), op);
    assert!(format!("{op:?}").contains("Ne"));
    assert_ne!(op, CompareOp::Lt);
}
