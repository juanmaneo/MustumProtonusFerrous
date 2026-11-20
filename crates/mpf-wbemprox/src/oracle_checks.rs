//! Replays of the oracle rows r01 to r18 in `target/canon/wbemprox/rows.json`. The oracle compiles
//! the verbatim Wine C with `-fsanitize=address,undefined`, and the sanitizers report nothing for
//! any row, so no vector is excluded. Each test names its row and quotes the row's commands. The
//! expected values are the outputs of that row. In the commands, `@` is the empty string and
//! `NULL` is a NULL pointer.

use crate::compare::{
    eval_boolcmp, eval_refcmp, eval_strcmp, Operand, OP_AND, OP_EQ, OP_GE, OP_GT, OP_ISNULL, OP_LE,
    OP_LIKE, OP_LT, OP_NE,
};
use crate::expr::{
    is_boolcmp, is_refcmp, is_strcmp, EXPR_BVAL, EXPR_IVAL, EXPR_PROPVAL, EXPR_SVAL,
};
use crate::format::format_int;
use crate::like::eval_like;
use crate::types::*;
use crate::{resolve_type, HRESULT, WBEM_E_INVALID_QUERY};

fn w(s: &str) -> Vec<u16> {
    if s == "@" {
        Vec::new()
    } else {
        s.encode_utf16().collect()
    }
}

fn c(s: &str) -> Option<Vec<u16>> {
    if s == "NULL" {
        None
    } else {
        Some(w(s))
    }
}

fn like(text: &str, pattern: &str) -> bool {
    eval_like(&w(text), &w(pattern))
}

fn strcmp(op: u32, l: &str, r: &str) -> Result<i64, HRESULT> {
    eval_strcmp(op, c(l).as_deref(), c(r).as_deref())
}

fn refcmp(op: u32, l: &str, r: &str) -> Result<i64, HRESULT> {
    eval_refcmp(op, c(l).as_deref(), c(r).as_deref())
}

/// `like abc a%;like abc a_c;like abc a_;like abc a%d`
#[test]
fn r01_like_wildcards() {
    assert!(like("abc", "a%"));
    assert!(like("abc", "a_c"));
    assert!(!like("abc", "a_"));
    assert!(!like("abc", "a%d"));
}

/// `like ABC a%;like Abc %BC;like xabcx %bc%;like abc %x%`
#[test]
fn r02_like_folds_case() {
    assert!(like("ABC", "a%"));
    assert!(like("Abc", "%BC"));
    assert!(like("xabcx", "%bc%"));
    assert!(!like("abc", "%x%"));
}

/// `like @ %;like @ _;like @ @;like abc %%%`
#[test]
fn r03_like_empty_text() {
    assert!(like("", "%"));
    assert!(!like("", "_"));
    assert!(like("", ""));
    assert!(like("abc", "%%%"));
}

/// `like a\b a\b;like a\b a\\b;like a\\b a\b;like a\\b a\\b`
#[test]
fn r04_like_backslash() {
    assert!(like("a\\b", "a\\b"));
    assert!(like("a\\b", "a\\\\b"));
    assert!(!like("a\\\\b", "a\\b"));
    assert!(!like("a\\\\b", "a\\\\b"));
}

/// `like Éte été%;like Éte ÉTE;like ß ẞ;like a[b a[b`
#[test]
fn r05_like_non_ascii_and_brackets() {
    assert!(!like("Éte", "été%"));
    assert!(like("Éte", "ÉTE"));
    assert!(!like("ß", "ẞ"));
    assert!(like("a[b", "a[b"));
}

/// `strcmp 1 abc ABC;strcmp 8 abc abd;strcmp 8 abc ABC;strcmp 8 abd abc`
#[test]
fn r06_strcmp_eq_and_ne() {
    assert_eq!(strcmp(OP_EQ, "abc", "ABC"), Ok(1));
    assert_eq!(strcmp(OP_NE, "abc", "abd"), Ok(-1));
    assert_eq!(strcmp(OP_NE, "abc", "ABC"), Ok(0));
    assert_eq!(strcmp(OP_NE, "abd", "abc"), Ok(1));
}

