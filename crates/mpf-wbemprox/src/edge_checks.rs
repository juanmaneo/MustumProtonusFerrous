//! Rust-only tests for the branches that no oracle row reaches. Each expectation follows from the
//! C control flow in `dlls/wbemprox/query.c`, or from the deviation that `lib.rs` documents.

use crate::compare::{
    eval_boolcmp, eval_refcmp, eval_strcmp, Operand, OP_AND, OP_EQ, OP_GE, OP_GT, OP_ISNULL, OP_LE,
    OP_LIKE, OP_LT, OP_NE, OP_NOT, OP_NOTNULL, OP_OR,
};
use crate::expr::{
    is_boolcmp, is_refcmp, is_strcmp, EXPR_BVAL, EXPR_COMPLEX, EXPR_IVAL, EXPR_PROPVAL, EXPR_SVAL,
    EXPR_UNARY,
};
use crate::format::format_int;
use crate::like::eval_like;
use crate::types::*;
use crate::wide::{at, rest, towlower, towupper, wcsicmp};
use crate::{is_int, resolve_type, WBEM_E_INVALID_QUERY};

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn like(text: &str, pattern: &str) -> bool {
    eval_like(&w(text), &w(pattern))
}

#[test]
fn at_reads_nul_past_the_end() {
    assert_eq!(at(&[], 0), 0);
    assert_eq!(at(&[0x61], 1), 0);
    assert_eq!(at(&[0x61, 0x62], 1), 0x62);
}

#[test]
fn rest_drops_one_unit_and_keeps_empty_empty() {
    assert_eq!(rest(&[0x61, 0x62]), &[0x62]);
    assert_eq!(rest(&[0x61]), &[] as &[u16]);
    assert_eq!(rest(&[]), &[] as &[u16]);
}

#[test]
fn case_mapping_maps_letters_and_keeps_other_units() {
    assert_eq!(towupper(u16::from(b'a')), u16::from(b'A'));
    assert_eq!(towlower(u16::from(b'A')), u16::from(b'a'));
    assert_eq!(towupper(u16::from(b'1')), u16::from(b'1'));
    assert_eq!(towlower(u16::from(b'!')), u16::from(b'!'));
}

#[test]
fn multi_character_mappings_keep_the_unit() {
    assert_eq!(towupper(0xDF), 0xDF);
    assert_eq!(towlower(0x1E9E), 0xDF);
    assert_eq!(towupper(0x1E9E), 0x1E9E);
}

#[test]
fn surrogate_units_keep_their_value() {
    assert_eq!(towupper(0xD800), 0xD800);
    assert_eq!(towlower(0xDC00), 0xDC00);
}

#[test]
fn dotted_capital_i_keeps_its_unit_unlike_glibc() {
    // glibc lowers U+0130 to `i`. Rust's full mapping has two characters, so the port keeps it.
    assert_eq!(towlower(0x130), 0x130);
}

#[test]
fn greek_ypogegrammeni_keeps_its_unit_unlike_glibc() {
    // glibc uppers U+1F80 to U+1F88. Rust's full mapping has two characters, so the port keeps it.
    assert_eq!(towupper(0x1F80), 0x1F80);
}

#[test]
fn wcsicmp_compares_case_folded_units_and_terminators() {
    assert_eq!(wcsicmp(&w("ab"), &w("abc")), -99);
    assert_eq!(wcsicmp(&w("abc"), &w("ab")), 99);
    assert_eq!(wcsicmp(&w(""), &w("")), 0);
    assert_eq!(wcsicmp(&w(""), &w("a")), -97);
    assert_eq!(wcsicmp(&w("a"), &w("b")), -1);
    assert_eq!(wcsicmp(&w("B"), &w("a")), 1);
    assert_eq!(wcsicmp(&w("ABC"), &w("abc")), 0);
}

#[test]
fn like_percent_runs_and_end_of_text() {
    assert!(like("abc", "%"));
    assert!(like("", "%%"));
    assert!(!like("", "a%"));
    assert!(like("abc", "abc%"));
    assert!(!like("abc", "%x"));
    assert!(like("abc", "%c"));
    assert!(like("abc", "%_"));
    assert!(!like("abcd", "%bd"));
}

#[test]
fn like_underscore_matches_one_unit() {
    assert!(like("x", "_"));
    assert!(!like("", "_"));
    assert!(!like("ab", "_"));
}

#[test]
fn like_mismatch_and_leftover_text() {
    assert!(!like("abc", "abd"));
    assert!(!like("abc", "ab"));
}

