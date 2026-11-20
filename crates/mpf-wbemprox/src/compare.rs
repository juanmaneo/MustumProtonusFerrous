//! `eval_strcmp`, `eval_refcmp` and `eval_boolcmp` from `query.c`: the operators of a WQL
//! predicate. A NULL string operand gives 0 before the operator is checked, as in C. An operator
//! that the C code does not handle gives `WBEM_E_INVALID_QUERY`.

use crate::like::eval_like;
use crate::wide::wcsicmp;
use crate::{HRESULT, WBEM_E_INVALID_QUERY};

pub const OP_EQ: u32 = 1;
pub const OP_AND: u32 = 2;
pub const OP_OR: u32 = 3;
pub const OP_GT: u32 = 4;
pub const OP_LT: u32 = 5;
pub const OP_LE: u32 = 6;
pub const OP_GE: u32 = 7;
pub const OP_NE: u32 = 8;
pub const OP_ISNULL: u32 = 9;
pub const OP_NOTNULL: u32 = 10;
pub const OP_LIKE: u32 = 11;
pub const OP_NOT: u32 = 12;

/// An operand of `eval_boolcmp`: an integer, or a string that is compared with `"True"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand<'a> {
    Int(i64),
    Str(&'a [u16]),
}

const TRUE_STR: &[u16] = &[b'T' as u16, b'r' as u16, b'u' as u16, b'e' as u16];

/// The value of a string comparison: `OP_EQ`, `OP_NE`, the ordering operators, or `OP_LIKE`.
pub fn eval_strcmp(op: u32, lstr: Option<&[u16]>, rstr: Option<&[u16]>) -> Result<i64, HRESULT> {
    match (lstr, rstr) {
        (Some(l), Some(r)) => compare_strings(op, l, r),
        _ => Ok(0),
    }
}

/// The value of a reference comparison: `OP_EQ` or `OP_NE`.
pub fn eval_refcmp(op: u32, lstr: Option<&[u16]>, rstr: Option<&[u16]>) -> Result<i64, HRESULT> {
    match (lstr, rstr) {
        (Some(l), Some(r)) => match op {
            OP_EQ | OP_NE => compare_strings(op, l, r),
            _ => Err(WBEM_E_INVALID_QUERY),
        },
        _ => Ok(0),
    }
}

/// The value of a boolean comparison: `OP_EQ` or `OP_NE`. A string operand is -1 when it reads
/// "True" ignoring case, and 0 otherwise.
pub fn eval_boolcmp(op: u32, left: Operand, right: Operand) -> Result<i64, HRESULT> {
    let (l, r) = (bool_value(left), bool_value(right));
    match op {
        OP_EQ => Ok(i64::from(l == r)),
        OP_NE => Ok(i64::from(l != r)),
        _ => Err(WBEM_E_INVALID_QUERY),
    }
}

fn compare_strings(op: u32, l: &[u16], r: &[u16]) -> Result<i64, HRESULT> {
    let order = wcsicmp(l, r);
    match op {
        OP_EQ => Ok(i64::from(order == 0)),
        OP_GT => Ok(i64::from(order > 0)),
        OP_LT => Ok(i64::from(order < 0)),
        OP_LE => Ok(i64::from(order <= 0)),
        OP_GE => Ok(i64::from(order >= 0)),
        OP_NE => Ok(i64::from(order)),
        OP_LIKE => Ok(i64::from(eval_like(l, r))),
        _ => Err(WBEM_E_INVALID_QUERY),
    }
}

fn bool_value(operand: Operand) -> i64 {
    match operand {
        Operand::Int(value) => value,
        Operand::Str(s) => {
            if wcsicmp(s, TRUE_STR) == 0 {
                -1
            } else {
                0
            }
        }
    }
}