/// `strcmp 4 b a;strcmp 5 B a;strcmp 6 a A;strcmp 7 a b;strcmp 7 b B`
#[test]
fn r07_strcmp_ordering() {
    assert_eq!(strcmp(OP_GT, "b", "a"), Ok(1));
    assert_eq!(strcmp(OP_LT, "B", "a"), Ok(0));
    assert_eq!(strcmp(OP_LE, "a", "A"), Ok(1));
    assert_eq!(strcmp(OP_GE, "a", "b"), Ok(0));
    assert_eq!(strcmp(OP_GE, "b", "B"), Ok(1));
}

/// `strcmp 11 abc a%;strcmp 11 abc b%;strcmp 11 @ %;strcmp 11 abc NULL`
#[test]
fn r08_strcmp_like_and_null() {
    assert_eq!(strcmp(OP_LIKE, "abc", "a%"), Ok(1));
    assert_eq!(strcmp(OP_LIKE, "abc", "b%"), Ok(0));
    assert_eq!(strcmp(OP_LIKE, "@", "%"), Ok(1));
    assert_eq!(strcmp(OP_LIKE, "abc", "NULL"), Ok(0));
}

/// `strcmp 2 NULL abc;strcmp 2 abc abc;strcmp 0 abc abc;strcmp 9 abc abc`
#[test]
fn r09_strcmp_invalid_operator() {
    assert_eq!(strcmp(OP_AND, "NULL", "abc"), Ok(0));
    assert_eq!(strcmp(OP_AND, "abc", "abc"), Err(WBEM_E_INVALID_QUERY));
    assert_eq!(strcmp(0, "abc", "abc"), Err(WBEM_E_INVALID_QUERY));
    assert_eq!(strcmp(OP_ISNULL, "abc", "abc"), Err(WBEM_E_INVALID_QUERY));
}

/// `refcmp 1 Ab aB;refcmp 8 x y;refcmp 8 NULL NULL;refcmp 1 NULL x;refcmp 4 a a`
#[test]
fn r10_refcmp() {
    assert_eq!(refcmp(OP_EQ, "Ab", "aB"), Ok(1));
    assert_eq!(refcmp(OP_NE, "x", "y"), Ok(-1));
    assert_eq!(refcmp(OP_NE, "NULL", "NULL"), Ok(0));
    assert_eq!(refcmp(OP_EQ, "NULL", "x"), Ok(0));
    assert_eq!(refcmp(OP_GT, "a", "a"), Err(WBEM_E_INVALID_QUERY));
}

/// `boolcmp 1 s:TRUE -1;boolcmp 8 s:yes 1;boolcmp 1 0 s:x;boolcmp 4 1 0`
#[test]
fn r11_boolcmp() {
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Str(&w("TRUE")), Operand::Int(-1)),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_NE, Operand::Str(&w("yes")), Operand::Int(1)),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Int(0), Operand::Str(&w("x"))),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_GT, Operand::Int(1), Operand::Int(0)),
        Err(WBEM_E_INVALID_QUERY)
    );
}

/// `resolve 16 19;resolve 8 3;resolve 3 8;resolve 102 102;resolve 11 11;resolve 8 8;resolve 4 4`
#[test]
fn r12_resolve_type() {
    assert_eq!(resolve_type(CIM_SINT8, CIM_UINT32), CIM_UINT64);
    assert_eq!(resolve_type(CIM_STRING, CIM_SINT32), CIM_ILLEGAL);
    assert_eq!(resolve_type(CIM_SINT32, CIM_STRING), CIM_ILLEGAL);
    assert_eq!(resolve_type(CIM_REFERENCE, CIM_REFERENCE), CIM_REFERENCE);
    assert_eq!(resolve_type(CIM_BOOLEAN, CIM_BOOLEAN), CIM_BOOLEAN);
    assert_eq!(resolve_type(CIM_STRING, CIM_STRING), CIM_STRING);
    assert_eq!(resolve_type(CIM_REAL32, CIM_REAL32), CIM_ILLEGAL);
}

/// `fmt 3 4294967301;fmt 19 -1;fmt 20 -9223372036854775808;fmt 21 -1;fmt 17 300;fmt 4 0`
#[test]
fn r13_format_int() {
    assert_eq!(format_int(CIM_SINT32, 4_294_967_301).as_deref(), Some("5"));
    assert_eq!(format_int(CIM_UINT32, -1).as_deref(), Some("4294967295"));
    assert_eq!(
        format_int(CIM_SINT64, i64::MIN).as_deref(),
        Some("-9223372036854775808")
    );
    assert_eq!(
        format_int(CIM_UINT64, -1).as_deref(),
        Some("18446744073709551615")
    );
    assert_eq!(format_int(CIM_UINT8, 300).as_deref(), Some("300"));
    assert_eq!(format_int(CIM_REAL32, 0), None);
}