#[test]
fn like_backslash_edge_cases() {
    assert!(like("\\", "\\\\"));
    assert!(like("\\", "\\"));
    assert!(!like("a", "a\\"));
    assert!(!like("", "\\\\"));
}

#[test]
fn strcmp_applies_every_operator_to_a_pair() {
    let less = [
        (OP_EQ, 0),
        (OP_NE, -1),
        (OP_GT, 0),
        (OP_LT, 1),
        (OP_LE, 1),
        (OP_GE, 0),
        (OP_LIKE, 0),
    ];
    for (op, expected) in less {
        assert_eq!(
            eval_strcmp(op, Some(&w("a")), Some(&w("b"))),
            Ok(expected),
            "op {op}"
        );
    }
    let equal = [
        (OP_EQ, 1),
        (OP_NE, 0),
        (OP_GT, 0),
        (OP_LT, 0),
        (OP_LE, 1),
        (OP_GE, 1),
        (OP_LIKE, 1),
    ];
    for (op, expected) in equal {
        assert_eq!(
            eval_strcmp(op, Some(&w("a")), Some(&w("A"))),
            Ok(expected),
            "op {op}"
        );
    }
}

#[test]
fn strcmp_rejects_operators_that_are_not_comparisons() {
    for op in [0, OP_AND, OP_OR, OP_ISNULL, OP_NOTNULL, OP_NOT, 13] {
        assert_eq!(
            eval_strcmp(op, Some(&w("a")), Some(&w("a"))),
            Err(WBEM_E_INVALID_QUERY),
            "op {op}"
        );
    }
}

#[test]
fn strcmp_null_operand_gives_zero_before_checking_the_operator() {
    assert_eq!(eval_strcmp(OP_EQ, None, Some(&w("a"))), Ok(0));
    assert_eq!(eval_strcmp(OP_GT, Some(&w("a")), None), Ok(0));
    assert_eq!(eval_strcmp(99, None, None), Ok(0));
}

#[test]
fn refcmp_null_operand_gives_zero_before_checking_the_operator() {
    assert_eq!(eval_refcmp(OP_GT, None, Some(&w("a"))), Ok(0));
    assert_eq!(eval_refcmp(OP_LIKE, Some(&w("a")), None), Ok(0));
}

#[test]
fn refcmp_accepts_only_eq_and_ne() {
    assert_eq!(eval_refcmp(OP_EQ, Some(&w("a")), Some(&w("A"))), Ok(1));
    assert_eq!(eval_refcmp(OP_NE, Some(&w("a")), Some(&w("b"))), Ok(-1));
    for op in [OP_LIKE, OP_LT, OP_AND] {
        assert_eq!(
            eval_refcmp(op, Some(&w("a")), Some(&w("a"))),
            Err(WBEM_E_INVALID_QUERY),
            "op {op}"
        );
    }
}

#[test]
fn boolcmp_reads_string_operands_as_true_or_false() {
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Str(&w("true")), Operand::Int(-1)),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Str(&w("TRUE")), Operand::Int(-1)),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Str(&w("Tru")), Operand::Int(0)),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Str(&w("")), Operand::Int(0)),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_NE, Operand::Str(&w("True")), Operand::Int(0)),
        Ok(1)
    );
}

#[test]
fn boolcmp_compares_integers_and_string_pairs() {
    assert_eq!(eval_boolcmp(OP_EQ, Operand::Int(3), Operand::Int(3)), Ok(1));
    assert_eq!(eval_boolcmp(OP_EQ, Operand::Int(3), Operand::Int(4)), Ok(0));
    assert_eq!(eval_boolcmp(OP_NE, Operand::Int(3), Operand::Int(4)), Ok(1));
    assert_eq!(eval_boolcmp(OP_NE, Operand::Int(3), Operand::Int(3)), Ok(0));
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Str(&w("True")), Operand::Str(&w("true"))),
        Ok(1)
    );
    assert_eq!(
        eval_boolcmp(OP_EQ, Operand::Str(&w("True")), Operand::Int(-1)),
        Ok(1)
    );
}

#[test]
fn boolcmp_rejects_operators_other_than_eq_and_ne() {
    for op in [OP_GT, OP_LT, OP_LIKE, 0] {
        assert_eq!(
            eval_boolcmp(op, Operand::Int(1), Operand::Int(1)),
            Err(WBEM_E_INVALID_QUERY),
            "op {op}"
        );
    }
}

#[test]
fn is_int_is_true_for_the_eight_integer_types_only() {
    for ty in [
        CIM_SINT8, CIM_SINT16, CIM_SINT32, CIM_SINT64, CIM_UINT8, CIM_UINT16, CIM_UINT32,
        CIM_UINT64,
    ] {
        assert!(is_int(ty), "type {ty}");
    }
    for ty in [
        CIM_EMPTY,
        CIM_REAL32,
        CIM_REAL64,
        CIM_STRING,
        CIM_BOOLEAN,
        CIM_DATETIME,
        CIM_REFERENCE,
        CIM_CHAR16,
        CIM_ILLEGAL,
    ] {
        assert!(!is_int(ty), "type {ty}");
    }
}

#[test]
fn resolve_type_gives_uint64_for_any_integer_pair() {
    assert_eq!(resolve_type(CIM_SINT64, CIM_SINT8), CIM_UINT64);
    assert_eq!(resolve_type(CIM_UINT8, CIM_UINT64), CIM_UINT64);
}

#[test]
fn resolve_type_rejects_mixed_and_unsupported_pairs() {
    assert_eq!(resolve_type(CIM_SINT8, CIM_STRING), CIM_ILLEGAL);
    assert_eq!(resolve_type(CIM_STRING, CIM_SINT8), CIM_ILLEGAL);
    assert_eq!(resolve_type(CIM_STRING, CIM_BOOLEAN), CIM_ILLEGAL);
    assert_eq!(resolve_type(CIM_REFERENCE, CIM_STRING), CIM_ILLEGAL);
    assert_eq!(resolve_type(CIM_DATETIME, CIM_DATETIME), CIM_ILLEGAL);
}

#[test]
fn format_int_prints_the_low_32_bits_of_the_narrow_types() {
    assert_eq!(format_int(CIM_SINT8, 200).as_deref(), Some("200"));
    assert_eq!(format_int(CIM_SINT16, 70_000).as_deref(), Some("70000"));
    assert_eq!(format_int(CIM_SINT32, -5).as_deref(), Some("-5"));
    assert_eq!(format_int(CIM_UINT8, -1).as_deref(), Some("4294967295"));
    assert_eq!(format_int(CIM_UINT16, 65_537).as_deref(), Some("65537"));
}

#[test]
fn format_int_prints_the_64_bit_types_in_full() {
    assert_eq!(format_int(CIM_SINT64, -1).as_deref(), Some("-1"));
    assert_eq!(
        format_int(CIM_UINT64, -1).as_deref(),
        Some("18446744073709551615")
    );
}

#[test]
fn format_int_has_no_text_for_other_types() {
    assert_eq!(format_int(CIM_STRING, 1), None);
    assert_eq!(format_int(CIM_BOOLEAN, 1), None);
    assert_eq!(format_int(CIM_EMPTY, 1), None);
}

#[test]
fn is_strcmp_pairs_a_string_or_integer_property_with_a_string_literal() {
    let cases = [
        (CIM_STRING, CIM_BOOLEAN, EXPR_PROPVAL, EXPR_SVAL, true),
        (CIM_SINT8, CIM_BOOLEAN, EXPR_PROPVAL, EXPR_SVAL, true),
        (CIM_BOOLEAN, CIM_SINT8, EXPR_SVAL, EXPR_PROPVAL, true),
        (CIM_BOOLEAN, CIM_STRING, EXPR_SVAL, EXPR_PROPVAL, true),
        (CIM_BOOLEAN, CIM_STRING, EXPR_UNARY, EXPR_SVAL, false),
        (CIM_BOOLEAN, CIM_STRING, EXPR_SVAL, EXPR_UNARY, false),
        (CIM_BOOLEAN, CIM_STRING, EXPR_SVAL, EXPR_IVAL, false),
        (CIM_BOOLEAN, CIM_STRING, EXPR_IVAL, EXPR_SVAL, false),
        (CIM_STRING, CIM_BOOLEAN, EXPR_PROPVAL, EXPR_COMPLEX, false),
        (CIM_BOOLEAN, CIM_BOOLEAN, EXPR_PROPVAL, EXPR_SVAL, false),
        (CIM_STRING, CIM_STRING, EXPR_UNARY, EXPR_SVAL, false),
    ];
    for (ltype, rtype, left, right, expected) in cases {
        assert_eq!(
            is_strcmp(ltype, rtype, left, right),
            expected,
            "{ltype} {rtype} {left} {right}"
        );
    }
}