/// `strpred 8 16 3 4;strpred 16 8 4 3;strpred 11 11 3 4;strpred 3 8 3 5`
#[test]
fn r14_strcmp_predicate() {
    assert!(is_strcmp(CIM_STRING, CIM_SINT8, EXPR_PROPVAL, EXPR_SVAL));
    assert!(is_strcmp(CIM_SINT8, CIM_STRING, EXPR_SVAL, EXPR_PROPVAL));
    assert!(!is_strcmp(
        CIM_BOOLEAN,
        CIM_BOOLEAN,
        EXPR_PROPVAL,
        EXPR_SVAL
    ));
    assert!(!is_strcmp(CIM_SINT32, CIM_STRING, EXPR_PROPVAL, EXPR_IVAL));
}

/// `boolpred 11 8 3 4;boolpred 8 11 4 6;boolpred 11 11 3 3;boolpred 11 8 3 5;boolpred 11 11 3 6`
#[test]
fn r15_boolcmp_predicate() {
    assert!(is_boolcmp(CIM_BOOLEAN, CIM_STRING, EXPR_PROPVAL, EXPR_SVAL));
    assert!(!is_boolcmp(CIM_STRING, CIM_BOOLEAN, EXPR_SVAL, EXPR_BVAL));
    assert!(!is_boolcmp(
        CIM_BOOLEAN,
        CIM_BOOLEAN,
        EXPR_PROPVAL,
        EXPR_PROPVAL
    ));
    assert!(!is_boolcmp(
        CIM_BOOLEAN,
        CIM_STRING,
        EXPR_PROPVAL,
        EXPR_IVAL
    ));
    assert!(is_boolcmp(
        CIM_BOOLEAN,
        CIM_BOOLEAN,
        EXPR_PROPVAL,
        EXPR_BVAL
    ));
}

/// `refpred 102 8 3 4;refpred 8 102 4 3;refpred 102 102 3 3;refpred 102 8 3 6`
#[test]
fn r16_refcmp_predicate() {
    assert!(is_refcmp(
        CIM_REFERENCE,
        CIM_STRING,
        EXPR_PROPVAL,
        EXPR_SVAL
    ));
    assert!(is_refcmp(
        CIM_STRING,
        CIM_REFERENCE,
        EXPR_SVAL,
        EXPR_PROPVAL
    ));
    assert!(!is_refcmp(
        CIM_REFERENCE,
        CIM_REFERENCE,
        EXPR_PROPVAL,
        EXPR_PROPVAL
    ));
    assert!(!is_refcmp(
        CIM_REFERENCE,
        CIM_STRING,
        EXPR_PROPVAL,
        EXPR_BVAL
    ));
}

/// `tovt 11;tovt 102;tovt 101;tovt 103;tovt 18;tovt 3;tovt 4;tovt 5;tovt 17;tovt 21`
#[test]
fn r17_to_vartype() {
    let got = [11, 102, 101, 103, 18, 3, 4, 5, 17, 21].map(to_vartype);
    assert_eq!(
        got,
        [VT_BOOL, VT_BSTR, VT_BSTR, 0, VT_I4, VT_I4, VT_R4, 0, VT_UI1, VT_UI8]
    );
}

/// `tocim 11;tocim 8;tocim 2;tocim 18;tocim 19;tocim 20;tocim 21;tocim 4;tocim 0`
#[test]
fn r18_to_cimtype() {
    let got = [
        VT_BOOL, VT_BSTR, VT_I2, VT_UI2, VT_UI4, VT_I8, VT_UI8, VT_R4, 0,
    ]
    .map(to_cimtype);
    assert_eq!(
        got,
        [
            CIM_BOOLEAN,
            CIM_STRING,
            CIM_SINT16,
            CIM_UINT16,
            CIM_UINT32,
            CIM_SINT64,
            CIM_UINT64,
            CIM_EMPTY,
            CIM_EMPTY
        ]
    );
}