#[test]
fn is_boolcmp_pairs_a_boolean_property_with_a_literal() {
    let cases = [
        (CIM_BOOLEAN, CIM_STRING, EXPR_PROPVAL, EXPR_SVAL, true),
        (CIM_BOOLEAN, CIM_BOOLEAN, EXPR_PROPVAL, EXPR_BVAL, true),
        (CIM_STRING, CIM_BOOLEAN, EXPR_SVAL, EXPR_PROPVAL, true),
        (CIM_BOOLEAN, CIM_BOOLEAN, EXPR_BVAL, EXPR_PROPVAL, true),
        (CIM_BOOLEAN, CIM_STRING, EXPR_UNARY, EXPR_SVAL, false),
        (CIM_BOOLEAN, CIM_STRING, EXPR_PROPVAL, EXPR_IVAL, false),
        (CIM_STRING, CIM_BOOLEAN, EXPR_IVAL, EXPR_PROPVAL, false),
        (CIM_STRING, CIM_BOOLEAN, EXPR_SVAL, EXPR_COMPLEX, false),
        (CIM_STRING, CIM_STRING, EXPR_PROPVAL, EXPR_SVAL, false),
        (CIM_STRING, CIM_BOOLEAN, EXPR_UNARY, EXPR_PROPVAL, false),
    ];
    for (ltype, rtype, left, right, expected) in cases {
        assert_eq!(
            is_boolcmp(ltype, rtype, left, right),
            expected,
            "{ltype} {rtype} {left} {right}"
        );
    }
}

#[test]
fn is_refcmp_pairs_a_reference_property_with_a_string_literal() {
    let cases = [
        (CIM_REFERENCE, CIM_STRING, EXPR_PROPVAL, EXPR_SVAL, true),
        (CIM_STRING, CIM_REFERENCE, EXPR_SVAL, EXPR_PROPVAL, true),
        (CIM_REFERENCE, CIM_STRING, EXPR_PROPVAL, EXPR_BVAL, false),
        (
            CIM_REFERENCE,
            CIM_REFERENCE,
            EXPR_PROPVAL,
            EXPR_PROPVAL,
            false,
        ),
        (CIM_STRING, CIM_REFERENCE, EXPR_PROPVAL, EXPR_SVAL, false),
        (CIM_STRING, CIM_STRING, EXPR_SVAL, EXPR_SVAL, false),
        (CIM_REFERENCE, CIM_STRING, EXPR_SVAL, EXPR_SVAL, false),
    ];
    for (ltype, rtype, left, right, expected) in cases {
        assert_eq!(
            is_refcmp(ltype, rtype, left, right),
            expected,
            "{ltype} {rtype} {left} {right}"
        );
    }
}

#[test]
fn to_vartype_maps_each_cim_type() {
    let cases = [
        (CIM_BOOLEAN, VT_BOOL),
        (CIM_STRING, VT_BSTR),
        (CIM_REFERENCE, VT_BSTR),
        (CIM_DATETIME, VT_BSTR),
        (CIM_SINT8, VT_I1),
        (CIM_UINT8, VT_UI1),
        (CIM_SINT16, VT_I2),
        (CIM_UINT16, VT_I4),
        (CIM_SINT32, VT_I4),
        (CIM_UINT32, VT_I4),
        (CIM_SINT64, VT_I8),
        (CIM_UINT64, VT_UI8),
        (CIM_REAL32, VT_R4),
        (CIM_REAL64, 0),
        (CIM_EMPTY, 0),
    ];
    for (cim, vt) in cases {
        assert_eq!(to_vartype(cim), vt, "cim {cim}");
    }
}

#[test]
fn to_cimtype_maps_each_variant_type() {
    let cases = [
        (VT_BOOL, CIM_BOOLEAN),
        (VT_BSTR, CIM_STRING),
        (VT_I1, CIM_SINT8),
        (VT_UI1, CIM_UINT8),
        (VT_I2, CIM_SINT16),
        (VT_UI2, CIM_UINT16),
        (VT_I4, CIM_SINT32),
        (VT_UI4, CIM_UINT32),
        (VT_I8, CIM_SINT64),
        (VT_UI8, CIM_UINT64),
        (VT_R4, CIM_EMPTY),
        (0, CIM_EMPTY),
    ];
    for (vt, cim) in cases {
        assert_eq!(to_cimtype(vt), cim, "vt {vt}");
    }
}

#[test]
fn invalid_query_code_is_the_wbem_hresult() {
    assert_eq!(WBEM_E_INVALID_QUERY as u32, 0x8004_1017);
}
